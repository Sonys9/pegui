use crate::{Command, Display, Object, Text, errors::Error};
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyle, Rectangle, Styled},
    text::Alignment,
};
use std::collections::HashMap;
use tokio::sync::{mpsc::Sender, oneshot};

/// UI struct, used to draw objects
pub struct Ui {
    pub(crate) tx: Sender<Command>,
    /// Fonts as hashmap
    ///
    /// You can find needed font like this:
    ///
    /// # Example
    ///
    /// ```ignore
    /// if Some(font) = fonts.get("my_very_cool_font") {
    ///     // here you do something
    /// } else {
    ///     // happens if the tag is wrong
    /// };
    /// ```
    pub fonts: HashMap<&'static str, MonoTextStyle<'static, BinaryColor>>,
    /// Display info
    ///
    /// Uses Display structure
    pub(crate) display: Display,
}

impl Ui {
    /// Used to draw a text from scratch
    ///
    /// # Text example
    /// ```ignore
    /// {
    ///     text: String,
    ///     position: embedded_graphics::geometry::Point,
    ///     alignment: embedded_graphics::text::Alignment,
    ///     font: embedded_graphics::mono_font::MonoTextStyle<'static, BinaryColor>
    /// }
    /// ```
    pub async fn text(&mut self, text: Text) -> Result<(), Error> {
        self.tx
            .send(Command::DrawObject(Object::Text(text)))
            .await
            .map_err(|e| {
                Error::SendError(format!("Failed to send the text to other thread: {}", e))
            })?;
        Ok(())
    }

    /// Used to easily draw a text
    ///
    /// # Example
    /// ```ignore
    /// ui.label("Hello, world!".to_string(), "default_font").ok()
    /// ```
    pub async fn label(&mut self, text: String, font_tag: &str) -> Result<(), Error> {
        let position = self.display.bounding_box.top_left + Point::new(0, 10);
        let Some(&font) = self.fonts.get(font_tag) else {
            return Err(Error::FailedToGet(format!(
                "Failed to get the font by it's tag: {}",
                font_tag
            )));
        };
        self.tx
            .send(Command::DrawObject(Object::Text(Text {
                text,
                position,
                alignment: Alignment::Left,
                font,
            })))
            .await
            .map_err(|e| {
                Error::SendError(format!("Failed to send the text to other thread: {}", e))
            })?;
        Ok(())
    }

    /// Used to draw a rectangle from scratch
    ///
    /// # Example
    /// ```ignore
    /// let style = PrimitiveStyleBuilder::new()
    ///     .stroke_color(BinaryColor::On)
    ///     .stroke_width(3)
    ///     .fill_color(BinaryColor::Off)
    ///     .build(); // Create a style for the rectangle using embedded_graphics
    ///
    /// let rectangle = Rectangle::new(Point::new(30, 20), Size::new(10, 15))
    ///     .into_styled(style); // Create the rectangle using embedded_graphics
    ///
    /// ui.rectangle(rectangle).await.ok(); // Send it to UI
    /// ```
    pub async fn rectangle(
        &mut self,
        rectangle: Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
    ) -> Result<(), Error> {
        self.tx
            .send(Command::DrawObject(Object::Rectangle(rectangle)))
            .await
            .map_err(|e| {
                Error::SendError(format!(
                    "Failed to send the rectangle to other thread: {}",
                    e
                ))
            })?;
        Ok(())
    }

    /// Returns affected area
    ///
    /// Only works with MockDisplay!
    pub async fn affected_area(&mut self) -> Result<Rectangle, Error> {
        let (channel_tx, channel_rx) = oneshot::channel::<Option<Rectangle>>();
        self.tx
            .send(Command::GetAffectedArea(channel_tx))
            .await
            .map_err(|e| {
                Error::SendError(format!("Failed to send the command to other thread: {}", e))
            })?;
        match channel_rx.await.map_err(|e| {
            Error::ReceiveError(format!("Failed to receive the affected area: {}", e))
        })? {
            Some(area) => Ok(area),
            None => Err(Error::ReceiveError(
                "Failed to receive the affected area (got nothing)".to_string(),
            )),
        }
    }
}
