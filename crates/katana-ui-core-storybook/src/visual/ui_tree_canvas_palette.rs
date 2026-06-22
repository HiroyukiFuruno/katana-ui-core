use super::palette::VisualPalette;
use katana_ui_core::theme::{Rgba, ThemeSnapshot};

const FALLBACK_LINK: u32 = 0x4da3ff;
const FALLBACK_ALERT_NOTE: u32 = 0x0969da;
const FALLBACK_ALERT_TIP: u32 = 0x1a7f37;
const FALLBACK_ALERT_IMPORTANT: u32 = 0x8250df;
const FALLBACK_ALERT_WARNING: u32 = 0xbf8700;
const FALLBACK_ALERT_CAUTION: u32 = 0xd1242f;
const FALLBACK_DANGER_ACCENT: u32 = 0xe05252;
const FALLBACK_PENDING_BACKGROUND: u32 = 0x1d2630;
const FALLBACK_HOVER_BACKGROUND: u32 = 0x243041;
const FALLBACK_DOCUMENT_RULE_BORDER_LIGHT: u32 = 0xd0d7de;
const FALLBACK_DOCUMENT_RULE_BORDER_DARK: u32 = 0x30363d;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const LIGHT_LUMA_THRESHOLD: u32 = 127;
const LUMA_RED_WEIGHT: u32 = 299;
const LUMA_GREEN_WEIGHT: u32 = 587;
const LUMA_BLUE_WEIGHT: u32 = 114;
const LUMA_WEIGHT_SCALE: u32 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct UiTreeCanvasPalette {
    pub(super) visual: VisualPalette,
    pub background: u32,
    pub preview_background: u32,
    pub selection: u32,
    pub text: u32,
    pub link: u32,
    pub code_background: u32,
    pub inline_code_background: u32,
    pub table_background: u32,
    pub table_header_background: u32,
    pub table_even_row_background: u32,
    pub alert_background: u32,
    pub alert_note_accent: u32,
    pub alert_tip_accent: u32,
    pub alert_important_accent: u32,
    pub alert_warning_accent: u32,
    pub alert_caution_accent: u32,
    pub quote_background: u32,
    pub footnote_background: u32,
    pub document_rule_border: u32,
    pub danger_accent: u32,
    pub muted_border: u32,
    pub pending_background: u32,
    pub hover_background: u32,
}

impl UiTreeCanvasPalette {
    pub(super) fn from_theme(theme: &ThemeSnapshot) -> Self {
        let visual = VisualPalette::from_theme(theme);
        Self {
            visual,
            background: visual.background,
            preview_background: visual.background,
            selection: visual.selection,
            text: visual.text,
            link: color(theme, "link", FALLBACK_LINK),
            code_background: visual.code_background,
            inline_code_background: color(theme, "inline-code-background", visual.code_background),
            table_background: color(theme, "table-row-background", visual.background),
            table_header_background: color(theme, "table-header-background", visual.surface),
            table_even_row_background: color(theme, "table-even-row-background", visual.surface),
            alert_background: color(theme, "alert-background", visual.surface),
            alert_note_accent: color(theme, "alert-note", FALLBACK_ALERT_NOTE),
            alert_tip_accent: color(theme, "alert-tip", FALLBACK_ALERT_TIP),
            alert_important_accent: color(theme, "alert-important", FALLBACK_ALERT_IMPORTANT),
            alert_warning_accent: color(theme, "alert-warning", FALLBACK_ALERT_WARNING),
            alert_caution_accent: color(theme, "alert-caution", FALLBACK_ALERT_CAUTION),
            quote_background: color(theme, "quote-background", visual.background),
            footnote_background: color(theme, "footnote-background", visual.background),
            document_rule_border: color(
                theme,
                "document-rule-border",
                document_rule_border_fallback(visual.background),
            ),
            danger_accent: FALLBACK_DANGER_ACCENT,
            muted_border: visual.border,
            pending_background: FALLBACK_PENDING_BACKGROUND,
            hover_background: FALLBACK_HOVER_BACKGROUND,
        }
    }

    pub(super) fn border_color(&self, token: &str) -> u32 {
        match token {
            "document.rule.border" => self.document_rule_border,
            _ => self.muted_border,
        }
    }
}

fn color(theme: &ThemeSnapshot, name: &str, fallback: u32) -> u32 {
    theme.color(name).map_or(fallback, rgb)
}

fn document_rule_border_fallback(background: u32) -> u32 {
    if color_luma(background) > LIGHT_LUMA_THRESHOLD {
        FALLBACK_DOCUMENT_RULE_BORDER_LIGHT
    } else {
        FALLBACK_DOCUMENT_RULE_BORDER_DARK
    }
}

fn color_luma(color: u32) -> u32 {
    let red = (color >> RED_SHIFT) & CHANNEL_MASK;
    let green = (color >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = color & CHANNEL_MASK;
    (red * LUMA_RED_WEIGHT + green * LUMA_GREEN_WEIGHT + blue * LUMA_BLUE_WEIGHT)
        / LUMA_WEIGHT_SCALE
}

fn rgb(rgba: Rgba) -> u32 {
    ((rgba[0] as u32) << RED_SHIFT) | ((rgba[1] as u32) << GREEN_SHIFT) | rgba[2] as u32
}

#[cfg(test)]
mod tests {
    use super::UiTreeCanvasPalette;
    use katana_ui_core::theme::{ThemeId, ThemeSnapshot};

    #[test]
    fn document_rule_border_uses_katana_markdown_rule_color() {
        assert_eq!(
            0xd0d7de,
            UiTreeCanvasPalette::from_theme(&ThemeSnapshot::light()).document_rule_border
        );
        assert_eq!(
            0x30363d,
            UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark()).document_rule_border
        );
    }

    #[test]
    fn document_rule_border_uses_background_luma_for_custom_light_theme_ids() {
        let mut theme = ThemeSnapshot::light();
        theme.id = ThemeId::new("document");

        let palette = UiTreeCanvasPalette::from_theme(&theme);

        assert_eq!(0xd0d7de, palette.document_rule_border);
    }
}
