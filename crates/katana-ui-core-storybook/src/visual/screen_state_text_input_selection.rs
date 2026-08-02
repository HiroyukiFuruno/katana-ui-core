use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::widget::atoms::Input;

impl StorybookScreenState {
    pub(in crate::visual) fn text_input_selection_for(
        &self,
        instance: &'static str,
    ) -> (usize, usize, usize) {
        self.text_inputs.selection(instance)
    }

    pub(in crate::visual) fn set_text_input_selection_for(
        &mut self,
        instance: &'static str,
        start: usize,
        end: usize,
    ) {
        self.text_inputs.set_selection(instance, start, end);
    }

    #[cfg(test)]
    pub(in crate::visual) fn set_text_input_selection_for_test(
        &mut self,
        instance: &'static str,
        start: usize,
        end: usize,
    ) {
        self.set_text_input_selection_for(instance, start, end);
    }

    pub(in crate::visual) fn apply_core_text_input_paste(
        &self,
        instance: &'static str,
        text: &str,
        readonly: bool,
    ) -> Option<katana_ui_core::state::UiComponentState> {
        let mut input = Input::new("Storybook text input")
            .value(self.text_input_value_for(instance))
            .readonly(readonly);
        let state_id = input.state_id().clone();
        let (_, selection_start, selection_end) = self.text_input_selection_for(instance);
        let selection = input.apply_action(&UiAction::cursor_selection(
            state_id.clone(),
            selection_end,
            selection_start,
            selection_end,
        ));
        assert!(
            selection.handled,
            "the Storybook selection action must target its own input"
        );
        let result = input.apply_action(&UiAction::paste_text(state_id, text));
        result.handled.then(|| input.state_snapshot())
    }
}
