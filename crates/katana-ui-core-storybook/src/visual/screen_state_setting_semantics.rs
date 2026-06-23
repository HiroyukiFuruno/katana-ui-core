use super::screen_state_setting_semantics_chip::{
    attachment_chip_state, chip_group_state, chip_state,
};
use super::screen_state_setting_semantics_collection::{
    breadcrumb_state, collapsible_panel_state, form_field_state, hover_card_state, list_state,
    menu_state, panel_state, side_menu_state, tree_state,
};
use super::screen_state_setting_semantics_core::{
    color_picker_state, search_control_state, settings_list_state, status_bar_state, toolbar_state,
    virtualization_state,
};
use super::screen_state_setting_semantics_foundation::{
    binary_choice_state, icon_state, layout_state, loading_indicator_state, primitive_state,
    progress_bar_state, skeleton_state, split_pane_state, text_state,
};
use super::screen_state_setting_semantics_foundation_extra::{
    key_cap_state, motion_state, theme_state,
};
use super::screen_state_setting_semantics_live::{drag_and_drop_state, dynamic_array_state};
use super::screen_state_setting_semantics_overlay::{
    modal_overlay_state, modal_state, popover_state, tooltip_state,
};
use super::screen_state_setting_semantics_selection::{
    combo_box_state, menu_button_state, search_box_state, select_box_state, selection_list_state,
};
use super::screen_state_setting_semantics_specialized::{
    accordion_state, code_diff_state, command_palette_state, context_menu_state,
    diagnostics_list_state, shortcut_cheatsheet_state, shortcut_combo_state,
    skeleton_cluster_state, startup_state, window_control_state,
};
use super::screen_state_setting_semantics_surface::{
    badge_state, banner_state, card_state, empty_state_state, feedback_state,
};
use super::screen_state_setting_semantics_text_entry::{text_area_state, text_input_state};
use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(in crate::visual) fn semantic_setting_state(
    page: &str,
    option: StorybookUiOptionContract,
) -> &'static str {
    match page {
        "theme-tokens" => theme_state(option.setting),
        "toolbar" => toolbar_state(option.setting),
        "settings-list" => settings_list_state(option.setting),
        "color-picker-rgba" => color_picker_state(option.setting),
        "text" => text_state(option.setting),
        "icon" => icon_state(option.setting),
        "key-cap" => key_cap_state(option.setting),
        "skeleton" => skeleton_state(option.setting),
        "motion" => motion_state(option.setting),
        "loading-dots" => loading_indicator_state("loading_dots", option.setting),
        "spinner" => loading_indicator_state("spinner", option.setting),
        "progress-bar" => progress_bar_state(option.setting),
        "split-pane" => split_pane_state(option.setting),
        "scroll-area" => layout_state("scroll_area", option.setting),
        "align-center" => layout_state("align_center", option.setting),
        "divider" => primitive_state("divider", option.setting),
        "spacer" => primitive_state("spacer", option.setting),
        "color-swatch" => primitive_state("color_swatch", option.setting),
        "slide-control" => primitive_state("slide_control", option.setting),
        "checkbox" => binary_choice_state("checkbox", option.setting),
        "radio" => binary_choice_state("radio", option.setting),
        "toggle" => binary_choice_state("toggle", option.setting),
        "segmented-toggle" => binary_choice_state("segmented_toggle", option.setting),
        "text-input" => text_input_state(option.setting),
        "text-area" => text_area_state(option.setting),
        "badge" => badge_state(option.setting),
        "banner" => banner_state(option.setting),
        "card" => card_state(option.setting),
        "empty-state" => empty_state_state(option.setting),
        "toast-stack-manager" => feedback_state("toast_stack", option.setting),
        "notification-toast" => feedback_state("notification_toast", option.setting),
        "hover-card" => hover_card_state(option.setting),
        "menu" => menu_state(option.setting),
        "form-field" => form_field_state(option.setting),
        "breadcrumb" => breadcrumb_state(option.setting),
        "side-menu" => side_menu_state(option.setting),
        "list" => list_state(option.setting),
        "collapsible-panel" => collapsible_panel_state(option.setting),
        "tree-view" => tree_state(option.setting),
        "panel" => panel_state(option.setting),
        "virtualization" => virtualization_state(option.setting),
        "search-control-strip" => search_control_state(option.setting),
        "status-bar" => status_bar_state(option.setting),
        "chip" => chip_state(option.setting),
        "attachment-chip" => attachment_chip_state(option.setting),
        "chip-group" => chip_group_state(option.setting),
        "command-palette" => command_palette_state(option.setting),
        "dynamic-array-editor" => dynamic_array_state(option.setting),
        "diagnostics-list" => diagnostics_list_state(option.setting),
        "shortcut-cheatsheet" => shortcut_cheatsheet_state(option.setting),
        "context-menu" => context_menu_state(option.setting),
        "startup-state-panel" => startup_state(option.setting),
        "code-diff" => code_diff_state(option.setting),
        "shortcut-combo" => shortcut_combo_state(option.setting),
        "skeleton-cluster" => skeleton_cluster_state(option.setting),
        "window-control-button-group" => window_control_state(option.setting),
        "accordion" => accordion_state(option.setting),
        "drag-and-drop" => drag_and_drop_state(option.setting),
        "tooltip" => tooltip_state(option.setting),
        "popover" => popover_state(option.setting),
        "modal" => modal_state(option.setting),
        "modal-overlay" => modal_overlay_state(option.setting),
        "search-box" => search_box_state(option.setting),
        "combo-box" => combo_box_state(option.setting),
        "select-box" => select_box_state(option.setting),
        "selection-list" => selection_list_state(option.setting),
        "menu-button" => menu_button_state(option.setting),
        _ => option.setting,
    }
}
