use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::ModalOverlay;

#[test]
fn modal_overlay_owns_focus_and_dismiss_policies() {
    let overlay = ModalOverlay::new("Overlay")
        .backdrop("dim")
        .focus_trap(true)
        .focus_return("settings-button")
        .dismiss_policy("escape-only")
        .escape_dismiss(true)
        .outside_click_dismiss(false);

    assert_eq!("dim", overlay.backdrop_model());
    assert!(overlay.traps_focus());
    assert_eq!("settings-button", overlay.focus_return_model());
    assert_eq!("escape-only", overlay.dismiss_policy_model());
    assert!(overlay.dismisses_on_escape());
    assert!(!overlay.dismisses_on_outside_click());
}

#[test]
fn modal_overlay_escape_and_backdrop_actions_follow_policy() {
    let mut escape_overlay = ModalOverlay::new("Overlay").open(true).escape_dismiss(true);
    let escape_result =
        escape_overlay.apply_action(&UiAction::modal_escape(escape_overlay.state_id().clone()));

    let mut backdrop_overlay = ModalOverlay::new("Overlay")
        .open(true)
        .outside_click_dismiss(false);
    let backdrop_result = backdrop_overlay.apply_action(&UiAction::modal_backdrop_click(
        backdrop_overlay.state_id().clone(),
    ));

    assert!(escape_result.handled);
    assert_eq!("modal_escape", escape_result.callback_log[0].action);
    assert!(!escape_result.after.open);
    assert!(!backdrop_result.handled);
    assert!(backdrop_result.after.open);
}
