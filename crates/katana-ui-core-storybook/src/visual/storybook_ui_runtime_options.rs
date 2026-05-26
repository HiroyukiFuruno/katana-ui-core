use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const RUNTIME_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("platform", "auto", "mac"),
    StorybookUiOptionContract::new("state", "idle", "active"),
    StorybookUiOptionContract::new("event", "none", "logged"),
    StorybookUiOptionContract::new("presentation", "default", "changed"),
];

pub(super) const SHORTCUT_COMBO_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("shortcut_combo.platform_display", "Auto", "MacOS"),
    StorybookUiOptionContract::new("shortcut_combo.separator", "Plus", "None"),
    StorybookUiOptionContract::new("shortcut_combo.size", "Medium", "Large"),
    StorybookUiOptionContract::new("shortcut_combo.tone", "Neutral", "Accent"),
];

pub(super) const SHORTCUT_CHEATSHEET_OPTIONS: [StorybookUiOptionContract; 4] = [
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

pub(super) const CODE_DIFF_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("code_diff.mode", "Unified", "Split"),
    StorybookUiOptionContract::new("code_diff.whitespace", "Preserve", "Visible"),
    StorybookUiOptionContract::new("code_diff.context_lines", "3", "0"),
    StorybookUiOptionContract::new("code_diff.scroll_sync", "true", "false"),
    StorybookUiOptionContract::new("code_diff.language", "rust", "markdown"),
];
