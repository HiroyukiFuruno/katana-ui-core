use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{Modal, ModalOverlay, ModalParentInteraction};
use katana_ui_core::render_model::UiTree;

const FOCUS_RETURN_TARGET: &str = "trigger:open-modal";
const OVERLAY_FOCUS_RETURN_TARGET: &str = "trigger:open-overlay";

impl StorybookScreenState {
    pub(in crate::visual) fn register_modal_escape_close(&mut self) -> bool {
        if !self.modal_open {
            return false;
        }
        let result = modal_escape_result();
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
        if !modal_focus_trap_enabled() {
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
        let result = modal_overlay_backdrop_result(true);
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
        let result = modal_overlay_hover_result();
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
        let result = modal_overlay_focus_result();
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
        let result = modal_overlay_escape_result();
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
        let result = modal_overlay_backdrop_result(false);
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
