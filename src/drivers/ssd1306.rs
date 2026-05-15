use crate::DisplayDevice;
use alloc::{format, string::String};
use display_interface::{AsyncWriteOnlyDataCommand};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
};
use ssd1306::{Ssd1306Async, mode::BufferedGraphicsModeAsync, size::DisplaySizeAsync};

impl<T: DisplaySizeAsync, DI> OriginDimensions for Ssd1306Display<T, DI> where DI: AsyncWriteOnlyDataCommand {
    fn size(&self) -> Size {
        self.display.size()
    }
}

impl<T: DisplaySizeAsync, DI> DrawTarget for Ssd1306Display<T, DI> where DI: AsyncWriteOnlyDataCommand {
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
pub struct Ssd1306Display<T: DisplaySizeAsync, DI> {
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
    pub display: Ssd1306Async<DI, T, BufferedGraphicsModeAsync<T>>
}

impl<T: DisplaySizeAsync, DI> DisplayDevice for Ssd1306Display<T, DI> where DI: AsyncWriteOnlyDataCommand {
    type Error = String;
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.display.flush().await.map_err(|e| format!("{:?}", e))
    }

    fn is_monochrome(&self) -> bool {
        // it's always monochrome
        true
    }

    fn affected_area(&self) -> Option<embedded_graphics::primitives::Rectangle> {
        None
    }
}
