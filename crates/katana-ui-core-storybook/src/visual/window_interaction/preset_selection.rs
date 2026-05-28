use super::StorybookWindowState;
use crate::catalog::StoryPresetLabels;
use crate::visual::preset_tab_scroll;

impl StorybookWindowState {
    pub(in crate::visual) fn select_page(&mut self, page: &'static str) {
        let preset_index = self
            .selected_component_presets
            .get(page)
            .copied()
            .unwrap_or_default();
        self.switch_screen_state(page, normalized_preset_index(page, preset_index));
    }

    pub(in crate::visual) fn select_preset(&mut self, preset_index: usize) {
        self.switch_screen_state(
            self.selected_page,
            normalized_preset_index(self.selected_page, preset_index),
        );
    }

    pub(in crate::visual) fn scroll_preset_tabs(&mut self, delta: f32) -> bool {
        let before = self.preset_tab_scroll_x;
        self.preset_tab_scroll_x =
            preset_tab_scroll::scroll_delta(self.selected_page, before, delta);
        before != self.preset_tab_scroll_x
    }

    fn switch_screen_state(&mut self, page: &'static str, preset_index: usize) {
        self.screen_states.save(
            self.selected_page,
            self.preset_index,
            self.screen_state.clone(),
        );
        self.selected_component_presets
            .insert(self.selected_page, self.preset_index);
        self.selected_page = page;
        self.preset_index = preset_index;
        self.selected_component_presets.insert(page, preset_index);
        self.screen_state = self.screen_states.restore(page, preset_index);
        self.follow_selected_preset();
    }

    fn follow_selected_preset(&mut self) {
        self.preset_tab_scroll_x = preset_tab_scroll::ensure_index_visible(
            self.selected_page,
            self.preset_index,
            self.preset_tab_scroll_x,
        );
    }
}

fn normalized_preset_index(page: &str, preset_index: usize) -> usize {
    preset_index.min(StoryPresetLabels::for_page(page).len().saturating_sub(1))
}
