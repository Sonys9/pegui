use crate::DisplayDevice;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
};
use linux_embedded_hal::I2cdev;
use ssd1306::{Ssd1306, prelude::I2CInterface};

impl<T: ssd1306::prelude::DisplaySize> OriginDimensions for Ssd1306Display<T> {
    fn size(&self) -> Size {
        self.display.size()
    }
}

impl<T: ssd1306::prelude::DisplaySize> DrawTarget for Ssd1306Display<T> {
    type Color = BinaryColor;
    type Error = String;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
    {
        DrawTarget::draw_iter(&mut self.display, pixels).map_err(|e| format!("{:?}", e))
    }
}

/// Used for ssd1306 based displays
///
/// # Example
/// ```ignore
/// Ssd1306Display {
///     Ssd1306::new(
///         interface,
///         DisplaySize128x64,
///         DisplayRotation::Rotate0
///     ).into_buffered_graphics_mode()
/// }
/// ```
pub struct Ssd1306Display<T: ssd1306::prelude::DisplaySize> {
    /// The display
    ///
    /// # Example
    /// ```ignore
    /// Ssd1306::new(
    ///     interface,
    ///     DisplaySize128x64,
    ///     DisplayRotation::Rotate0
    /// ).into_buffered_graphics_mode()
    /// ```
    pub display: Ssd1306<I2CInterface<I2cdev>, T, ssd1306::mode::BufferedGraphicsMode<T>>,
}

impl<T: ssd1306::prelude::DisplaySize> DisplayDevice for Ssd1306Display<T> {
    type Error = String;
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.display.flush().map_err(|e| format!("{:?}", e))
    }

    fn is_monochrome(&self) -> bool {
        // it's always monochrome
        true
    }

    fn affected_area(&self) -> Option<embedded_graphics::primitives::Rectangle> {
        None
    }
}
