use super::{PresetDifferenceReport, SettingsMutationReport, StorybookPanelInteractionReport};
use crate::catalog::StoryCatalog;
use std::collections::BTreeSet;

mod settings_switch_app_assertions;
mod settings_switch_core_assertions;
mod settings_switch_interaction_assertions;
use settings_switch_app_assertions::*;
use settings_switch_core_assertions::*;
use settings_switch_interaction_assertions::*;
const LEGACY_UI_MARKER_COUNT: usize = 30;
const SELECTOR_OPERATION_COUNT: usize = 13;
const DND_SETTINGS_MUTATION_COUNT: usize = 3;
const CLOSEABLE_TAB_STRIP_SETTINGS_MUTATION_COUNT: usize = 5;
const OVERLAY_SETTINGS_MUTATION_COUNT: usize = 9;
const TOOLBAR_SETTINGS_MUTATION_COUNT: usize = 5;
const TEXT_AREA_SETTINGS_MUTATION_COUNT: usize = 10;
const CHIP_SETTINGS_MUTATION_COUNT: usize = 6;
const COLOR_PICKER_SETTINGS_MUTATION_COUNT: usize = 9;
const COLOR_PICKER_UPDATE_COUNT: usize = 10;
const COMMAND_PALETTE_SETTINGS_MUTATION_COUNT: usize = 5;
const DIAGNOSTICS_SETTINGS_MUTATION_COUNT: usize = 5;
const EMPTY_STATE_SETTINGS_MUTATION_COUNT: usize = 4;
const MOTION_SETTINGS_MUTATION_COUNT: usize = 6;
const BANNER_SETTINGS_MUTATION_COUNT: usize = 5;
const TOAST_STACK_SETTINGS_MUTATION_COUNT: usize = 6;
const STATUS_BAR_SETTINGS_MUTATION_COUNT: usize = 3;
const SHORTCUT_COMBO_SETTINGS_MUTATION_COUNT: usize = 4;
const SEARCH_CONTROL_STRIP_SETTINGS_MUTATION_COUNT: usize = 7;
const SCROLL_AREA_SETTINGS_MUTATION_COUNT: usize = 6;
const SPLIT_PANE_SETTINGS_MUTATION_COUNT: usize = 6;
const SETTINGS_LIST_SETTINGS_MUTATION_COUNT: usize = 6;
const COLLAPSIBLE_PANEL_SETTINGS_MUTATION_COUNT: usize = 5;
const WINDOW_CONTROL_SETTINGS_MUTATION_COUNT: usize = 4;
const STARTUP_STATE_SETTINGS_MUTATION_COUNT: usize = 5;

#[test]
fn report_covers_selector_overlay_and_color_picker_sequences() {
    let examples = StoryCatalog.examples();
    let report = StorybookPanelInteractionReport::build(&examples);

    assert_eq!(SELECTOR_OPERATION_COUNT, report.selector_operations.len());
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
            + COMMAND_PALETTE_SETTINGS_MUTATION_COUNT
            + DIAGNOSTICS_SETTINGS_MUTATION_COUNT
            + EMPTY_STATE_SETTINGS_MUTATION_COUNT
            + MOTION_SETTINGS_MUTATION_COUNT
            + BANNER_SETTINGS_MUTATION_COUNT
            + TOAST_STACK_SETTINGS_MUTATION_COUNT
            + STATUS_BAR_SETTINGS_MUTATION_COUNT
            + STARTUP_STATE_SETTINGS_MUTATION_COUNT
            + SHORTCUT_COMBO_SETTINGS_MUTATION_COUNT
            + SEARCH_CONTROL_STRIP_SETTINGS_MUTATION_COUNT
            + SCROLL_AREA_SETTINGS_MUTATION_COUNT
            + SPLIT_PANE_SETTINGS_MUTATION_COUNT
            + SETTINGS_LIST_SETTINGS_MUTATION_COUNT
            + COLLAPSIBLE_PANEL_SETTINGS_MUTATION_COUNT
            + WINDOW_CONTROL_SETTINGS_MUTATION_COUNT
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
    assert_command_palette_settings_are_switchable(&report.settings_mutations);
    assert_diagnostics_settings_are_switchable(&report.settings_mutations);
    assert_empty_state_settings_are_switchable(&report.settings_mutations);
    assert_motion_settings_are_switchable(&report.settings_mutations);
    assert_banner_settings_are_switchable(&report.settings_mutations);
    assert_toast_stack_settings_are_switchable(&report.settings_mutations);
    assert_status_bar_settings_are_switchable(&report.settings_mutations);
    assert_startup_state_settings_are_switchable(&report.settings_mutations);
    assert_shortcut_combo_settings_are_switchable(&report.settings_mutations);
    assert_search_control_strip_settings_are_switchable(&report.settings_mutations);
    assert_scroll_area_settings_are_switchable(&report.settings_mutations);
    assert_split_pane_settings_are_switchable(&report.settings_mutations);
    assert_settings_list_settings_are_switchable(&report.settings_mutations);
    assert_collapsible_panel_settings_are_switchable(&report.settings_mutations);
    assert_window_control_settings_are_switchable(&report.settings_mutations);
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
