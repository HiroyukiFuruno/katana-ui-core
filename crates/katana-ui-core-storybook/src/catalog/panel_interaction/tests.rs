use super::StorybookPanelInteractionReport;
use crate::catalog::StoryCatalog;
use std::collections::BTreeSet;

const LEGACY_UI_MARKER_COUNT: usize = 27;
const DND_SETTINGS_MUTATION_COUNT: usize = 3;
const CLOSEABLE_TAB_STRIP_SETTINGS_MUTATION_COUNT: usize = 5;
const OVERLAY_SETTINGS_MUTATION_COUNT: usize = 9;

#[test]
fn report_covers_selector_overlay_and_color_picker_sequences() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    assert_eq!(4, report.selector_operations.len());
    assert_eq!(5, report.overlay_dismissals.len());
    assert_eq!(1, report.color_picker_updates.len());
    assert_eq!(
        examples.len()
            + 1
            + DND_SETTINGS_MUTATION_COUNT
            + 3
            + OVERLAY_SETTINGS_MUTATION_COUNT
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
