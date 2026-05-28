use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const TABS_OPTIONS: [StorybookUiOptionContract; 7] = [
    StorybookUiOptionContract::new("tabs.add", "none", "notes.md"),
    StorybookUiOptionContract::new("tabs.close", "unpinned", "removed"),
    StorybookUiOptionContract::new("tabs.pin", "false", "left-fixed"),
    StorybookUiOptionContract::new("tabs.move", "index=3", "index=4"),
    StorybookUiOptionContract::new("tabs.group", "none", "Docs"),
    StorybookUiOptionContract::new("tabs.overflow", "hidden", "menu"),
    StorybookUiOptionContract::new("tabs.active_scroll", "manual", "follow"),
];
