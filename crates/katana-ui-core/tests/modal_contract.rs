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
