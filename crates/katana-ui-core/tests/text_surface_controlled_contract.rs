use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAutomaticGutterPresentation, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceViewport,
};

#[test]
fn controlled_synchronization_noop_contract_when_payload_is_unchanged() {
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface").value("a\nb"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 40),
    ));
    let presentation = TextSurfacePresentation::from_props(surface.props());

    assert!(!surface.synchronize_presentation(presentation));
}

#[test]
fn controlled_automatic_gutter_ownership_is_contract_visible() {
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface").value("owned"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 120, 40),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());

    assert!(!surface.has_controlled_automatic_gutter());
    presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(surface.synchronize_presentation(presentation));
    assert!(surface.has_controlled_automatic_gutter());

    let mut unchanged = TextSurfacePresentation::from_props(surface.props());
    unchanged.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(!surface.synchronize_presentation(unchanged));
}

#[test]
fn controlled_value_update_resets_scroll_bounds_to_avoid_stale_host_geometry_contract() {
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface").value("abc\nxyz"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 120, 40),
    ));
    surface.synchronize_scroll_bounds(UiRect::new(0, 0, 120, 200), UiRect::new(0, 0, 120, 40));
    assert!(surface.state().scroll_bounds.is_some());

    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.value = "abc\nxyz\nchanged".to_string();
    assert!(surface.synchronize_presentation(presentation));

    assert!(surface.state().scroll_bounds.is_none());
    assert_eq!("abc\nxyz\nchanged", surface.state().text_area.value);
}
