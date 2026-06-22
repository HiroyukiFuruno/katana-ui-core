use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::Tooltip;

const TOOLTIP_DELAY_MS: u16 = 240;
const TOOLTIP_MAX_WIDTH: u16 = 280;

impl StorybookScreenState {
    pub(in crate::visual) fn register_tooltip_hover_open(&mut self) {
        self.register_tooltip_open(
            "tooltip_hover",
            "tooltip_opened",
            "interaction.hovered",
            UiAction::hover,
        );
    }

    pub(in crate::visual) fn register_tooltip_focus_open(&mut self) {
        self.register_tooltip_open(
            "tooltip_focus",
            "tooltip_focused",
            "interaction.focused",
            |target, _| UiAction::focus(target),
        );
        self.button_focused = true;
    }

    pub(in crate::visual) fn register_tooltip_hover_close(&mut self) -> bool {
        let result = tooltip_open_result(UiAction::hover, false);
        if !result.handled || result.after.open || !self.is_tooltip_open() {
            return false;
        }
        self.action_count += 1;
        self.last_action = "tooltip_hover";
        self.last_event = "tooltip_closed";
        self.last_setting = "interaction.hovered";
        self.last_setting_value = "false";
        self.state_label = "hover=false focus=false";
        true
    }

    pub(in crate::visual) fn is_tooltip_open(&self) -> bool {
        matches!(
            (self.last_action, self.last_event),
            ("tooltip_hover", "tooltip_opened") | ("tooltip_focus", "tooltip_focused")
        )
    }

    fn register_tooltip_open(
        &mut self,
        action: &'static str,
        event: &'static str,
        setting: &'static str,
        action_builder: impl FnOnce(katana_ui_core::render_model::UiStateId, bool) -> UiAction,
    ) {
        let result = tooltip_open_result(action_builder, true);
        if !result.handled || !result.after.open {
            return;
        }
        if self.last_action == action
            && self.last_event == event
            && self.state_label == "hover=true focus=true"
        {
            return;
        }
        self.action_count += 1;
        self.last_action = action;
        self.last_event = event;
        self.last_setting = setting;
        self.last_setting_value = "true";
        self.state_label = "hover=true focus=true";
    }
}

fn tooltip_open_result(
    action_builder: impl FnOnce(katana_ui_core::render_model::UiStateId, bool) -> UiAction,
    active: bool,
) -> katana_ui_core::interaction::UiActionResult {
    let mut tooltip = Tooltip::new("Tooltip")
        .hover_trigger(true)
        .focus_trigger(true)
        .delay_ms(TOOLTIP_DELAY_MS)
        .max_width(TOOLTIP_MAX_WIDTH);
    let target = tooltip.state_id().clone();
    tooltip.apply_action(&action_builder(target, active))
}
