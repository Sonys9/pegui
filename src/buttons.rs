use alloc::vec::Vec;

/// Button with a tag
pub struct ButtonTag {
    /// Tag, `&'static str`
    pub tag: &'static str,
    /// Pin, `InputPin` from `rppal::gpio::InputPin`
    ///
    /// # Example to get the pin
    /// ```ignore
    /// gpio.get(17).unwrap().into_input_pullup()
    /// ```
    ///
    /// Always call `.into_input_pullup()` so it will work!
    pub pin: InputPin,
}

/// A button
#[derive(Debug, Clone, Copy)]
pub struct Button {
    /// Button tag, `&'static str`
    pub tag: &'static str,
    /// Button pin (GPIO), `u8`
    pub pin: u8,
    /// Shows if the button is holded
    pub holded: bool,
    /// Shows if the button is clicked
    pub clicked: bool,
}

/// Buttons
#[derive(Debug, Clone, Default)]
pub struct Buttons {
    /// A vector with buttons
    ///
    /// # Example
    ///
    /// ```ignore
    /// Buttons {
    ///     tag: "default",
    ///     pin: 17,
    ///     holded: false,
    ///     clicked: false
    /// }
    /// ```
    pub buttons: Vec<Button>,
}

impl Buttons {
    /// Copies the structure
    pub fn copy(&self) -> Self {
        Self {
            buttons: self.buttons.clone(),
        }
    }

    /// A cool function which helps checking if button is holded by its tag
    pub fn holded(&self, tag: impl AsRef<str>) -> bool {
        let tag = tag.as_ref();
        self.buttons
            .iter()
            .any(|button| button.holded && button.tag == tag)
    }

    /// A cool function which helps checking if button is holded by its pin
    pub fn pin_holded(&self, pin: u8) -> bool {
        self.buttons
            .iter()
            .any(|button| button.holded && button.pin == pin)
    }

    /// A cool function which helps checking if button is clicked by its tag
    pub fn clicked(&self, tag: impl AsRef<str>) -> bool {
        let tag = tag.as_ref();
        self.buttons
            .iter()
            .any(|button| button.clicked && button.tag == tag)
    }

    /// A cool function which helps checking if button is clicked by its pin
    pub fn pin_clicked(&self, pin: u8) -> bool {
        self.buttons
            .iter()
            .any(|button| button.clicked && button.pin == pin)
    }
}
