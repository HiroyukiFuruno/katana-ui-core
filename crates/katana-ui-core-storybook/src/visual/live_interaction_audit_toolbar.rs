use crate::visual::{
    dedicated_status_bar, dedicated_toolbar,
    layout_metrics::LayoutRect,
    window_interaction::{
        apply_align_center_resize_for_audit, apply_clickable_keyboard_activation_for_audit,
        apply_column_resize_for_audit, apply_context_click,
        apply_diagnostics_list_scroll_for_audit, apply_drag_and_drop_drag_for_audit,
        apply_drag_and_drop_resize_for_audit, apply_drag_and_drop_scroll_for_audit,
        apply_grid_resize_for_audit, apply_hover_at, apply_panel_resize_for_audit,
        apply_row_resize_for_audit, apply_settings_list_scroll_for_audit,
        apply_shortcut_cheatsheet_scroll_for_audit, apply_stack_resize_for_audit,
        apply_theme_tokens_resize_for_audit, apply_tree_view_scroll_for_audit,
        apply_virtualization_scroll_for_audit, focus_clickable_at_for_audit,
    },
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const PAGE: &str = "toolbar";
const COLLAPSIBLE_PANEL_PAGE: &str = "collapsible-panel";
const MOTION_PAGE: &str = "motion";
const WINDOW_CONTROL_PAGE: &str = "window-control-button-group";
const STARTUP_STATE_PAGE: &str = "startup-state-panel";
const ATTACHMENT_CHIP_PAGE: &str = "attachment-chip";
const CHIP_GROUP_PAGE: &str = "chip-group";
const DIAGNOSTICS_LIST_PAGE: &str = "diagnostics-list";
const EMPTY_STATE_PAGE: &str = "empty-state";
const TREE_VIEW_PAGE: &str = "tree-view";
const DRAG_AND_DROP_PAGE: &str = "drag-and-drop";
const PANEL_PAGE: &str = "panel";
const ROW_PAGE: &str = "row";
const COLUMN_PAGE: &str = "column";
const STACK_PAGE: &str = "stack";
const GRID_PAGE: &str = "grid";
const ALIGN_CENTER_PAGE: &str = "align-center";
const VIRTUALIZATION_PAGE: &str = "virtualization";
const SETTINGS_LIST_PAGE: &str = "settings-list";
const SKELETON_CLUSTER_PAGE: &str = "skeleton-cluster";
const SHORTCUT_CHEATSHEET_PAGE: &str = "shortcut-cheatsheet";
const SHORTCUT_COMBO_PAGE: &str = "shortcut-combo";
const STATUS_BAR_PAGE: &str = "status-bar";
const THEME_TOKENS_PAGE: &str = "theme-tokens";
const CLICK_OFFSET: usize = 4;
const SAVE_ACTION_INDEX: usize = 0;
const BRANCH_SEGMENT_INDEX: usize = 0;
const MOTION_KEYBOARD_PHASE: u16 = 3;
const MOTION_KEYBOARD_STATE_LABEL: &str = "phase=3";

#[path = "live_interaction_audit_toolbar_collapsible_virtualization.rs"]
mod live_interaction_audit_toolbar_collapsible_virtualization;
use live_interaction_audit_toolbar_collapsible_virtualization::{
    collapsible_panel_context_scenario, collapsible_panel_focus_scenario,
    collapsible_panel_hover_scenario, collapsible_panel_keyboard_scenario,
    virtualization_focus_scenario, virtualization_keyboard_scenario,
    virtualization_scroll_scenario,
};
#[path = "live_interaction_audit_toolbar_diagnostics_empty.rs"]
mod live_interaction_audit_toolbar_diagnostics_empty;
use live_interaction_audit_toolbar_diagnostics_empty::{
    diagnostics_list_focus_scenario, diagnostics_list_hover_scenario,
    diagnostics_list_keyboard_scenario, diagnostics_list_scroll_scenario,
    empty_state_focus_scenario, empty_state_hover_scenario, empty_state_keyboard_scenario,
};
#[path = "live_interaction_audit_toolbar_tree_settings.rs"]
mod live_interaction_audit_toolbar_tree_settings;
use live_interaction_audit_toolbar_tree_settings::{
    settings_list_focus_scenario, settings_list_hover_scenario, settings_list_keyboard_scenario,
    settings_list_scroll_scenario, tree_view_focus_scenario, tree_view_hover_scenario,
    tree_view_keyboard_scenario, tree_view_scroll_scenario,
};
#[path = "live_interaction_audit_toolbar_drag.rs"]
mod live_interaction_audit_toolbar_drag;
use live_interaction_audit_toolbar_drag::{
    drag_and_drop_drag_scenario, drag_and_drop_focus_scenario, drag_and_drop_hover_scenario,
    drag_and_drop_keyboard_scenario, drag_and_drop_resize_scenario, drag_and_drop_scroll_scenario,
};
#[path = "live_interaction_audit_toolbar_layout_panel_row.rs"]
mod live_interaction_audit_toolbar_layout_panel_row;
use live_interaction_audit_toolbar_layout_panel_row::{
    panel_focus_scenario, panel_hover_scenario, panel_keyboard_scenario, panel_resize_scenario,
    row_focus_scenario, row_hover_scenario, row_keyboard_scenario, row_resize_scenario,
};
#[path = "live_interaction_audit_toolbar_layout_column_stack.rs"]
mod live_interaction_audit_toolbar_layout_column_stack;
use live_interaction_audit_toolbar_layout_column_stack::{
    column_focus_scenario, column_hover_scenario, column_keyboard_scenario, column_resize_scenario,
    stack_focus_scenario, stack_hover_scenario, stack_keyboard_scenario, stack_resize_scenario,
};
#[path = "live_interaction_audit_toolbar_layout_grid_align.rs"]
mod live_interaction_audit_toolbar_layout_grid_align;
use live_interaction_audit_toolbar_layout_grid_align::{
    align_center_focus_scenario, align_center_hover_scenario, align_center_keyboard_scenario,
    align_center_resize_scenario, grid_focus_scenario, grid_hover_scenario, grid_keyboard_scenario,
    grid_resize_scenario,
};
#[path = "live_interaction_audit_toolbar_runtime_window.rs"]
mod live_interaction_audit_toolbar_runtime_window;
use live_interaction_audit_toolbar_runtime_window::{
    motion_focus_scenario, motion_hover_scenario, motion_keyboard_scenario,
    skeleton_cluster_focus_scenario, skeleton_cluster_hover_scenario,
    skeleton_cluster_keyboard_scenario, window_control_focus_scenario,
    window_control_hover_scenario, window_control_keyboard_scenario,
};
#[path = "live_interaction_audit_toolbar_chip_startup.rs"]
mod live_interaction_audit_toolbar_chip_startup;
use live_interaction_audit_toolbar_chip_startup::{
    attachment_chip_focus_scenario, attachment_chip_hover_scenario,
    attachment_chip_keyboard_scenario, chip_group_focus_scenario, chip_group_hover_scenario,
    chip_group_keyboard_scenario, startup_state_focus_scenario, startup_state_hover_scenario,
    startup_state_keyboard_scenario,
};
#[path = "live_interaction_audit_toolbar_shortcut_cheatsheet.rs"]
mod live_interaction_audit_toolbar_shortcut_cheatsheet;
use live_interaction_audit_toolbar_shortcut_cheatsheet::{
    shortcut_cheatsheet_focus_scenario, shortcut_cheatsheet_hover_scenario,
    shortcut_cheatsheet_keyboard_scenario, shortcut_cheatsheet_scroll_scenario,
};
#[path = "live_interaction_audit_toolbar_shortcut_theme.rs"]
mod live_interaction_audit_toolbar_shortcut_theme;
use live_interaction_audit_toolbar_shortcut_theme::{
    shortcut_combo_focus_scenario, shortcut_combo_hover_scenario, shortcut_combo_keyboard_scenario,
    theme_tokens_focus_scenario, theme_tokens_hover_scenario, theme_tokens_keyboard_scenario,
    theme_tokens_resize_scenario,
};
#[path = "live_interaction_audit_toolbar_status.rs"]
mod live_interaction_audit_toolbar_status;
use live_interaction_audit_toolbar_status::{
    status_bar_focus_scenario, status_bar_hover_scenario, status_bar_keyboard_scenario,
};
#[path = "live_interaction_audit_toolbar_page.rs"]
mod live_interaction_audit_toolbar_page;
use live_interaction_audit_toolbar_page::{
    toolbar_focus_scenario, toolbar_hover_scenario, toolbar_keyboard_scenario,
};
pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        COLLAPSIBLE_PANEL_PAGE => vec![
            collapsible_panel_hover_scenario(),
            collapsible_panel_focus_scenario(),
            collapsible_panel_keyboard_scenario(),
            collapsible_panel_context_scenario(),
        ],
        VIRTUALIZATION_PAGE => vec![
            virtualization_focus_scenario(),
            virtualization_keyboard_scenario(),
            virtualization_scroll_scenario(),
        ],
        DIAGNOSTICS_LIST_PAGE => vec![
            diagnostics_list_hover_scenario(),
            diagnostics_list_focus_scenario(),
            diagnostics_list_keyboard_scenario(),
            diagnostics_list_scroll_scenario(),
        ],
        EMPTY_STATE_PAGE => vec![
            empty_state_hover_scenario(),
            empty_state_focus_scenario(),
            empty_state_keyboard_scenario(),
        ],
        TREE_VIEW_PAGE => vec![
            tree_view_hover_scenario(),
            tree_view_focus_scenario(),
            tree_view_keyboard_scenario(),
            tree_view_scroll_scenario(),
        ],
        DRAG_AND_DROP_PAGE => vec![
            drag_and_drop_hover_scenario(),
            drag_and_drop_focus_scenario(),
            drag_and_drop_keyboard_scenario(),
            drag_and_drop_drag_scenario(),
            drag_and_drop_scroll_scenario(),
            drag_and_drop_resize_scenario(),
        ],
        PANEL_PAGE => vec![
            panel_hover_scenario(),
            panel_focus_scenario(),
            panel_keyboard_scenario(),
            panel_resize_scenario(),
        ],
        ROW_PAGE => vec![
            row_hover_scenario(),
            row_focus_scenario(),
            row_keyboard_scenario(),
            row_resize_scenario(),
        ],
        COLUMN_PAGE => vec![
            column_hover_scenario(),
            column_focus_scenario(),
            column_keyboard_scenario(),
            column_resize_scenario(),
        ],
        STACK_PAGE => vec![
            stack_hover_scenario(),
            stack_focus_scenario(),
            stack_keyboard_scenario(),
            stack_resize_scenario(),
        ],
        GRID_PAGE => vec![
            grid_hover_scenario(),
            grid_focus_scenario(),
            grid_keyboard_scenario(),
            grid_resize_scenario(),
        ],
        ALIGN_CENTER_PAGE => vec![
            align_center_hover_scenario(),
            align_center_focus_scenario(),
            align_center_keyboard_scenario(),
            align_center_resize_scenario(),
        ],
        SKELETON_CLUSTER_PAGE => vec![
            skeleton_cluster_hover_scenario(),
            skeleton_cluster_focus_scenario(),
            skeleton_cluster_keyboard_scenario(),
        ],
        MOTION_PAGE => vec![
            motion_hover_scenario(),
            motion_focus_scenario(),
            motion_keyboard_scenario(),
        ],
        WINDOW_CONTROL_PAGE => vec![
            window_control_hover_scenario(),
            window_control_focus_scenario(),
            window_control_keyboard_scenario(),
        ],
        STARTUP_STATE_PAGE => vec![
            startup_state_hover_scenario(),
            startup_state_focus_scenario(),
            startup_state_keyboard_scenario(),
        ],
        ATTACHMENT_CHIP_PAGE => vec![
            attachment_chip_hover_scenario(),
            attachment_chip_focus_scenario(),
            attachment_chip_keyboard_scenario(),
        ],
        CHIP_GROUP_PAGE => vec![
            chip_group_hover_scenario(),
            chip_group_focus_scenario(),
            chip_group_keyboard_scenario(),
        ],
        STATUS_BAR_PAGE => vec![
            status_bar_hover_scenario(),
            status_bar_focus_scenario(),
            status_bar_keyboard_scenario(),
        ],
        THEME_TOKENS_PAGE => vec![
            theme_tokens_hover_scenario(),
            theme_tokens_focus_scenario(),
            theme_tokens_keyboard_scenario(),
            theme_tokens_resize_scenario(),
        ],
        SHORTCUT_COMBO_PAGE => vec![
            shortcut_combo_hover_scenario(),
            shortcut_combo_focus_scenario(),
            shortcut_combo_keyboard_scenario(),
        ],
        SHORTCUT_CHEATSHEET_PAGE => vec![
            shortcut_cheatsheet_hover_scenario(),
            shortcut_cheatsheet_focus_scenario(),
            shortcut_cheatsheet_keyboard_scenario(),
            shortcut_cheatsheet_scroll_scenario(),
        ],
        SETTINGS_LIST_PAGE => vec![
            settings_list_hover_scenario(),
            settings_list_focus_scenario(),
            settings_list_keyboard_scenario(),
            settings_list_scroll_scenario(),
        ],
        PAGE => vec![
            toolbar_hover_scenario(),
            toolbar_focus_scenario(),
            toolbar_keyboard_scenario(),
        ],
        _ => Vec::new(),
    }
}
