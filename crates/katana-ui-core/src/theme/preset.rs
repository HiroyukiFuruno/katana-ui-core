use super::{
    BorderToken, ColorToken, FontFamily, FontToken, RadiusToken, Rgba, ShadowToken, SpacingToken,
    ThemeId, ThemeSnapshot, ZIndexToken,
};

const LIGHT_BACKGROUND: Rgba = [255, 255, 255, 255];
const LIGHT_SURFACE: Rgba = [243, 243, 243, 255];
const LIGHT_PANEL: Rgba = [243, 243, 243, 255];
const LIGHT_CODE_BACKGROUND: Rgba = [243, 243, 243, 255];
const LIGHT_TEXT: Rgba = [36, 36, 36, 255];
const LIGHT_MUTED: Rgba = [106, 106, 106, 255];
const LIGHT_ACCENT: Rgba = [0, 120, 212, 255];
const LIGHT_BORDER: Rgba = [220, 220, 220, 255];
const LIGHT_SELECTION: Rgba = [173, 214, 255, 255];
const DARK_BACKGROUND: Rgba = [30, 30, 30, 255];
const DARK_SURFACE: Rgba = [37, 37, 38, 255];
const DARK_PANEL: Rgba = [37, 37, 38, 255];
const DARK_CODE_BACKGROUND: Rgba = [40, 40, 40, 255];
const DARK_TEXT: Rgba = [212, 212, 212, 255];
const DARK_MUTED: Rgba = [142, 142, 142, 255];
const DARK_ACCENT: Rgba = [86, 156, 214, 255];
const DARK_BORDER: Rgba = [60, 60, 60, 255];
const DARK_SELECTION: Rgba = [38, 79, 120, 255];
const BODY_FONT_SIZE: f32 = 14.0;
const CODE_FONT_SIZE: f32 = 13.0;
const BODY_FONT_WEIGHT: u16 = 400;
const SPACING_SM: f32 = 8.0;
const RADIUS_SM: f32 = 4.0;
const SHADOW_NONE: f32 = 0.0;
const BORDER_THIN: f32 = 1.0;
const Z_INDEX_OVERLAY: i32 = 100;

pub(super) struct ThemePreset {
    id: &'static str,
    background: Rgba,
    surface: Rgba,
    panel: Rgba,
    code_background: Rgba,
    text: Rgba,
    muted: Rgba,
    accent: Rgba,
    border: Rgba,
    selection: Rgba,
}

impl ThemePreset {
    pub(super) fn light() -> Self {
        Self {
            id: "light",
            background: LIGHT_BACKGROUND,
            surface: LIGHT_SURFACE,
            panel: LIGHT_PANEL,
            code_background: LIGHT_CODE_BACKGROUND,
            text: LIGHT_TEXT,
            muted: LIGHT_MUTED,
            accent: LIGHT_ACCENT,
            border: LIGHT_BORDER,
            selection: LIGHT_SELECTION,
        }
    }

    pub(super) fn dark() -> Self {
        Self {
            id: "dark",
            background: DARK_BACKGROUND,
            surface: DARK_SURFACE,
            panel: DARK_PANEL,
            code_background: DARK_CODE_BACKGROUND,
            text: DARK_TEXT,
            muted: DARK_MUTED,
            accent: DARK_ACCENT,
            border: DARK_BORDER,
            selection: DARK_SELECTION,
        }
    }

    pub(super) fn into_snapshot(self) -> ThemeSnapshot {
        ThemeSnapshot {
            id: ThemeId::new(self.id),
            colors: self.colors(),
            fonts: font_tokens(),
            spacing: vec![SpacingToken {
                name: "sm".to_string(),
                px: SPACING_SM,
            }],
            radii: vec![RadiusToken {
                name: "sm".to_string(),
                px: RADIUS_SM,
            }],
            shadows: vec![ShadowToken {
                name: "none".to_string(),
                blur: SHADOW_NONE,
                spread: SHADOW_NONE,
            }],
            borders: vec![BorderToken {
                name: "thin".to_string(),
                width: BORDER_THIN,
            }],
            z_indexes: vec![ZIndexToken {
                name: "overlay".to_string(),
                value: Z_INDEX_OVERLAY,
            }],
        }
    }

    fn colors(&self) -> Vec<ColorToken> {
        vec![
            color_token("background", self.background),
            color_token("surface", self.surface),
            color_token("panel", self.panel),
            color_token("code-background", self.code_background),
            color_token("text", self.text),
            color_token("muted", self.muted),
            color_token("accent", self.accent),
            color_token("border", self.border),
            color_token("selection", self.selection),
        ]
    }
}

fn color_token(name: &str, rgba: Rgba) -> ColorToken {
    ColorToken {
        name: name.to_string(),
        rgba,
    }
}

fn font_tokens() -> Vec<FontToken> {
    vec![
        FontToken {
            name: "body".to_string(),
            family: FontFamily::Proportional,
            size: BODY_FONT_SIZE,
            weight: BODY_FONT_WEIGHT,
        },
        FontToken {
            name: "code".to_string(),
            family: FontFamily::Monospace,
            size: CODE_FONT_SIZE,
            weight: BODY_FONT_WEIGHT,
        },
    ]
}
