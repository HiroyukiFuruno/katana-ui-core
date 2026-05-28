use super::StorybookWindowState;
use crate::visual::button_options::{StorybookButtonOptionControl, control_at, is_button_page};
use crate::visual::dedicated_dod_form_binary_choice_live as binary_choice_live;
use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::dedicated_tabs;
use crate::visual::layout_metrics::{button_setting_hit_rect, dark_theme_rect, light_theme_rect};
use crate::visual::panel_options;
use crate::visual::panel_screen_state::{PanelChildKey, PanelOptionControl};
use crate::visual::preset_tab_scroll;
use crate::visual::screen_state_tabs::TabsScreenAction;
use crate::visual::search_box_screen_state::SearchBoxScreenAction;
use crate::visual::selection_screen_state::SelectionScreenAction;
use crate::visual::{preview, preview_detail};

#[path = "button_operation/selection_operation.rs"]
mod selection_operation;
#[path = "button_operation/text_input_operation.rs"]
mod text_input_operation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StorybookButtonOperation {
    LightTheme,
    DarkTheme,
    Preset(usize),
    PreviewButton,
    PreviewComponent,
    ButtonOption(StorybookButtonOptionControl),
    PanelOption(PanelOptionControl),
    PanelChild(PanelChildKey),
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
    SearchStateRead,
    SearchTypeQuery,
    SearchSubmit,
    SearchClear,
    SearchCaseToggle,
    SearchRegexToggle,
    TabsControl(TabsScreenAction),
    TextInputFocus {
        initial_value: &'static str,
        readonly: bool,
    },
    TextInputIconButton,
    TextAreaFocus,
}

impl StorybookButtonOperation {
    pub(super) fn apply(self, state: &mut StorybookWindowState) -> bool {
        match self {
            Self::LightTheme => state.theme_id = "light",
            Self::DarkTheme => state.theme_id = "dark",
            Self::Preset(index) => state.select_preset(index),
            Self::PreviewButton => state
                .screen_state
                .register_button_click(state.selected_page),
            Self::PreviewComponent => state
                .screen_state
                .register_preview_action(state.selected_page),
            Self::ButtonOption(control) => state.screen_state.register_button_option(control),
            Self::PanelOption(control) => state.screen_state.register_panel_option(control),
            Self::PanelChild(panel) => state.screen_state.register_panel_active_child(panel),
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
            Self::SearchStateRead => state
                .screen_state
                .register_search_box_action(SearchBoxScreenAction::StateRead),
            Self::SearchTypeQuery => state
                .screen_state
                .register_search_box_action(SearchBoxScreenAction::TypeQuery),
            Self::SearchSubmit => state
                .screen_state
                .register_search_box_action(SearchBoxScreenAction::Submit),
            Self::SearchClear => state
                .screen_state
                .register_search_box_action(SearchBoxScreenAction::Clear),
            Self::SearchCaseToggle => state
                .screen_state
                .register_search_box_action(SearchBoxScreenAction::ToggleCase),
            Self::SearchRegexToggle => state
                .screen_state
                .register_search_box_action(SearchBoxScreenAction::ToggleRegex),
            Self::TabsControl(action) => state.screen_state.register_tabs_action(action),
            Self::TextInputFocus {
                initial_value,
                readonly,
            } => state
                .screen_state
                .register_text_input_focus(initial_value, readonly),
            Self::TextInputIconButton => state.screen_state.register_text_input_icon_button(),
            Self::TextAreaFocus => state.screen_state.register_text_area_focus(),
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
        .or_else(|| preset_operation_at(state, x, y))
        .or_else(|| selection_operation::SelectionOperation::operation_at(state, x, y))
        .or_else(|| checkbox_operation_at(state.selected_page, x, y))
        .or_else(|| radio_operation_at(state.selected_page, x, y))
        .or_else(|| panel_operation_at(state.selected_page, x, y))
        .or_else(|| text_input_operation::operation_at(state, x, y))
        .or_else(|| text_area_operation_at(state, x, y))
        .or_else(|| tabs_operation_at(state, x, y))
        .or_else(|| preview_operation_at(state.selected_page, x, y))
        .or_else(|| settings_operation_at(state.selected_page, x, y))
}

pub(in crate::visual) fn apply_hover_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    let icon_button_changed = state.screen_state.set_hovered_text_input_icon_button_index(
        text_input_operation::hovered_icon_button_index_at(state, x, y),
    );
    let summary_changed = state
        .screen_state
        .set_hovered_summary_index(preview::summary_control_index_at(x, y));
    let hovered = preview_detail::component_action_hit_rect(state.selected_page).contains(x, y);
    state.screen_state.set_preview_hovered(hovered) || summary_changed || icon_button_changed
}

pub(super) fn is_button_preview_page(page: &str) -> bool {
    is_button_page(page)
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

fn preset_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    preset_tab_scroll::hit_index_at(state.selected_page, x, y, state.preset_tab_scroll_x)
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

fn panel_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
    if page != "panel" {
        return None;
    }
    if let Some(control) = panel_options::control_at(x, y) {
        return Some(StorybookButtonOperation::PanelOption(control));
    }
    let origin = preview_detail::component_action_hit_rect(page);
    crate::visual::dedicated_foundation_panel::panel_at(origin.x, origin.y, x, y)
        .map(StorybookButtonOperation::PanelChild)
}

fn text_area_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "text-area" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if input_live::text_area_rect_for_screen_state(origin.x, origin.y, &state.screen_state)
        .contains(x, y)
    {
        return Some(StorybookButtonOperation::TextAreaFocus);
    }
    None
}

fn tabs_operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<StorybookButtonOperation> {
    if state.selected_page != "tabs" {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    dedicated_tabs::control_at(origin.x, origin.y, x, y).map(StorybookButtonOperation::TabsControl)
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

fn checkbox_operation_at(page: &str, x: usize, y: usize) -> Option<StorybookButtonOperation> {
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
