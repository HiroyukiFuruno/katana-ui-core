use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::widget::atoms::{TextArea, TextAreaAction, TextAreaActionOutcome};

impl StorybookScreenState {
    pub(in crate::visual) fn apply_core_text_area_action_for(
        &self,
        instance: &'static str,
        action: TextAreaAction,
    ) -> TextAreaActionOutcome {
        let resize_enabled = self.text_area_runtime_for(instance).resize_enabled();
        self.apply_core_text_area_action_with_resize_for(instance, action, resize_enabled)
    }

    pub(in crate::visual) fn apply_core_text_area_resize_action_for(
        &self,
        instance: &'static str,
        action: TextAreaAction,
    ) -> TextAreaActionOutcome {
        self.apply_core_text_area_action_with_resize_for(instance, action, true)
    }

    pub(in crate::visual) fn sync_text_area_runtime_for(
        &mut self,
        instance: &'static str,
        outcome: TextAreaActionOutcome,
    ) {
        let runtime = self.text_area_runtime_mut_for(instance);
        runtime.value = outcome.state.value;
        runtime.caret = outcome.state.caret;
        runtime.selection_start = outcome.state.selection.start;
        runtime.selection_end = outcome.state.selection.end;
        runtime.resize_width_delta = usize::from(outcome.state.resize_width_delta);
        runtime.resize_height_delta = usize::from(outcome.state.resize_height_delta);
    }

    pub(in crate::visual) fn apply_core_text_area_paste_for(
        &self,
        instance: &'static str,
        text: &str,
    ) -> Option<katana_ui_core::render_model::UiInteractionState> {
        let runtime = self.text_area_runtime_for(instance);
        let mut text_area = TextArea::new("Storybook text area")
            .value(runtime.value())
            .readonly(runtime.readonly())
            .disabled(runtime.disabled());
        let state_id = text_area.state_id().clone();
        let (_, selection_start, selection_end) = runtime.selection();
        let selection = text_area.apply_action(&UiAction::cursor_selection(
            state_id.clone(),
            selection_end,
            selection_start,
            selection_end,
        ));
        if !selection.handled {
            return None;
        }
        let result = text_area.apply_action(&UiAction::paste_text(state_id, text));
        result.handled.then_some(result.after)
    }

    fn apply_core_text_area_action_with_resize_for(
        &self,
        instance: &'static str,
        action: TextAreaAction,
        resize_enabled: bool,
    ) -> TextAreaActionOutcome {
        let runtime = self.text_area_runtime_for(instance);
        let mut text_area = TextArea::new("Storybook text area")
            .value(runtime.value())
            .readonly(runtime.readonly())
            .disabled(runtime.disabled())
            .resize_enabled(resize_enabled)
            .vertical_scroll_enabled(runtime.vertical_scroll_enabled())
            .horizontal_scroll_enabled(runtime.horizontal_scroll_enabled())
            .vertical_scrollbar_visible(runtime.vertical_scrollbar_visible())
            .horizontal_scrollbar_visible(runtime.horizontal_scrollbar_visible());
        text_area.apply_text_area_action(action)
    }
}
