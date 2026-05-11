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
        AppState { frame_number: 0 },
    )
    .await
    .start_rendering(false)
    .await;
}

struct AppState {
    frame_number: u32,
}

impl App for AppState {
    async fn update(&mut self, ui: &mut Ui, _buttons: &Buttons) -> Result<(), Error> {
        let text = "Test".to_string();
        let expected_width = 23;
        let expected_height = 7;
        match self.frame_number {
            0 => {
                ui.label(text, "default").await.ok();
            }
            _ => {
                let affected_area = ui
                    .affected_area()
                    .await
                    .expect("Failed to get affected area");
                assert_eq!(affected_area.size.height, expected_height);
                assert_eq!(affected_area.size.width as usize, expected_width);
                return Err(Error::End(()));
            }
        };
        self.frame_number += 1;
        Ok(())
    }
}
