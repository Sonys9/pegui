use embedded_graphics::primitives::Rectangle;

/// The trait used for all supported displays
pub trait DisplayDevice {
    /// Error, `String`
    type Error;
    /// Flushes (or updates) the screen
    async fn flush(&mut self) -> Result<(), Self::Error>;
    /// Shows if the display is monochrome
    fn is_monochrome(&self) -> bool;
    /// Returns affected area as embedded_graphics::primitives::Rectangle
    fn affected_area(&self) -> Option<Rectangle>;
}
