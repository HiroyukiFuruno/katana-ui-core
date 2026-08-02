use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionResult};
use katana_ui_core::molecule::{Modal, ModalOverlay, ModalParentInteraction};
use katana_ui_core::render_model::UiTree;

const FOCUS_RETURN_TARGET: &str = "trigger:open-modal";
const OVERLAY_FOCUS_RETURN_TARGET: &str = "trigger:open-overlay";

impl StorybookScreenState {
    pub(in crate::visual) fn register_modal_escape_close(&mut self) -> bool {
        if !self.modal_open {
            return false;
        }
        self.register_modal_escape_result(modal_escape_result())
    }

    fn register_modal_escape_result(&mut self, result: UiActionResult) -> bool {
        if !result.handled || result.after.open {
            return false;
        }
        self.modal_open = false;
        self.action_count += 1;
        self.last_action = "modal_escape";
        self.last_event = "modal_closed";
        self.last_setting = "interaction.open";
        self.last_setting_value = "false";
        self.state_label = "open=false";
        true
    }

    pub(in crate::visual) fn register_modal_focus_trap(&mut self) -> bool {
        if !self.modal_open {
            return false;
        }
        self.register_modal_focus_trap_with(modal_focus_trap_enabled())
    }

    fn register_modal_focus_trap_with(&mut self, enabled: bool) -> bool {
        if !enabled {
            return false;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "modal_focus_trap";
        self.last_event = "modal_focused";
        self.last_setting = "modal.focus_trap";
        self.last_setting_value = "true";
        self.state_label = "focus=trapped";
        true
    }

    pub(in crate::visual) fn register_modal_keyboard_escape(&mut self) -> bool {
        if !self.modal_open {
            return false;
        }
        if !self.button_focused {
            self.last_action = "modal_keyboard_without_focus";
            self.last_event = "modal_keyboard_ignored";
            self.state_label = "focused=false";
            return false;
        }
        self.register_modal_escape_close()
    }

    pub(in crate::visual) fn register_modal_overlay_backdrop_close(&mut self) {
        self.register_modal_overlay_backdrop_result(modal_overlay_backdrop_result(true));
    }

    fn register_modal_overlay_backdrop_result(&mut self, result: UiActionResult) {
        if !result.handled || result.after.open {
            return;
        }
        self.action_count += 1;
        self.last_action = "overlay_close";
        self.last_event = "overlay_closed";
        self.last_setting = "interaction.open";
        self.last_setting_value = "false";
        self.state_label = "open=false";
    }

    pub(in crate::visual) fn register_modal_overlay_hover(&mut self) {
        self.register_modal_overlay_hover_result(modal_overlay_hover_result());
    }

    fn register_modal_overlay_hover_result(&mut self, result: UiActionResult) {
        if !result.handled || !result.after.hovered {
            return;
        }
        if self.preview_hovered
            && self.last_action == "modal_overlay_hover"
            && self.last_event == "modal_overlay_hovered"
        {
            return;
        }
        self.action_count += 1;
        self.preview_hovered = true;
        self.last_action = "modal_overlay_hover";
        self.last_event = "modal_overlay_hovered";
        self.last_setting = "modal_overlay.hover";
        self.last_setting_value = "true";
        self.state_label = "hover=true";
    }

    pub(in crate::visual) fn register_modal_overlay_focus(&mut self) {
        self.register_modal_overlay_focus_result(modal_overlay_focus_result());
    }

    fn register_modal_overlay_focus_result(&mut self, result: UiActionResult) {
        if !result.handled || !result.after.focused {
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "modal_overlay_focus";
        self.last_event = "modal_overlay_focused";
        self.last_setting = "modal_overlay.focus_trap";
        self.last_setting_value = "true";
        self.state_label = "focus=trapped";
    }

    pub(in crate::visual) fn register_modal_overlay_keyboard_escape(&mut self) {
        if !self.button_focused {
            self.last_action = "modal_overlay_keyboard_without_focus";
            self.last_event = "modal_overlay_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.register_modal_overlay_escape_result(modal_overlay_escape_result());
    }

    fn register_modal_overlay_escape_result(&mut self, result: UiActionResult) {
        if !result.handled || result.after.open {
            return;
        }
        self.action_count += 1;
        self.last_action = "modal_overlay_escape";
        self.last_event = "modal_overlay_closed";
        self.last_setting = "modal_overlay.escape_dismiss";
        self.last_setting_value = "true";
        self.state_label = "open=false";
    }

    pub(in crate::visual) fn register_modal_overlay_context_block(&mut self) {
        self.register_modal_overlay_context_result(modal_overlay_backdrop_result(false));
    }

    fn register_modal_overlay_context_result(&mut self, result: UiActionResult) {
        if result.handled || !result.after.open {
            return;
        }
        self.action_count += 1;
        self.last_action = "modal_overlay_context_block";
        self.last_event = "modal_overlay_context_ignored";
        self.last_setting = "modal_overlay.outside_click_dismiss";
        self.last_setting_value = "false";
        self.state_label = "interaction=blocked";
    }
}

fn modal_escape_result() -> katana_ui_core::interaction::UiActionResult {
    let mut modal = modal_fixture();
    let target = modal.state_id().clone();
    modal.apply_action(&UiAction::modal_escape(target))
}

fn modal_focus_trap_enabled() -> bool {
    let tree = UiTree::new(modal_fixture());
    tree.root().props().modal.focus_trap
}

fn modal_fixture() -> Modal {
    Modal::new("Modal")
        .open(true)
        .native_window_mode(true)
        .title("Preferences")
        .panel_size("medium")
        .footer("Cancel / Save")
        .parent_interaction(ModalParentInteraction::Block)
        .escape_dismiss(true)
        .focus_return(FOCUS_RETURN_TARGET)
}

fn modal_overlay_backdrop_result(
    outside_click_dismiss: bool,
) -> katana_ui_core::interaction::UiActionResult {
    let mut overlay = modal_overlay_fixture().outside_click_dismiss(outside_click_dismiss);
    let target = overlay.state_id().clone();
    overlay.apply_action(&UiAction::modal_backdrop_click(target))
}

fn modal_overlay_escape_result() -> katana_ui_core::interaction::UiActionResult {
    let mut overlay = modal_overlay_fixture().escape_dismiss(true);
    let target = overlay.state_id().clone();
    overlay.apply_action(&UiAction::modal_escape(target))
}

fn modal_overlay_hover_result() -> katana_ui_core::interaction::UiActionResult {
    let mut overlay = modal_overlay_fixture();
    let target = overlay.state_id().clone();
    overlay.apply_action(&UiAction::hover(target, true))
}

fn modal_overlay_focus_result() -> katana_ui_core::interaction::UiActionResult {
    let mut overlay = modal_overlay_fixture();
    let target = overlay.state_id().clone();
    overlay.apply_action(&UiAction::focus(target))
}

fn modal_overlay_fixture() -> ModalOverlay {
    ModalOverlay::new("Overlay")
        .open(true)
        .backdrop("dim")
        .focus_trap(true)
        .focus_return(OVERLAY_FOCUS_RETURN_TARGET)
        .dismiss_policy("outside")
        .placement(katana_ui_core::render_model::UiModalPlacement::Center)
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::UiInteractionState;

    #[test]
    fn modal_rejects_closed_unfocused_and_invalid_core_results() {
        let mut state = StorybookScreenState {
            modal_open: false,
            ..StorybookScreenState::default()
        };
        assert!(!state.register_modal_escape_close());
        assert!(!state.register_modal_focus_trap());
        assert!(!state.register_modal_keyboard_escape());

        state.modal_open = true;
        assert!(!state.register_modal_keyboard_escape());
        assert_eq!("modal_keyboard_ignored", state.last_event);
        assert!(!state.register_modal_focus_trap_with(false));

        let ignored = ignored_result(true);
        assert!(!state.register_modal_escape_result(ignored.clone()));
        state.register_modal_overlay_backdrop_result(ignored.clone());
        state.register_modal_overlay_hover_result(ignored.clone());
        state.register_modal_overlay_focus_result(ignored.clone());
        state.register_modal_overlay_escape_result(ignored);
        state.register_modal_overlay_context_result(ignored_result(false));
        assert_eq!(0, state.action_count);
    }

    #[test]
    fn modal_overlay_hover_is_idempotent_after_the_first_core_event() {
        let mut state = StorybookScreenState::default();
        state.register_modal_overlay_hover();
        let count = state.action_count;
        state.register_modal_overlay_hover();

        assert_eq!(count, state.action_count);
        assert!(state.preview_hovered);
        assert_eq!("modal_overlay_hovered", state.last_event);
    }

    #[test]
    fn modal_and_overlay_register_successful_core_actions() {
        let mut modal = StorybookScreenState::default();
        assert!(modal.register_modal_focus_trap());
        assert!(modal.register_modal_keyboard_escape());
        modal.modal_open = true;
        assert!(modal.register_modal_escape_close());

        let mut overlay = StorybookScreenState::default();
        overlay.register_modal_overlay_backdrop_close();
        overlay.register_modal_overlay_hover();
        overlay.register_modal_overlay_focus();
        overlay.register_modal_overlay_keyboard_escape();
        overlay.register_modal_overlay_context_block();

        assert_eq!(5, overlay.action_count);
        assert_eq!("modal_overlay_context_block", overlay.last_action);
        assert_eq!("interaction=blocked", overlay.state_label);
    }

    #[test]
    fn modal_overlay_keyboard_requires_focus_and_result_state() {
        let mut state = StorybookScreenState::default();
        state.register_modal_overlay_keyboard_escape();
        assert_eq!("modal_overlay_keyboard_ignored", state.last_event);

        let open = handled_result(true, false, false);
        assert!(!state.register_modal_escape_result(open.clone()));
        state.register_modal_overlay_backdrop_result(open.clone());
        state.register_modal_overlay_escape_result(open.clone());
        state.register_modal_overlay_context_result(open);
        state.register_modal_overlay_hover_result(handled_result(false, false, false));
        state.register_modal_overlay_focus_result(handled_result(false, false, false));

        assert_eq!(0, state.action_count);
    }

    fn ignored_result(open: bool) -> UiActionResult {
        UiActionResult::ignored(
            modal_fixture().state_id().clone(),
            UiInteractionState {
                open,
                ..UiInteractionState::default()
            },
        )
    }

    fn handled_result(open: bool, hovered: bool, focused: bool) -> UiActionResult {
        let target = modal_fixture().state_id().clone();
        UiActionResult::handled(
            target.clone(),
            &UiAction::modal_escape(target),
            UiInteractionState::default(),
            UiInteractionState {
                open,
                hovered,
                focused,
                ..UiInteractionState::default()
            },
        )
    }
}
