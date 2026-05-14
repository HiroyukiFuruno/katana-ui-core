use crate::composite::notification_toast::{
    NotificationToast, NotificationToastPosition, NotificationToastSeverity, NotificationToastStack,
};
use crate::theme::Theme;

#[test]
fn default_severity_is_info() {
    let toast = NotificationToast::new("done");
    assert_eq!(toast.props.severity, NotificationToastSeverity::Info);
}

#[test]
fn action_and_duration_are_stored() {
    let toast = NotificationToast::new("x")
        .duration(1_500)
        .action("Undo", || {});

    assert_eq!(toast.props.duration, Some(1_500));
    assert!(toast.props.action.is_some());
}

#[test]
fn stack_default_position_is_top_right() {
    let stack = NotificationToastStack::new(Vec::new());
    assert_eq!(stack.position, NotificationToastPosition::TopRight);
}

#[test]
fn stack_view_builds_with_theme() {
    let theme = Theme::default_light();
    let toast = NotificationToast::new("x").duration(1_000);
    let stack = NotificationToastStack::new(vec![toast]);
    let _ = stack.view(theme);
}
