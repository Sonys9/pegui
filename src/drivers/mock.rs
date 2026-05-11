use crate::DisplayDevice;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    mock_display::MockDisplay,
    pixelcolor::BinaryColor,
};

impl OriginDimensions for Mock {
    fn size(&self) -> Size {
        self.display.size()
    }
}

impl DrawTarget for Mock {
    type Color = BinaryColor;
    type Error = String;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
    {
        DrawTarget::draw_iter(&mut self.display, pixels).map_err(|e| format!("{:?}", e))
    }
}

/// MockDisplay
///
/// Used only for tests
pub struct Mock {
    /// The MockDisplay
    ///
    /// Currently supports only BinaryColor
    pub display: MockDisplay<BinaryColor>,
}

impl DisplayDevice for Mock {
    type Error = String;
    fn flush(&mut self) -> Result<(), Self::Error> {
        // We don't need that
        Ok(())
    }

    fn is_monochrome(&self) -> bool {
        // it's always monochrome
        true
    }

    fn affected_area(&self) -> Option<embedded_graphics::primitives::Rectangle> {
        Some(self.display.affected_area())
    }
}
