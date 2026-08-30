use super::{SanitizedDocumentRootStyleKey, resolve_style};
use katana_ui_core::theme::FontFamily;

#[test]
fn default_key_resolves_to_a_valid_generic_style() -> Result<(), String> {
    let style = resolve_style(SanitizedDocumentRootStyleKey::Default)
        .map_err(|error| format!("{error:?}"))?;

    assert!(style.text_raster.line_height_px.is_finite());
    assert!(style.text_raster.line_height_px > 0.0);
    assert_eq!(FontFamily::Monospace, style.text_raster.font.family);
    assert!(style.chrome_raster.line_height_px > 0.0);
    assert!(style.chrome_raster.icon_size_px > 0);
    assert!(style.search.input_width_px > 0);
    assert!(style.search.input_height_px > 0);
    assert!(style.search.gap_px > 0);
    assert!(style.search.control_padding_px > 0);
    assert!(
        !style
            .text_paint
            .background_rgba
            .iter()
            .all(|channel| *channel == 0)
    );
    assert!(
        !style
            .chrome_paint
            .action_rgba
            .iter()
            .all(|channel| *channel == 0)
    );
    Ok(())
}

#[test]
fn public_sanitized_input_contains_no_concrete_style_values() {
    let source = include_str!("sanitized_document_root_input.rs");

    assert!(source.contains("SanitizedDocumentRootStyleKey"));
    for forbidden in [
        "FontToken",
        "TextSurface",
        "CommandChrome",
        "[u8; 4]",
        "line_height_px",
        "rgba",
    ] {
        assert!(
            !source.contains(forbidden),
            "public sanitized input leaked concrete style value: {forbidden}"
        );
    }
}
