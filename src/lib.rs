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
//! # use pegui::{App, ButtonTag, Buttons, Colors, Engine, Font, Settings, Ssd1306Display, Ui};
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
//!     let buttons = [(17, "fourth button"), (22, "third button"), (23, "second button"), (27, "first button")]
//!         .iter()
//!         .map(|pin| ButtonTag { pin: gpio.get(pin.0).expect(&format!("Failed to get GPIO pin {} with tag {}", pin.0, pin.1)).into_input_pullup(), tag: pin.1 })
//!         .collect::<Vec<ButtonTag>>();
//!     Engine::new(
//!         Settings { 
//!             colors: Colors { main: BinaryColor::On, secondary: BinaryColor::Off }, 
//!             display: Ssd1306Display { display },
//!             fps: 20,
//!             fonts: vec![ Font::new("default", MonoTextStyle::new(&FONT_6X10, BinaryColor::On)) ]
//!         },
//!         buttons,
//!         app_state
//!     ).await.start_rendering().await;
//! }
//! 
//! struct AppState {
//!     counter: u32
//! }
//! 
//! impl App for AppState {
//!     async fn update(&mut self, ui: &mut Ui, buttons: &Buttons) {
//!         info!("Buttons state: {:?}", buttons);
//!         ui.label(format!("Clicks: {}", self.counter), "default").await.ok();
//!         if buttons.clicked("fourth button") {
//!             self.counter += 1;
//!         }
//!     }
//! }
//! ```

use std::{collections::HashMap, sync::Arc};
use tokio::{sync::Mutex, time::{Duration, Instant, sleep}};
use embedded_graphics::{Drawable, mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::{DrawTarget, Point}, primitives::{Circle, Primitive, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Styled, StyledDrawable, Triangle}, text::{self, Alignment}};
use log::{error, debug, warn};
use tokio::sync::{mpsc::{self, Sender}, oneshot};
/// An UI module used for creating elements on the screen
pub mod ui;
/// This module is used to connect the display
pub mod display_device;
/// This module is used to interact with buttons
pub mod buttons;
/// This module is used for errors
pub mod errors;
/// This module is used for fonts
pub mod fonts;
pub use crate::buttons::{Button, ButtonTag, Buttons};
pub use crate::display_device::{DisplayDevice, Ssd1306Display};
pub use crate::ui::Ui;
pub use crate::fonts::Font;
use crate::errors::Error;

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
    pub text: String,
    /// Text position, it's a point from `embedded_graphics_core::geometry::point`: struct `{ x: i32, y: i32 }`
    pub position: Point,
    /// Text alignment, it' an alignment from `embedded_graphics::text::Alignment`: enum `{ Left, Center, Right }`, works like CSS alignment
    pub alignment: Alignment,
    /// Text font, it's a MonoTextStyle font from `embedded_graphics::mono_font::mono_text_style`, currently only supports BinaryColor
    pub font: MonoTextStyle<'static, BinaryColor>
}

#[derive(Debug)]
enum Object {
    Rectangle(Styled<Rectangle, PrimitiveStyle<BinaryColor>>),
    Triangle(Triangle),
    Circle(Circle),
    Text(Text)
}

#[derive(Debug)]
enum Command {
    DrawObject(Object),
    Flush,
    Clear(BinaryColor)
}

#[derive(Debug)]
struct Message {
    tx: Option<oneshot::Sender<Command>>,
    command: Command
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
    /// Display width
    pub width: u32,
    /// Display height
    pub height: u32,
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
    pub framerate: u8
}

/// Colors: `BinaryColor::On` or `BinaryColor::Off`
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    /// The color used for objects
    pub main: BinaryColor,
    /// The color used for background and other not main stuff
    pub secondary: BinaryColor
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
    /// To get max possible framerate you can use this formula: `ScreenKHz ÷ (ScreenWidth × ScreenHeight × 2)` where `2` are some header bytes and some delays
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
    pub fonts: Vec<Font>
}

/// The gui engine
pub struct Engine<A: App> {
    tx: Sender<Message>,
    delay: Duration,
    app: A,
    colors: Colors,
    bounding_box: Rectangle,
    fonts: HashMap<&'static str, MonoTextStyle<'static, BinaryColor>>,
    buttons: Arc<Mutex<Arc<Buttons>>>,
    framerate: u8
}

/// App trait
pub trait App {
    /// App update function
    /// 
    /// Asynchronous!!!
    fn update(&mut self, ui: &mut Ui, buttons: &Buttons) -> impl std::future::Future<Output = ()> + Send;
}

impl<A: App> Engine<A> {
    /// Creates new Engine
    /// 
    /// You should provide settings, buttons and an app state
    pub async fn new<D: DisplayDevice + std::marker::Send + 'static>(mut settings: Settings<D>, buttons: Vec<ButtonTag>, app: A) -> Self 
    where D: DrawTarget<Color = BinaryColor> {
        let bounding_box = settings.display.bounding_box();
        let delay = Duration::from_millis(1000 / settings.framerate as u64);
        let shared_buttons_state = Arc::new(Mutex::new(Arc::new(Buttons::default())));
        let draw_object = move |object: Object, display: &mut D| {
            match object {
                Object::Text(text) => { text::Text::with_alignment(&text.text, text.position, text.font, text.alignment).draw(display).ok(); },
                Object::Rectangle(rectangle) => { rectangle.draw(display).ok(); },
                _ => {}
            };
        };
        

        let (tx, mut rx) = mpsc::channel::<Message>(3);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                debug!("Got message: {:?}", message);
                match message.command {
                    Command::DrawObject(object) => draw_object(object, &mut settings.display),
                    Command::Flush => { settings.display.flush().ok(); },
                    Command::Clear(color) => { settings.display.clear(color).ok(); },
                    _ => {}
                }
            };
        });

        tokio::spawn({
            let shared_buttons_state = Arc::clone(&shared_buttons_state);
            async move {
                loop {
                    let buttons_state = Arc::new(Buttons { buttons: buttons.iter()
                        .map(|button| Button { pin: button.pin.pin(), tag: button.tag, holded: button.pin.is_low(), clicked: false })
                        .collect()
                    });
                    {
                        *shared_buttons_state.lock().await = buttons_state;
                    };
                    sleep(delay).await;
                }
            }}
        );

        Self { 
            tx: tx, 
            app, delay, 
            colors: settings.colors, 
            bounding_box: bounding_box, 
            fonts: Self::vec_to_hashmap(settings.fonts), 
            buttons: shared_buttons_state, 
            framerate: settings.framerate 
        }
    }

    fn send_response<'a>(sender: Option<oneshot::Sender<Command>>, message: Command) {
        if let Some(sender) = sender {
            debug!("Sending {:?}", message);
            sender.send(message).ok();
        };
    }

    fn vec_to_hashmap(fonts: Vec<Font>) -> HashMap<&'static str, MonoTextStyle<'static, BinaryColor>> {
        fonts.into_iter().map(|font| (font.tag, font.font)).collect()
    }

    /// Starts a loop which calls update every `1000 / fps`
    /// 
    /// It also clears the screen and flushes it
    pub async fn start_rendering(&mut self) {
        let mut ui = Ui { 
            tx: self.tx.clone(), 
            fonts: self.fonts.clone(),
            display: Display { 
                width: self.bounding_box.columns().end as u32, 
                height: self.bounding_box.rows().end as u32, 
                is_monochrome: true, 
                framerate: self.framerate, 
                bounding_box: self.bounding_box 
            }
        };
        let mut last_buttons_state: Option<Buttons> = None;
        loop {
            let start_time = Instant::now();
            self.tx.send(Message { tx: None, command: Command::Clear(self.colors.secondary) }).await.ok();
            /*let mut buttons = match self.get_buttons_state().await {
                Ok(buttons) => buttons,
                Err(e) => {
                    error!("Failed to get buttons, got error: {:?}. Returning empty Buttons", e);
                    Buttons::default()
                }
            };*/
            let arc_buttons = { 
                let mutex = self.buttons.lock().await;
                Arc::clone(&*mutex)
            };
            let buttons = if let Some(last_buttons_state) = last_buttons_state {
                Buttons { buttons: 
                    arc_buttons.buttons
                        .iter()
                        .map(|button| {
                            let mut button = *button;
                            if button.holded && !last_buttons_state.pin_holded(button.pin) { 
                                button.clicked = true
                            }; 
                            button 
                        } )
                        .collect() 
                }
            } else {
                arc_buttons.copy()
            };
            last_buttons_state = Some(buttons.copy());
            self.app.update(&mut ui, &buttons).await;
            self.tx.send(Message { tx: None, command: Command::Flush }).await.ok();
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