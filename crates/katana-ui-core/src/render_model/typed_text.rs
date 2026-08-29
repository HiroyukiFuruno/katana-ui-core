use serde::{Deserialize, Serialize};

pub const APPLE_COLOR_EMOJI_FONT_FAMILY: &str = "Apple Color Emoji";
pub const LINUX_COLOR_EMOJI_FONT_FAMILY: &str = "Noto Color Emoji";
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

    #[must_use]
    pub fn emoji_marked_spans(text: impl AsRef<str>, base_style: UiTextSpanStyle) -> Vec<Self> {
        UiEmojiTextSegments::split(text.as_ref())
            .into_iter()
            .map(|segment| Self {
                text: segment.text,
                style: if segment.emoji {
                    base_style.emoji()
                } else {
                    base_style
                },
                link_target: String::new(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiEmojiTextSegment {
    pub text: String,
    pub emoji: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiEmojiTextSegments;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiPlatformEmojiFontFamily {
    #[default]
    None,
    AppleColorEmoji,
    SegoeUiEmoji,
    NotoColorEmoji,
}

impl UiPlatformEmojiFontFamily {
    #[must_use]
    pub const fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::AppleColorEmoji => Some(APPLE_COLOR_EMOJI_FONT_FAMILY),
            Self::SegoeUiEmoji => Some("Segoe UI Emoji"),
            Self::NotoColorEmoji => Some(LINUX_COLOR_EMOJI_FONT_FAMILY),
            Self::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        APPLE_COLOR_EMOJI_FONT_FAMILY, LINUX_COLOR_EMOJI_FONT_FAMILY, UiPlatformEmojiFontFamily,
    };

    #[test]
    fn platform_emoji_family_names_cover_every_profile() {
        assert_eq!(
            UiPlatformEmojiFontFamily::AppleColorEmoji.as_str(),
            Some(APPLE_COLOR_EMOJI_FONT_FAMILY)
        );
        assert_eq!(
            UiPlatformEmojiFontFamily::SegoeUiEmoji.as_str(),
            Some("Segoe UI Emoji")
        );
        assert_eq!(
            UiPlatformEmojiFontFamily::NotoColorEmoji.as_str(),
            Some(LINUX_COLOR_EMOJI_FONT_FAMILY)
        );
        assert_eq!(UiPlatformEmojiFontFamily::None.as_str(), None);
    }
}
