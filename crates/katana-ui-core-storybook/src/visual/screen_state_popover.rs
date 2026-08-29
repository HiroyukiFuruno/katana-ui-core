use super::screen_state::StorybookScreenState;
use katana_ui_core::atom::{Button, Text};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::interaction::placement::Placement;
use katana_ui_core::molecule::{
    Popover, PopoverActionSlot, PopoverArrowSpec, PopoverFocusManagement, PopoverSlots,
};
use katana_ui_core::render_model::UiNodeId;

const POPOVER_FOCUS_RETURN_TARGET: &str = "toolbar-anchor";
const POPOVER_OFFSET_X: i16 = 8;
const POPOVER_OFFSET_Y: i16 = 12;
const POPOVER_ARROW_SIZE: u16 = 10;

impl StorybookScreenState {
    pub(in crate::visual) fn register_popover_open(&mut self) {
        let result = popover_open_result();
        assert!(
            result.handled && result.after.open,
            "the popover toggle action must open the fixture"
        );
        self.action_count += 1;
        self.last_action = "popover_open";
        self.last_event = "popover_opened";
        self.last_setting = "interaction.open";
        self.last_setting_value = "true";
        self.state_label = "open=true";
    }

    pub(in crate::visual) fn register_popover_hover(&mut self) {
        let result = popover_action_result(|target| UiAction::hover(target, true));
        assert!(
            result.handled && result.after.hovered,
            "the popover hover action must update hover state"
        );
        self.action_count += 1;
        self.preview_hovered = true;
        self.last_action = "popover_hover";
        self.last_event = "popover_hovered";
        self.last_setting = "interaction.hovered";
        self.last_setting_value = "true";
        self.state_label = "hover=true";
    }

    pub(in crate::visual) fn register_popover_focus(&mut self) {
        let result = popover_action_result(UiAction::focus);
        assert!(
            result.handled && result.after.focused,
            "the popover focus action must update focus state"
        );
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "popover_focus";
        self.last_event = "popover_focused";
        self.last_setting = "popover.focus_management";
        self.last_setting_value = "first";
        self.state_label = "focus=true";
    }

    pub(in crate::visual) fn register_popover_keyboard_escape(&mut self) {
        if !self.button_focused {
            self.last_action = "popover_keyboard_without_focus";
            self.last_event = "popover_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        let result = popover_action_result(UiAction::modal_escape);
        assert!(
            result.handled && !result.after.open,
            "the popover escape action must close the fixture"
        );
        self.action_count += 1;
        self.last_action = "popover_keyboard_escape";
        self.last_event = "popover_closed";
        self.last_setting = "popover.escape_dismiss";
        self.last_setting_value = "true";
        self.state_label = "open=false";
    }
}

fn popover_open_result() -> katana_ui_core::interaction::UiActionResult {
    let mut popover = popover_fixture().open(false);
    let target = popover.state_id().clone();
    popover.apply_action(&UiAction::popover_toggle(target))
}

fn popover_action_result(
    action_builder: impl FnOnce(katana_ui_core::render_model::UiStateId) -> UiAction,
) -> katana_ui_core::interaction::UiActionResult {
    let mut popover = popover_fixture().open(true);
    let target = popover.state_id().clone();
    popover.apply_action(&action_builder(target))
}

fn popover_fixture() -> Popover {
    Popover::new("Actions")
        .anchor_summary("toolbar.action")
        .placement("bottom-start")
        .offset(POPOVER_OFFSET_X, POPOVER_OFFSET_Y)
        .width("320px")
        .focus_handling("return-to-anchor")
        .focus_return_target(UiNodeId::new(POPOVER_FOCUS_RETURN_TARGET))
        .outside_click_dismiss(true)
        .escape_dismiss(true)
        .arrow(PopoverArrowSpec::new(
            true,
            POPOVER_ARROW_SIZE,
            "surface-raised",
        ))
        .slots(
            PopoverSlots::new()
                .heading("Quick actions")
                .body("Operate on the selection")
                .footer("2 actions")
                .action(PopoverActionSlot::new("copy-action", "Copy")),
        )
        .focus_management(PopoverFocusManagement::FirstInteractive)
        .keep_open_on_inner_focus(true)
        .auto_flip_priority([Placement::BottomStart, Placement::TopStart])
        .child(Button::new("Copy"))
        .child(Text::new("Complex content"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popover_keyboard_requires_focus_before_escape() {
        let mut state = StorybookScreenState::default();
        state.register_popover_keyboard_escape();
        assert_eq!("popover_keyboard_ignored", state.last_event);
        state.register_popover_focus();
        state.register_popover_keyboard_escape();
        assert_eq!("popover_closed", state.last_event);
    }
}
