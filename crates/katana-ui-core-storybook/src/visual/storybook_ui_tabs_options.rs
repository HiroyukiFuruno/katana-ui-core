use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const TABS_OPTIONS: [StorybookUiOptionContract; 17] = [
    StorybookUiOptionContract::new("tabs.add", "none", "notes.md"),
    StorybookUiOptionContract::new("tabs.close", "unpinned", "removed"),
    StorybookUiOptionContract::new("tabs.pin", "false", "left-fixed"),
    StorybookUiOptionContract::new("tabs.move", "index=3", "index=4"),
    StorybookUiOptionContract::new("tabs.group", "none", "Docs"),
    StorybookUiOptionContract::new("tabs.overflow", "hidden", "menu"),
    StorybookUiOptionContract::new("tabs.active_scroll", "manual", "follow"),
    StorybookUiOptionContract::new("tabs.icon", "none", "svg"),
    StorybookUiOptionContract::new("tabs.dirty", "false", "true"),
    StorybookUiOptionContract::new("tabs.closeable", "true", "false"),
    StorybookUiOptionContract::new("tabs.tone", "default", "warning"),
    StorybookUiOptionContract::new("tabs.tooltip", "none", "visible"),
    StorybookUiOptionContract::new("tabs.accessibility_label", "title", "custom"),
    StorybookUiOptionContract::new("tabs.group_color", "default", "accent"),
    StorybookUiOptionContract::new("tabs.group_collapsed", "false", "true"),
    StorybookUiOptionContract::new("tabs.overflow_width", "44", "72"),
    StorybookUiOptionContract::new("tabs.group_auto_expand", "500", "1000"),
];
