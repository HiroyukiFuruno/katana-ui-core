use super::*;

#[test]
fn empty_accessibility_runs_use_the_null_text_position() {
    let position = text_position(&[], 0);
    assert_eq!(position.node, egui::Id::NULL.accesskit_id());
    assert_eq!(position.character_index, 0);
}

#[test]
fn accessibility_target_keys_are_stable_for_surface_gutter_marker_and_context() {
    assert_eq!(
        "surface",
        accessibility_target_key(&TextSurfaceAccessibilityTarget::Surface)
    );
    assert_eq!(
        "gutter-row:3",
        accessibility_target_key(&TextSurfaceAccessibilityTarget::GutterRow { logical_row: 3 })
    );
    assert_eq!(
        "gutter-marker:3:breakpoint",
        accessibility_target_key(&TextSurfaceAccessibilityTarget::GutterMarker {
            logical_row: 3,
            marker_id: "breakpoint".to_owned(),
        })
    );
    assert_eq!(
        "context-selection",
        accessibility_target_key(&TextSurfaceAccessibilityTarget::ContextSelection)
    );
}
