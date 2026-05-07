<!DOCTYPE HTML>
<body>
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github.min.css">
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
  <script>hljs.highlightAll();</script>
  
  <h1>Pegui</h1>
  <p>⚡️ It's an IMGUI-like library which allows developers write beautiful code ⚡️</p>
  <i>🛠 Currently it's in developing 🛠</i>
  <h1>Why?</h1>
  <ul>
    <li>✨ It uses tokio so it fully asynchronous and eats small amount of memory</li>
    <li>🌧 Very simple app compiled with musl mode (binary includes every dependency so it will work on every system) only takes <b>1.2</b> megabytes!</li>
    <li>📋 This list will be bigger in the future</li>
  </ul>
  <h1>Quick start!</h1>
<pre><code class="language-rust">
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
</code></pre>
  <h1>Todo</h1>
  <ul>
    <li>🧊 Add different objects support</li>
    <li>💃 Add flexbox-like and grid-like something</li>
    <li>♟  Add bitmap "videos" and just images support</li>
    <li>📱 Add touchscreen support</li>
    <li>💾 Add more drivers</li>
    <s><li>❌ Become a millionaire</li></s>
    <li>📋 This list will also be bigger in the future</li>
  </ul>
  <h1>😅 Enjoy the library!</h1>
  <p>🙏 Please contribute if you can!</p>
</body>
