use super::*;

pub(super) fn assert_command_palette_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "command_palette.query",
        "command_palette.highlight",
        "command_palette.row_count",
        "command_palette.provider_group",
        "command_palette.shortcut_display",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "command-palette"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "command_palette_settings_changed"
            }),
            "missing command-palette setting mutation for {option}"
        );
    }
}

pub(super) fn assert_startup_state_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "startup_state.state",
        "startup_state.progress",
        "startup_state.label",
        "startup_state.retry",
        "startup_state.cancel",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "startup-state-panel"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "startup_state_settings_changed"
            }),
            "missing startup-state-panel setting mutation for {option}"
        );
    }
}

pub(super) fn assert_window_control_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "window_control.position",
        "window_control.size",
        "window_control.controls",
        "window_control.visibility",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "window-control-button-group"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "window_control_settings_changed"
            }),
            "missing window-control-button-group setting mutation for {option}"
        );
    }
}

pub(super) fn assert_collapsible_panel_settings_are_switchable(
    settings: &[SettingsMutationReport],
) {
    for option in [
        "collapsible_panel.mode",
        "collapsible_panel.width",
        "collapsible_panel.pinned",
        "collapsible_panel.expand_on_hover",
        "collapsible_panel.resize_handle",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "collapsible-panel"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "collapsible_panel_settings_changed"
            }),
            "missing collapsible-panel setting mutation for {option}"
        );
    }
}

pub(super) fn assert_search_control_strip_settings_are_switchable(
    settings: &[SettingsMutationReport],
) {
    for option in [
        "search_control.query",
        "search_control.match_case",
        "search_control.whole_word",
        "search_control.use_regex",
        "search_control.replace_mode",
        "search_control.result_count",
        "search_control.active_index",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "search-control-strip"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "search_control_strip_settings_changed"
            }),
            "missing search-control-strip setting mutation for {option}"
        );
    }
}

pub(super) fn assert_scroll_area_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "scroll_area.axis",
        "scroll_area.offset",
        "scroll_area.viewport",
        "scroll_area.content",
        "scroll_area.scrollbar_visibility",
        "scroll_area.scrollbar_placement",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "scroll-area"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "scroll_area_settings_changed"
            }),
            "missing scroll-area setting mutation for {option}"
        );
    }
}

pub(super) fn assert_split_pane_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "split_pane.axis",
        "split_pane.ratio",
        "split_pane.min",
        "split_pane.max",
        "split_pane.reset",
        "split_pane.resize_mode",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "split-pane"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "split_pane_settings_changed"
            }),
            "missing split-pane setting mutation for {option}"
        );
    }
}

pub(super) fn assert_settings_list_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for (option, action, event) in [
        (
            "settings_list.density",
            "set_settings_list.density",
            "settings_list_settings_changed",
        ),
        (
            "settings_list.dirty_visualization",
            "set_settings_list.dirty_visualization",
            "settings_list_settings_changed",
        ),
        (
            "settings_list.query",
            "settings_query_filter",
            "settings_list_query_changed",
        ),
        (
            "settings_list.sections",
            "settings_toggle_section",
            "settings_list_section_collapsed",
        ),
        (
            "settings_list.control_kind",
            "settings_update_field",
            "settings_list_field_changed",
        ),
        (
            "settings_list.reset",
            "settings_reset_field",
            "settings_list_field_reset",
        ),
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "settings-list"
                    && it.option.name == option
                    && it.action == action
                    && it.event == event
            }),
            "missing settings-list setting mutation for {option}"
        );
    }
}

pub(super) fn assert_motion_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "motion.primitive",
        "motion.duration",
        "motion.easing",
        "motion.distance",
        "motion.reduced_policy",
        "motion.disable_context",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "motion"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "motion_settings_changed"
            }),
            "missing motion setting mutation for {option}"
        );
    }
}
