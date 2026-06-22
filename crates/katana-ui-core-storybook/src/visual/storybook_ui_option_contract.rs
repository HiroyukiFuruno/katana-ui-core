use super::storybook_ui_form_options::{
    BUTTON_OPTIONS, COMBO_BOX_OPTIONS, INPUT_OPTIONS, SEARCH_BOX_OPTIONS,
    SEARCH_CONTROL_STRIP_OPTIONS,
};
use super::storybook_ui_foundation_options::{
    BADGE_OPTIONS, BINARY_SELECTION_OPTIONS, CONTENT_PRIMITIVE_OPTIONS, ICON_OPTIONS,
    LAYOUT_OPTIONS, LOADING_INDICATOR_OPTIONS, MENU_BUTTON_OPTIONS, MOTION_OPTIONS,
    PRIMITIVE_OPTIONS, PROGRESS_BAR_OPTIONS, SELECT_BOX_OPTIONS, SELECTION_LIST_OPTIONS,
    SKELETON_OPTIONS, SPLIT_PANE_OPTIONS, TEXT_OPTIONS, THEME_OPTIONS,
};
use super::storybook_ui_molecule_options::{
    ATTACHMENT_CHIP_OPTIONS, CHIP_GROUP_OPTIONS, CHIP_OPTIONS, COMMAND_PALETTE_OPTIONS,
    DIAGNOSTICS_LIST_OPTIONS, DYNAMIC_ARRAY_EDITOR_OPTIONS, EMPTY_STATE_OPTIONS,
    SETTINGS_LIST_OPTIONS, STATUS_BAR_OPTIONS,
};
use super::storybook_ui_runtime_options::{
    ACCORDION_OPTIONS, CLOSEABLE_TAB_STRIP_OPTIONS, CODE_DIFF_OPTIONS, CONTEXT_MENU_OPTIONS,
    DRAG_AND_DROP_OPTIONS, SHORTCUT_CHEATSHEET_OPTIONS, SHORTCUT_COMBO_OPTIONS,
    SKELETON_CLUSTER_OPTIONS, STARTUP_STATE_OPTIONS, TEXT_AREA_OPTIONS, TOOLBAR_OPTIONS,
    WINDOW_CONTROL_OPTIONS,
};
use super::storybook_ui_surface_options::{
    BANNER_OPTIONS, BREADCRUMB_OPTIONS, CARD_OPTIONS, COLLAPSIBLE_PANEL_OPTIONS,
    COLOR_PICKER_OPTIONS, FEEDBACK_OPTIONS, FORM_FIELD_OPTIONS, HOVER_CARD_OPTIONS, LIST_OPTIONS,
    MENU_OPTIONS, OVERLAY_OPTIONS, PANEL_OPTIONS, SIDE_MENU_OPTIONS, TREE_OPTIONS,
    VIRTUALIZATION_OPTIONS,
};
use super::storybook_ui_tabs_options::TABS_OPTIONS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StorybookUiOptionContract {
    pub(super) setting: &'static str,
    pub(super) before: &'static str,
    pub(super) after: &'static str,
}

impl StorybookUiOptionContract {
    pub(super) const fn new(
        setting: &'static str,
        before: &'static str,
        after: &'static str,
    ) -> Self {
        Self {
            setting,
            before,
            after,
        }
    }

    fn row(self) -> String {
        format!("{}: {} -> {}", self.setting, self.before, self.after)
    }
}

pub(super) fn settings_rows_for(page: &str) -> Vec<String> {
    options_for_page(page).iter().map(|it| it.row()).collect()
}

pub(super) fn options_for_page(page: &str) -> &'static [StorybookUiOptionContract] {
    match page {
        "theme-tokens" => &THEME_OPTIONS,
        "text" => &TEXT_OPTIONS,
        "icon" => &ICON_OPTIONS,
        "key-cap" => &CONTENT_PRIMITIVE_OPTIONS,
        "loading-dots" | "spinner" => &LOADING_INDICATOR_OPTIONS,
        "progress-bar" => &PROGRESS_BAR_OPTIONS,
        "divider" | "spacer" | "color-swatch" | "slide-control" => &PRIMITIVE_OPTIONS,
        "skeleton" => &SKELETON_OPTIONS,
        "motion" => &MOTION_OPTIONS,
        "button" | "text-button" | "svg-button" | "icon-text-button" => &BUTTON_OPTIONS,
        "text-input" => &INPUT_OPTIONS,
        "text-area" => &TEXT_AREA_OPTIONS,
        "search-box" => &SEARCH_BOX_OPTIONS,
        "combo-box" => &COMBO_BOX_OPTIONS,
        "search-control-strip" => &SEARCH_CONTROL_STRIP_OPTIONS,
        "checkbox" | "radio" | "toggle" | "segmented-toggle" => &BINARY_SELECTION_OPTIONS,
        "select-box" => &SELECT_BOX_OPTIONS,
        "selection-list" => &SELECTION_LIST_OPTIONS,
        "menu-button" => &MENU_BUTTON_OPTIONS,
        "badge" => &BADGE_OPTIONS,
        "chip" => &CHIP_OPTIONS,
        "attachment-chip" => &ATTACHMENT_CHIP_OPTIONS,
        "chip-group" => &CHIP_GROUP_OPTIONS,
        "tooltip" | "popover" | "modal" | "modal-overlay" => &OVERLAY_OPTIONS,
        "hover-card" => &HOVER_CARD_OPTIONS,
        "context-menu" => &CONTEXT_MENU_OPTIONS,
        "card" => &CARD_OPTIONS,
        "list" => &LIST_OPTIONS,
        "menu" => &MENU_OPTIONS,
        "form-field" => &FORM_FIELD_OPTIONS,
        "breadcrumb" => &BREADCRUMB_OPTIONS,
        "side-menu" => &SIDE_MENU_OPTIONS,
        "toolbar" => &TOOLBAR_OPTIONS,
        "tabs" => &TABS_OPTIONS,
        "accordion" => &ACCORDION_OPTIONS,
        "settings-list" => &SETTINGS_LIST_OPTIONS,
        "collapsible-panel" => &COLLAPSIBLE_PANEL_OPTIONS,
        "banner" => &BANNER_OPTIONS,
        "toast-stack-manager" | "notification-toast" => &FEEDBACK_OPTIONS,
        "status-bar" => &STATUS_BAR_OPTIONS,
        "empty-state" => &EMPTY_STATE_OPTIONS,
        "code-diff" => &CODE_DIFF_OPTIONS,
        "color-picker-rgba" => &COLOR_PICKER_OPTIONS,
        "dynamic-array-editor" => &DYNAMIC_ARRAY_EDITOR_OPTIONS,
        "command-palette" => &COMMAND_PALETTE_OPTIONS,
        "diagnostics-list" => &DIAGNOSTICS_LIST_OPTIONS,
        "virtualization" => &VIRTUALIZATION_OPTIONS,
        "row" | "column" | "stack" | "grid" | "scroll-area" | "align-center" => &LAYOUT_OPTIONS,
        "split-pane" => &SPLIT_PANE_OPTIONS,
        "skeleton-cluster" => &SKELETON_CLUSTER_OPTIONS,
        "shortcut-combo" => &SHORTCUT_COMBO_OPTIONS,
        "shortcut-cheatsheet" => &SHORTCUT_CHEATSHEET_OPTIONS,
        "drag-and-drop" => &DRAG_AND_DROP_OPTIONS,
        "window-control-button-group" => &WINDOW_CONTROL_OPTIONS,
        "startup-state-panel" => &STARTUP_STATE_OPTIONS,
        "closeable-tab-strip" => &CLOSEABLE_TAB_STRIP_OPTIONS,
        "tree-view" => &TREE_OPTIONS,
        "panel" => &PANEL_OPTIONS,
        _ => &[],
    }
}
