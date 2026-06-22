use super::window_interaction::StorybookWindowState;

pub(super) fn expected_action(page: &str) -> &'static str {
    match page {
        "shortcut-combo" => "settings_shortcut_combo_option",
        "skeleton-cluster" => "settings_skeleton_cluster_option",
        "window-control-button-group" => "settings_window_control_option",
        "accordion" => "settings_accordion_option",
        _ => "settings_option_changed",
    }
}

pub(super) fn assert_runtime_structured_state(
    page: &str,
    setting: &str,
    state: &StorybookWindowState,
) {
    match page {
        "shortcut-combo" => assert_shortcut_combo(setting, state),
        "skeleton-cluster" => assert_skeleton_cluster(setting, state),
        "window-control-button-group" => assert_window_control(setting, state),
        "accordion" => assert_accordion(setting, state),
        _ => {}
    }
}

fn assert_shortcut_combo(setting: &str, state: &StorybookWindowState) {
    let runtime = state.screen_state.runtime_structured.shortcut_combo;
    match setting {
        "shortcut_combo.platform_display" => assert!(runtime.platform_display_macos),
        "shortcut_combo.separator" => assert!(runtime.separator_none),
        "shortcut_combo.size" => assert!(runtime.size_large),
        "shortcut_combo.tone" => assert!(runtime.tone_accent),
        "shortcut_combo.a11y_label" => assert!(runtime.a11y_custom),
        _ => {}
    }
}

fn assert_skeleton_cluster(setting: &str, state: &StorybookWindowState) {
    let runtime = state.screen_state.runtime_structured.skeleton_cluster;
    match setting {
        "skeleton_cluster.preset" => assert!(runtime.preset_card),
        "skeleton_cluster.children" => assert!(runtime.children_three),
        "skeleton_cluster.live_region" => assert!(runtime.live_region_card),
        "skeleton_cluster.reduced_motion" => assert!(runtime.reduced_motion),
        _ => {}
    }
}

fn assert_window_control(setting: &str, state: &StorybookWindowState) {
    let runtime = state.screen_state.runtime_structured.window_control;
    match setting {
        "window_control.position" => assert!(runtime.position_trailing),
        "window_control.size" => assert!(runtime.size_tall),
        "window_control.controls" => assert!(runtime.controls_close_only),
        "window_control.visibility" => assert!(runtime.visibility_hover),
        _ => {}
    }
}

fn assert_accordion(setting: &str, state: &StorybookWindowState) {
    let runtime = state.screen_state.runtime_structured.accordion;
    match setting {
        "accordion.expanded" => assert!(runtime.expanded),
        "accordion.disabled" => assert!(runtime.disabled),
        "accordion.controlled" => assert!(runtime.controlled),
        "accordion.trigger_area" => assert!(runtime.trigger_area_full_row),
        "accordion.reduced_motion" => assert!(runtime.reduced_motion),
        _ => {}
    }
}
