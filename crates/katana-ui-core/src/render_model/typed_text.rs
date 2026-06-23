use serde::{Deserialize, Serialize};

pub const RGBA_CHANNEL_COUNT: usize = 4;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextProps {
    pub role: String,
    pub color_token: String,
    pub line_height_px: u16,
    pub baseline_offset_px: i16,
    pub vertical_centered: bool,
    pub wrap: UiTextWrapMode,
    pub spans: Vec<UiTextSpan>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTextWrapMode {
    #[default]
    NoWrap,
    Wrap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextSpan {
    pub text: String,
    pub style: UiTextSpanStyle,
    pub link_target: String,
}

impl UiTextSpan {
    #[must_use]
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: UiTextSpanStyle::default(),
            link_target: String::new(),
        }
    }

    #[must_use]
    pub fn emoji(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: UiTextSpanStyle::default().emoji(),
            link_target: String::new(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextSpanStyle {
    pub bold: bool,
    pub italic: bool,
    pub monospace: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub highlight: bool,
    pub current_highlight: bool,
    pub inline_code: bool,
    pub inline_math: bool,
    pub emoji: bool,
    pub color_rgba: [u8; RGBA_CHANNEL_COUNT],
}

impl UiTextSpanStyle {
    #[must_use]
    pub fn emoji(mut self) -> Self {
        self.emoji = true;
        self
    }

    #[must_use]
    pub fn inline_math(mut self) -> Self {
        self.inline_math = true;
        self
    }
}
