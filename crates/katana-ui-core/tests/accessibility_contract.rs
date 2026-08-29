use katana_ui_core::accessibility::{
    AccessibilityLabel, AccessibilityNode, AccessibilityRole, ReducedMotionPreference,
    ReducedMotionQuery,
};
use katana_ui_core::render_model::UiNodeId;

#[test]
fn accessibility_value_objects_expose_labels_nodes_and_motion_preference() {
    let label = AccessibilityLabel::new("Document page");
    assert_eq!("Document page", label.as_str());

    let node = AccessibilityNode::new(
        UiNodeId::new("page"),
        AccessibilityRole::Dialog,
        label.clone(),
    );
    assert_eq!(UiNodeId::new("page"), node.target);
    assert_eq!(AccessibilityRole::Dialog, node.role);
    assert_eq!(label, node.label);

    assert!(
        !ReducedMotionQuery::new(ReducedMotionPreference::NoPreference).prefers_reduced_motion()
    );
    assert!(ReducedMotionQuery::new(ReducedMotionPreference::Reduce).prefers_reduced_motion());
}
