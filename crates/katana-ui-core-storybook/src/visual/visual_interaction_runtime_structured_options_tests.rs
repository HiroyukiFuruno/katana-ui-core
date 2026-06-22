use super::visual_interaction_runtime_structured_assertions::{
    assert_runtime_structured_state, expected_action,
};
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn shortcut_combo_inspector_options_mutate_display_size_tone_and_a11y_semantic_state()
-> Result<(), String> {
    assert_options(
        "shortcut-combo",
        &[
            (
                "shortcut_combo.platform_display",
                "MacOS",
                "shortcut_combo.platform_display=MacOS",
            ),
            (
                "shortcut_combo.separator",
                "None",
                "shortcut_combo.separator=None",
            ),
            ("shortcut_combo.size", "Large", "shortcut_combo.size=Large"),
            (
                "shortcut_combo.tone",
                "Accent",
                "shortcut_combo.tone=Accent",
            ),
            (
                "shortcut_combo.a11y_label",
                "custom",
                "shortcut_combo.a11y_label=custom",
            ),
        ],
    )
}

#[test]
fn skeleton_cluster_inspector_options_mutate_preset_children_and_motion_semantic_state()
-> Result<(), String> {
    assert_options(
        "skeleton-cluster",
        &[
            (
                "skeleton_cluster.preset",
                "Card",
                "skeleton_cluster.preset=Card",
            ),
            (
                "skeleton_cluster.children",
                "3",
                "skeleton_cluster.children=3",
            ),
            (
                "skeleton_cluster.live_region",
                "card",
                "skeleton_cluster.live_region=card",
            ),
            (
                "skeleton_cluster.reduced_motion",
                "true",
                "skeleton_cluster.reduced_motion=true",
            ),
        ],
    )
}

#[test]
fn window_control_inspector_options_mutate_position_size_controls_and_visibility_semantic_state()
-> Result<(), String> {
    assert_options(
        "window-control-button-group",
        &[
            (
                "window_control.position",
                "Trailing",
                "window_control.position=Trailing",
            ),
            ("window_control.size", "Tall", "window_control.size=Tall"),
            (
                "window_control.controls",
                "Close",
                "window_control.controls=Close",
            ),
            (
                "window_control.visibility",
                "Hover",
                "window_control.visibility=Hover",
            ),
        ],
    )
}

#[test]
fn accordion_inspector_options_mutate_controlled_trigger_and_motion_semantic_state()
-> Result<(), String> {
    assert_options(
        "accordion",
        &[
            ("accordion.expanded", "true", "accordion.expanded=true"),
            ("accordion.disabled", "true", "accordion.disabled=true"),
            ("accordion.controlled", "true", "accordion.controlled=true"),
            (
                "accordion.trigger_area",
                "full-row",
                "accordion.trigger_area=full-row",
            ),
            (
                "accordion.reduced_motion",
                "true",
                "accordion.reduced_motion=true",
            ),
        ],
    )
}

fn assert_options(
    page: &'static str,
    expected_states: &'static [(&'static str, &'static str, &'static str)],
) -> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(expected_action(page), state.screen_state.last_action);
        assert_eq!("runtime_settings_changed", state.screen_state.last_event);
        assert_eq!(expected_value, state.screen_state.last_setting_value);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_runtime_structured_state(page, setting, &state);
        assert!(
            component_body_pixel_diff(page, &before, &after) > 0,
            "{page} option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn click_option(state: &mut StorybookWindowState, page: &str, setting: &str) -> Result<(), String> {
    let index = option_index(page, setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(page: &str, setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing {page} option `{setting}`"))
}

fn render_state(state: &StorybookWindowState, page: &str) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
