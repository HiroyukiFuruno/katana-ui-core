use katana_ui_core::theme::{Rgba, ThemeSnapshot};

pub(super) const DEFAULT_BACKGROUND: u32 = 0x1e1e1e;
const DEFAULT_SURFACE: u32 = 0x252526;
const DEFAULT_PANEL: u32 = 0x282828;
const DEFAULT_CODE_BACKGROUND: u32 = 0x282828;
const DEFAULT_BORDER: u32 = 0x3c3c3c;
const DEFAULT_TEXT: u32 = 0xd4d4d4;
const DEFAULT_MUTED: u32 = 0x8e8e8e;
const DEFAULT_ACCENT: u32 = 0x569cd6;
const DEFAULT_SELECTION: u32 = 0x264f78;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VisualPalette {
    pub background: u32,
    pub surface: u32,
    pub panel: u32,
    pub code_background: u32,
    pub border: u32,
    pub text: u32,
    pub muted: u32,
    pub accent: u32,
    pub selection: u32,
}

impl VisualPalette {
    #[must_use]
    pub(super) fn from_theme(theme: &ThemeSnapshot) -> Self {
        Self {
            background: color(theme, "background", DEFAULT_BACKGROUND),
            surface: color(theme, "surface", DEFAULT_SURFACE),
            panel: color(theme, "panel", DEFAULT_PANEL),
            code_background: color(theme, "code-background", DEFAULT_CODE_BACKGROUND),
            border: color(theme, "border", DEFAULT_BORDER),
            text: color(theme, "text", DEFAULT_TEXT),
            muted: color(theme, "muted", DEFAULT_MUTED),
            accent: color(theme, "accent", DEFAULT_ACCENT),
            selection: color(theme, "selection", DEFAULT_SELECTION),
        }
    }
}

fn color(theme: &ThemeSnapshot, name: &str, fallback: u32) -> u32 {
    theme.color(name).map_or(fallback, rgb)
}

fn rgb(rgba: Rgba) -> u32 {
    ((rgba[0] as u32) << RED_SHIFT) | ((rgba[1] as u32) << GREEN_SHIFT) | rgba[2] as u32
}
