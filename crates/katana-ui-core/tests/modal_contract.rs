use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{Modal, ModalOverlay, ModalParentInteraction};
use katana_ui_core::render_model::{
    UiModalParentInteraction, UiModalPlacement, UiModalPresentation, UiModalSize, UiNodeKind,
    UiTree,
};

#[test]
fn modal_render_props_identify_native_window_contract() {
    let tree = UiTree::new(
        Modal::new("Dialog")
            .open(true)
            .title("Preferences")
            .panel_size("large")
            .footer("Cancel / Save")
            .native_window_mode(true)
            .parent_interaction(ModalParentInteraction::Block)
            .focus_return("settings-button")
            .escape_dismiss(true)
            .outside_click_dismiss(false)
            .child(Text::new("Body")),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::Modal, tree.root().kind());
    assert_eq!(UiModalPresentation::NativeWindow, props.modal.presentation);
    assert_eq!(
        UiModalParentInteraction::Block,
        props.modal.parent_interaction
    );
    assert_eq!(UiModalSize::Large, props.modal.size);
    assert_eq!("Preferences", props.modal.title);
    assert_eq!("Cancel / Save", props.modal.footer);
    assert_eq!("settings-button", props.modal.focus_return);
    assert!(props.modal.focus_trap);
    assert!(props.modal.dismiss_on_escape);
    assert!(!props.modal.dismiss_on_backdrop);
    assert!(props.interaction.open);
}

#[test]
fn modal_overlay_render_props_identify_same_window_overlay_contract() {
    let tree = UiTree::new(
        ModalOverlay::new("Overlay")
            .open(true)
            .backdrop("dim")
            .focus_trap(true)
            .focus_return("trigger-button")
            .dismiss_policy("escape+backdrop")
            .escape_dismiss(true)
            .outside_click_dismiss(true)
            .placement(UiModalPlacement::Right)
            .child(Modal::new("Dialog").title("Inline dialog")),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::ModalOverlay, tree.root().kind());
    assert_eq!(UiModalPresentation::OverlayDialog, props.modal.presentation);
    assert_eq!("dim", props.modal.backdrop);
    assert_eq!("trigger-button", props.modal.focus_return);
    assert_eq!("escape+backdrop", props.modal.dismiss_policy);
    assert_eq!(UiModalPlacement::Right, props.modal.placement);
    assert!(props.modal.focus_trap);
    assert!(props.modal.dismiss_on_escape);
    assert!(props.modal.dismiss_on_backdrop);
}

#[test]
fn modal_escape_closes_and_records_focus_return_contract() {
    let mut modal = Modal::new("Dialog")
        .open(true)
        .focus_return("settings-button")
        .escape_dismiss(true)
        .parent_interaction(ModalParentInteraction::Block);

    let result = modal.apply_action(&UiAction::modal_escape(modal.state_id().clone()));

    assert!(result.handled);
    assert_eq!("modal_escape", result.callback_log[0].action);
    assert!(!result.after.open);
    assert_eq!("escape", result.after.dismiss_reason);
    assert_eq!("focus_return=settings-button", result.after.value);
}

#[test]
fn modal_backdrop_respects_dismiss_policy() {
    let mut modal = Modal::new("Dialog")
        .open(true)
        .outside_click_dismiss(false)
        .focus_return("settings-button");

    let result = modal.apply_action(&UiAction::modal_backdrop_click(modal.state_id().clone()));

    assert!(!result.handled);
    assert!(result.after.open);
}

#[test]
fn modal_overlay_delegates_non_lifecycle_press() {
    let mut modal = ModalOverlay::new("Overlay");
    let result = modal.apply_action(&UiAction::press(modal.state_id().clone()));

    assert!(result.handled);
}

#[test]
fn modal_custom_size_parent_allow_accessors_and_selection_state_are_typed() {
    let modal = Modal::new("Dialog")
        .selected_index(2)
        .item_count(4)
        .backdrop("transparent")
        .dismiss_policy("explicit")
        .panel_size("custom:640px")
        .parent_interaction(ModalParentInteraction::Allow);
    assert_eq!("transparent", modal.backdrop_model());
    assert_eq!("explicit", modal.dismiss_policy_model());
    assert_eq!(
        ModalParentInteraction::Allow,
        modal.parent_interaction_model()
    );
    let tree = UiTree::new(modal);
    assert_eq!(
        UiModalSize::Custom { width_px: 640 },
        tree.root().props().modal.size
    );
    assert_eq!(
        UiModalParentInteraction::Allow,
        tree.root().props().modal.parent_interaction
    );
    assert_eq!(2, tree.root().props().interaction.selected_index);
    assert_eq!(4, tree.root().props().interaction.item_count);

    let invalid = UiTree::new(Modal::new("Dialog").panel_size("custom:invalid"));
    assert_eq!(
        UiModalSize::Custom { width_px: 0 },
        invalid.root().props().modal.size
    );
}

#[test]
fn modal_denied_lifecycle_actions_and_unrelated_actions_are_ignored() {
    let mut modal = Modal::new("Dialog")
        .open(true)
        .escape_dismiss(false)
        .outside_click_dismiss(false);
    assert!(
        !modal
            .apply_action(&UiAction::modal_escape(modal.state_id().clone()))
            .handled
    );
    assert!(
        !modal
            .apply_action(&UiAction::modal_backdrop_click(modal.state_id().clone()))
            .handled
    );
    let value = modal.apply_action(&UiAction::set_value(modal.state_id().clone(), "generic"));
    assert!(value.handled);
    assert_eq!("generic", value.after.value);
}
