use super::*;
use crate::theme::FontFamily;

#[test]
fn standard_factory_is_deterministic_and_nonempty() -> Result<(), String> {
    let first = TextCommandSurfaceStyle::standard().map_err(|error| error.to_string());
    let second = TextCommandSurfaceStyle::standard().map_err(|error| error.to_string());

    assert_eq!(first, second);
    let first = first.map_err(|error| error.to_string())?;
    assert!(first.text_raster.line_height_px > 0.0);
    assert!(first.chrome_raster.line_height_px > 0.0);
    assert!(first.chrome_raster.icon_size_px > 0);
    assert!(first.search.input_width_px > 0);
    assert!(first.search.input_height_px > 0);
    assert!(first.search.gap_px > 0);
    assert!(first.search.control_padding_px > 0);
    assert_eq!(FontFamily::Monospace, first.text_raster.font.family);
    Ok::<(), String>(())
}

#[test]
fn factory_uses_supplied_theme_tokens() -> Result<(), EguiTextCommandSurfaceError> {
    let light = TextCommandSurfaceStyle::from_theme(&ThemeSnapshot::light())?;
    let dark = TextCommandSurfaceStyle::from_theme(&ThemeSnapshot::dark())?;

    assert_ne!(
        light.text_paint.background_rgba,
        dark.text_paint.background_rgba
    );
    assert_ne!(
        light.chrome_paint.action_rgba,
        dark.chrome_paint.action_rgba
    );
    Ok(())
}

#[test]
fn missing_color_is_reported_without_substitution() {
    let mut theme = ThemeSnapshot::dark();
    theme.colors.retain(|token| token.name != "accent");

    assert!(matches!(
        TextCommandSurfaceStyle::from_theme(&theme),
        Err(EguiTextCommandSurfaceError::MissingThemeColor { token })
            if token == "accent"
    ));
}

#[test]
fn invalid_font_and_spacing_are_rejected() {
    let mut invalid_font = ThemeSnapshot::dark();
    if let Some(font) = invalid_font
        .fonts
        .iter_mut()
        .find(|font| font.name == "body")
    {
        font.size = f32::NAN;
    }
    assert!(matches!(
        TextCommandSurfaceStyle::from_theme(&invalid_font),
        Err(EguiTextCommandSurfaceError::InvalidThemeFont { token, .. })
            if token == "body"
    ));

    let mut invalid_spacing = ThemeSnapshot::dark();
    if let Some(spacing) = invalid_spacing
        .spacing
        .iter_mut()
        .find(|token| token.name == "sm")
    {
        spacing.px = 0.0;
    }
    assert!(matches!(
        TextCommandSurfaceStyle::from_theme(&invalid_spacing),
        Err(EguiTextCommandSurfaceError::InvalidThemeSpacing { token, .. })
            if token == "sm"
    ));
}
