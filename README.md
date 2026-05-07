## Pegui

⚡️ It's an IMGUI-like library which allows developers write beautiful code ⚡️

*🛠 Currently it's in developing 🛠*

## Why?

- ✨ It uses tokio so it fully asynchronous and eats small amount of memory
- 🌧 Very simple app compiled with musl mode (binary includes every dependency so it will work on every system) only takes **1.2** megabytes!
- 📋 This list will be bigger in the future

## Quick start!

```rust
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_6X10}, pixelcolor::BinaryColor};
use linux_embedded_hal::I2cdev;
use log::{error, info};
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use rppal::gpio::Gpio;
use pegui::{App, ButtonTag, Buttons, Colors, Engine, Font, Settings, Ssd1306Display, ui::Ui};

#[tokio::main]
async fn main() {
    let i2c_interface = "/dev/i2c-1".to_string();
    let i2c = I2cdev::new(i2c_interface).expect("Failed to open I2C! Please enable I2C and connect the screen if you did not");
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0
    ).into_buffered_graphics_mode();
    if let Err(e) = display.init() {
        error!("Failed to init the screen! Everything on the screen could be placed with some mistakes or even dont appear. Error: {:?}", e);
    };
    let app_state = AppState { counter: 0 };
    let gpio = Gpio::new().expect("Failed to init GPIO");
    let buttons = [(17, "fourth button"), (22, "third button"), (23, "second button"), (27, "first button")]
        .iter()
        .map(|pin| ButtonTag { pin: gpio.get(pin.0).expect(&format!("Failed to get GPIO pin {} with tag {}", pin.0, pin.1)).into_input_pullup(), tag: pin.1 })
        .collect::<Vec<ButtonTag>>();
    Engine::new(
        Settings { 
            colors: Colors { main: BinaryColor::On, secondary: BinaryColor::Off }, 
            display: Ssd1306Display { display },
            fps: 20,
            fonts: vec![ Font { font: MonoTextStyle::new(&FONT_6X10, BinaryColor::On), tag: "default" } ]
        },
        buttons,
        app_state
    ).await.start_rendering().await;
}

struct AppState {
    counter: u32
}

impl App for AppState {
    async fn update(&mut self, ui: &mut Ui, buttons: Buttons) {
        info!("Buttons state: {:?}", buttons);
        ui.label(format!("Clicks: {}", self.counter), "default").ok();
        if buttons.clicked("fourth button") {
            self.counter += 1;
        }
    }
}
```

## Todo
- 🧊 Add different objects support
- 💃 Add flexbox-like and grid-like something
- ♟ Add bitmap "videos" and just images support
- 📱 Add touchscreen support
- 💾 Add more drivers
- ~❌ Become a millionaire~
- 📋 This list will also be bigger in the future

## 😅 Enjoy the library!
🙏 Please contribute if you can!
