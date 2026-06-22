use super::StorybookWindowState;
use super::panel_scroll_state_store::StorybookPanelScrollState;
use super::state_store::DEFAULT_INSTANCE_ID;
use crate::catalog::StoryPresetLabels;
use crate::visual::button_options;
use crate::visual::preset_tab_scroll;

const CHECKBOX_PAGE: &str = "checkbox";
const CHECKBOX_CHECKED_PRESET_INDEX: usize = 1;
const CHECKBOX_DISABLED_PRESET_INDEX: usize = 2;
const CHECKBOX_FOCUS_PRESET_INDEX: usize = 3;
const TOGGLE_PAGE: &str = "toggle";
const TOGGLE_ON_PRESET_INDEX: usize = 1;

impl StorybookWindowState {
    pub(in crate::visual) fn select_page(&mut self, page: &'static str) {
        let preset_index = self
            .selected_component_presets
            .get(page)
            .copied()
            .unwrap_or_default();
        let instance_id = self
            .selected_component_instances
            .get(page)
            .copied()
            .unwrap_or(DEFAULT_INSTANCE_ID);
        self.switch_screen_state(
            page,
            normalized_preset_index(page, preset_index),
            instance_id,
        );
    }

    pub(in crate::visual) fn select_preset(&mut self, preset_index: usize) {
        self.switch_screen_state(
            self.selected_page,
            normalized_preset_index(self.selected_page, preset_index),
            self.selected_instance_id,
        );
    }

    #[cfg(test)]
    pub(in crate::visual) fn select_instance(&mut self, instance_id: &'static str) {
        self.switch_screen_state(self.selected_page, self.preset_index, instance_id);
    }

    pub(in crate::visual) fn scroll_preset_tabs(&mut self, delta: f32) -> bool {
        let before = self.preset_tab_scroll_x;
        self.preset_tab_scroll_x =
            preset_tab_scroll::scroll_delta(self.selected_page, before, delta);
        before != self.preset_tab_scroll_x
    }

    fn switch_screen_state(
        &mut self,
        page: &'static str,
        preset_index: usize,
        instance_id: &'static str,
    ) {
        let shell_root_x = self.panel_scroll.root_x;
        let shell_root_y = self.panel_scroll.root_y;
        let shell_navigation_x = self.panel_scroll.navigation_x;
        let shell_navigation_y = self.panel_scroll.navigation_y;
        self.screen_states.save_instance(
            self.selected_page,
            self.preset_index,
            self.selected_instance_id,
            self.screen_state.clone(),
        );
        self.panel_scroll_states.save_instance(
            self.selected_page,
            self.preset_index,
            self.selected_instance_id,
            StorybookPanelScrollState {
                offsets: self.panel_scroll,
                scroll_y: self.scroll_y,
            },
        );
        self.selected_component_presets
            .insert(self.selected_page, self.preset_index);
        self.selected_component_instances
            .insert(self.selected_page, self.selected_instance_id);
        self.selected_page = page;
        self.preset_index = preset_index;
        self.selected_instance_id = instance_id;
        self.selected_component_presets.insert(page, preset_index);
        self.selected_component_instances.insert(page, instance_id);
        self.screen_state = self
            .screen_states
            .restore_instance(page, preset_index, instance_id);
        let panel_scroll_state =
            self.panel_scroll_states
                .restore_instance(page, preset_index, instance_id);
        self.panel_scroll = panel_scroll_state.offsets;
        self.panel_scroll.root_x = shell_root_x;
        self.panel_scroll.root_y = shell_root_y;
        self.panel_scroll.navigation_x = shell_navigation_x;
        self.panel_scroll.navigation_y = shell_navigation_y;
        self.scroll_y = shell_root_y;
        self.apply_preset_default_screen_state();
        self.follow_selected_preset();
    }

    fn follow_selected_preset(&mut self) {
        self.preset_tab_scroll_x = preset_tab_scroll::ensure_index_visible(
            self.selected_page,
            self.preset_index,
            self.preset_tab_scroll_x,
        );
    }

    fn apply_preset_default_screen_state(&mut self) {
        if self.selected_page == CHECKBOX_PAGE {
            self.apply_checkbox_preset_default_screen_state();
            return;
        }
        if self.selected_page == TOGGLE_PAGE {
            self.apply_toggle_preset_default_screen_state();
            return;
        }
        if !button_options::is_button_page(self.selected_page) {
            return;
        }
        if !self.screen_state.uses_default_button_options() {
            return;
        }
        self.screen_state.button_options = button_options::preset_button_options(self.preset_index);
    }

    fn apply_checkbox_preset_default_screen_state(&mut self) {
        if !self.screen_state.uses_default_checkbox_state() {
            return;
        }
        if self.preset_index == CHECKBOX_CHECKED_PRESET_INDEX {
            self.screen_state.apply_checkbox_checked_preset_default();
        }
        if self.preset_index == CHECKBOX_DISABLED_PRESET_INDEX {
            self.screen_state.apply_checkbox_disabled_preset_default();
        }
        if self.preset_index == CHECKBOX_FOCUS_PRESET_INDEX {
            self.screen_state.apply_checkbox_focus_preset_default();
        }
    }

    fn apply_toggle_preset_default_screen_state(&mut self) {
        if !self.screen_state.uses_default_toggle_state() {
            return;
        }
        if self.preset_index == TOGGLE_ON_PRESET_INDEX {
            self.screen_state.apply_toggle_checked_preset_default();
        }
    }
}

fn normalized_preset_index(page: &str, preset_index: usize) -> usize {
    preset_index.min(StoryPresetLabels::for_page(page).len().saturating_sub(1))
}
