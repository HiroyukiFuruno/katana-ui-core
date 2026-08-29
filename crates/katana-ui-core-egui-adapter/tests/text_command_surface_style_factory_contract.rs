use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_egui_adapter::text_command_surface::TextCommandSurfaceStyle;

#[test]
fn public_factory_is_deterministic_and_produces_required_dimensions() -> Result<(), String> {
    let first = TextCommandSurfaceStyle::standard().map_err(|error| error.to_string())?;
    let second = TextCommandSurfaceStyle::standard().map_err(|error| error.to_string())?;

    assert_eq!(first, second);
    assert!(first.text_raster.line_height_px.is_finite());
    assert!(first.text_raster.line_height_px > 0.0);
    assert!(first.chrome_raster.line_height_px > 0.0);
    assert!(first.chrome_raster.icon_size_px > 0);
    assert!(first.search.input_width_px > 0);
    assert!(first.search.input_height_px > 0);
    assert!(first.search.gap_px > 0);
    assert!(first.search.control_padding_px > 0);
    Ok(())
}

#[test]
fn public_factory_reads_generic_theme_tokens() -> Result<(), String> {
    let light = TextCommandSurfaceStyle::from_theme(&ThemeSnapshot::light())
        .map_err(|error| error.to_string())?;
    let dark = TextCommandSurfaceStyle::from_theme(&ThemeSnapshot::dark())
        .map_err(|error| error.to_string())?;

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
fn sanitized_root_delegates_to_the_generic_factory() {
    let builder =
        include_str!("../src/text_command_surface/sanitized_document_root_style_builder.rs");
    assert!(builder.contains("TextCommandSurfaceStyle::standard()"));

    let factory = include_str!("../src/text_command_surface/text_command_surface_style_factory.rs");
    assert!(factory.contains("pub(super) fn from_theme"));
}

#[test]
fn style_code_has_no_product_specific_terms() {
    for source in [
        include_str!("../src/text_command_surface/text_command_surface_style_factory.rs"),
        include_str!("../src/text_command_surface/sanitized_document_root_style_builder.rs"),
    ] {
        for forbidden in ["Katana", "KatanA", "KLE"] {
            assert!(
                !source.contains(forbidden),
                "style code leaked `{forbidden}`"
            );
        }
    }
}
