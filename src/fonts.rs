use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::BinaryColor};

/// A font used for text
#[derive(Debug, Clone, Copy)]
pub struct Font {
    /// Font
    ///
    /// # Example
    ///
    /// ```ignore
    /// MonoTextStyle::new(&FONT_6X10, BinaryColor::On)
    /// ```
    pub font: MonoTextStyle<'static, BinaryColor>,
    /// Font tag
    ///
    /// You should use it to find the font and use it
    ///
    /// # Example how it works
    ///
    /// ```ignore
    /// ui.label(format!("Clicks: {}", self.counter), "default").ok()
    /// ```
    ///
    /// Where `default` is a tag
    pub tag: &'static str,
}

impl Font {
    /// Creates a new Font
    ///
    /// # Example
    ///
    /// ```ignore
    /// Font::new("font tag", MonoTextStyle::new(&FONT_6X10, BinaryColor::On))
    /// ```
    pub fn new(tag: &'static str, font: MonoTextStyle<'static, BinaryColor>) -> Self {
        Self { font, tag }
    }
}
