use embedded_graphics::pixelcolor::BinaryColor;
use linux_embedded_hal::I2cdev;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use pegui::{App, Buttons, Colors, Engine, Settings, Ssd1306Display, Ui};

#[tokio::main]
async fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").expect("Failed to open I2C! Please enable I2C and connect the screen if you did not");
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0
    ).into_buffered_graphics_mode();
    if let Err(e) = display.init() {
        eprintln!("Failed to init the screen! Everything on the screen could be placed with some mistakes or even dont appear. Error: {:?}", e);
    };
    let app_state = AppState {};
    Engine::new( // fonts and buttons are not initialized
        Settings { 
            colors: Colors { main: BinaryColor::On, secondary: BinaryColor::Off }, 
            display: Ssd1306Display { display },
            framerate: 20,
            fonts: Vec::new()
        },
        Vec::new(),
        app_state
    ).await.start_rendering().await;
}

struct AppState {}

impl App for AppState {
    async fn update(&mut self, _ui: &mut Ui, _buttons: &Buttons) {
        // your code here
    }
}