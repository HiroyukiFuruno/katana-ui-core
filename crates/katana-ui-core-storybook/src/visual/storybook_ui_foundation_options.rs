use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const THEME_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("theme.id", "dark", "light"),
    StorybookUiOptionContract::new("color.background", "dark", "light"),
    StorybookUiOptionContract::new("color.surface", "panel", "contrast"),
    StorybookUiOptionContract::new("color.accent", "blue", "green"),
];

pub(super) const TEXT_OPTIONS: [StorybookUiOptionContract; 9] = [
    StorybookUiOptionContract::new("text.role", "body", "heading"),
    StorybookUiOptionContract::new("text.content", "mixed", "empty"),
    StorybookUiOptionContract::new("text.script", "latin", "jp+emoji"),
    StorybookUiOptionContract::new("text.color", "theme", "accent"),
    StorybookUiOptionContract::new("text.color_token", "text", "accent"),
    StorybookUiOptionContract::new("text.line_metrics", "default", "compact"),
    StorybookUiOptionContract::new("text.vertical_centered", "false", "true"),
    StorybookUiOptionContract::new("text.spans", "plain", "rich"),
    StorybookUiOptionContract::new("text.wrap", "single", "multi"),
];

pub(super) const CONTENT_PRIMITIVE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("content.value", "default", "custom"),
    StorybookUiOptionContract::new("visual.role", "content", "icon"),
    StorybookUiOptionContract::new("a11y.label", "present", "changed"),
    StorybookUiOptionContract::new("theme.color", "text", "accent"),
];

pub(super) const ICON_OPTIONS: [StorybookUiOptionContract; 12] = [
    StorybookUiOptionContract::new("content.value", "default", "custom"),
    StorybookUiOptionContract::new("visual.role", "content", "icon"),
    StorybookUiOptionContract::new("a11y.label", "present", "changed"),
    StorybookUiOptionContract::new("theme.color", "text", "accent"),
    StorybookUiOptionContract::new("icon.svg_source", "default-svg", "custom-svg"),
    StorybookUiOptionContract::new("icon.svg_icon", "source", "props-object"),
    StorybookUiOptionContract::new("icon.view_box", "0 0 16 16", "0 0 24 24"),
    StorybookUiOptionContract::new("icon.path_summary", "cross", "search-outline"),
    StorybookUiOptionContract::new("icon.paint_policy", "inherit", "currentColor"),
    StorybookUiOptionContract::new("icon.role", "decorative", "action"),
    StorybookUiOptionContract::new("icon.color_token", "text", "accent"),
    StorybookUiOptionContract::new("icon.theme_token", "text", "muted"),
];

pub(super) const PRIMITIVE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("variant", "default", "alternate"),
    StorybookUiOptionContract::new("tone", "neutral", "accent"),
    StorybookUiOptionContract::new("size", "medium", "large"),
    StorybookUiOptionContract::new("theme.slot", "default", "custom"),
];

pub(super) const SKELETON_OPTIONS: [StorybookUiOptionContract; 11] = [
    StorybookUiOptionContract::new("skeleton.shape", "Text", "Line"),
    StorybookUiOptionContract::new("skeleton.text_lines", "3", "2"),
    StorybookUiOptionContract::new("skeleton.last_line_ratio", "0.58", "0.62"),
    StorybookUiOptionContract::new("skeleton.line_thickness", "8", "12"),
    StorybookUiOptionContract::new("size", "220x44", "Fill"),
    StorybookUiOptionContract::new("skeleton.animation", "Shimmer", "Wave"),
    StorybookUiOptionContract::new("tone", "Neutral", "Accent"),
    StorybookUiOptionContract::new("skeleton.radius_px", "4", "14"),
    StorybookUiOptionContract::new("skeleton.reduced_motion", "false", "true"),
    StorybookUiOptionContract::new("a11y.label", "Loading", "Loading profile"),
    StorybookUiOptionContract::new("skeleton.aspect_ratio", "none", "16:9"),
];

pub(super) const LOADING_INDICATOR_OPTIONS: [StorybookUiOptionContract; 8] = [
    StorybookUiOptionContract::new("variant", "default", "alternate"),
    StorybookUiOptionContract::new("loading.animation_state", "Running", "Paused"),
    StorybookUiOptionContract::new("loading.reduced_motion", "false", "true"),
    StorybookUiOptionContract::new("loading.label", "Loading", "Saving"),
    StorybookUiOptionContract::new("loading.speed_ms", "240", "96"),
    StorybookUiOptionContract::new("loading.dot_count", "3", "5"),
    StorybookUiOptionContract::new("tone", "neutral", "accent"),
    StorybookUiOptionContract::new("size", "medium", "large"),
];

pub(super) const PROGRESS_BAR_OPTIONS: [StorybookUiOptionContract; 9] = [
    StorybookUiOptionContract::new("variant", "default", "alternate"),
    StorybookUiOptionContract::new("progress.percent", "65", "82"),
    StorybookUiOptionContract::new("loading.animation_state", "Running", "Paused"),
    StorybookUiOptionContract::new("loading.label", "Progress", "Syncing"),
    StorybookUiOptionContract::new("loading.speed_ms", "240", "96"),
    StorybookUiOptionContract::new("loading.dot_count", "3", "5"),
    StorybookUiOptionContract::new("loading.reduced_motion", "false", "true"),
    StorybookUiOptionContract::new("tone", "neutral", "accent"),
    StorybookUiOptionContract::new("size", "medium", "large"),
];

pub(super) const MOTION_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("motion.primitive", "Fade", "Shimmer"),
    StorybookUiOptionContract::new("motion.duration", "Default", "Fast"),
    StorybookUiOptionContract::new("motion.distance", "Default", "Compact"),
    StorybookUiOptionContract::new("motion.reduced_policy", "Respect", "ForceReduced"),
];

pub(super) const BINARY_SELECTION_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("selected", "false", "true"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("focus", "none", "visible"),
    StorybookUiOptionContract::new("checked", "false", "true"),
];

pub(super) const SELECT_BOX_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("select.items", "2", "6"),
    StorybookUiOptionContract::new("interaction.open", "false", "true"),
    StorybookUiOptionContract::new("interaction.selected_index", "none", "1"),
    StorybookUiOptionContract::new("placeholder", "hidden", "visible"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
];

pub(super) const MENU_BUTTON_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("menu.items", "2", "4"),
    StorybookUiOptionContract::new("interaction.open", "false", "true"),
    StorybookUiOptionContract::new("disabled", "false", "true"),
    StorybookUiOptionContract::new("menu.select_action", "none", "callback"),
];

pub(super) const SELECTION_LIST_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("selection_list.items", "4", "1000"),
    StorybookUiOptionContract::new("interaction.selected_index", "none", "2"),
    StorybookUiOptionContract::new("selection_list.section", "none", "Recent"),
    StorybookUiOptionContract::new("selection_list.marker", "none", "check"),
    StorybookUiOptionContract::new("selection_list.more_row", "false", "true"),
];

pub(super) const BADGE_OPTIONS: [StorybookUiOptionContract; 6] = [
    StorybookUiOptionContract::new("status.severity", "Neutral", "Danger"),
    StorybookUiOptionContract::new("badge.passive", "status", "use-chip"),
    StorybookUiOptionContract::new("size", "medium", "small"),
    StorybookUiOptionContract::new("tone", "neutral", "accent"),
    StorybookUiOptionContract::new("badge.leading_icon", "none", "dot"),
    StorybookUiOptionContract::new("variant", "plain", "filled"),
];

pub(super) const LAYOUT_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("axis", "x", "y"),
    StorybookUiOptionContract::new("gap", "medium", "large"),
    StorybookUiOptionContract::new("overflow", "fit", "scroll"),
    StorybookUiOptionContract::new("alignment", "start", "center"),
];

pub(super) const SPLIT_PANE_OPTIONS: [StorybookUiOptionContract; 10] = [
    StorybookUiOptionContract::new("axis", "Horizontal", "Vertical"),
    StorybookUiOptionContract::new("gap", "0", "12"),
    StorybookUiOptionContract::new("alignment", "Start", "Center"),
    StorybookUiOptionContract::new("overflow", "Fit", "Scroll"),
    StorybookUiOptionContract::new("split_pane.ratio_percent", "50", "64"),
    StorybookUiOptionContract::new("split_pane.min_percent", "20", "24"),
    StorybookUiOptionContract::new("split_pane.max_percent", "80", "76"),
    StorybookUiOptionContract::new("split_pane.reset_percent", "50", "55"),
    StorybookUiOptionContract::new("split_pane.handle_width_px", "8", "10"),
    StorybookUiOptionContract::new(
        "split_pane.resize_mode",
        "PointerAndKeyboard",
        "KeyboardOnly",
    ),
];
