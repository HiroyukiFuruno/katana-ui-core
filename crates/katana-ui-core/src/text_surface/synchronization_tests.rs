use super::{
    TextSurface, TextSurfaceAccessibilityTarget, TextSurfaceAction,
    TextSurfaceAutomaticGutterOverride, TextSurfaceAutomaticGutterPresentation, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfaceLayout, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceViewport, TextSurfaceViewportSizing,
};
use crate::atom::{TextArea, TextAreaAction, TextAreaSelection};
use crate::render_model::UiRect;

#[test]
fn controlled_synchronization_keeps_viewport_sizing_mode() {
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("surface").value("text"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 320, 40),
        )
        .adapter_measured_viewport(),
    );

    assert_eq!(
        surface.props().viewport_sizing,
        TextSurfaceViewportSizing::AdapterMeasured
    );

    let presentation = TextSurfacePresentation::from_props(surface.props());
    let _ = surface.synchronize_presentation(presentation);

    assert_eq!(
        surface.props().viewport_sizing,
        TextSurfaceViewportSizing::AdapterMeasured
    );
}

#[test]
fn viewport_sizing_defaults_to_fixed() {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("viewport-mode").value("text"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 320, 24),
    ));
    assert_eq!(
        surface.props().viewport_sizing,
        TextSurfaceViewportSizing::Fixed
    );
}

#[test]
fn adapter_measured_viewport_builder_switches_mode_without_width_height_side_effects() {
    let surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("viewport-mode").value("text"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 1, 1),
        )
        .adapter_measured_viewport(),
    );
    assert_eq!(
        surface.props().viewport_sizing,
        TextSurfaceViewportSizing::AdapterMeasured
    );
    assert_eq!(surface.props().viewport.width, 1);
    assert_eq!(surface.props().viewport.height, 1);
}

#[test]
fn automatic_numbered_gutter_derives_rows_and_applies_sparse_overrides() {
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "rows",
        UiRect::new(32, 0, 160, 48),
        "a\nb\nc",
        vec![
            super::TextSurfaceGraphemeBox {
                grapheme_index: 0,
                byte_start: 0,
                byte_end: 1,
                bounds: UiRect::new(32, 0, 8, 16),
            },
            super::TextSurfaceGraphemeBox {
                grapheme_index: 1,
                byte_start: 2,
                byte_end: 3,
                bounds: UiRect::new(32, 16, 8, 16),
            },
            super::TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: 4,
                byte_end: 5,
                bounds: UiRect::new(32, 32, 8, 16),
            },
        ],
    );
    let surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("surface").value("a\nb\nc"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 192, 48),
        )
        .gutter(
            TextSurfaceGutter::new(32).automatic_numbered().row(
                TextSurfaceGutterRow::new(1, "wrong-consumer-label")
                    .marker_id("opaque-marker")
                    .visual_role("changed"),
            ),
        ),
    );
    let frame = surface.frame(&layout);
    assert_eq!(frame.gutter.len(), 3);
    assert_eq!(frame.gutter[0].display_label, "1");
    assert_eq!(frame.gutter[1].display_label, "2");
    assert_eq!(frame.accessibility.gutter_targets[2].label.as_str(), "2");
    assert_eq!(frame.gutter[1].marker_id.as_deref(), Some("opaque-marker"));
    assert_eq!(frame.gutter[2].bounds.y, 32);
}

#[test]
fn controlled_automatic_gutter_uses_only_kuc_issued_identity_and_sparse_metadata() {
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "rows",
        UiRect::new(32, 0, 160, 32),
        "a\nb",
        vec![
            super::TextSurfaceGraphemeBox {
                grapheme_index: 0,
                byte_start: 0,
                byte_end: 1,
                bounds: UiRect::new(32, 0, 8, 16),
            },
            super::TextSurfaceGraphemeBox {
                grapheme_index: 1,
                byte_start: 2,
                byte_end: 3,
                bounds: UiRect::new(32, 16, 8, 16),
            },
        ],
    );
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface").value("a\nb"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 192, 32),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(surface.synchronize_presentation(presentation));
    let initial = surface.frame(&layout);
    let row_id = initial.gutter[1].row_id.clone();

    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.automatic_gutter = Some(
        TextSurfaceAutomaticGutterPresentation::new().override_row(
            row_id,
            TextSurfaceAutomaticGutterOverride::new()
                .marker_id("opaque")
                .visual_role("changed")
                .accessibility_label("changed row"),
        ),
    );
    assert!(surface.synchronize_presentation(presentation));
    let frame = surface.frame(&layout);
    assert_eq!(frame.gutter.len(), 2);
    assert_eq!(frame.gutter[0].display_label, "1");
    assert_eq!(frame.gutter[1].display_label, "2");
    assert_eq!(frame.gutter[1].marker_id.as_deref(), Some("opaque"));
    assert_eq!(frame.gutter[1].bounds.y, 16);
    assert_eq!(
        frame.accessibility.gutter_targets[2].label.as_str(),
        "changed row"
    );
}

#[test]
fn automatic_gutter_active_and_hovered_state_is_resolved_from_caret_and_controlled_input() {
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "rows",
        UiRect::new(32, 0, 160, 48),
        "a\nb\nc\nd",
        vec![
            super::TextSurfaceGraphemeBox {
                grapheme_index: 0,
                byte_start: 0,
                byte_end: 1,
                bounds: UiRect::new(32, 0, 8, 16),
            },
            super::TextSurfaceGraphemeBox {
                grapheme_index: 1,
                byte_start: 2,
                byte_end: 3,
                bounds: UiRect::new(32, 16, 8, 16),
            },
            super::TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: 4,
                byte_end: 5,
                bounds: UiRect::new(32, 32, 8, 16),
            },
            super::TextSurfaceGraphemeBox {
                grapheme_index: 3,
                byte_start: 6,
                byte_end: 7,
                bounds: UiRect::new(32, 48, 8, 16),
            },
        ],
    );
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("surface").value("a\nb\nc\nd"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 192, 80),
        )
        .gutter(TextSurfaceGutter::new(32).automatic_numbered()),
    );
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    let mut controlled = TextSurfaceAutomaticGutterPresentation::new();
    controlled.hovered_rows = vec![1, 1, 99];
    presentation.automatic_gutter = Some(controlled);
    assert!(surface.synchronize_presentation(presentation));

    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection {
            start: "a\n".len(),
            end: "a\n".len(),
        },
    )));

    let frame = surface.frame(&layout);
    assert_eq!(4, frame.gutter.len());
    assert!(
        frame.gutter[1].active,
        "row containing caret must be active"
    );
    assert!(!frame.gutter[0].active);
    assert!(!frame.gutter[2].active);
    assert!(
        !frame.gutter[0].hovered,
        "hovered row 0 must be ignored by controlled input"
    );
    assert!(frame.gutter[1].hovered, "hovered row 1 must remain hovered");
    assert!(
        frame.accessibility.gutter_targets.iter().all(|target| {
            frame
                .gutter
                .iter()
                .find(|row| {
                    matches!(
                        target.target,
                        TextSurfaceAccessibilityTarget::GutterRow {
                            logical_row
                        } if logical_row == row.logical_row
                    )
                })
                .is_some_and(|row| target.active == row.active && target.hovered == row.hovered)
        }),
        "accessibility rows should mirror gutter active/hover state"
    );

    let mut update = TextSurfacePresentation::from_props(surface.props());
    let mut controlled = TextSurfaceAutomaticGutterPresentation::new();
    controlled.hovered_rows = vec![99, 2, 2, 3, 3];
    update.automatic_gutter = Some(controlled);
    assert!(surface.synchronize_presentation(update));
    let moved = surface.frame(&layout);

    assert!(!moved.gutter[1].hovered);
    assert!(moved.gutter[2].hovered);
    assert!(moved.gutter[3].hovered);
    assert!(moved.gutter.iter().any(|row| row.hovered));
    assert!(
        moved
            .gutter
            .iter()
            .filter(|row| row.hovered)
            .all(|row| row.logical_row != 99)
    );
    assert_eq!(2, moved.gutter.iter().filter(|row| row.hovered).count());
}
