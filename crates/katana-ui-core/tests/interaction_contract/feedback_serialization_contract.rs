use katana_ui_core::atom::Input;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionResult};
use katana_ui_core::molecule::{NotificationToast, StatusBar};
use katana_ui_core::render_model::{UiDismissAction, UiTone, UiTree, UiVariant};

#[test]
fn notification_toast_dismiss_closes_owned_state() {
    let status = UiTree::new(
        StatusBar::new("System")
            .severity(UiTone::Warning)
            .variant(UiVariant::Outline)
            .dismiss_action(UiDismissAction::Available),
    );
    let mut toast = NotificationToast::new("Saved")
        .open(true)
        .severity(UiTone::Success)
        .variant(UiVariant::Filled)
        .dismiss_action(UiDismissAction::Available);
    let result = toast.apply_action(&UiAction::dismiss(toast.state_id().clone()));
    let toast_tree = UiTree::new(toast);

    assert_eq!(UiTone::Warning, status.root().props().status.severity);
    assert_eq!(UiVariant::Outline, status.root().props().status.variant);
    assert_eq!(
        UiDismissAction::Available,
        status.root().props().status.dismiss_action
    );
    assert!(result.handled);
    assert!(!toast_tree.root().props().interaction.open);
    assert_eq!(UiTone::Success, toast_tree.root().props().status.severity);
    assert_eq!(UiVariant::Filled, toast_tree.root().props().status.variant);
}

#[test]
fn action_result_is_serializable_snapshot() -> serde_json::Result<()> {
    let mut input = Input::new("Text input");
    let result = input.apply_action(&UiAction::set_value(input.state_id().clone(), "typed"));
    let encoded = serde_json::to_string(&result)?;
    let decoded: UiActionResult = serde_json::from_str(&encoded)?;

    assert_eq!(result, decoded);
    Ok(())
}
