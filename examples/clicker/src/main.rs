use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
};
use linux_embedded_hal::I2cdev;
use log::error;
use pegui::{App, ButtonTag, Buttons, Colors, Engine, Font, Settings, Ssd1306Display, Ui};
use rppal::gpio::Gpio;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

#[tokio::main]
async fn main() {
    let i2c = I2cdev::new("/dev/i2c-1")
        .expect("Failed to open I2C! Please enable I2C and connect the screen if you did not");
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    if let Err(e) = display.init() {
        error!(
            "Failed to init the screen! Everything on the screen could be placed with some mistakes or even dont appear. Error: {:?}",
            e
        );
    };
    let app_state = AppState { counter: 0 };
    let gpio = Gpio::new().expect("Failed to init GPIO");
    let buttons: Vec<ButtonTag> = [
        (17, "fourth button"),
        (22, "third button"),
        (23, "second button"),
        (27, "first button"),
    ]
    .iter()
    .map(|pin| ButtonTag {
        pin: gpio
            .get(pin.0)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to get GPIO pin {} with tag {} got error: {}",
                    pin.0, pin.1, e
                )
            })
            .into_input_pullup(),
        tag: pin.1,
    })
    .collect();
    Engine::new(
        Settings {
            colors: Colors {
                main: BinaryColor::On,
                secondary: BinaryColor::Off,
            },
            display: Ssd1306Display { display },
            framerate: 20,
            fonts: vec![Font::new(
                "default",
                MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
            )],
        },
        buttons,
        app_state,
    )
    .await
    .start_rendering()
    .await;
}

struct AppState {
    counter: u32,
}

impl App for AppState {
    async fn update(&mut self, ui: &mut Ui, buttons: &Buttons) {
        ui.label(format!("Clicks: {}", self.counter), "default")
            .await
            .ok();
        if buttons.clicked("fourth button") {
            self.counter += 1;
        };
    }
}
