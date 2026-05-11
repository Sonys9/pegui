#![no_std]
#![warn(missing_docs)]
//! # Pi Easy GUI (Pegui)
//!
//! It's a simple GUI library for screens working on popular drivers like Ssd1306
//!
//! It also support GPIO buttons but doesn't support touchscreen
//!
//! # Currently supported drivers
//!
//! Ssd1306
//!
//! ## Quick start
//!
//! ```rust,no_run
//! # use std::env;
//! # use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_6X10}, pixelcolor::BinaryColor};
//! # use linux_embedded_hal::I2cdev;
//! # use log::{error, info};
//! # use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
//! # use rppal::gpio::Gpio;
//! # use pegui::{App, ButtonTag, Buttons, Colors, Engine, Font, Settings, Ssd1306Display, Ui, errors::Error};
//! #[tokio::main]
//! async fn main() {
//!     # let i2c_interface = "/dev/i2c-1".to_string();
//!     let i2c = I2cdev::new(i2c_interface).expect("Failed to open I2C! Please enable I2C and connect the screen if you did not");
//!     let interface = I2CDisplayInterface::new(i2c);
//!     let mut display = Ssd1306::new(
//!         interface,
//!         DisplaySize128x64,
//!         DisplayRotation::Rotate0
//!     ).into_buffered_graphics_mode();
//!     if let Err(e) = display.init() {
//!         error!("Failed to init the screen! Everything on the screen could be placed with some mistakes or even dont appear. Error: {:?}", e);
//!     };
//!     let app_state = AppState { counter: 0 };
//!     let gpio = Gpio::new().expect("Failed to init GPIO");
//!     let buttons: Vec<ButtonTag> = [(17, "fourth button"), (22, "third button"), (23, "second button"), (27, "first button")]
//!         .iter()
//!         .map(|pin| ButtonTag { pin: gpio.get(pin.0).expect(&format!("Failed to get GPIO pin {} with tag {}", pin.0, pin.1)).into_input_pullup(), tag: pin.1 })
//!         .collect();
//!     Engine::new(
//!         Settings {
//!             colors: Colors { main: BinaryColor::On, secondary: BinaryColor::Off },
//!             display: Ssd1306Display { display },
//!             framerate: 20,
//!             fonts: vec![ Font::new("default", MonoTextStyle::new(&FONT_6X10, BinaryColor::On)) ]
//!         },
//!         buttons,
//!         app_state
//!     ).await.start_rendering(true).await; // true is for clearing the screen before every update call
//! }
//!
//! struct AppState {
//!     counter: u32
//! }
//!
//! impl App for AppState {
//!     async fn update(&mut self, ui: &mut Ui, buttons: &Buttons) -> Result<(), Error> { // Error is pegui::errors::Error
//!         info!("Buttons state: {:?}", buttons);
//!         ui.label(format!("Clicks: {}", self.counter), "default").await.ok();
//!         if buttons.clicked("fourth button") {
//!             self.counter += 1;
//!         };
//!         Ok(())
//!     }
//! }
//! ```

use embedded_graphics::{
    Drawable,
    geometry::Size,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::{Circle, PrimitiveStyle, Rectangle, Styled, Triangle},
    text::{self, Alignment},
};
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use embassy_sync::mutex::Mutex;
use static_cell::StaticCell;
use log::{debug, error, warn};
/// This module is used to interact with buttons
pub mod buttons;
/// This module is used to connect the display
pub mod display_device;
/// A module with every supported driver
pub mod drivers;
/// This module is used for errors
pub mod errors;
/// This module is used for fonts
pub mod fonts;
/// An UI module used for creating elements on the screen
pub mod ui;
pub use crate::buttons::{Button, ButtonTag, Buttons};
pub use crate::display_device::DisplayDevice;
pub use crate::drivers::ssd1306::Ssd1306Display;
use crate::errors::Error;
pub use crate::fonts::Font;
pub use crate::ui::Ui;

static SHARED_BUTTONS_STATE: StaticCell<Mutex<CriticalSectionRawMutex, Command>> = StaticCell::new();

/// A structure used for creating a text
///
/// Arguments:
///
///  - `text` - String
///
///  - `position` - Point from `embedded_graphics_core::geometry::point`: struct `{ x: i32, y: i32 }`,
///
///  - `alignment` - Alignment from `embedded_graphics::text::Alignment`: enum `{ Left, Center, Right }`, works like CSS alignment,
///
///  - `font` - MonoTextStyle font from `embedded_graphics::mono_font::mono_text_style`, currently only supports BinaryColor
///
/// # Example
///
/// ```ignore
/// Text {
///     text: "Hello, world!".to_string(),
///     position: Point { x: 10, y: 10 },
///     alignment: Alignment::Left,
///     font: MonoTextStyle::new(&FONT_6X10, BinaryColor::On)
/// }
/// ```
#[derive(Debug)]
pub struct Text {
    /// Text
    pub text: &'static str,
    /// Text position, it's a point from `embedded_graphics_core::geometry::point`: struct `{ x: i32, y: i32 }`
    pub position: Point,
    /// Text alignment, it' an alignment from `embedded_graphics::text::Alignment`: enum `{ Left, Center, Right }`, works like CSS alignment
    pub alignment: Alignment,
    /// Text font, it's a MonoTextStyle font from `embedded_graphics::mono_font::mono_text_style`, currently only supports BinaryColor
    pub font: MonoTextStyle<'static, BinaryColor>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Object {
    Rectangle(Styled<Rectangle, PrimitiveStyle<BinaryColor>>),
    Triangle(Triangle),
    Circle(Circle),
    Text(Text),
}

#[derive(Debug)]
enum Command {
    DrawObject(Object),
    Flush,
    Clear(BinaryColor),
    GetAffectedArea(oneshot::Sender<Option<Rectangle>>),
}

/// Display info
///
/// # Example
///
/// ```ignore
/// Display {
///     width: 128,
///     height: 64,
///     bounding_box: embedded_graphics::primitives::Rectangle {
///         top_left: embedded_graphics::geometry::Point {
///             x: 0,
///             y: 0
///         },
///         size: embedded_graphics::geometry::Size {
///             width: 128,
///             height: 64
///         }
///     },
///     is_monochrome: true, // if it only supports 2 colors like black and white or blue and yellow
///     framerate: 20 // frames per second (or just FPS)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Display {
    /// Display size (a copy from bounding box)
    ///
    /// # Example
    ///
    /// ```ignore
    /// embedded_graphics::geometry::Size {
    ///     width: 128,
    ///     height: 64
    /// }
    /// ```
    pub size: Size,
    /// Bounding box
    ///
    /// # Example
    ///
    /// ```ignore
    /// embedded_graphics::primitives::Rectangle {
    ///     top_left: embedded_graphics::geometry::Point {
    ///         x: 0,
    ///         y: 0
    ///     },
    ///     size: embedded_graphics::geometry::Size {
    ///         width: 128,
    ///         height: 64
    ///     }
    /// }
    /// ```
    pub bounding_box: Rectangle,
    /// Tells you if the screen is monochrome
    pub is_monochrome: bool,
    /// Framerate
    pub framerate: u8,
}

/// Colors: `BinaryColor::On` or `BinaryColor::Off`
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    /// The color used for objects
    pub main: BinaryColor,
    /// The color used for background and other not main stuff
    pub secondary: BinaryColor,
}

/// Settings for an Engine
pub struct Settings<D: DisplayDevice> {
    /// Colors (struct Colors)
    ///
    /// # Example
    ///
    /// ```ignore
    /// Colors {
    ///     main: BinaryColor::On,
    ///     secondary: BinaryColor::off
    /// }
    /// ```
    pub colors: Colors,
    /// Display
    ///
    /// # Example
    ///
    /// ```ignore
    /// Ssd1306Display {
    ///     Ssd1306::new(...blablabla...).into_buffered_graphics_mode()
    /// }
    /// ```
    pub display: D,
    /// Framerate (FPS)
    ///
    /// # Example
    ///
    /// `20`
    ///
    /// # How to get max possible framerate
    ///
    /// To get max possible framerate you can use this formula: `BusSpeed(KHz) ÷ (ScreenWidth × ScreenHeight × 2)` where `2` are some header bytes and some delays
    ///
    /// # Formula example
    ///
    /// `400.000` ÷ (`128` × `64` × `2`) = `400.000` ÷ `16384` ~= `24,41 fps` ~= `24 fps`
    ///
    /// If you see these warnings (you have to initialize the logger first): `Update took too much! (some number ms)`, you should a little decrease your framerate until warnings gone or just use this formula: `1000` ÷ `number from warning`
    pub framerate: u8,
    /// Fonts
    ///
    /// # Example
    ///
    /// ```ignore
    /// vec![
    ///     Font {
    ///         font: MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
    ///         tag: "default"
    ///     },
    ///     ...
    /// ]
    /// ```
    ///
    /// You will able to use them by using a tag
    pub fonts: Vec<Font>,
}

/// The gui engine
pub struct Engine<A: App> {
    tx: Sender<Command>,
    delay: Duration,
    app: A,
    colors: Colors,
    fonts: HashMap<&'static str, MonoTextStyle<'static, BinaryColor>>,
    buttons: Arc<Mutex<Arc<Buttons>>>,
    display: Display,
    tasks: Vec<JoinHandle<()>>,
}

/// App trait
pub trait App {
    /// App update function
    ///
    /// Asynchronous!!!
    fn update(
        &mut self,
        ui: &mut Ui,
        buttons: &Buttons,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

impl<A: App> Engine<A> {
    /// Creates new Engine
    ///
    /// You should provide settings, buttons and an app state
    pub async fn new<
        D: DisplayDevice + DrawTarget<Color = BinaryColor> + std::marker::Send + 'static,
    >(
        mut settings: Settings<D>,
        buttons: Vec<ButtonTag>,
        app: A,
    ) -> Self {
        let bounding_box = settings.display.bounding_box();
        let is_monochrome = settings.display.is_monochrome();
        let delay = Duration::from_millis(1000 / settings.framerate as u64);

        let channel = Channel::new();
        let display_task = tokio::spawn(async move );

        let buttons_task = tokio::spawn({
            let shared_buttons_state = Arc::clone(&shared_buttons_state);
            async move {
                loop {
                    let buttons_state = Arc::new(Buttons {
                        buttons: buttons
                            .iter()
                            .map(|button| Button {
                                pin: button.pin.pin(),
                                tag: button.tag,
                                holded: button.pin.is_low(),
                                clicked: false,
                            })
                            .collect(),
                    });
                    {
                        *shared_buttons_state.lock().await = buttons_state;
                    };
                    sleep(delay).await;
                }
            }
        });

        Self {
            tx,
            app,
            delay,
            colors: settings.colors,
            fonts: Self::vec_to_hashmap(settings.fonts),
            buttons: shared_buttons_state,
            display: Display {
                size: bounding_box.size,
                bounding_box,
                is_monochrome,
                framerate: settings.framerate,
            },
            tasks: vec![display_task, buttons_task],
        }
    }
    
    async fn display_loop() {
        while let Some(command) = rx.recv().await {
            debug!("Got command: {:?}", command);
            match command {
                Command::DrawObject(object) => Self::draw_object(object, &mut settings.display),
                Command::Flush => {
                    settings.display.flush().ok();
                }
                Command::Clear(color) => {
                    settings.display.clear(color).ok();
                }
                Command::GetAffectedArea(tx) => {
                    tx.send(settings.display.affected_area()).ok();
                }
            }
        }
    }

    fn draw_object<
        D: DisplayDevice + DrawTarget<Color = BinaryColor> + std::marker::Send + 'static,
    >(
        object: Object,
        display: &mut D,
    ) {
        match object {
            Object::Text(text) => {
                text::Text::with_alignment(&text.text, text.position, text.font, text.alignment)
                    .draw(display)
                    .ok();
            }
            Object::Rectangle(rectangle) => {
                rectangle.draw(display).ok();
            }
            _ => {}
        };
    }

    #[allow(dead_code)]
    fn send_response(sender: Option<oneshot::Sender<Command>>, message: Command) {
        if let Some(sender) = sender {
            debug!("Sending {:?}", message);
            sender.send(message).ok();
        };
    }

    fn vec_to_hashmap(
        fonts: Vec<Font>,
    ) -> HashMap<&'static str, MonoTextStyle<'static, BinaryColor>> {
        fonts
            .into_iter()
            .map(|font| (font.tag, font.font))
            .collect()
    }

    /// Instantly kills every task in the background
    pub fn exit(&self) {
        for task in &self.tasks {
            task.abort();
        }
    }

    /// Starts a loop which calls update every `1000 / fps`
    ///
    /// It also clears the screen (if clear = true) and flushes it
    pub async fn start_rendering(&mut self, clear: bool) {
        let mut ui = Ui {
            tx: self.tx.clone(),
            fonts: self.fonts.clone(),
            display: self.display,
        };
        let mut last_buttons_state: Option<Buttons> = None;
        loop {
            let start_time = Instant::now();
            let arc_buttons = {
                let mutex = self.buttons.lock().await;
                Arc::clone(&*mutex)
            };
            let buttons = if let Some(last_buttons_state) = last_buttons_state {
                Buttons {
                    buttons: arc_buttons
                        .buttons
                        .iter()
                        .map(|button| {
                            let mut button = *button;
                            if button.holded && !last_buttons_state.pin_holded(button.pin) {
                                button.clicked = true
                            };
                            button
                        })
                        .collect(),
                }
            } else {
                arc_buttons.copy()
            };
            last_buttons_state = Some(buttons.copy());
            if clear {
                self.tx
                    .send(Command::Clear(self.colors.secondary))
                    .await
                    .ok();
            };
            if let Err(e) = self.app.update(&mut ui, &buttons).await {
                error!("Got error after update: {}. Exiting.", e);
                self.exit();
                return;
            };
            self.tx.send(Command::Flush).await.ok();
            let update_time = Instant::now() - start_time;
            if update_time > self.delay {
                warn!("Update took too much! ({} ms)", update_time.as_millis());
                continue;
            };
            debug!("Update took {} ms", update_time.as_millis());
            sleep(self.delay - update_time).await;
        }
    }
}
