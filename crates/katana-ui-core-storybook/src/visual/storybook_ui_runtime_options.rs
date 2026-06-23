use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const CLOSEABLE_TAB_STRIP_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("active_tab_id", "preview", "settings"),
    StorybookUiOptionContract::new("tabs.overflow", "fit", "menu"),
    StorybookUiOptionContract::new("tabs.pin", "false", "true"),
    StorybookUiOptionContract::new("tabs.group", "none", "created"),
];

pub(super) const TEXT_AREA_OPTIONS: [StorybookUiOptionContract; 24] = [
    StorybookUiOptionContract::new("text_area.submit_key", "Enter", "ModEnter"),
    StorybookUiOptionContract::new("text_area.newline_key", "ShiftEnter", "Enter"),
    StorybookUiOptionContract::new("text_area.tab_behavior", "MoveFocus", "InsertTab"),
    StorybookUiOptionContract::new("text_area.auto_grow", "true", "false"),
    StorybookUiOptionContract::new("text_area.wrap_policy", "Soft", "None"),
    StorybookUiOptionContract::new("text_area.resize_enabled", "false", "true"),
    StorybookUiOptionContract::new("text_area.vertical_scroll_enabled", "false", "true"),
    StorybookUiOptionContract::new("text_area.horizontal_scroll_enabled", "false", "true"),
    StorybookUiOptionContract::new("text_area.vertical_scrollbar_visible", "false", "true"),
    StorybookUiOptionContract::new("text_area.horizontal_scrollbar_visible", "false", "true"),
    StorybookUiOptionContract::new("text_area.leading_slot.icon", "none", "search-svg"),
    StorybookUiOptionContract::new("text_area.trailing_icon_buttons", "none", "callbacks"),
    StorybookUiOptionContract::new("text_area.clear_action", "none", "visible"),
    StorybookUiOptionContract::new("text_area.value", "sample", "typed"),
    StorybookUiOptionContract::new("text_area.placeholder", "hidden", "visible"),
    StorybookUiOptionContract::new("text_area.font_role", "body", "monospace"),
    StorybookUiOptionContract::new("text_area.disabled", "false", "true"),
    StorybookUiOptionContract::new("text_area.readonly", "false", "true"),
    StorybookUiOptionContract::new("text_area.invalid", "false", "true"),
    StorybookUiOptionContract::new("text_area.min_rows", "2", "3"),
    StorybookUiOptionContract::new("text_area.max_rows", "6", "8"),
    StorybookUiOptionContract::new("text_area.ime_enabled", "true", "false"),
    StorybookUiOptionContract::new("text_area.leading_slot_reserved", "false", "true"),
    StorybookUiOptionContract::new("text_area.trailing_slot_reserved", "false", "true"),
];

pub(super) const SHORTCUT_COMBO_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("shortcut_combo.platform_display", "Auto", "MacOS"),
    StorybookUiOptionContract::new("shortcut_combo.separator", "Plus", "None"),
    StorybookUiOptionContract::new("shortcut_combo.size", "Medium", "Large"),
    StorybookUiOptionContract::new("shortcut_combo.tone", "Neutral", "Accent"),
    StorybookUiOptionContract::new("shortcut_combo.a11y_label", "generated", "custom"),
];

pub(super) const SHORTCUT_CHEATSHEET_OPTIONS: [StorybookUiOptionContract; 9] = [
    StorybookUiOptionContract::new("shortcut_cheatsheet.label", "Shortcuts", "Editor keys"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.groups", "2", "3"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.group_title", "Editing", "Navigation"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.items", "2", "4"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.item_combo", "Cmd+F", "Cmd+Shift+P"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.group_layout", "TwoColumn", "OneColumn"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.query", "format", "カテゴリ"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.selected", "None", "format"),
    StorybookUiOptionContract::new("shortcut_cheatsheet.result_count", "2", "1"),
];

pub(super) const DRAG_AND_DROP_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("drag.accept_policy", "Reject", "Accept(after)"),
    StorybookUiOptionContract::new("drag.autoscroll", "disabled", "edge=24"),
    StorybookUiOptionContract::new("drag.keyboard_draggable", "false", "true"),
    StorybookUiOptionContract::new("drag.drop_indicator", "none", "after"),
];

pub(super) const SKELETON_CLUSTER_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("skeleton_cluster.preset", "ListRow", "Card"),
    StorybookUiOptionContract::new("skeleton_cluster.children", "2", "3"),
    StorybookUiOptionContract::new("skeleton_cluster.live_region", "list", "card"),
    StorybookUiOptionContract::new("skeleton_cluster.reduced_motion", "false", "true"),
];

pub(super) const WINDOW_CONTROL_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("window_control.position", "Leading", "Trailing"),
    StorybookUiOptionContract::new("window_control.size", "Compact", "Tall"),
    StorybookUiOptionContract::new(
        "window_control.controls",
        "Close+Minimize+Maximize",
        "Close",
    ),
    StorybookUiOptionContract::new("window_control.visibility", "Always", "Hover"),
];

pub(super) const TOOLBAR_OPTIONS: [StorybookUiOptionContract; 18] = [
    StorybookUiOptionContract::new("toolbar.display_mode", "IconLeading", "IconOnly"),
    StorybookUiOptionContract::new("toolbar.density", "Default", "Compact"),
    StorybookUiOptionContract::new("toolbar.overflow_strategy", "Menu", "Custom"),
    StorybookUiOptionContract::new("toolbar.actions", "3", "4"),
    StorybookUiOptionContract::new("toolbar.groups", "primary", "primary+secondary"),
    StorybookUiOptionContract::new("toolbar.context_menu_anchor", "none", "pointer"),
    StorybookUiOptionContract::new("toolbar.action_priority", "normal", "critical"),
    StorybookUiOptionContract::new("toolbar.action_accelerator", "Cmd+S", "Alt+P"),
    StorybookUiOptionContract::new("toolbar.action_split", "none", "menu"),
    StorybookUiOptionContract::new("toolbar.action_group", "primary", "edit"),
    StorybookUiOptionContract::new("toolbar.action_tooltip", "none", "Save file"),
    StorybookUiOptionContract::new("toolbar.action_a11y", "label", "custom"),
    StorybookUiOptionContract::new("toolbar.action_disabled", "false", "true"),
    StorybookUiOptionContract::new("toolbar.group_label", "none", "File actions"),
    StorybookUiOptionContract::new("toolbar.group_divider", "true", "false"),
    StorybookUiOptionContract::new("toolbar.split_disabled", "false", "true"),
    StorybookUiOptionContract::new("toolbar.split_tooltip", "none", "More save options"),
    StorybookUiOptionContract::new("toolbar.split_a11y", "generated", "custom"),
];

pub(super) const CONTEXT_MENU_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("context_menu.anchor", "Pointer(192,128)", "Pointer(0,0)"),
    StorybookUiOptionContract::new(
        "context_menu.placement_priority",
        "BelowStart>AboveEnd",
        "AboveEnd>BelowStart",
    ),
    StorybookUiOptionContract::new("context_menu.placement_used", "BelowStart", "AboveEnd"),
    StorybookUiOptionContract::new("context_menu.min_width", "240", "280"),
    StorybookUiOptionContract::new("context_menu.max_height", "260", "320"),
];

pub(super) const STARTUP_STATE_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("startup_state.state", "Loading", "Error"),
    StorybookUiOptionContract::new("startup_state.progress", "None", "64"),
    StorybookUiOptionContract::new(
        "startup_state.label",
        "Preparing session",
        "Loading workspace",
    ),
    StorybookUiOptionContract::new("startup_state.retry", "false", "true"),
    StorybookUiOptionContract::new("startup_state.cancel", "false", "true"),
];

pub(super) const ACCORDION_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("accordion.expanded", "false", "true"),
    StorybookUiOptionContract::new("accordion.disabled", "false", "true"),
    StorybookUiOptionContract::new("accordion.controlled", "false", "true"),
    StorybookUiOptionContract::new("accordion.trigger_area", "icon", "full-row"),
    StorybookUiOptionContract::new("accordion.reduced_motion", "false", "true"),
];

pub(super) const CODE_DIFF_OPTIONS: [StorybookUiOptionContract; 7] = [
    StorybookUiOptionContract::new("code_diff.mode", "Unified", "Split"),
    StorybookUiOptionContract::new("code_diff.whitespace", "Preserve", "Visible"),
    StorybookUiOptionContract::new("code_diff.direction", "Horizontal", "Vertical"),
    StorybookUiOptionContract::new("code_diff.context_lines", "3", "0"),
    StorybookUiOptionContract::new("code_diff.item_count", "5", "3"),
    StorybookUiOptionContract::new("code_diff.scroll_sync", "true", "false"),
    StorybookUiOptionContract::new("code_diff.language", "rust", "markdown"),
];
