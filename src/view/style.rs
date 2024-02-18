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
    White,
    Magenta,
    // Rgb(i32, i32, i32),
}

impl From<Color> for ColorCrossterm {
    fn from(val: Color) -> Self {
        match val {
            Color::Blue => ColorCrossterm::Blue,
            Color::Red => ColorCrossterm::Red,
            Color::Yellow => ColorCrossterm::DarkYellow,
            Color::White => ColorCrossterm::White,
            Color::Magenta => ColorCrossterm::Magenta,
        }
    }
}

// ----------------
// Styled Structs to manipulate
// ----------------
#[derive(Clone, Default)]
pub struct StyledStr<'a> {
    pub value: &'a str,
    color_text: Option<Color>,
    color_bg: Option<Color>,
    style_text: Option<TextStyle>,
}

impl StyledStr<'_> {
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

impl From<StyledStr<'_>> for String {
    fn from(val: StyledStr) -> Self {
        val.value.to_string()
    }
}

impl<'a> From<&'a str> for StyledStr<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            value,
            color_text: None,
            color_bg: None,
            style_text: None,
        }
    }
}

impl<'a> From<&'a String> for StyledStr<'a> {
    fn from(value: &'a String) -> Self {
        Self {
            value,
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

impl From<StyledStr<'_>> for StyledContent<String> {
    fn from(val: StyledStr) -> Self {
        let mut style_text = StyledContent::new(ContentStyle::default(), val.value.to_string());

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
pub fn create_vec_styled_string_from<'a>(
    values: impl IntoIterator<Item = &'a str>,
) -> Vec<StyledStr<'a>> {
    values.into_iter().map(StyledStr::from).collect()
}
