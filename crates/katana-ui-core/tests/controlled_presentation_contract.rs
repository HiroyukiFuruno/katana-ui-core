use katana_ui_core::atom::TextArea;
use katana_ui_core::interaction::placement::Rect;
use katana_ui_core::molecule::command_chrome::{
    FloatingCommandToolbarPresentation, FloatingCommandToolbarVisibility,
};
use katana_ui_core::text_surface::{
    TextSurfaceAutomaticGutterPresentation, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceViewport,
};
use std::error::Error;

#[test]
fn controlled_presentations_expose_only_kuc_owned_geometry_boundaries() -> Result<(), Box<dyn Error>>
{
    let gutter = TextSurfaceAutomaticGutterPresentation::new();
    let gutter = serde_json::to_value(gutter)?;
    let gutter = gutter
        .as_object()
        .ok_or_else(|| std::io::Error::other("controlled gutter is an object"))?;
    assert!(gutter.contains_key("overrides"));
    for forbidden in [
        "width",
        "display_label",
        "logical_row",
        "bounds",
        "coordinate",
    ] {
        assert!(!gutter.contains_key(forbidden));
    }

    let floating = FloatingCommandToolbarPresentation::new(
        Rect::new(10, 20, 30, 40),
        Rect::new(0, 0, 320, 240),
        FloatingCommandToolbarVisibility::Visible,
    );
    let floating = serde_json::to_value(floating)?;
    let floating = floating
        .as_object()
        .ok_or_else(|| std::io::Error::other("floating presentation is an object"))?;
    assert!(floating.contains_key("anchor"));
    assert!(floating.contains_key("viewport"));
    assert!(floating.contains_key("visibility"));
    for forbidden in ["panel_size", "width", "height", "bounds"] {
        assert!(!floating.contains_key(forbidden));
    }

    let presentation = TextSurfacePresentation::from_props(&TextSurfaceProps::new(
        TextArea::new("controlled-contract").value("かな⭐️"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 120),
    ));
    let presentation = serde_json::to_value(presentation)?;
    let presentation = presentation
        .as_object()
        .ok_or_else(|| std::io::Error::other("controlled text presentation is an object"))?;
    assert!(presentation.contains_key("automatic_gutter"));
    assert!(!presentation.contains_key("gutter"));
    Ok(())
}
