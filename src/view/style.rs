use crossterm::style::{Attribute as TextStyleCrossterm, Color as ColorCrossterm};

// ---------------------------------
// Styles and flavors properties
// ---------------------------------
#[derive(Clone, Copy)]
pub enum TextStyle {
    Bold,
    Italic,
}

impl From<TextStyle> for TextStyleCrossterm {
    fn from(val: TextStyle) -> Self {
        match val {
            TextStyle::Bold => TextStyleCrossterm::Bold,
            TextStyle::Italic => TextStyleCrossterm::Italic,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Yellow,
    Blue,
    Red,
    // Rgb(i32, i32, i32),
}

impl From<Color> for ColorCrossterm {
    fn from(val: Color) -> Self {
        match val {
            Color::Blue => ColorCrossterm::Blue,
            Color::Red => ColorCrossterm::Red,
            Color::Yellow => ColorCrossterm::DarkYellow,
        }
    }
}

// ----------------
// Styled Structs to manipulate
// ----------------
#[derive(Clone, Default)]
pub struct StyledString {
    pub value: String,
    color_text: Option<Color>,
    color_bg: Option<Color>,
    style_text: Option<TextStyle>,
}

impl StyledString {
    pub fn with_color_text(mut self, color: Color) -> Self {
        self.color_text = Some(color);
        self
    }

    pub fn with_color_bg(mut self, color: Color) -> Self {
        self.color_bg = Some(color);
        self
    }
    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.style_text = Some(style);
        self
    }
}

impl From<StyledString> for String {
    fn from(val: StyledString) -> Self {
        val.value
    }
}

impl From<String> for StyledString {
    fn from(value: String) -> Self {
        Self {
            value,
            color_text: None,
            color_bg: None,
            style_text: None,
        }
    }
}

impl From<&str> for StyledString {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
            color_text: None,
            color_bg: None,
            style_text: None,
        }
    }
}

// ---------------------------
// Crossterm converter
// ---------------------------

use crossterm::style::{ContentStyle, StyledContent, Stylize};

impl From<StyledString> for StyledContent<String> {
    fn from(val: StyledString) -> Self {
        let mut style_text = StyledContent::new(ContentStyle::default(), val.value);

        if let Some(color) = val.color_text {
            style_text = style_text.with(color.into());
        }

        if let Some(color_bg) = val.color_bg {
            style_text = style_text.on(color_bg.into());
        }

        if let Some(style) = val.style_text {
            style_text = style_text.attribute(style.into());
        }

        style_text
    }
}

// ------------------
// Utils
// ------------------
pub fn create_vec_styled_string_from<T: Into<String>>(
    values: impl IntoIterator<Item = T>,
) -> Vec<StyledString> {
    values
        .into_iter()
        .map(|item| item.into())
        .map(StyledString::from)
        .collect()
}
