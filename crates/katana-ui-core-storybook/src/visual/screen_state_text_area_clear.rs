use super::screen_state::StorybookScreenState;
use katana_ui_core::widget::atoms::TextAreaAction;

impl StorybookScreenState {
    #[cfg(test)]
    pub(in crate::visual) fn register_text_area_clear_action(
        &mut self,
        readonly: bool,
        disabled: bool,
    ) -> bool {
        self.register_text_area_clear_action_for(
            super::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE,
            readonly,
            disabled,
        )
    }

    pub(in crate::visual) fn register_text_area_clear_action_for(
        &mut self,
        instance: &'static str,
        readonly: bool,
        disabled: bool,
    ) -> bool {
        {
            let runtime = self.text_area_runtime_mut_for(instance);
            runtime.readonly = readonly;
            runtime.disabled = disabled;
            runtime.uses_live_value = true;
        }
        let outcome = self.apply_core_text_area_action_for(instance, TextAreaAction::Clear);
        if !outcome.handled {
            self.register_text_area_mutation_block_for(instance);
            return true;
        }
        self.sync_text_area_runtime_for(instance, outcome);
        self.action_count += 1;
        {
            let runtime = self.text_area_runtime_mut_for(instance);
            runtime.uses_live_value = true;
            runtime.caret_visible = true;
        }
        self.last_action = "text_area_clear_action";
        self.last_event = "text_area_changed";
        self.last_setting = "text_area.clear_action";
        self.last_setting_value = "cleared";
        self.state_label = "value=cleared";
        true
    }
}
