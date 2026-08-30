use crate::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeSearchStyle,
};
use crate::text_command_surface::{EguiTextCommandSurfaceError, TextCommandSurfaceStyle};
use crate::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::theme::{FontToken, Rgba, ThemeSnapshot};

const LINE_HEIGHT_MULTIPLIER: f32 = 1.5;
const ICON_SIZE_MULTIPLIER: f32 = 1.0;
const SEARCH_INPUT_WIDTH_MULTIPLIER: f32 = 14.0;

pub(super) fn from_theme(
    theme: &ThemeSnapshot,
) -> Result<TextCommandSurfaceStyle, EguiTextCommandSurfaceError> {
    let body_font = required_font(theme, "body")?;
    let code_font = required_font(theme, "code")?;
    let panel = required_color(theme, "panel")?;
    let code_background = required_color(theme, "code-background")?;
    let text = required_color(theme, "text")?;
    let muted = required_color(theme, "muted")?;
    let accent = required_color(theme, "accent")?;
    let accent_foreground = required_color(theme, "accent-foreground")?;
    let selection = required_color(theme, "selection")?;
    let spacing_px = pixels(required_spacing(theme, "sm")?);
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

    Ok(TextCommandSurfaceStyle {
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
    })
}

fn required_color(
    theme: &ThemeSnapshot,
    name: &'static str,
) -> Result<Rgba, EguiTextCommandSurfaceError> {
    theme
        .color(name)
        .ok_or(EguiTextCommandSurfaceError::MissingThemeColor { token: name })
}

fn required_font(
    theme: &ThemeSnapshot,
    name: &'static str,
) -> Result<FontToken, EguiTextCommandSurfaceError> {
    let font = theme
        .font(name)
        .cloned()
        .ok_or(EguiTextCommandSurfaceError::MissingThemeFont { token: name })?;
    if !font.size.is_finite() || font.size <= 0.0 {
        return Err(EguiTextCommandSurfaceError::InvalidThemeFont {
            token: name,
            reason: "size must be finite and greater than zero",
        });
    }
    Ok(font)
}

fn required_spacing(
    theme: &ThemeSnapshot,
    name: &'static str,
) -> Result<f32, EguiTextCommandSurfaceError> {
    let value = theme
        .spacing
        .iter()
        .find(|token| token.name == name)
        .map(|token| token.px)
        .ok_or(EguiTextCommandSurfaceError::MissingThemeSpacing { token: name })?;
    if !value.is_finite() || value <= 0.0 {
        return Err(EguiTextCommandSurfaceError::InvalidThemeSpacing {
            token: name,
            reason: "pixels must be finite and greater than zero",
        });
    }
    Ok(value)
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
#[path = "text_command_surface_style_factory_inline_tests.rs"]
mod tests;
