use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const STATUS_BAR_OPTIONS: [StorybookUiOptionContract; 8] = [
    StorybookUiOptionContract::new("status_bar.mode", "SingleMessage", "MultiSegment"),
    StorybookUiOptionContract::new("status_bar.segments", "1", "4"),
    StorybookUiOptionContract::new("status_bar.density", "Default", "Compact"),
    StorybookUiOptionContract::new("status_bar.progress_popover", "false", "true"),
    StorybookUiOptionContract::new("status_bar.message", "none", "Ready"),
    StorybookUiOptionContract::new("status_bar.severity", "Neutral", "Warning"),
    StorybookUiOptionContract::new("status_bar.dismiss", "none", "available"),
    StorybookUiOptionContract::new("status_bar.segment_a11y", "label", "custom"),
];

pub(super) const CHIP_OPTIONS: [StorybookUiOptionContract; 12] = [
    StorybookUiOptionContract::new("chip.label", "filter: docs", "filter: rust"),
    StorybookUiOptionContract::new("chip.leading_icon", "filter", "tag"),
    StorybookUiOptionContract::new("chip.trailing_icon", "none", "close"),
    StorybookUiOptionContract::new("chip.variant", "Outline", "Filled"),
    StorybookUiOptionContract::new("chip.tone", "Accent", "Danger"),
    StorybookUiOptionContract::new("chip.size", "Medium", "Large"),
    StorybookUiOptionContract::new("chip.interactive", "false", "true"),
    StorybookUiOptionContract::new("chip.selected", "false", "true"),
    StorybookUiOptionContract::new("chip.disabled", "false", "true"),
    StorybookUiOptionContract::new("chip.dismissible", "false", "true"),
    StorybookUiOptionContract::new("chip.a11y_label", "none", "Filter chip"),
    StorybookUiOptionContract::new("chip.focused", "false", "true"),
];

pub(super) const ATTACHMENT_CHIP_OPTIONS: [StorybookUiOptionContract; 7] = [
    StorybookUiOptionContract::new("attachment.kind", "File", "Image"),
    StorybookUiOptionContract::new("attachment.name", "design.md", "proposal.pdf"),
    StorybookUiOptionContract::new("attachment.meta", "none", "size+mime"),
    StorybookUiOptionContract::new("attachment.thumbnail", "none", "preview"),
    StorybookUiOptionContract::new("attachment.status", "Uploading", "Error"),
    StorybookUiOptionContract::new("attachment.progress", "42", "100"),
    StorybookUiOptionContract::new("attachment.retry", "hidden", "visible"),
];

pub(super) const CHIP_GROUP_OPTIONS: [StorybookUiOptionContract; 9] = [
    StorybookUiOptionContract::new("chip_group.label", "Filters", "Active filters"),
    StorybookUiOptionContract::new("chip_group.chip_count", "3", "5"),
    StorybookUiOptionContract::new("chip_group.wrap", "false", "true"),
    StorybookUiOptionContract::new("chip_group.overflow", "Inline", "Menu"),
    StorybookUiOptionContract::new("chip_group.reorder", "false", "true"),
    StorybookUiOptionContract::new("chip_group.gap", "0", "8"),
    StorybookUiOptionContract::new("chip_group.available_width", "88", "132"),
    StorybookUiOptionContract::new("chip_group.overflow_trigger_width", "24", "32"),
    StorybookUiOptionContract::new("chip_group.hidden_count", "0", "2"),
];

pub(super) const EMPTY_STATE_OPTIONS: [StorybookUiOptionContract; 8] = [
    StorybookUiOptionContract::new("empty_state.heading", "No diagnostics", "Empty project"),
    StorybookUiOptionContract::new("empty_state.body", "mixed text", "create a file"),
    StorybookUiOptionContract::new("empty_state.icon", "none", "search"),
    StorybookUiOptionContract::new("empty_state.illustration", "none", "folder"),
    StorybookUiOptionContract::new("empty_state.tone", "Accent", "Danger"),
    StorybookUiOptionContract::new("empty_state.size", "Default", "Large"),
    StorybookUiOptionContract::new("empty_state.alignment", "Center", "Leading"),
    StorybookUiOptionContract::new("empty_state.actions", "Primary", "Primary+Secondary"),
];

pub(super) const DIAGNOSTICS_LIST_OPTIONS: [StorybookUiOptionContract; 7] = [
    StorybookUiOptionContract::new("diagnostics.group_by", "Severity", "Source"),
    StorybookUiOptionContract::new("diagnostics.sort_by", "Severity", "Location"),
    StorybookUiOptionContract::new("diagnostics.severity_filter", "Error+Warning", "Error"),
    StorybookUiOptionContract::new("diagnostics.wrap_error_navigation", "true", "false"),
    StorybookUiOptionContract::new("diagnostics.virtualization", "None", "Windowed"),
    StorybookUiOptionContract::new("diagnostics.bulk_action", "Preview", "Apply"),
    StorybookUiOptionContract::new("diagnostics.fix_preview", "Expanded", "Collapsed"),
];

pub(super) const SETTINGS_LIST_OPTIONS: [StorybookUiOptionContract; 19] = [
    StorybookUiOptionContract::new("settings_list.label", "Settings", "Workspace settings"),
    StorybookUiOptionContract::new("settings_list.density", "Default", "Compact"),
    StorybookUiOptionContract::new("settings_list.dirty_visualization", "Marker", "Highlight"),
    StorybookUiOptionContract::new("settings_list.query", "None", "format"),
    StorybookUiOptionContract::new("settings_list.sections", "app+chat+lint", "app+lint"),
    StorybookUiOptionContract::new("settings_list.section_label", "App settings", "Editor"),
    StorybookUiOptionContract::new("settings_list.section_description", "none", "visible"),
    StorybookUiOptionContract::new("settings_list.section_icon", "none", "gear"),
    StorybookUiOptionContract::new("settings_list.field_count", "3", "5"),
    StorybookUiOptionContract::new("settings_list.section_footer", "none", "policy"),
    StorybookUiOptionContract::new("settings_list.section_collapsible", "false", "true"),
    StorybookUiOptionContract::new("settings_list.default_collapsed", "false", "true"),
    StorybookUiOptionContract::new("settings_list.field_label", "Format on save", "Font size"),
    StorybookUiOptionContract::new("settings_list.field_description", "none", "visible"),
    StorybookUiOptionContract::new("settings_list.control_kind", "Toggle", "Number"),
    StorybookUiOptionContract::new("settings_list.control_options", "2", "4"),
    StorybookUiOptionContract::new("settings_list.custom_control", "none", "button"),
    StorybookUiOptionContract::new("settings_list.set_value", "idle", "changed"),
    StorybookUiOptionContract::new("settings_list.reset", "dirty", "default"),
];

pub(super) const COMMAND_PALETTE_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("command_palette.query", "open", "theme"),
    StorybookUiOptionContract::new("command_palette.highlight", "0", "2"),
    StorybookUiOptionContract::new("command_palette.row_count", "5", "50"),
    StorybookUiOptionContract::new(
        "command_palette.provider_group",
        "workspace",
        "workspace/editor/app",
    ),
    StorybookUiOptionContract::new("command_palette.shortcut_display", "true", "false"),
];

pub(super) const DYNAMIC_ARRAY_EDITOR_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("array.rows", "1", "3"),
    StorybookUiOptionContract::new("array.add_remove", "none", "add+remove"),
    StorybookUiOptionContract::new("array.reorder", "false", "true"),
    StorybookUiOptionContract::new("array.theme_row", "default", "accent"),
];
