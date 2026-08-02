use super::selection_screen_state::SelectionScreenState;
use super::storybook_ui_option_contract::StorybookUiOptionContract;

const COMBO_LONG_ITEM_COUNT: usize = 6;

impl SelectionScreenState {
    pub(super) fn apply_combo_contract_option(&mut self, option: StorybookUiOptionContract) {
        match option.setting {
            "combo.items" => {
                self.combo_contract.item_count = COMBO_LONG_ITEM_COUNT;
                self.combo_open = true;
            }
            "interaction.open" => self.combo_open = true,
            "interaction.selected_index" => self.combo_selected_index = Some(1),
            "interaction.value" => self.combo_contract.value_applied = true,
            "placeholder" => self.combo_contract.placeholder_visible = true,
            "disabled" => self.combo_contract.disabled = true,
            "readonly" => self.combo_contract.readonly = true,
            "combo.input_value" => self.combo_contract.input_value = true,
            "combo.filter_result" => {
                self.combo_contract.filter_result = true;
                self.combo_filtered = true;
                self.combo_open = true;
            }
            "combo.free_input" => self.combo_contract.free_input = true,
            "combo.keyboard_navigation" => {
                self.combo_contract.keyboard_navigation = true;
                self.combo_open = true;
            }
            "combo.placement" => {
                self.combo_contract.placement_above = true;
                self.combo_open = true;
            }
            "combo.highlighted_index" => {
                self.combo_contract.highlighted_index = Some(1);
                self.combo_open = true;
            }
            "combo.long_list" => {
                self.combo_contract.long_list = true;
                self.combo_open = true;
            }
            "combo.outside_click_dismiss" => self.combo_contract.outside_click_dismiss = true,
            "combo.framed" => self.combo_contract.framed = true,
            "combo.trigger_summary" => self.combo_contract.trigger_summary = true,
            "combo.select_action" => self.combo_contract.select_action = true,
            "validation" => self.combo_contract.invalid = true,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectionScreenState, StorybookUiOptionContract};

    #[test]
    fn unknown_combo_contract_option_is_a_noop() {
        let mut state = SelectionScreenState::default();
        state.apply_combo_contract_option(StorybookUiOptionContract::new(
            "unknown", "before", "after",
        ));

        assert!(!state.combo_open);
        assert!(!state.combo_filtered);
    }
}
