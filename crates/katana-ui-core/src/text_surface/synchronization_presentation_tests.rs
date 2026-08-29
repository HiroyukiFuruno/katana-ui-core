use super::{
    TextSurface, TextSurfaceAnnotation, TextSurfaceAnnotationStyle, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::{TextArea, TextAreaAction, TextAreaSelection};
use crate::render_model::UiTextSpan;
use crate::text_selection::UiTextSelectionRange;

#[test]
fn controlled_presentation_preserves_kuc_owned_interaction_state_without_events() {
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("surface").value("Japanese \u{2606}\u{fe0f}"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 200, 80),
    ));
    let _ = surface.apply_action(super::TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(super::TextSurfaceAction::ScrollBy {
        delta_x: 0,
        delta_y: 12,
    });
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.value = "\u{65e5}\u{672c}\u{8a9e} \u{2b50}\u{fe0f}\u{200d}\u{1f4bb}".to_string();
    presentation.selection_start = "\u{65e5}".len();
    presentation.selection_end = presentation.value.len();
    presentation.readonly = true;
    presentation.spans = vec![UiTextSpan::plain("\u{65e5}\u{672c}\u{8a9e}")];
    presentation.annotations = vec![TextSurfaceAnnotation::new(
        "opaque",
        UiTextSelectionRange::new(0, 1),
        "opaque-role",
        TextSurfaceAnnotationStyle::Underline,
    )];
    assert!(surface.synchronize_presentation(presentation));
    assert!(surface.state().text_area.focused);
    assert!(surface.state().text_area.readonly);
    assert_eq!(
        surface.state().text_area.selection,
        TextAreaSelection {
            start: 3,
            end: surface.state().text_area.value.len()
        }
    );
}

#[test]
fn synchronize_measured_viewport_size_updates_only_layout_dimensions() {
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("surface").value("text"),
            Vec::new(),
            TextSurfaceViewport::new(7, 9, 1, 1),
        )
        .annotation(TextSurfaceAnnotation::new(
            "role",
            UiTextSelectionRange::new(0, 4),
            "label",
            TextSurfaceAnnotationStyle::Underline,
        ))
        .gutter(
            TextSurfaceGutter::new(12).row(TextSurfaceGutterRow::new(0, "row").marker_id("marker")),
        ),
    );
    let _ = surface.apply_action(super::TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(super::TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: 1, end: 2 },
    )));
    let _ = surface.apply_action(super::TextSurfaceAction::ScrollBy {
        delta_x: 4,
        delta_y: 5,
    });

    let before = surface.state().text_area.clone();
    let before_viewport = surface.props().viewport;
    let before_annotations = surface.props().annotations.clone();
    let before_gutter = surface.props().gutter.clone();

    assert!(surface.synchronize_measured_viewport_size(640, 360));
    assert_eq!(surface.props().viewport.width, 640);
    assert_eq!(surface.props().viewport.height, 360);
    assert!(!surface.synchronize_measured_viewport_size(640, 360));

    assert_eq!(surface.state().text_area, before);
    assert_eq!(surface.state().scroll_x, 4);
    assert_eq!(surface.state().scroll_y, 5);
    assert_eq!(surface.state().text_area.selection, before.selection);
    assert_eq!(surface.state().text_area.focused, before.focused);
    assert_eq!(surface.props().viewport.x, before_viewport.x);
    assert_eq!(surface.props().viewport.y, before_viewport.y);
    assert_eq!(surface.props().annotations, before_annotations);
    assert_eq!(surface.props().gutter, before_gutter);
}
