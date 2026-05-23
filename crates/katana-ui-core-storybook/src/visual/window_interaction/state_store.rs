use crate::visual::screen_state::StorybookScreenState;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::visual) struct StorybookScreenStateStore {
    states: BTreeMap<StorybookScreenStateKey, StorybookScreenState>,
}

impl StorybookScreenStateStore {
    pub(super) fn save(
        &mut self,
        component_id: &'static str,
        preset_index: usize,
        state: StorybookScreenState,
    ) {
        let key = StorybookScreenStateKey {
            component_id,
            preset_index,
        };
        if state == StorybookScreenState::default() {
            self.states.remove(&key);
            return;
        }
        self.states.insert(key, state);
    }

    pub(super) fn restore(
        &self,
        component_id: &'static str,
        preset_index: usize,
    ) -> StorybookScreenState {
        self.states
            .get(&StorybookScreenStateKey {
                component_id,
                preset_index,
            })
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StorybookScreenStateKey {
    component_id: &'static str,
    preset_index: usize,
}
