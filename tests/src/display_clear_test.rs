use embedded_graphics::{
    mock_display::MockDisplay,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
};
use pegui::{App, Buttons, Colors, Engine, Font, Settings, Ui, drivers::mock::Mock, errors::Error};

#[tokio::test]
async fn test() {
    let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    display.set_allow_overdraw(true);
    Engine::new(
        // buttons are not initialized
        Settings {
            colors: Colors {
                main: BinaryColor::On,
                secondary: BinaryColor::Off,
            },
            display: Mock { display },
            framerate: 60,
            fonts: vec![Font::new(
                "default",
                MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
            )],
        },
        Vec::new(),
        AppState {},
    )
    .await
    .start_rendering(true)
    .await;
}

struct AppState {}

impl App for AppState {
    async fn update(&mut self, ui: &mut Ui, _buttons: &Buttons) -> Result<(), Error> {
        // The library clears the screen before calling update
        let affected_area = ui
            .affected_area()
            .await
            .expect("Failed to get affected area");
        assert_eq!(affected_area.size.width, 64);
        assert_eq!(affected_area.size.height, 64);
        Err(Error::End(()))
    }
}
