use crate::visual::screen_state::StorybookScreenState;
use std::collections::BTreeMap;

pub(in crate::visual) const DEFAULT_INSTANCE_ID: &str = "storybook.preview";

#[derive(Debug, Clone, PartialEq, Default)]
pub(in crate::visual) struct StorybookScreenStateStore {
    states: BTreeMap<StorybookScreenStateKey, StorybookScreenState>,
}

impl StorybookScreenStateStore {
    pub(super) fn save_instance(
        &mut self,
        component_id: &'static str,
        preset_index: usize,
        instance_id: &'static str,
        state: StorybookScreenState,
    ) {
        let key = StorybookScreenStateKey {
            component_id,
            preset_index,
            instance_id,
        };
        self.states.remove(&key);
        if state != StorybookScreenState::default() {
            self.states.insert(key, state);
        }
    }

    pub(super) fn restore_instance(
        &self,
        component_id: &'static str,
        preset_index: usize,
        instance_id: &'static str,
    ) -> StorybookScreenState {
        self.states
            .get(&StorybookScreenStateKey {
                component_id,
                preset_index,
                instance_id,
            })
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StorybookScreenStateKey {
    component_id: &'static str,
    preset_index: usize,
    instance_id: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::screen_state_tabs::TabsScreenAction;
    use crate::visual::selection_screen_state::SelectionScreenAction;

    #[test]
    fn screen_state_store_keeps_page_and_preset_state_separate() {
        let mut store = StorybookScreenStateStore::default();
        let text_state = screen_state("text_action", "text_event", "text_state");
        let other_preset_state = screen_state("preset_action", "preset_event", "preset_state");
        let button_state = screen_state("button_action", "button_event", "button_state");

        store.save_instance("text", 0, DEFAULT_INSTANCE_ID, text_state.clone());
        store.save_instance("text", 1, DEFAULT_INSTANCE_ID, other_preset_state.clone());
        store.save_instance("button", 0, DEFAULT_INSTANCE_ID, button_state.clone());

        assert_eq!(
            text_state,
            store.restore_instance("text", 0, DEFAULT_INSTANCE_ID)
        );
        assert_eq!(
            other_preset_state,
            store.restore_instance("text", 1, DEFAULT_INSTANCE_ID)
        );
        assert_eq!(
            button_state,
            store.restore_instance("button", 0, DEFAULT_INSTANCE_ID)
        );
        assert_default_interaction_state(store.restore_instance("button", 1, DEFAULT_INSTANCE_ID));
    }

    #[test]
    fn screen_state_store_removes_default_state_for_page_preset_key_only() {
        let mut store = StorybookScreenStateStore::default();
        let text_state = screen_state("text_action", "text_event", "text_state");
        let other_preset_state = screen_state("preset_action", "preset_event", "preset_state");

        store.save_instance("text", 0, DEFAULT_INSTANCE_ID, text_state.clone());
        store.save_instance("text", 1, DEFAULT_INSTANCE_ID, other_preset_state.clone());
        store.save_instance(
            "text",
            0,
            DEFAULT_INSTANCE_ID,
            std::hint::black_box(StorybookScreenState::default()),
        );

        assert_default_interaction_state(store.restore_instance("text", 0, DEFAULT_INSTANCE_ID));
        assert_eq!(
            other_preset_state,
            store.restore_instance("text", 1, DEFAULT_INSTANCE_ID)
        );
    }

    #[test]
    fn screen_state_store_keeps_non_input_component_instances_separate() {
        let mut store = StorybookScreenStateStore::default();
        let mut tabs_primary = StorybookScreenState::default();
        let mut tabs_secondary = StorybookScreenState::default();

        tabs_primary.register_tabs_action(TabsScreenAction::AddTab);
        tabs_secondary.register_tabs_action(TabsScreenAction::TogglePinActive);

        store.save_instance("tabs", 0, "tabs.primary", tabs_primary.clone());
        store.save_instance("tabs", 0, "tabs.secondary", tabs_secondary.clone());

        assert_eq!(
            tabs_primary,
            store.restore_instance("tabs", 0, "tabs.primary")
        );
        assert_eq!(
            tabs_secondary,
            store.restore_instance("tabs", 0, "tabs.secondary")
        );
        assert_ne!(
            store.restore_instance("tabs", 0, "tabs.primary"),
            store.restore_instance("tabs", 0, "tabs.secondary")
        );
        assert_default_interaction_state(store.restore_instance("tabs", 0, DEFAULT_INSTANCE_ID));
    }

    #[test]
    fn screen_state_store_keeps_selection_component_instances_separate() {
        let mut store = StorybookScreenStateStore::default();
        let mut combo_primary = StorybookScreenState::default();
        let mut combo_secondary = StorybookScreenState::default();

        combo_primary
            .selection
            .apply(SelectionScreenAction::ComboFilter);
        combo_secondary
            .selection
            .apply(SelectionScreenAction::ComboOption(2));

        store.save_instance("combo-box", 0, "combo.primary", combo_primary.clone());
        store.save_instance("combo-box", 0, "combo.secondary", combo_secondary.clone());

        assert_eq!(
            combo_primary,
            store.restore_instance("combo-box", 0, "combo.primary")
        );
        assert_eq!(
            combo_secondary,
            store.restore_instance("combo-box", 0, "combo.secondary")
        );
        assert_ne!(
            store.restore_instance("combo-box", 0, "combo.primary"),
            store.restore_instance("combo-box", 0, "combo.secondary")
        );
        assert_default_interaction_state(store.restore_instance(
            "combo-box",
            0,
            DEFAULT_INSTANCE_ID,
        ));
    }

    fn assert_default_interaction_state(state: StorybookScreenState) {
        assert_eq!(0, state.action_count);
        assert_eq!(0, state.settings_revision);
        assert_eq!("none", state.last_action);
        assert_eq!("none", state.last_event);
        assert_eq!("idle", state.state_label);
    }

    fn screen_state(
        last_action: &'static str,
        last_event: &'static str,
        state_label: &'static str,
    ) -> StorybookScreenState {
        StorybookScreenState {
            action_count: 1,
            last_action,
            last_event,
            state_label,
            ..Default::default()
        }
    }
}
