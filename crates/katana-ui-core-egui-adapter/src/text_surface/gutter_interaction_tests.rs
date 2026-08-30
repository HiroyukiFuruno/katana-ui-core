use super::*;
use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceGutter, TextSurfaceGutterRow, TextSurfaceLayout, TextSurfaceProps,
    TextSurfaceViewport,
};

#[test]
fn marker_without_icon_bounds_activates_marker_from_the_whole_row() {
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("line").value("line"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 100, 20),
        )
        .gutter(TextSurfaceGutter::new(24).row(TextSurfaceGutterRow::new(0, "1").marker_id("bp"))),
    );
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "marker-layout",
        UiRect::new(24, 0, 76, 20),
        "line",
        vec![katana_ui_core::text_surface::TextSurfaceGraphemeBox {
            grapheme_index: 0,
            byte_start: 0,
            byte_end: 4,
            bounds: UiRect::new(24, 0, 30, 20),
        }],
    );
    let frame = surface.frame(&layout);
    let events = gutter_pointer_events(&mut surface, &frame, TextSurfacePoint::new(2, 2))
        .expect("gutter row should be hit");
    assert!(events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::GutterMarkerActivated { logical_row: 0, marker_id }
            if marker_id == "bp"
    )));
}
