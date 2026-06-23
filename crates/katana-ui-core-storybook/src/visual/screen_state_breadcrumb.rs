use super::screen_state::StorybookScreenState;
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{Breadcrumb, ChoiceItem};
use katana_ui_core::render_model::UiStateId;

impl StorybookScreenState {
    pub(in crate::visual) fn register_breadcrumb_click(&mut self, index: usize) {
        if !core_breadcrumb_selection(index).handled {
            return;
        }
        self.apply_breadcrumb_route(index);
    }

    pub(in crate::visual) fn register_breadcrumb_hover(&mut self, index: usize) {
        if !core_breadcrumb_transient(UiAction::hover, true).handled {
            return;
        }
        self.action_count += 1;
        self.apply_breadcrumb_transient(
            "breadcrumb_hover",
            "breadcrumb_hovered",
            "interaction.hovered",
            selected_value(index),
            state_label(index),
        );
    }

    pub(in crate::visual) fn register_breadcrumb_focus(&mut self, index: usize) {
        if !core_breadcrumb_transient(|target, _| UiAction::focus(target), true).handled {
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.apply_breadcrumb_transient(
            "breadcrumb_focus",
            "breadcrumb_focused",
            "interaction.focused",
            selected_value(index),
            state_label(index),
        );
    }

    pub(in crate::visual) fn register_breadcrumb_keyboard_next(&mut self) {
        if !self.button_focused {
            self.last_action = "breadcrumb_keyboard_without_focus";
            self.last_event = "breadcrumb_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        const NEXT_INDEX: usize = 1;
        if !core_breadcrumb_selection(NEXT_INDEX).handled {
            return;
        }
        self.apply_breadcrumb_route(NEXT_INDEX);
    }

    pub(in crate::visual) fn apply_breadcrumb_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        if option.setting != "interaction.selected_index" {
            return false;
        }
        let index = match option.after {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            _ => return false,
        };
        self.breadcrumb_selected_index = index;
        self.settings_revision += 1;
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.apply_breadcrumb_state(index);
        true
    }

    pub(in crate::visual) fn register_breadcrumb_preview_action(&mut self) {
        self.register_breadcrumb_click(2);
    }

    fn apply_breadcrumb_route(&mut self, index: usize) {
        self.action_count += 1;
        self.breadcrumb_selected_index = index;
        self.apply_breadcrumb_state(index);
    }

    fn apply_breadcrumb_transient(
        &mut self,
        action: &'static str,
        event: &'static str,
        setting: &'static str,
        setting_value: &'static str,
        state: &'static str,
    ) {
        self.last_action = action;
        self.last_event = event;
        self.last_setting = setting;
        self.last_setting_value = setting_value;
        self.state_label = state;
    }

    fn apply_breadcrumb_state(&mut self, index: usize) {
        self.last_action = "breadcrumb_click";
        self.last_event = "route_changed";
        self.last_setting = "interaction.selected_index";
        self.last_setting_value = selected_value(index);
        self.state_label = state_label(index);
    }
}

fn state_label(index: usize) -> &'static str {
    match index {
        0 => "route=0",
        1 => "route=1",
        _ => "route=2",
    }
}

fn selected_value(index: usize) -> &'static str {
    match index {
        0 => "0",
        1 => "1",
        _ => "2",
    }
}

fn core_breadcrumb_selection(index: usize) -> katana_ui_core::interaction::UiActionResult {
    let mut breadcrumb = breadcrumb_contract_model();
    let target = breadcrumb.state_id().clone();
    breadcrumb.apply_action(&UiAction::set_selected_index(target, index))
}

fn core_breadcrumb_transient(
    action_builder: impl FnOnce(UiStateId, bool) -> UiAction,
    value: bool,
) -> katana_ui_core::interaction::UiActionResult {
    let mut breadcrumb = breadcrumb_contract_model();
    let target = breadcrumb.state_id().clone();
    breadcrumb.apply_action(&action_builder(target, value))
}

fn breadcrumb_contract_model() -> Breadcrumb {
    Breadcrumb::new("Breadcrumb")
        .item(ChoiceItem::new("root", "Root"))
        .item(ChoiceItem::new("src", "src"))
        .item(ChoiceItem::new("lib", "lib.rs"))
        .crumb_action("breadcrumb_click")
        .long_list(true)
        .open(true)
        .placement("bottom-start")
        .selected_index(0)
        .value("root")
}
