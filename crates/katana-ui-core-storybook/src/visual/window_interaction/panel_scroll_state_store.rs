use crate::visual::panel_scroll_state::PanelScrollOffsets;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct StorybookPanelScrollState {
    pub(super) offsets: PanelScrollOffsets,
    pub(super) scroll_y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::visual) struct StorybookPanelScrollStateStore {
    states: BTreeMap<StorybookPanelScrollStateKey, StorybookPanelScrollState>,
}

impl StorybookPanelScrollStateStore {
    pub(super) fn save_instance(
        &mut self,
        component_id: &'static str,
        preset_index: usize,
        instance_id: &'static str,
        state: StorybookPanelScrollState,
    ) {
        let key = StorybookPanelScrollStateKey {
            component_id,
            preset_index,
            instance_id,
        };
        if state == StorybookPanelScrollState::default() {
            self.states.remove(&key);
            return;
        }
        self.states.insert(key, state);
    }

    pub(super) fn restore_instance(
        &self,
        component_id: &'static str,
        preset_index: usize,
        instance_id: &'static str,
    ) -> StorybookPanelScrollState {
        self.states
            .get(&StorybookPanelScrollStateKey {
                component_id,
                preset_index,
                instance_id,
            })
            .copied()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StorybookPanelScrollStateKey {
    component_id: &'static str,
    preset_index: usize,
    instance_id: &'static str,
}
