use crate::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeSearchStyle,
};
use crate::text_command_surface::TextCommandSurfaceStyle;
use crate::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::theme::{FontFamily, FontToken, Rgba, ThemeSnapshot};

const LINE_HEIGHT_MULTIPLIER: f32 = 1.5;
const ICON_SIZE_MULTIPLIER: f32 = 1.0;
const SEARCH_INPUT_WIDTH_MULTIPLIER: f32 = 14.0;
const DEFAULT_FONT_SIZE: f32 = 14.0;
const DEFAULT_FONT_WEIGHT: u16 = 400;

pub(super) fn from_theme(theme: &ThemeSnapshot) -> TextCommandSurfaceStyle {
    let body_font = required_font(theme, "body");
    let code_font = required_font(theme, "code");
    let panel = required_color(theme, "panel");
    let code_background = required_color(theme, "code-background");
    let text = required_color(theme, "text");
    let muted = required_color(theme, "muted");
    let accent = required_color(theme, "accent");
    let accent_foreground = required_color(theme, "accent-foreground");
    let selection = required_color(theme, "selection");
    let spacing_px = pixels(required_spacing(theme, "sm"));
    let text_line_height = line_height(&code_font);
    let chrome_line_height = line_height(&body_font);
    let icon_size_px = pixels(body_font.size * ICON_SIZE_MULTIPLIER);
    let search_input_height_px = pixels(chrome_line_height + spacing_px as f32);
    let search_input_width_px = pixels(body_font.size * SEARCH_INPUT_WIDTH_MULTIPLIER);
    let input_paint = TextSurfacePaintStyle {
        background_rgba: panel,
        gutter_background_rgba: panel,
        gutter_paints: Vec::new(),
        selection_rgba: selection,
        preedit_rgba: accent,
        caret_rgba: accent_foreground,
        annotation_paints: Vec::new(),
    };

    TextCommandSurfaceStyle {
        text_raster: TextSurfaceRasterStyle::new(code_font, text, text_line_height),
        text_paint: TextSurfacePaintStyle {
            background_rgba: code_background,
            gutter_background_rgba: panel,
            gutter_paints: Vec::new(),
            selection_rgba: selection,
            preedit_rgba: accent,
            caret_rgba: accent_foreground,
            annotation_paints: Vec::new(),
        },
        chrome_raster: CommandChromeRasterStyle {
            font: body_font.clone(),
            text_color_rgba: text,
            icon_color: rgba_color(text),
            line_height_px: chrome_line_height,
            icon_size_px,
        },
        chrome_paint: CommandChromePaintStyle {
            action_rgba: panel,
            hovered_action_rgba: selection,
            disabled_action_rgba: muted,
        },
        search: EguiCommandChromeSearchStyle {
            input_raster: TextSurfaceRasterStyle::new(body_font, text, chrome_line_height),
            input_paint,
            input_width_px: search_input_width_px,
            input_height_px: search_input_height_px,
            gap_px: spacing_px,
            control_padding_px: spacing_px,
            active_control_rgba: accent,
        },
    }
}

fn required_color(theme: &ThemeSnapshot, name: &str) -> Rgba {
    theme.color(name).unwrap_or_else(|| {
        ThemeSnapshot::dark()
            .color(name)
            .unwrap_or([0, 0, 0, u8::MAX])
    })
}

fn required_font(theme: &ThemeSnapshot, name: &str) -> FontToken {
    theme.font(name).cloned().unwrap_or_else(|| {
        ThemeSnapshot::dark()
            .font(name)
            .cloned()
            .unwrap_or_else(|| FontToken {
                name: name.to_owned(),
                family: FontFamily::Proportional,
                size: DEFAULT_FONT_SIZE,
                weight: DEFAULT_FONT_WEIGHT,
            })
    })
}

fn required_spacing(theme: &ThemeSnapshot, name: &str) -> f32 {
    theme
        .spacing
        .iter()
        .find(|token| token.name == name)
        .map(|token| token.px)
        .filter(|value| value.is_finite() && *value > 0.0)
        .or_else(|| {
            ThemeSnapshot::dark()
                .spacing
                .into_iter()
                .find(|token| token.name == name)
                .map(|token| token.px)
                .filter(|value| value.is_finite() && *value > 0.0)
        })
        .unwrap_or(1.0)
}

fn line_height(font: &FontToken) -> f32 {
    (font.size * LINE_HEIGHT_MULTIPLIER).max(1.0)
}

fn pixels(value: f32) -> u32 {
    value.ceil().max(1.0) as u32
}

fn rgba_color([red, green, blue, alpha]: Rgba) -> RgbaColor {
    RgbaColor::new(red, green, blue, alpha)
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::theme::FontFamily;

    #[test]
    fn standard_factory_is_deterministic_and_nonempty() {
        let first = TextCommandSurfaceStyle::standard();
        let second = TextCommandSurfaceStyle::standard();

        assert_eq!(first, second);
        assert!(first.text_raster.line_height_px > 0.0);
        assert!(first.chrome_raster.line_height_px > 0.0);
        assert!(first.chrome_raster.icon_size_px > 0);
        assert!(first.search.input_width_px > 0);
        assert!(first.search.input_height_px > 0);
        assert!(first.search.gap_px > 0);
        assert!(first.search.control_padding_px > 0);
        assert_eq!(FontFamily::Monospace, first.text_raster.font.family);
    }

    #[test]
    fn factory_uses_supplied_theme_tokens() {
        let light = TextCommandSurfaceStyle::from_theme(&ThemeSnapshot::light());
        let dark = TextCommandSurfaceStyle::from_theme(&ThemeSnapshot::dark());

        assert_ne!(
            light.text_paint.background_rgba,
            dark.text_paint.background_rgba
        );
        assert_ne!(
            light.chrome_paint.action_rgba,
            dark.chrome_paint.action_rgba
        );
    }

    #[test]
    fn incomplete_theme_uses_deterministic_standard_tokens() {
        let mut incomplete = ThemeSnapshot::dark();
        incomplete.colors.clear();
        incomplete.fonts.clear();
        incomplete.spacing.clear();

        let style = TextCommandSurfaceStyle::from_theme(&incomplete);

        assert!(style.text_raster.line_height_px > 0.0);
        assert!(style.chrome_raster.line_height_px > 0.0);
        assert!(style.search.gap_px > 0);

        let fallback = required_font(&incomplete, "missing-font-token");
        assert_eq!(fallback.name, "missing-font-token");
        assert_eq!(fallback.family, FontFamily::Proportional);
        assert_eq!(fallback.size, DEFAULT_FONT_SIZE);
        assert_eq!(fallback.weight, DEFAULT_FONT_WEIGHT);
    }
}
