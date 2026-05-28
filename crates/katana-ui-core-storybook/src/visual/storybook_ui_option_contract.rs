use super::storybook_ui_runtime_options::{
    ACCORDION_OPTIONS, CODE_DIFF_OPTIONS, DRAG_AND_DROP_OPTIONS, RUNTIME_OPTIONS,
    SHORTCUT_CHEATSHEET_OPTIONS, SHORTCUT_COMBO_OPTIONS, SKELETON_CLUSTER_OPTIONS,
    STARTUP_STATE_OPTIONS, TEXT_AREA_OPTIONS, WINDOW_CONTROL_OPTIONS,
};
use super::storybook_ui_tabs_options::TABS_OPTIONS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StorybookUiOptionContract {
    pub(super) setting: &'static str,
    before: &'static str,
    after: &'static str,
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
        "icon" | "key-cap" => &CONTENT_PRIMITIVE_OPTIONS,
        "divider" | "spacer" | "loading-dots" | "spinner" | "progress-bar" | "color-swatch"
        | "slide-control" | "skeleton" | "motion" => &PRIMITIVE_OPTIONS,
        "button" | "text-button" | "svg-button" | "icon-text-button" => &BUTTON_OPTIONS,
        "text-input" => &INPUT_OPTIONS,
        "text-area" => &TEXT_AREA_OPTIONS,
        "search-box" => &SEARCH_BOX_OPTIONS,
        "combo-box" => &INPUT_FLOW_OPTIONS,
        "search-control-strip" => &SEARCH_CONTROL_STRIP_OPTIONS,
        "checkbox" | "radio" | "toggle" | "segmented-toggle" | "select-box" | "selection-list"
        | "menu-button" => &SELECTION_OPTIONS,
        "badge" => &STATUS_OPTIONS,
        "chip" => &CHIP_OPTIONS,
        "attachment-chip" => &ATTACHMENT_CHIP_OPTIONS,
        "chip-group" => &CHIP_GROUP_OPTIONS,
        "tooltip" | "popover" | "hover-card" | "context-menu" | "modal" | "modal-overlay" => {
            &OVERLAY_OPTIONS
        }
        "card" => &CARD_OPTIONS,
        "list" => &LIST_OPTIONS,
        "menu" | "toolbar" | "form-field" | "breadcrumb" | "side-menu" => &COMPOSITE_OPTIONS,
        "tabs" => &TABS_OPTIONS,
        "accordion" => &ACCORDION_OPTIONS,
        "settings-list" => &SETTINGS_LIST_OPTIONS,
        "collapsible-panel" => &COLLAPSIBLE_PANEL_OPTIONS,
        "banner" | "toast-stack-manager" | "notification-toast" => &FEEDBACK_OPTIONS,
        "status-bar" => &STATUS_BAR_OPTIONS,
        "empty-state" => &EMPTY_STATE_OPTIONS,
        "code-diff" => &CODE_DIFF_OPTIONS,
        "color-picker-rgba" => &COMPLEX_OPTIONS,
        "dynamic-array-editor" => &DYNAMIC_ARRAY_EDITOR_OPTIONS,
        "command-palette" => &COMMAND_PALETTE_OPTIONS,
        "diagnostics-list" => &DIAGNOSTICS_LIST_OPTIONS,
        "virtualization" => &VIRTUALIZATION_OPTIONS,
        "row" | "column" | "stack" | "grid" | "scroll-area" | "split-pane" | "align-center" => {
            &LAYOUT_OPTIONS
        }
        "skeleton-cluster" => &SKELETON_CLUSTER_OPTIONS,
        "shortcut-combo" => &SHORTCUT_COMBO_OPTIONS,
        "shortcut-cheatsheet" => &SHORTCUT_CHEATSHEET_OPTIONS,
        "drag-and-drop" => &DRAG_AND_DROP_OPTIONS,
        "window-control-button-group" => &WINDOW_CONTROL_OPTIONS,
        "startup-state-panel" => &STARTUP_STATE_OPTIONS,
        "closeable-tab-strip" => &RUNTIME_OPTIONS,
        "tree-view" => &TREE_OPTIONS,
        "panel" => &PANEL_OPTIONS,
        _ => &[],
    }
}

const THEME_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("theme.id", "dark", "light"),
    StorybookUiOptionContract::new("color.background", "dark", "light"),
    StorybookUiOptionContract::new("color.surface", "panel", "contrast"),
    StorybookUiOptionContract::new("color.accent", "blue", "green"),
];
const TEXT_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("text.role", "body", "heading"),
    StorybookUiOptionContract::new("text.content", "mixed", "empty"),
    StorybookUiOptionContract::new("text.script", "latin", "jp+emoji"),
    StorybookUiOptionContract::new("text.color", "theme", "accent"),
    StorybookUiOptionContract::new("text.wrap", "single", "multi"),
];
const CONTENT_PRIMITIVE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("content.value", "default", "custom"),
    StorybookUiOptionContract::new("visual.role", "content", "icon"),
    StorybookUiOptionContract::new("a11y.label", "present", "changed"),
    StorybookUiOptionContract::new("theme.color", "text", "accent"),
];
const PRIMITIVE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("variant", "default", "alternate"),
    StorybookUiOptionContract::new("tone", "neutral", "accent"),
    StorybookUiOptionContract::new("size", "medium", "large"),
    StorybookUiOptionContract::new("theme.slot", "default", "custom"),
];
const BUTTON_OPTIONS: [StorybookUiOptionContract; 6] = [
    StorybookUiOptionContract::new("visible", "true", "false"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("focusable", "true", "false"),
    StorybookUiOptionContract::new("width", "auto", "fixed"),
    StorybookUiOptionContract::new("height", "auto", "fixed"),
    StorybookUiOptionContract::new("label", "default", "custom"),
];
const INPUT_OPTIONS: [StorybookUiOptionContract; 9] = [
    StorybookUiOptionContract::new("interaction.value", "typed", "typed 日本語"),
    StorybookUiOptionContract::new("readonly", "false", "true"),
    StorybookUiOptionContract::new("placeholder", "visible", "hidden"),
    StorybookUiOptionContract::new("text_entry.leading_slot_reserved", "false", "true"),
    StorybookUiOptionContract::new("text_entry.leading_slot.icon", "none", "search-svg"),
    StorybookUiOptionContract::new("text_entry.trailing_icon_buttons", "none", "callbacks"),
    StorybookUiOptionContract::new("validation", "valid", "invalid"),
    StorybookUiOptionContract::new("ime", "idle", "composition"),
    StorybookUiOptionContract::new("theme.input_bg", "surface", "light"),
];
const SEARCH_BOX_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("text_entry.value", "query", "typed query"),
    StorybookUiOptionContract::new("text_entry.submit_on_enter", "false", "true"),
    StorybookUiOptionContract::new("text_entry.clear_button", "visible", "cleared"),
    StorybookUiOptionContract::new("text_entry.regex_case", "false/false", "true/true"),
];
const SEARCH_CONTROL_STRIP_OPTIONS: [StorybookUiOptionContract; 7] = [
    StorybookUiOptionContract::new("search_control.query", "head", "heading"),
    StorybookUiOptionContract::new("search_control.match_case", "false", "true"),
    StorybookUiOptionContract::new("search_control.whole_word", "false", "true"),
    StorybookUiOptionContract::new("search_control.use_regex", "false", "true"),
    StorybookUiOptionContract::new("search_control.replace_mode", "Visible", "Disabled"),
    StorybookUiOptionContract::new("search_control.result_count", "12", "0"),
    StorybookUiOptionContract::new("search_control.active_index", "Some(2)", "None"),
];
const INPUT_FLOW_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("value", "idle", "changed"),
    StorybookUiOptionContract::new("submit", "none", "requested"),
    StorybookUiOptionContract::new("validation", "valid", "invalid"),
    StorybookUiOptionContract::new("composition", "idle", "ime"),
];
const SELECTION_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("selected", "false", "true"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("focus", "none", "visible"),
    StorybookUiOptionContract::new("theme.marker", "default", "accent"),
];
const STATUS_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("tone", "neutral", "danger"),
    StorybookUiOptionContract::new("variant", "outline", "filled"),
    StorybookUiOptionContract::new("dismiss", "none", "enabled"),
    StorybookUiOptionContract::new("state", "idle", "changed"),
];
const STATUS_BAR_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("status_bar.mode", "SingleMessage", "MultiSegment"),
    StorybookUiOptionContract::new("status_bar.segments", "1", "4"),
    StorybookUiOptionContract::new("status_bar.density", "Default", "Compact"),
    StorybookUiOptionContract::new("status_bar.progress_popover", "false", "true"),
];
const CHIP_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("chip.variant", "Outline", "Filled"),
    StorybookUiOptionContract::new("chip.tone", "Accent", "Danger"),
    StorybookUiOptionContract::new("chip.size", "Medium", "Large"),
    StorybookUiOptionContract::new("chip.dismissible", "false", "true"),
];
const ATTACHMENT_CHIP_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("attachment.kind", "File", "Image"),
    StorybookUiOptionContract::new("attachment.status", "Uploading", "Error"),
    StorybookUiOptionContract::new("attachment.progress", "42", "100"),
    StorybookUiOptionContract::new("attachment.retry", "hidden", "visible"),
];
const CHIP_GROUP_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("chip_group.wrap", "false", "true"),
    StorybookUiOptionContract::new("chip_group.overflow", "Inline", "Menu"),
    StorybookUiOptionContract::new("chip_group.reorder", "false", "true"),
    StorybookUiOptionContract::new("chip_group.hidden_count", "0", "2"),
];
const EMPTY_STATE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("empty_state.tone", "Accent", "Danger"),
    StorybookUiOptionContract::new("empty_state.size", "Default", "Large"),
    StorybookUiOptionContract::new("empty_state.alignment", "Center", "Leading"),
    StorybookUiOptionContract::new("empty_state.actions", "Primary", "Primary+Secondary"),
];
const DIAGNOSTICS_LIST_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("diagnostics.group_by", "Severity", "Source"),
    StorybookUiOptionContract::new("diagnostics.sort_by", "Severity", "Location"),
    StorybookUiOptionContract::new("diagnostics.severity_filter", "Error+Warning", "Error"),
    StorybookUiOptionContract::new("diagnostics.bulk_action", "Preview", "Apply"),
    StorybookUiOptionContract::new("diagnostics.fix_preview", "Expanded", "Collapsed"),
];
const SETTINGS_LIST_OPTIONS: [StorybookUiOptionContract; 6] = [
    StorybookUiOptionContract::new("settings_list.density", "Default", "Compact"),
    StorybookUiOptionContract::new("settings_list.dirty_visualization", "Marker", "Highlight"),
    StorybookUiOptionContract::new("settings_list.query", "None", "format"),
    StorybookUiOptionContract::new("settings_list.sections", "app+chat+lint", "app+lint"),
    StorybookUiOptionContract::new("settings_list.control_kind", "Toggle", "Number"),
    StorybookUiOptionContract::new("settings_list.reset", "dirty", "default"),
];
const COMMAND_PALETTE_OPTIONS: [StorybookUiOptionContract; 5] = [
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
const DYNAMIC_ARRAY_EDITOR_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("array.rows", "1", "3"),
    StorybookUiOptionContract::new("array.add_remove", "none", "add+remove"),
    StorybookUiOptionContract::new("array.reorder", "false", "true"),
    StorybookUiOptionContract::new("array.theme_row", "default", "accent"),
];
const OVERLAY_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("open", "false", "true"),
    StorybookUiOptionContract::new("placement", "default", "edge"),
    StorybookUiOptionContract::new("focus", "none", "first"),
    StorybookUiOptionContract::new("dismiss", "manual", "outside"),
];
const COMPOSITE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("layout", "default", "dense"),
    StorybookUiOptionContract::new("children", "static", "changed"),
    StorybookUiOptionContract::new("selection", "none", "active"),
    StorybookUiOptionContract::new("overflow", "fit", "menu"),
];
const CARD_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("card.variant", "slots", "theme_border"),
    StorybookUiOptionContract::new("card.clickable", "false", "true"),
    StorybookUiOptionContract::new("card.nested_controls", "static", "interactive"),
    StorybookUiOptionContract::new("card.child_state", "isolated", "changed"),
];
const LIST_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("list.rows", "3", "200"),
    StorybookUiOptionContract::new("list.selection", "none", "row-2"),
    StorybookUiOptionContract::new("list.empty_state", "false", "true"),
    StorybookUiOptionContract::new("list.virtualization", "off", "visible_range"),
    StorybookUiOptionContract::new("list.theme_row", "default", "accent"),
];
const COLLAPSIBLE_PANEL_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("collapsible_panel.mode", "inline", "floating_overlay"),
    StorybookUiOptionContract::new("collapsible_panel.width", "240", "320"),
    StorybookUiOptionContract::new("collapsible_panel.pinned", "true", "false"),
    StorybookUiOptionContract::new("collapsible_panel.expand_on_hover", "false", "true"),
    StorybookUiOptionContract::new("collapsible_panel.resize_handle", "false", "true"),
];
const FEEDBACK_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("severity", "info", "warning"),
    StorybookUiOptionContract::new("duration", "default", "custom"),
    StorybookUiOptionContract::new("action", "none", "visible"),
    StorybookUiOptionContract::new("dismiss", "false", "true"),
];
const COMPLEX_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("query", "idle", "changed"),
    StorybookUiOptionContract::new("selection", "first", "next"),
    StorybookUiOptionContract::new("virtual_range", "top", "scrolled"),
    StorybookUiOptionContract::new("command", "enabled", "blocked"),
];
const LAYOUT_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("axis", "x", "y"),
    StorybookUiOptionContract::new("gap", "medium", "large"),
    StorybookUiOptionContract::new("overflow", "fit", "scroll"),
    StorybookUiOptionContract::new("alignment", "start", "center"),
];
const VIRTUALIZATION_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("viewport.offset", "0", "1260"),
    StorybookUiOptionContract::new("virtualization.overscan", "2", "4"),
    StorybookUiOptionContract::new("virtualization.row_height_provider", "Fixed", "Variable"),
    StorybookUiOptionContract::new("virtualization.focused_index", "none", "42"),
    StorybookUiOptionContract::new("virtualization.measured_correction", "0", "+8"),
];
const TREE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("line", "visible", "hidden"),
    StorybookUiOptionContract::new("node_marker", "branch", "leaf"),
    StorybookUiOptionContract::new("trigger", "icon+text", "text"),
    StorybookUiOptionContract::new("context_menu", "disabled", "enabled"),
];
const PANEL_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("active_panel", "preview", "details"),
    StorybookUiOptionContract::new("vertical_scroll", "0", "changed"),
    StorybookUiOptionContract::new("horizontal_scroll", "0", "changed"),
    StorybookUiOptionContract::new("scrollbar_visibility", "on", "off"),
    StorybookUiOptionContract::new("nested_state", "shared", "independent"),
];
