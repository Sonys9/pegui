use embedded_graphics::{geometry::Point, primitives::Rectangle, text::Alignment};
use tokio::sync::mpsc::UnboundedSender;
use crate::{Command, Font, Message, Object, Text, errors::Error};

/// UI struct, used to draw objects
pub struct Ui {
    tx: UnboundedSender<Message>,
    bounding_box: Rectangle,
    fonts: Vec<Font>
}

impl Ui {
    pub(crate) fn new(tx: UnboundedSender<Message>, bounding_box: Rectangle, fonts: Vec<Font>) -> Self {
        Self { tx, bounding_box, fonts }
    }

    /// Used to draw a text from scratch
    /// 
    /// Text example: `{ text: String, position: embedded_graphics::geometry::Point, alignment: embedded_graphics::text::Alignment, font: embedded_graphics::mono_font::MonoTextStyle<'static, BinaryColor> }`
    pub fn text(&mut self, text: Text) {
        self.tx.send(Message { tx: None, command: Command::DrawObject(Object::Text(text)) }).ok();
    }

    /// Used to easily draw a text
    /// 
    /// Example: `ui.label("Hello, world!".to_string(), "default_font").ok()`
    pub fn label(&mut self, text: String, font_tag: &str) -> Result<(), Error> {
        let position = self.bounding_box.top_left + Point::new(0, 10);
        let Some(font) = self.fonts.iter().find(|font| font.tag == font_tag).cloned() else {
            return Err(Error::FailedToGet(format!("Failed to get the font by it's tag: {}", font_tag)));
        };
        self.tx.send(Message { tx: None, command: Command::DrawObject(Object::Text(Text { text: text, position: position, alignment: Alignment::Left, font: font.font }))}).ok();
        Ok(())
    }

    /// Used to get a bounding box as a embedded_graphics::primitives::Rectangle
    pub fn bounding_box(&mut self) -> Rectangle {
        self.bounding_box
    }
}