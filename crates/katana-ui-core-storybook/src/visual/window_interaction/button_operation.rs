use super::StorybookWindowState;
use crate::visual::dedicated_dod_form_binary_choice_live as binary_choice_live;
use crate::visual::dedicated_dod_form_combo_live as combo_live;
use crate::visual::dedicated_dod_form_select_live as select_live;
use crate::catalog::StoryPresetLabels;
use crate::visual::button_options::{StorybookButtonOptionControl, control_at, is_button_page};
use crate::visual::layout_metrics::{
    button_setting_hit_rect, dark_theme_rect, light_theme_rect, preset_tab_rect,
    scrollbar_off_rect, scrollbar_on_rect,
};
use crate::visual::selection_control_metrics;
use crate::visual::selection_screen_state::SelectionScreenAction;
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
    SelectionControl(SelectionScreenAction),
    CheckboxStateRead,
    CheckboxToggle,
    CheckboxReset,
    RadioStateRead,
    RadioSelect,
    RadioReset,
    ComboStateRead,
    ComboFilter,
    ComboSelect,
    ComboReset,
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
            Self::SelectionControl(action) => state.screen_state.register_selection_action(action),
            Self::CheckboxStateRead => state.screen_state.register_checkbox_state_read(),
            Self::CheckboxToggle => state.screen_state.register_checkbox_toggle(),
            Self::CheckboxReset => state.screen_state.register_checkbox_reset(),
            Self::RadioStateRead => state.screen_state.register_radio_state_read(),
            Self::RadioSelect => state.screen_state.register_radio_select(),
            Self::RadioReset => state.screen_state.register_radio_reset(),
            Self::ComboStateRead => state
                .screen_state
                .register_selection_action(SelectionScreenAction::ComboStateRead),
            Self::ComboFilter => state
                .screen_state
                .register_selection_action(SelectionScreenAction::ComboFilter),
            Self::ComboSelect => state
                .screen_state
                .register_selection_action(SelectionScreenAction::ComboOption(1)),
            Self::ComboReset => state
                .screen_state
                .register_selection_action(SelectionScreenAction::ComboReset),
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
        .or_else(|| selection_control_operation_at(state, x, y))
        .or_else(|| checkbox_operation_at(state.selected_page, x, y))
        .or_else(|| radio_operation_at(state.selected_page, x, y))
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

fn selection_control_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    let page = state.selected_page;
    let component = preview_detail::component_action_hit_rect(page);
    if page == "select-box" {
        if select_live::select_state_read_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::SelectionControl(
                SelectionScreenAction::SelectStateRead,
            ));
        }
        if select_live::select_open_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::SelectionControl(
                SelectionScreenAction::SelectOpen,
            ));
        }
        if select_live::select_close_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::SelectionControl(
                SelectionScreenAction::SelectClose,
            ));
        }
        if select_live::select_reset_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::SelectionControl(
                SelectionScreenAction::SelectReset,
            ));
        }
    }
    if page == "combo-box" {
        if combo_live::combo_state_read_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::ComboStateRead);
        }
        if combo_live::combo_filter_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::ComboFilter);
        }
        if combo_live::combo_select_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::ComboSelect);
        }
        if combo_live::combo_reset_button_rect(component.x, component.y).contains(x, y) {
            return Some(StorybookButtonOperation::ComboReset);
        }
    }
    let action = match page {
        "select-box" => selection_control_metrics::select_action_at(
            component,
            state.screen_state.selection.select_open,
            x,
            y,
        ),
        "combo-box" => combo_action_at(state, component, x, y),
        "selection-list" => selection_control_metrics::selection_list_action_at(component, x, y),
        _ => None,
    };
    action.map(StorybookButtonOperation::SelectionControl)
}

fn combo_action_at(
    state: &StorybookWindowState,
    component: crate::visual::layout_metrics::LayoutRect,
    x: usize,
    y: usize,
) -> Option<SelectionScreenAction> {
    let action = selection_control_metrics::combo_action_at(
        component,
        state.screen_state.selection.combo_open,
        state.screen_state.selection.combo_filtered,
        x,
        y,
    )?;
    match action {
        SelectionScreenAction::ComboOption(index)
            if state.screen_state.selection.combo_filtered =>
        {
            Some(SelectionScreenAction::ComboOption(index + 1))
        }
        _ => Some(action),
    }
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

fn checkbox_operation_at(
    page: &str,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if page != "checkbox" {
        return None;
    }
    let base = preview_detail::component_action_hit_rect(page);
    if binary_choice_live::checkbox_state_read_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::CheckboxStateRead);
    }
    if binary_choice_live::checkbox_toggle_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::CheckboxToggle);
    }
    if binary_choice_live::checkbox_reset_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::CheckboxReset);
    }
    None
}

fn radio_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if page != "radio" {
        return None;
    }
    let base = preview_detail::component_action_hit_rect(page);
    if binary_choice_live::radio_state_read_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioStateRead);
    }
    if binary_choice_live::radio_select_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioSelect);
    }
    if binary_choice_live::radio_reset_button_rect(base.x, base.y).contains(x, y) {
        return Some(StorybookButtonOperation::RadioReset);
    }
    None
}
