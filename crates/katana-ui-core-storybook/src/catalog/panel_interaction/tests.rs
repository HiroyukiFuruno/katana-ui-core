use super::StorybookPanelInteractionReport;
use crate::catalog::StoryCatalog;
use std::collections::BTreeSet;

const LEGACY_UI_MARKER_COUNT: usize = 30;
const DND_SETTINGS_MUTATION_COUNT: usize = 3;
const CLOSEABLE_TAB_STRIP_SETTINGS_MUTATION_COUNT: usize = 5;
const OVERLAY_SETTINGS_MUTATION_COUNT: usize = 9;
const TOOLBAR_SETTINGS_MUTATION_COUNT: usize = 5;
const TEXT_AREA_SETTINGS_MUTATION_COUNT: usize = 5;
const CHIP_SETTINGS_MUTATION_COUNT: usize = 6;
const COLOR_PICKER_SETTINGS_MUTATION_COUNT: usize = 9;
const COLOR_PICKER_UPDATE_COUNT: usize = 10;
const DIAGNOSTICS_SETTINGS_MUTATION_COUNT: usize = 5;
const EMPTY_STATE_SETTINGS_MUTATION_COUNT: usize = 4;
const BANNER_SETTINGS_MUTATION_COUNT: usize = 5;
const TOAST_STACK_SETTINGS_MUTATION_COUNT: usize = 6;
const STATUS_BAR_SETTINGS_MUTATION_COUNT: usize = 3;
const SHORTCUT_COMBO_SETTINGS_MUTATION_COUNT: usize = 4;
const SEARCH_CONTROL_STRIP_SETTINGS_MUTATION_COUNT: usize = 7;
const SCROLL_AREA_SETTINGS_MUTATION_COUNT: usize = 6;
const SPLIT_PANE_SETTINGS_MUTATION_COUNT: usize = 6;
const SETTINGS_LIST_SETTINGS_MUTATION_COUNT: usize = 6;
const COLLAPSIBLE_PANEL_SETTINGS_MUTATION_COUNT: usize = 5;

#[test]
fn report_covers_selector_overlay_and_color_picker_sequences() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    assert_eq!(4, report.selector_operations.len());
    assert_eq!(5, report.overlay_dismissals.len());
    assert_eq!(COLOR_PICKER_UPDATE_COUNT, report.color_picker_updates.len());
    assert_eq!(
        examples.len()
            + 1
            + DND_SETTINGS_MUTATION_COUNT
            + 3
            + OVERLAY_SETTINGS_MUTATION_COUNT
            + TOOLBAR_SETTINGS_MUTATION_COUNT
            + TEXT_AREA_SETTINGS_MUTATION_COUNT
            + CHIP_SETTINGS_MUTATION_COUNT
            + COLOR_PICKER_SETTINGS_MUTATION_COUNT
            + DIAGNOSTICS_SETTINGS_MUTATION_COUNT
            + EMPTY_STATE_SETTINGS_MUTATION_COUNT
            + BANNER_SETTINGS_MUTATION_COUNT
            + TOAST_STACK_SETTINGS_MUTATION_COUNT
            + STATUS_BAR_SETTINGS_MUTATION_COUNT
            + SHORTCUT_COMBO_SETTINGS_MUTATION_COUNT
            + SEARCH_CONTROL_STRIP_SETTINGS_MUTATION_COUNT
            + SCROLL_AREA_SETTINGS_MUTATION_COUNT
            + SPLIT_PANE_SETTINGS_MUTATION_COUNT
            + SETTINGS_LIST_SETTINGS_MUTATION_COUNT
            + COLLAPSIBLE_PANEL_SETTINGS_MUTATION_COUNT
            + CLOSEABLE_TAB_STRIP_SETTINGS_MUTATION_COUNT,
        report.settings_mutations.len()
    );
    assert_eq!(LEGACY_UI_MARKER_COUNT, report.legacy_ui_markers.len());
    assert_eq!(LEGACY_UI_MARKER_COUNT, report.preset_differences.len());
    assert_eq!(12, report.tree_view_option_mutations.len());
    assert!(
        report
            .selector_operations
            .iter()
            .any(|it| it.action == "select_box_selected")
    );
    assert!(
        report
            .overlay_dismissals
            .iter()
            .any(|it| it.action == "modal_escape")
    );
    assert!(
        report
            .color_picker_updates
            .iter()
            .any(|it| it.action == "color_drag")
    );
    assert!(
        report
            .settings_mutations
            .iter()
            .all(|it| it.option.before_value != it.option.after_value)
    );
    assert!(
        report
            .settings_mutations
            .iter()
            .filter(|it| it.ui_marker.starts_with("legacy-"))
            .all(is_typed_settings_record)
    );
    assert!(
        report
            .settings_mutations
            .iter()
            .all(settings_state_uses_actual_option_after_value)
    );
    assert!(
        report
            .settings_mutations
            .iter()
            .all(|it| !it.option.after_value.ends_with("-settings"))
    );
    assert!(
        report
            .settings_mutations
            .iter()
            .any(|it| it.page == "text" && it.option.name == "text.role")
    );
    assert!(report.settings_mutations.iter().any(
        |it| it.page == "color-picker-rgba" && it.option.name == "color_swatch.selected_color"
    ));
    assert_drag_and_drop_settings_are_switchable(&report.settings_mutations);
    assert_context_menu_settings_are_switchable(&report.settings_mutations);
    assert_closeable_tab_strip_settings_are_switchable(&report.settings_mutations);
    assert_overlay_settings_are_switchable(&report.settings_mutations);
    assert_toolbar_settings_are_switchable(&report.settings_mutations);
    assert_text_area_settings_are_switchable(&report.settings_mutations);
    assert_chip_settings_are_switchable(&report.settings_mutations);
    assert_color_picker_settings_are_switchable(&report.settings_mutations);
    assert_diagnostics_settings_are_switchable(&report.settings_mutations);
    assert_empty_state_settings_are_switchable(&report.settings_mutations);
    assert_banner_settings_are_switchable(&report.settings_mutations);
    assert_toast_stack_settings_are_switchable(&report.settings_mutations);
    assert_status_bar_settings_are_switchable(&report.settings_mutations);
    assert_shortcut_combo_settings_are_switchable(&report.settings_mutations);
    assert_search_control_strip_settings_are_switchable(&report.settings_mutations);
    assert_scroll_area_settings_are_switchable(&report.settings_mutations);
    assert_split_pane_settings_are_switchable(&report.settings_mutations);
    assert_settings_list_settings_are_switchable(&report.settings_mutations);
    assert_collapsible_panel_settings_are_switchable(&report.settings_mutations);
    assert_eq!(
        report.legacy_ui_markers.len(),
        report
            .legacy_ui_markers
            .iter()
            .map(|it| it.ui_marker.as_str())
            .collect::<BTreeSet<_>>()
            .len()
    );
    assert!(
        report
            .preset_differences
            .iter()
            .all(preset_markers_are_ui_specific)
    );
    assert!(
        report
            .tree_view_option_mutations
            .iter()
            .any(|it| it.action == "tree_click_toggle" && it.after_summary.contains("open=false"))
    );
}

fn assert_collapsible_panel_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn assert_search_control_strip_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn assert_scroll_area_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn assert_split_pane_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn assert_settings_list_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn assert_banner_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "banner.severity",
        "banner.density",
        "banner.actions",
        "banner.details",
        "banner.dismissible",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "banner"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "banner_settings_changed"
            }),
            "missing banner setting mutation for {option}"
        );
    }
}

fn assert_toast_stack_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "toast_stack.position",
        "toast_stack.max_visible",
        "toast_stack.dedup",
        "toast_stack.duration",
        "toast_stack.pause_on_hover",
        "toast_stack.stack_gap",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "toast-stack-manager"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "toast_stack_settings_changed"
            }),
            "missing toast stack setting mutation for {option}"
        );
    }
}

fn assert_status_bar_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "status_bar.mode",
        "status_bar.segments",
        "status_bar.density",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "status-bar"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "status_bar_settings_changed"
            }),
            "missing status bar setting mutation for {option}"
        );
    }
}

fn assert_shortcut_combo_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "shortcut_combo.platform_display",
        "shortcut_combo.separator",
        "shortcut_combo.size",
        "shortcut_combo.tone",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "shortcut-combo"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "shortcut_combo_settings_changed"
            }),
            "missing shortcut combo setting mutation for {option}"
        );
    }
}

fn assert_empty_state_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "empty_state.tone",
        "empty_state.size",
        "empty_state.alignment",
        "empty_state.actions",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "empty-state"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "empty_state_settings_changed"
            }),
            "missing empty-state setting mutation for {option}"
        );
    }
}

fn assert_color_picker_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "color_picker.mode",
        "color_picker.red",
        "color_picker.green",
        "color_picker.blue",
        "color_picker.alpha",
        "color_picker.blending",
        "color_picker.eyedropper",
        "color_picker.readonly",
        "color_picker.disabled",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "color-picker-rgba"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "color_picker_settings_changed"
            }),
            "missing color picker setting mutation for {option}"
        );
    }
}

fn assert_diagnostics_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "diagnostics.group_by",
        "diagnostics.sort_by",
        "diagnostics.severity_filter",
        "diagnostics.bulk_action",
        "diagnostics.fix_preview",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "diagnostics-list"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "diagnostics_list_settings_changed"
            }),
            "missing diagnostics-list setting mutation for {option}"
        );
    }
}

fn assert_chip_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for (page, option, event) in [
        ("chip", "chip.variant", "chip_settings_changed"),
        ("chip", "chip.tone", "chip_settings_changed"),
        ("chip", "chip.size", "chip_settings_changed"),
        (
            "attachment-chip",
            "attachment.status",
            "attachment_chip_settings_changed",
        ),
        (
            "attachment-chip",
            "attachment.progress",
            "attachment_chip_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.overflow",
            "chip_group_settings_changed",
        ),
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == page
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == event
            }),
            "missing chip setting mutation for {page} {option}"
        );
    }
}

fn assert_toolbar_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "toolbar.action_count",
        "toolbar.priority",
        "toolbar.overflow_strategy",
        "toolbar.display_mode",
        "toolbar.density",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "toolbar"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "toolbar_settings_changed"
            }),
            "missing toolbar setting mutation for {option}"
        );
    }
}

fn assert_text_area_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "text_area.submit_key",
        "text_area.newline_key",
        "text_area.tab_behavior",
        "text_area.auto_grow",
        "text_area.wrap_policy",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "text-area"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "text_area_settings_changed"
            }),
            "missing text-area setting mutation for {option}"
        );
    }
}

fn assert_overlay_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "popover.placement",
        "popover.arrow",
        "popover.focus_management",
        "popover.slot",
        "hover_card.delay",
        "hover_card.placement",
        "hover_card.arrow",
        "hover_card.focus",
        "hover_card.slot",
    ] {
        assert!(
            settings.iter().any(|it| {
                (it.page == "popover" || it.page == "hover-card")
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event.ends_with("_settings_changed")
            }),
            "missing overlay setting mutation for {option}"
        );
    }
}

fn assert_closeable_tab_strip_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn assert_context_menu_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
    for option in [
        "context_menu.anchor",
        "context_menu.placement",
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

fn assert_drag_and_drop_settings_are_switchable(settings: &[super::SettingsMutationReport]) {
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

fn is_typed_settings_record(it: &super::SettingsMutationReport) -> bool {
    !it.page.is_empty()
        && !it.action.is_empty()
        && !it.event.is_empty()
        && !it.target_state_id.is_empty()
        && !it.option.name.is_empty()
        && !it.option.value_type.is_empty()
        && it.state.before != it.state.after
        && it.preview.before != it.preview.after
}

fn settings_state_uses_actual_option_after_value(it: &super::SettingsMutationReport) -> bool {
    it.state.after.contains(&format!(
        "option:{}={}",
        it.option.name, it.option.after_value
    )) && it.preview.after.contains(&it.option.after_value)
}

fn preset_markers_are_ui_specific(it: &super::PresetDifferenceReport) -> bool {
    let markers = [
        it.default_marker.as_str(),
        it.interactive_marker.as_str(),
        it.edge_marker.as_str(),
        it.theme_marker.as_str(),
    ];
    markers.iter().all(|marker| marker.contains(&it.ui_marker))
        && markers.iter().collect::<BTreeSet<_>>().len() == markers.len()
}
