pub(super) fn format_setting_action(option: &str) -> &'static str {
    match option {
        "theme_id" => "settings_theme_id",
        "text.role" => "settings_text_role",
        "icon.svg_source" => "settings_icon_svg_source",
        "interaction.open" => "settings_interaction_open",
        "interaction.selected_index" => "settings_selected_index",
        "interaction.value" => "settings_interaction_value",
        "color_swatch.selected_color" => "settings_color_value",
        "layout.align" => "settings_layout_align",
        "context_menu.anchor" => "settings_context_menu_anchor",
        _ => "settings_option_changed",
    }
}

pub(super) fn format_setting_event(page: &str) -> &'static str {
    match page {
        "theme-tokens" => "theme_settings_changed",
        "text" => "text_settings_changed",
        "icon" => "icon_settings_changed",
        "button" | "text-button" | "svg-button" | "icon-text-button" => "button_settings_changed",
        "text-input" | "search-box" => "input_settings_changed",
        "color-swatch" | "color-picker-rgba" => "color_settings_changed",
        "tree-view" => "tree_settings_changed",
        "context-menu" => "context_menu_settings_changed",
        _ => "component_settings_changed",
    }
}
