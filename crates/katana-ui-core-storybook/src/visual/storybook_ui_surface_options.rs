use super::storybook_ui_option_contract::StorybookUiOptionContract;

pub(super) const OVERLAY_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("open", "false", "true"),
    StorybookUiOptionContract::new("placement", "default", "edge"),
    StorybookUiOptionContract::new("focus", "none", "first"),
    StorybookUiOptionContract::new("dismiss", "manual", "outside"),
];

pub(super) const HOVER_CARD_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("hover_card.open_delay_ms", "0", "180"),
    StorybookUiOptionContract::new("hover_card.close_delay_ms", "0", "220"),
    StorybookUiOptionContract::new("hover_card.pointer_follow", "false", "true"),
    StorybookUiOptionContract::new("hover_card.slot_action", "none", "visible"),
];

pub(super) const MENU_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("menu.common_props", "default", "dense"),
    StorybookUiOptionContract::new("children", "static", "changed"),
    StorybookUiOptionContract::new("interaction.selected_index", "none", "1"),
    StorybookUiOptionContract::new("menu.panel_placement", "default", "resolved"),
];

pub(super) const FORM_FIELD_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("form_field.common_props", "default", "dense"),
    StorybookUiOptionContract::new("children", "static", "changed"),
    StorybookUiOptionContract::new("form_field.invalid", "false", "true"),
    StorybookUiOptionContract::new("form_field.helper_text", "short", "long"),
    StorybookUiOptionContract::new("form_field.required", "false", "true"),
];

pub(super) const BREADCRUMB_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("breadcrumb.items", "3", "4"),
    StorybookUiOptionContract::new("children", "static", "changed"),
    StorybookUiOptionContract::new("interaction.selected_index", "0", "2"),
    StorybookUiOptionContract::new("breadcrumb.crumb_action", "none", "callback"),
];

pub(super) const SIDE_MENU_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("side_menu.items", "3", "5"),
    StorybookUiOptionContract::new("children", "static", "changed"),
    StorybookUiOptionContract::new("interaction.selected_index", "0", "1"),
    StorybookUiOptionContract::new("side_menu.hover_expansion", "false", "true"),
];

pub(super) const CARD_OPTIONS: [StorybookUiOptionContract; 8] = [
    StorybookUiOptionContract::new("card.label", "Card", "Project summary"),
    StorybookUiOptionContract::new("card.header", "visible", "custom"),
    StorybookUiOptionContract::new("card.footer", "hidden", "visible"),
    StorybookUiOptionContract::new("card.variant", "slots", "theme_border"),
    StorybookUiOptionContract::new("card.padding", "Medium", "Large"),
    StorybookUiOptionContract::new("card.clickable", "false", "true"),
    StorybookUiOptionContract::new("card.nested_controls", "static", "interactive"),
    StorybookUiOptionContract::new("card.child_state", "isolated", "changed"),
];

pub(super) const LIST_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("list.rows", "3", "200"),
    StorybookUiOptionContract::new("list.selection", "none", "row-2"),
    StorybookUiOptionContract::new("list.empty_state", "false", "true"),
    StorybookUiOptionContract::new("list.virtualization", "off", "visible_range"),
    StorybookUiOptionContract::new("list.theme_row", "default", "accent"),
];

pub(super) const COLLAPSIBLE_PANEL_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("collapsible_panel.mode", "inline", "floating_overlay"),
    StorybookUiOptionContract::new("collapsible_panel.width", "240", "320"),
    StorybookUiOptionContract::new("collapsible_panel.pinned", "true", "false"),
    StorybookUiOptionContract::new("collapsible_panel.expand_on_hover", "false", "true"),
    StorybookUiOptionContract::new("collapsible_panel.resize_handle", "false", "true"),
];

pub(super) const FEEDBACK_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("severity", "info", "warning"),
    StorybookUiOptionContract::new("duration", "default", "custom"),
    StorybookUiOptionContract::new("action", "none", "visible"),
    StorybookUiOptionContract::new("dismiss", "false", "true"),
];

pub(super) const BANNER_OPTIONS: [StorybookUiOptionContract; 8] = [
    StorybookUiOptionContract::new("severity", "info", "warning"),
    StorybookUiOptionContract::new("density", "default", "compact"),
    StorybookUiOptionContract::new("action", "none", "visible"),
    StorybookUiOptionContract::new("dismiss", "false", "true"),
    StorybookUiOptionContract::new("banner.details", "collapsed", "expanded"),
    StorybookUiOptionContract::new("banner.title", "hidden", "visible"),
    StorybookUiOptionContract::new("banner.leading_icon", "default", "custom"),
    StorybookUiOptionContract::new("banner.placement", "inline", "sticky"),
];

pub(super) const COLOR_PICKER_OPTIONS: [StorybookUiOptionContract; 15] = [
    StorybookUiOptionContract::new("color_picker.rgba", "rgba(0,0,0,1)", "rgba(64,128,255,.8)"),
    StorybookUiOptionContract::new(
        "color_picker.value",
        "rgba(0,0,0,1)",
        "rgba(72,136,240,.74)",
    ),
    StorybookUiOptionContract::new("color_picker.open", "false", "true"),
    StorybookUiOptionContract::new("color_picker.hue", "0", "214"),
    StorybookUiOptionContract::new("color_picker.alpha", "255", "204"),
    StorybookUiOptionContract::new("color_picker.blending", "Replace", "Multiply"),
    StorybookUiOptionContract::new("color_picker.color_area", "empty", "saturation/value"),
    StorybookUiOptionContract::new("color_picker.trigger_size", "Medium", "Large"),
    StorybookUiOptionContract::new("color_picker.title", "empty", "Brand accent"),
    StorybookUiOptionContract::new("color_picker.rgba_mode", "true", "false"),
    StorybookUiOptionContract::new("color_picker.panel_scale_percent", "75", "100"),
    StorybookUiOptionContract::new("color_picker.trigger_border", "true", "false"),
    StorybookUiOptionContract::new(
        "color_picker.eyedropper_callback",
        "none",
        "storybook-eyedropper",
    ),
    StorybookUiOptionContract::new("color_picker.readonly", "false", "true"),
    StorybookUiOptionContract::new("color_picker.disabled", "false", "true"),
];

pub(super) const VIRTUALIZATION_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("viewport.offset", "0", "1260"),
    StorybookUiOptionContract::new("virtualization.overscan", "2", "4"),
    StorybookUiOptionContract::new("virtualization.row_height_provider", "Fixed", "Variable"),
    StorybookUiOptionContract::new("virtualization.focused_index", "none", "42"),
    StorybookUiOptionContract::new("virtualization.measured_correction", "0", "+8"),
];

pub(super) const TREE_OPTIONS: [StorybookUiOptionContract; 4] = [
    StorybookUiOptionContract::new("line", "visible", "hidden"),
    StorybookUiOptionContract::new("node_marker", "branch", "leaf"),
    StorybookUiOptionContract::new("trigger", "icon+text", "text"),
    StorybookUiOptionContract::new("context_menu", "disabled", "enabled"),
];

pub(super) const PANEL_OPTIONS: [StorybookUiOptionContract; 5] = [
    StorybookUiOptionContract::new("active_panel", "preview", "details"),
    StorybookUiOptionContract::new("vertical_scroll", "0", "changed"),
    StorybookUiOptionContract::new("horizontal_scroll", "0", "changed"),
    StorybookUiOptionContract::new("scrollbar_visibility", "on", "off"),
    StorybookUiOptionContract::new("nested_state", "shared", "independent"),
];
