mod palette;

use palette::{
    ALMOST_WHITE, BLUE_200, BLUE_400, BLUE_500, BLUE_600, GRAY_50, GRAY_200, GRAY_300, GRAY_400,
    GRAY_500, GRAY_700, GRAY_900, GREEN_400, GREEN_500, RED_400, RED_500, SURFACE_DARK,
    SURFACE_LIGHT, YELLOW_400, YELLOW_500,
};

/// RGBA color token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

const OPAQUE: u8 = u8::MAX;

impl Color {
    const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: OPAQUE }
    }
}

/// Semantic color tokens.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorTokens {
    pub bg: Color,
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub text_muted: Color,
    pub text_disabled: Color,
    pub accent: Color,
    pub accent_muted: Color,
    pub danger: Color,
    pub warning: Color,
    pub success: Color,
}

impl ColorTokens {
    #[must_use]
    pub fn light() -> Self {
        Self {
            bg: Color::rgb(ALMOST_WHITE.0, ALMOST_WHITE.1, ALMOST_WHITE.2),
            surface: Color::rgb(SURFACE_LIGHT.0, SURFACE_LIGHT.1, SURFACE_LIGHT.2),
            border: Color::rgb(GRAY_200.0, GRAY_200.1, GRAY_200.2),
            text: Color::rgb(GRAY_900.0, GRAY_900.1, GRAY_900.2),
            text_muted: Color::rgb(GRAY_500.0, GRAY_500.1, GRAY_500.2),
            text_disabled: Color::rgb(GRAY_300.0, GRAY_300.1, GRAY_300.2),
            accent: Color::rgb(BLUE_500.0, BLUE_500.1, BLUE_500.2),
            accent_muted: Color::rgb(BLUE_200.0, BLUE_200.1, BLUE_200.2),
            danger: Color::rgb(RED_500.0, RED_500.1, RED_500.2),
            warning: Color::rgb(YELLOW_500.0, YELLOW_500.1, YELLOW_500.2),
            success: Color::rgb(GREEN_500.0, GREEN_500.1, GREEN_500.2),
        }
    }

    #[must_use]
    pub fn dark() -> Self {
        Self {
            bg: Color::rgb(GRAY_900.0, GRAY_900.1, GRAY_900.2),
            surface: Color::rgb(SURFACE_DARK.0, SURFACE_DARK.1, SURFACE_DARK.2),
            border: Color::rgb(GRAY_700.0, GRAY_700.1, GRAY_700.2),
            text: Color::rgb(GRAY_50.0, GRAY_50.1, GRAY_50.2),
            text_muted: Color::rgb(GRAY_400.0, GRAY_400.1, GRAY_400.2),
            text_disabled: Color::rgb(GRAY_500.0, GRAY_500.1, GRAY_500.2),
            accent: Color::rgb(BLUE_400.0, BLUE_400.1, BLUE_400.2),
            accent_muted: Color::rgb(BLUE_600.0, BLUE_600.1, BLUE_600.2),
            danger: Color::rgb(RED_400.0, RED_400.1, RED_400.2),
            warning: Color::rgb(YELLOW_400.0, YELLOW_400.1, YELLOW_400.2),
            success: Color::rgb(GREEN_400.0, GREEN_400.1, GREEN_400.2),
        }
    }
}
