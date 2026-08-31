use super::*;

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
