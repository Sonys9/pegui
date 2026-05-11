use embedded_graphics::{
    geometry::{Point, Size},
    mock_display::MockDisplay,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    primitives::{Primitive, PrimitiveStyleBuilder, Rectangle},
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
        let expected_width = 25;
        let expected_height = 25;
        match self.frame_number {
            0 => {
                let style = PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(1)
                    .fill_color(BinaryColor::Off)
                    .build();

                let rectangle =
                    Rectangle::new(Point::new(0, 0), Size::new(expected_width, expected_height))
                        .into_styled(style);

                ui.rectangle(rectangle).await.ok();
            }
            _ => {
                let affected_area = ui
                    .affected_area()
                    .await
                    .expect("Failed to get affected area");
                assert_eq!(affected_area.size.height, expected_height);
                assert_eq!(affected_area.size.width, expected_width);
                return Err(Error::End(()));
            }
        };
        self.frame_number += 1;
        Ok(())
    }
}
