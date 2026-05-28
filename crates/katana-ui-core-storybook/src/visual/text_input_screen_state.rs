use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::state::UiComponentState;
use std::collections::BTreeMap;

const TEXT_INPUT_LABEL: &str = "Storybook TextInput";
const DEFAULT_TEXT_INPUT_VALUE: &str = "日本語 value 🔷";
pub(super) const DEFAULT_TEXT_INPUT_INSTANCE: &str = "text-input.preview";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TextInputRuntimeState {
    component: UiComponentState,
    uses_live_value: bool,
    caret_visible: bool,
}

impl Default for TextInputRuntimeState {
    fn default() -> Self {
        Self {
            component: default_text_input_state(),
            uses_live_value: false,
            caret_visible: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct TextInputStateStore {
    default_runtime: TextInputRuntimeState,
    instances: BTreeMap<&'static str, TextInputRuntimeState>,
}

impl TextInputStateStore {
    pub(super) fn focus(&mut self, instance: &'static str, initial_value: &str, readonly: bool) {
        let runtime = self.runtime_mut(instance);
        if !runtime.uses_live_value {
            runtime.component = apply_text_input_value_state(&runtime.component, initial_value);
        }
        runtime.component.readonly = readonly;
        runtime.uses_live_value = true;
        runtime.caret_visible = true;
        runtime.component = apply_text_input_focus_state(&runtime.component, true);
    }

    pub(super) fn apply_value(&mut self, instance: &'static str, value: &str) {
        let runtime = self.runtime_mut(instance);
        runtime.uses_live_value = true;
        runtime.caret_visible = true;
        runtime.component = apply_text_input_value_state(&runtime.component, value);
    }

    pub(super) fn submit(&mut self, instance: &'static str) {
        let runtime = self.runtime_mut(instance);
        runtime.uses_live_value = true;
        runtime.caret_visible = true;
        runtime.component = apply_text_input_submit_state(&runtime.component);
    }

    pub(super) fn value(&self, instance: &'static str) -> &str {
        text_input_value(&self.runtime(instance).component)
    }

    pub(super) fn focused(&self, instance: &'static str) -> bool {
        self.runtime(instance).component.interaction.focused
    }

    pub(super) fn uses_live_value(&self, instance: &'static str) -> bool {
        self.runtime(instance).uses_live_value
    }

    pub(super) fn caret_visible(&self, instance: &'static str) -> bool {
        self.runtime(instance).caret_visible
    }

    pub(super) fn set_caret_visibility(&mut self, instance: &'static str, visible: bool) -> bool {
        let runtime = self.runtime_mut(instance);
        if runtime.caret_visible == visible {
            return false;
        }
        runtime.caret_visible = visible;
        true
    }

    fn runtime(&self, instance: &'static str) -> &TextInputRuntimeState {
        self.instances
            .get(instance)
            .unwrap_or(&self.default_runtime)
    }

    fn runtime_mut(&mut self, instance: &'static str) -> &mut TextInputRuntimeState {
        if instance == DEFAULT_TEXT_INPUT_INSTANCE {
            return &mut self.default_runtime;
        }
        self.instances.entry(instance).or_default()
    }
}

pub(super) fn default_text_input_state() -> UiComponentState {
    atom::Input::new(TEXT_INPUT_LABEL)
        .focusable(true)
        .value(DEFAULT_TEXT_INPUT_VALUE)
        .state_snapshot()
}

pub(super) fn text_input_value(state: &UiComponentState) -> &str {
    state.interaction.value.as_str()
}

pub(super) fn apply_text_input_focus_state(
    before: &UiComponentState,
    focused: bool,
) -> UiComponentState {
    let mut input = atom::Input::new(TEXT_INPUT_LABEL).set_state(before.clone());
    let action = if focused {
        UiAction::focus(before.state_id.clone())
    } else {
        UiAction::blur(before.state_id.clone())
    };
    let _result = input.apply_action(&action);
    input.state_snapshot()
}

pub(super) fn apply_text_input_value_state(
    before: &UiComponentState,
    value: &str,
) -> UiComponentState {
    let mut input = atom::Input::new(TEXT_INPUT_LABEL).set_state(before.clone());
    let _result = input.apply_action(&UiAction::input_value(before.state_id.clone(), value));
    input.state_snapshot()
}

pub(super) fn apply_text_input_submit_state(before: &UiComponentState) -> UiComponentState {
    let mut input = atom::Input::new(TEXT_INPUT_LABEL).set_state(before.clone());
    let _result = input.apply_action(&UiAction::input_submitted(before.state_id.clone()));
    input.state_snapshot()
}
