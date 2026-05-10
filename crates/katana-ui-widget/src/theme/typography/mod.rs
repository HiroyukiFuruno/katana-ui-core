const WEIGHT_REGULAR: u16 = 400;
const WEIGHT_MEDIUM: u16 = 500;
const WEIGHT_SEMIBOLD: u16 = 600;
const WEIGHT_BOLD: u16 = 700;

const SIZE_CAPTION: f32 = 11.0;
const SIZE_CODE: f32 = 13.0;
const SIZE_BODY: f32 = 14.0;
const SIZE_H3: f32 = 16.0;
const SIZE_H2: f32 = 20.0;
const SIZE_H1: f32 = 24.0;

const LH_CAPTION: f32 = 16.0;
const LH_BODY: f32 = 20.0;
const LH_H3: f32 = 24.0;
const LH_H2: f32 = 28.0;
const LH_H1: f32 = 32.0;

/// Font weight values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Regular = WEIGHT_REGULAR as isize,
    Medium = WEIGHT_MEDIUM as isize,
    SemiBold = WEIGHT_SEMIBOLD as isize,
    Bold = WEIGHT_BOLD as isize,
}

/// A complete text style definition for one typographic role.
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub font_family: &'static str,
    pub font_size: f32,
    pub line_height: f32,
    pub weight: FontWeight,
}

/// Typographic role tokens.
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyTokens {
    pub body: TextStyle,
    pub body_strong: TextStyle,
    pub caption: TextStyle,
    pub code: TextStyle,
    pub heading_1: TextStyle,
    pub heading_2: TextStyle,
    pub heading_3: TextStyle,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            body: TextStyle {
                font_family: "system-ui",
                font_size: SIZE_BODY,
                line_height: LH_BODY,
                weight: FontWeight::Regular,
            },
            body_strong: TextStyle {
                font_family: "system-ui",
                font_size: SIZE_BODY,
                line_height: LH_BODY,
                weight: FontWeight::SemiBold,
            },
            caption: TextStyle {
                font_family: "system-ui",
                font_size: SIZE_CAPTION,
                line_height: LH_CAPTION,
                weight: FontWeight::Regular,
            },
            code: TextStyle {
                font_family: "monospace",
                font_size: SIZE_CODE,
                line_height: LH_BODY,
                weight: FontWeight::Regular,
            },
            heading_1: TextStyle {
                font_family: "system-ui",
                font_size: SIZE_H1,
                line_height: LH_H1,
                weight: FontWeight::Bold,
            },
            heading_2: TextStyle {
                font_family: "system-ui",
                font_size: SIZE_H2,
                line_height: LH_H2,
                weight: FontWeight::SemiBold,
            },
            heading_3: TextStyle {
                font_family: "system-ui",
                font_size: SIZE_H3,
                line_height: LH_H3,
                weight: FontWeight::SemiBold,
            },
        }
    }
}
