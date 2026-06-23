pub(in crate::visual) fn format_setting_action(option: &str) -> &'static str {
    match option {
        "theme.id" | "theme_id" => "settings_theme_id",
        "color.background" => "settings_color_background",
        "color.surface" => "settings_color_surface",
        "color.accent" => "settings_color_accent",
        "text.role" => "settings_text_role",
        "text.color_token" => "settings_text_color_token",
        "text.line_metrics" => "settings_text_line_metrics",
        "text.vertical_centered" => "settings_text_vertical_centered",
        "text.spans" => "settings_text_spans",
        "icon.svg_source" => "settings_icon_svg_source",
        "interaction.open" => "settings_interaction_open",
        "interaction.selected_index" => "settings_selected_index",
        "interaction.value" => "settings_interaction_value",
        "color_swatch.selected_color" => "settings_color_value",
        "layout.align" => "settings_layout_align",
        "context_menu.anchor" => "settings_context_menu_anchor",
        "readonly" => "settings_readonly",
        "placeholder" => "settings_placeholder",
        "validation" => "settings_validation",
        "ime" => "settings_ime",
        "disabled" => "settings_disabled",
        "font_role" => "settings_font_role",
        "text_entry.leading_slot_reserved" => "settings_input_leading_slot_reserved",
        "text_entry.leading_slot.icon" => "settings_input_leading_svg_icon",
        "text_entry.trailing_icon_buttons" => "settings_input_trailing_icon_buttons",
        "theme.input_bg" => "settings_input_theme_bg",
        "text_entry.trailing_slot_reserved" => "settings_input_trailing_slot_reserved",
        "text_entry.clear_action" => "settings_input_clear_action",
        "text_entry.submit_on_enter" => "settings_input_submit_on_enter",
        "text_entry.emoji_enabled" => "settings_input_emoji_enabled",
        "text_area.submit_key" => "settings_text_area_submit_key",
        "text_area.newline_key" => "settings_text_area_newline_key",
        "text_area.tab_behavior" => "settings_text_area_tab_behavior",
        "text_area.auto_grow" => "settings_text_area_auto_grow",
        "text_area.wrap_policy" => "settings_text_area_wrap_policy",
        "text_area.resize_enabled" => "settings_text_area_resize_enabled",
        "text_area.vertical_scroll_enabled" => "settings_text_area_vertical_scroll_enabled",
        "text_area.horizontal_scroll_enabled" => "settings_text_area_horizontal_scroll_enabled",
        "text_area.vertical_scrollbar_visible" => "settings_text_area_vertical_scrollbar_visible",
        "text_area.horizontal_scrollbar_visible" => {
            "settings_text_area_horizontal_scrollbar_visible"
        }
        "text_area.leading_slot.icon" => "settings_text_area_leading_svg_icon",
        "text_area.trailing_icon_buttons" => "settings_text_area_trailing_icon_buttons",
        "text_area.clear_action" => "settings_text_area_clear_action",
        "text_area.value" => "settings_text_area_value",
        "text_area.placeholder" => "settings_text_area_placeholder",
        "text_area.font_role" => "settings_text_area_font_role",
        "text_area.disabled" => "settings_text_area_disabled",
        "text_area.readonly" => "settings_text_area_readonly",
        "text_area.invalid" => "settings_text_area_invalid",
        "text_area.min_rows" => "settings_text_area_min_rows",
        "text_area.max_rows" => "settings_text_area_max_rows",
        "text_area.ime_enabled" => "settings_text_area_ime_enabled",
        "text_area.leading_slot_reserved" => "settings_text_area_leading_slot_reserved",
        "text_area.trailing_slot_reserved" => "settings_text_area_trailing_slot_reserved",
        "toolbar.display_mode" => "settings_toolbar_display_mode",
        "toolbar.density" => "settings_toolbar_density",
        "toolbar.overflow_strategy" => "settings_toolbar_overflow_strategy",
        "toolbar.actions" => "settings_toolbar_actions",
        "toolbar.groups" => "settings_toolbar_groups",
        "toolbar.context_menu_anchor" => "settings_toolbar_context_menu_anchor",
        "toolbar.action_priority" => "settings_toolbar_action_priority",
        "toolbar.action_accelerator" => "settings_toolbar_action_accelerator",
        "toolbar.action_split" => "settings_toolbar_action_split",
        "toolbar.action_group" => "settings_toolbar_action_group",
        "toolbar.action_tooltip" => "settings_toolbar_action_tooltip",
        "toolbar.action_a11y" => "settings_toolbar_action_a11y",
        "toolbar.action_disabled" => "settings_toolbar_action_disabled",
        "toolbar.group_label" => "settings_toolbar_group_label",
        "toolbar.group_divider" => "settings_toolbar_group_divider",
        "toolbar.split_disabled" => "settings_toolbar_split_disabled",
        "toolbar.split_tooltip" => "settings_toolbar_split_tooltip",
        "toolbar.split_a11y" => "settings_toolbar_split_a11y",
        "active_panel" => "settings_active_panel",
        "active_tab_id" => "settings_active_tab_id",
        "action" => "settings_action",
        "alignment" => "settings_alignment",
        "axis" => "settings_axis",
        "border" => "settings_border",
        "checked" => "settings_checked",
        "children" => "settings_children",
        "context_menu" => "settings_context_menu",
        "density" => "settings_density",
        "dismiss" => "settings_dismiss",
        "duration" => "settings_duration",
        "focus" => "settings_focus",
        "focusable" => "settings_focusable",
        "gap" => "settings_gap",
        "height" => "settings_height",
        "horizontal_scroll" => "settings_horizontal_scroll",
        "label" => "settings_label",
        "line" => "settings_line",
        "nested_state" => "settings_nested_state",
        "node_marker" => "settings_node_marker",
        "open" => "settings_open",
        "overflow" => "settings_overflow",
        "placement" => "settings_placement",
        "scrollbar_visibility" => "settings_scrollbar_visibility",
        "select" => "settings_select",
        "selected" => "settings_selected",
        "severity" => "settings_severity",
        "size" => "settings_size",
        "status" => "settings_status",
        "tab-index" => "settings_tab_index",
        "tone" => "settings_tone",
        "trigger" => "settings_trigger",
        "variant" => "settings_variant",
        "vertical_scroll" => "settings_vertical_scroll",
        "viewport" => "settings_viewport",
        "visible" => "settings_visible",
        "width" => "settings_width",
        "z-index" => "settings_z_index",
        _ => format_namespaced_setting_action(option).unwrap_or("settings_option_changed"),
    }
}

pub(in crate::visual) fn format_setting_event(page: &str) -> &'static str {
    match page {
        "theme-tokens" => "theme_settings_changed",
        "text" => "text_settings_changed",
        "icon" => "icon_settings_changed",
        "button" | "text-button" | "svg-button" | "icon-text-button" => "button_settings_changed",
        "text-input" | "search-box" => "input_settings_changed",
        "text-area" => "text_area_settings_changed",
        "color-swatch" | "color-picker-rgba" => "color_settings_changed",
        "tree-view" => "tree_settings_changed",
        "context-menu" => "context_menu_settings_changed",
        "tabs" => "tabs_settings_changed",
        "toolbar" => "toolbar_settings_changed",
        "badge" | "chip" | "attachment-chip" | "chip-group" => "chip_settings_changed",
        "divider" | "spacer" | "key-cap" | "loading-dots" | "spinner" | "progress-bar"
        | "skeleton" | "motion" | "slide-control" => "atom_settings_changed",
        "checkbox" | "radio" | "toggle" | "segmented-toggle" | "combo-box" | "select-box"
        | "selection-list" | "menu-button" => "selection_settings_changed",
        "tooltip" | "popover" | "modal" | "modal-overlay" | "hover-card" => {
            "overlay_settings_changed"
        }
        "card"
        | "list"
        | "menu"
        | "form-field"
        | "breadcrumb"
        | "side-menu"
        | "banner"
        | "toast-stack-manager"
        | "notification-toast"
        | "collapsible-panel" => "surface_settings_changed",
        "accordion"
        | "code-diff"
        | "drag-and-drop"
        | "shortcut-combo"
        | "shortcut-cheatsheet"
        | "skeleton-cluster"
        | "window-control-button-group"
        | "startup-state-panel"
        | "closeable-tab-strip" => "runtime_settings_changed",
        "command-palette"
        | "dynamic-array-editor"
        | "settings-list"
        | "status-bar"
        | "empty-state"
        | "diagnostics-list"
        | "search-control-strip" => "molecule_settings_changed",
        "row" | "column" | "stack" | "grid" | "scroll-area" | "split-pane" | "align-center"
        | "panel" | "virtualization" => "layout_settings_changed",
        _ => "component_settings_changed",
    }
}

fn format_namespaced_setting_action(option: &str) -> Option<&'static str> {
    let action = match option.split_once('.')?.0 {
        "a11y" => "settings_a11y_option",
        "accordion" => "settings_accordion_option",
        "action" => "settings_action_option",
        "array" => "settings_array_option",
        "attachment" => "settings_attachment_option",
        "badge" => "settings_badge_option",
        "banner" => "settings_banner_option",
        "breadcrumb" => "settings_breadcrumb_option",
        "button" => "settings_button_option",
        "card" => "settings_card_option",
        "chip" => "settings_chip_option",
        "chip_group" => "settings_chip_group_option",
        "code_diff" => "settings_code_diff_option",
        "collapsible_panel" => "settings_collapsible_panel_option",
        "color" => "settings_color_option",
        "color_picker" => "settings_color_picker_option",
        "combo" => "settings_combo_option",
        "command_palette" => "settings_command_palette_option",
        "content" => "settings_content_option",
        "context_menu" => "settings_context_menu_option",
        "diagnostics" => "settings_diagnostics_option",
        "drag" => "settings_drag_option",
        "empty_state" => "settings_empty_state_option",
        "form_field" => "settings_form_field_option",
        "hover_card" => "settings_hover_card_option",
        "icon" => "settings_icon_option",
        "interaction" => "settings_interaction_option",
        "list" => "settings_list_option",
        "loading" => "settings_loading_option",
        "menu" => "settings_menu_option",
        "motion" => "settings_motion_option",
        "progress" => "settings_progress_option",
        "search_control" => "settings_search_control_option",
        "selection_list" => "settings_selection_list_option",
        "settings_list" => "settings_settings_list_option",
        "select" => "settings_select_option",
        "shortcut_cheatsheet" => "settings_shortcut_cheatsheet_option",
        "shortcut_combo" => "settings_shortcut_combo_option",
        "side_menu" => "settings_side_menu_option",
        "skeleton" => "settings_skeleton_option",
        "skeleton_cluster" => "settings_skeleton_cluster_option",
        "split_pane" => "settings_split_pane_option",
        "startup_state" => "settings_startup_state_option",
        "status" => "settings_status_option",
        "status_bar" => "settings_status_bar_option",
        "tabs" => "settings_tabs_option",
        "text" => "settings_text_option",
        "text_area" => "settings_text_area_option",
        "text_entry" => "settings_text_entry_option",
        "theme" => "settings_theme_option",
        "virtualization" => "settings_virtualization_option",
        "viewport" => "settings_viewport_option",
        "visual" => "settings_visual_option",
        "window_control" => "settings_window_control_option",
        _ => return None,
    };
    Some(action)
}
