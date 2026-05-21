use super::StorybookWindowState;
use crate::catalog::StoryPresetLabels;
use crate::visual::button_options::{StorybookButtonOptionControl, control_at, is_button_page};
use crate::visual::layout_metrics::{
    button_setting_hit_rect, dark_theme_rect, light_theme_rect, preset_tab_rect,
    scrollbar_off_rect, scrollbar_on_rect,
};
use crate::visual::{preview, preview_detail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StorybookButtonOperation {
    LightTheme,
    DarkTheme,
    ScrollbarOn,
    ScrollbarOff,
    Preset(usize),
    PreviewButton,
    PreviewComponent,
    ButtonOption(StorybookButtonOptionControl),
    SettingsOption,
}

impl StorybookButtonOperation {
    pub(super) fn apply(self, state: &mut StorybookWindowState) -> bool {
        match self {
            Self::LightTheme => state.theme_id = "light",
            Self::DarkTheme => state.theme_id = "dark",
            Self::ScrollbarOn => state.scrollbar_visible = true,
            Self::ScrollbarOff => state.scrollbar_visible = false,
            Self::Preset(index) => state.select_preset(index),
            Self::PreviewButton => state
                .screen_state
                .register_button_click(state.selected_page),
            Self::PreviewComponent => state
                .screen_state
                .register_preview_action(state.selected_page),
            Self::ButtonOption(control) => state.screen_state.register_button_option(control),
            Self::SettingsOption => state
                .screen_state
                .register_settings_change(state.selected_page),
        }
        true
    }
}

pub(super) fn button_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    theme_operation_at(x, y)
        .or_else(|| scrollbar_operation_at(x, y))
        .or_else(|| preset_operation_at(state.selected_page, x, y))
        .or_else(|| preview_operation_at(state.selected_page, x, y))
        .or_else(|| settings_operation_at(state.selected_page, x, y))
}

pub(in crate::visual) fn apply_hover_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    let summary_changed = state
        .screen_state
        .set_hovered_summary_index(preview::summary_control_index_at(x, y));
    let hovered = preview_detail::component_action_hit_rect(state.selected_page).contains(x, y);
    state.screen_state.set_preview_hovered(hovered) || summary_changed
}

fn theme_operation_at(x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if light_theme_rect().contains(x, y) {
        return Some(StorybookButtonOperation::LightTheme);
    }
    if dark_theme_rect().contains(x, y) {
        return Some(StorybookButtonOperation::DarkTheme);
    }
    None
}

fn scrollbar_operation_at(x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if scrollbar_on_rect().contains(x, y) {
        return Some(StorybookButtonOperation::ScrollbarOn);
    }
    if scrollbar_off_rect().contains(x, y) {
        return Some(StorybookButtonOperation::ScrollbarOff);
    }
    None
}

fn preset_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    let visible_count = StoryPresetLabels::for_page(page)
        .len()
        .min(crate::visual::layout_metrics::PRESET_TAB_COUNT);
    (0..visible_count)
        .find(|index| preset_tab_rect(*index).contains(x, y))
        .map(StorybookButtonOperation::Preset)
}

fn preview_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if preview_detail::button_action_hit_rect(page).contains(x, y) {
        return Some(StorybookButtonOperation::PreviewButton);
    }
    if preview_detail::component_action_hit_rect(page).contains(x, y) {
        return Some(StorybookButtonOperation::PreviewComponent);
    }
    None
}

fn settings_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if let Some(control) = control_at(page, x, y) {
        return Some(StorybookButtonOperation::ButtonOption(control));
    }
    if is_button_page(page) {
        return None;
    }
    if button_setting_hit_rect().contains(x, y) {
        return Some(StorybookButtonOperation::SettingsOption);
    }
    None
}
