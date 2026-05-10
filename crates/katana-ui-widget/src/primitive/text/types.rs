use crate::theme::color::Color;

/// Maps to a `TypographyTokens` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextRole {
    #[default]
    Body,
    BodyStrong,
    Caption,
    Code,
    Heading1,
    Heading2,
    Heading3,
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Properties for the `Text` primitive.
#[derive(Debug, Clone, Default)]
pub struct TextProps {
    pub role: TextRole,
    pub color_override: Option<Color>,
    pub max_lines: Option<usize>,
    pub align: TextAlign,
}
