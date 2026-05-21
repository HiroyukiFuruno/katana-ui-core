use crate::visual::screen_state::StorybookScreenState;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::visual) struct StorybookScreenStateStore {
    states: BTreeMap<StorybookScreenStateKey, StorybookScreenState>,
}

impl StorybookScreenStateStore {
    pub(super) fn save(
        &mut self,
        page: &'static str,
        preset_index: usize,
        state: StorybookScreenState,
    ) {
        let key = StorybookScreenStateKey { page, preset_index };
        if state == StorybookScreenState::default() {
            self.states.remove(&key);
            return;
        }
        self.states.insert(key, state);
    }

    pub(super) fn restore(&self, page: &'static str, preset_index: usize) -> StorybookScreenState {
        self.states
            .get(&StorybookScreenStateKey { page, preset_index })
            .copied()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StorybookScreenStateKey {
    page: &'static str,
    preset_index: usize,
}
