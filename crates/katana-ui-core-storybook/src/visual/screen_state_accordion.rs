use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    Accordion, AccordionGroup, AccordionGroupItem, DisclosureTriggerArea,
};
use katana_ui_core::render_model::UiStateId;

impl StorybookScreenState {
    pub(in crate::visual) fn register_accordion_preview_toggle(&mut self) {
        let result = core_accordion_toggle();
        if !result.handled {
            return;
        }
        self.action_count += 1;
        self.last_action = "accordion_toggle";
        self.last_event = "accordion_changed";
        self.last_setting = "interaction.open";
        self.last_setting_value = "false";
        self.state_label = "open=false";
    }

    pub(in crate::visual) fn register_accordion_hover(&mut self) {
        let result = core_accordion_transient(UiAction::hover, true);
        if !result.handled || !result.after.hovered {
            return;
        }
        self.action_count += 1;
        self.last_action = "accordion_hover";
        self.last_event = "accordion_hovered";
        self.last_setting = "interaction.hovered";
        self.last_setting_value = "true";
        self.state_label = "hover=true";
    }

    pub(in crate::visual) fn register_accordion_focus(&mut self) {
        let result = core_accordion_transient(|target, _| UiAction::focus(target), true);
        if !result.handled || !result.after.focused {
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "accordion_focus";
        self.last_event = "accordion_focused";
        self.last_setting = "interaction.focused";
        self.last_setting_value = "true";
        self.state_label = "focus=true";
    }

    pub(in crate::visual) fn register_accordion_keyboard_toggle(&mut self) {
        if !self.button_focused {
            self.last_action = "accordion_keyboard_without_focus";
            self.last_event = "accordion_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.register_accordion_preview_toggle();
    }

    pub(in crate::visual) fn register_accordion_disabled_block(&mut self) {
        let mut accordion = accordion_contract_model().disabled(true);
        let target = accordion.state_id().clone();
        let result = accordion.apply_action(&UiAction::accordion_toggle(target));
        if result.handled {
            return;
        }
        self.last_action = "accordion_disabled_block";
        self.last_event = "accordion_toggle_ignored";
        self.last_setting = "accordion.disabled";
        self.last_setting_value = "true";
        self.state_label = "disabled=true";
    }

    pub(in crate::visual) fn register_accordion_group_toggle(&mut self) {
        let mut group = AccordionGroup::new("Accordion group")
            .multiple(true)
            .item(AccordionGroupItem::new("item-a", "Item A").open(true))
            .item(AccordionGroupItem::new("item-b", "Item B"))
            .item(AccordionGroupItem::new("item-c", "Item C"));
        let target = group.state_id().clone();
        let result = group.apply_action(&UiAction::set_selected_index(target, 1));
        if !result.handled {
            return;
        }
        self.action_count += 1;
        self.last_action = "accordion_group_toggle";
        self.last_event = "accordion_group_changed";
        self.last_setting = "accordion.multiple";
        self.last_setting_value = "true";
        self.state_label = "open=item-a,item-b";
    }
}

fn core_accordion_toggle() -> katana_ui_core::interaction::UiActionResult {
    let mut accordion = accordion_contract_model();
    let target = accordion.state_id().clone();
    accordion.apply_action(&UiAction::accordion_toggle(target))
}

fn core_accordion_transient(
    action_builder: impl FnOnce(UiStateId, bool) -> UiAction,
    value: bool,
) -> katana_ui_core::interaction::UiActionResult {
    let mut accordion = accordion_contract_model();
    let target = accordion.state_id().clone();
    accordion.apply_action(&action_builder(target, value))
}

fn accordion_contract_model() -> Accordion {
    Accordion::new("Accordion")
        .open(true)
        .controlled(false)
        .multiple(true)
        .indicator_position("leading")
        .trigger_area(DisclosureTriggerArea::IconAndText)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .tree_mode(true)
        .reduced_motion(true)
        .body_border(true)
}
