use super::screen_state::StorybookScreenState;

impl StorybookScreenState {
    pub(in crate::visual) fn register_text_area_paste_for(
        &mut self,
        instance: &'static str,
        text: &str,
    ) -> bool {
        if !self.text_area_focused_for(instance) {
            return false;
        }
        let Some(after) = self.apply_core_text_area_paste_for(instance, text) else {
            self.register_text_area_mutation_block_for(instance);
            return true;
        };
        {
            let runtime = self.text_area_runtime_mut_for(instance);
            runtime.value = after.value;
            runtime.caret = after.cursor;
            runtime.selection_start = after.selection_start;
            runtime.selection_end = after.selection_end;
            runtime.uses_live_value = true;
            runtime.caret_visible = true;
        }
        self.action_count += 1;
        self.last_action = "text_area_paste";
        self.last_event = "clipboard_paste";
        self.last_setting = "text_area.value";
        self.last_setting_value = "clipboard";
        self.state_label = "value=pasted";
        true
    }

    pub(in crate::visual) fn set_text_area_value_for(
        &mut self,
        instance: &'static str,
        value: &str,
    ) {
        let runtime = self.text_area_runtime_mut_for(instance);
        runtime.value = value.to_string();
        runtime.uses_live_value = true;
        runtime.caret = value.chars().count();
        runtime.selection_start = runtime.caret;
        runtime.selection_end = runtime.caret;
    }

    #[cfg(test)]
    pub(in crate::visual) fn set_text_area_value_for_test(
        &mut self,
        instance: &'static str,
        value: &str,
    ) {
        self.set_text_area_value_for(instance, value);
    }

    pub(in crate::visual) fn set_text_area_selection_for(
        &mut self,
        instance: &'static str,
        start: usize,
        end: usize,
    ) {
        let runtime = self.text_area_runtime_mut_for(instance);
        runtime.caret = end;
        runtime.selection_start = start;
        runtime.selection_end = end;
    }

    #[cfg(test)]
    pub(in crate::visual) fn set_text_area_selection_for_test(
        &mut self,
        instance: &'static str,
        start: usize,
        end: usize,
    ) {
        self.set_text_area_selection_for(instance, start, end);
    }
}
