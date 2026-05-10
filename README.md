# Pegui

⚡️ It's an IMGUI-like library which allows developers write beautiful code ⚡️

*🛠 Currently it's in developing 🛠*

# Why?

- ✨ It uses tokio so it fully asynchronous and eats small amount of memory
- 🌧 Very simple app compiled with musl mode and compiled using `cross build --release --target aarch64-unknown-linux-musl` only takes **1.22** megabytes!
- 📋 This list will be bigger in the future

# Quick start!

```rust
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_6X10}, pixelcolor::BinaryColor}; // A library which pegui uses
use linux_embedded_hal::I2cdev; // For I2C connection
use log::{error, info}; // For logging
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*}; // The display driver
use rppal::gpio::Gpio; // For buttons
use pegui::{App, ButtonTag, Buttons, Colors, Engine, Font, Settings, Ssd1306Display, ui::Ui};

#[tokio::main] // The library is asynchronous
async fn main() {
    let i2c_interface = "/dev/i2c-1".to_string();
    let i2c = I2cdev::new(i2c_interface).expect("Failed to open I2C! Please enable I2C and connect the screen if you did not");
    let interface = I2CDisplayInterface::new(i2c); // Creating the I2C connection
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0
    ).into_buffered_graphics_mode(); // You should use into_buffered_graphics_mode()!
    if let Err(e) = display.init() {
        error!("Failed to init the screen! Everything on the screen could be placed with some mistakes or even dont appear. Error: {:?}", e);
    }; // Without init everything on the screen may move to some direction
    let app_state = AppState { counter: 0 };
    let gpio = Gpio::new().expect("Failed to init GPIO");
    let buttons: <Vec<ButtonTag>> = [(17, "fourth button"), (22, "third button"), (23, "second button"), (27, "first button")]
        .iter()
        .map(|pin| ButtonTag { pin: gpio.get(pin.0).expect(&format!("Failed to get GPIO pin {} with tag {}", pin.0, pin.1)).into_input_pullup(), tag: pin.1 })
        .collect::<Vec<ButtonTag>>(); // Initializing buttons
    Engine::new(
        Settings { 
            colors: Colors { main: BinaryColor::On, secondary: BinaryColor::Off }, 
            display: Ssd1306Display { display },
            fps: 20,
            fonts: vec![ Font { font: MonoTextStyle::new(&FONT_6X10, BinaryColor::On), tag: "default" } ]
        },
        buttons,
        app_state
    ).await.start_rendering().await; // Initializing the engine and starting the render
}

struct AppState {
    counter: u32
} // Our app state

impl App for AppState {
    async fn update(&mut self, ui: &mut Ui, buttons: Buttons) { // Library calls this function every 1000 / fps milliseconds 
        info!("Buttons state: {:?}", buttons);
        ui.label(format!("Clicks: {}", self.counter), "default").ok(); // Creating a label with text
        if buttons.clicked("fourth button") { // Checking if the 4th button was clicked
            self.counter += 1;
        }
    }
}
```

# How to get max possible fps
 
To get max possible fps you can use this formula: `ScreenKHz ÷ (ScreenWidth × ScreenHeight × 2)` where `2` are some header bytes and some delays

# Formula example

`400.000` ÷ (`128` × `64` × `2`) = `400.000` ÷ `16384` ~= `24,41 fps` ~= `24 fps`

If you see these warnings (you have to initialize the logger first): `Update took too much! (`some number` ms)`, you should a little decrease fps until warnings gone or just use this formula: `1000` ÷ `number from warning` - `4` where `4` is safety stock

# Todo
- 🧊 Add different objects support
- 💃 Add flexbox-like and grid-like something
- ♟  Add bitmap "videos" and just images support
- 📱 Add touchscreen support
- 💾 Add more drivers
- ~❌ Become a millionaire~
- 📋 This list will also be bigger in the future

## 😅 Enjoy the library!
🙏 Please contribute if you can!
