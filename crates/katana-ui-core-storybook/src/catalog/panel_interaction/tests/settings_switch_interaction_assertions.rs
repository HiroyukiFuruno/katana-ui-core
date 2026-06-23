use super::*;

pub(super) fn assert_closeable_tab_strip_settings_are_switchable(
    settings: &[SettingsMutationReport],
) {
    for (option, action, event) in [
        ("tab.count", "add_tab", "closeable_tab_strip_tab_added"),
        (
            "tab.deleted",
            "delete_tab",
            "closeable_tab_strip_tab_deleted",
        ),
        ("tab.pinned", "pin_tab", "closeable_tab_strip_pin_changed"),
        (
            "tab.dirty",
            "dirty_toggle",
            "closeable_tab_strip_dirty_changed",
        ),
        (
            "tab.group",
            "group_toggle",
            "closeable_tab_strip_group_changed",
        ),
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "closeable-tab-strip"
                    && it.option.name == option
                    && it.action == action
                    && it.event == event
            }),
            "missing closeable-tab-strip setting mutation for {option}"
        );
    }
}

pub(super) fn assert_context_menu_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "context_menu.anchor",
        "context_menu.placement_priority",
        "context_menu.placement_used",
        "context_menu.min_width",
        "context_menu.max_height",
        "context_menu.item_kind",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "context-menu"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "context_menu_settings_changed"
            }),
            "missing context-menu setting mutation for {option}"
        );
    }
}

pub(super) fn assert_drag_and_drop_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "drag.accept_policy",
        "drag.autoscroll",
        "drag.keyboard_draggable",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "drag-and-drop"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "drag_and_drop_settings_changed"
            }),
            "missing drag-and-drop setting mutation for {option}"
        );
    }
}

pub(super) fn is_typed_settings_record(it: &SettingsMutationReport) -> bool {
    !it.page.is_empty()
        && !it.action.is_empty()
        && !it.event.is_empty()
        && !it.target_state_id.is_empty()
        && !it.option.name.is_empty()
        && !it.option.value_type.is_empty()
        && it.state.before != it.state.after
        && it.preview.before != it.preview.after
}

pub(super) fn settings_state_uses_actual_option_after_value(it: &SettingsMutationReport) -> bool {
    it.state.after.contains(&format!(
        "option:{}={}",
        it.option.name, it.option.after_value
    )) && it.preview.after.contains(&it.option.after_value)
}

pub(super) fn preset_markers_are_ui_specific(it: &PresetDifferenceReport) -> bool {
    let markers = [
        it.default_marker.as_str(),
        it.interactive_marker.as_str(),
        it.edge_marker.as_str(),
        it.theme_marker.as_str(),
    ];
    markers.iter().all(|marker| marker.contains(&it.ui_marker))
        && markers.iter().collect::<BTreeSet<_>>().len() == markers.len()
}
