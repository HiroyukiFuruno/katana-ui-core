use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn feedback_inspector_options_mutate_severity_duration_action_and_dismiss_semantic_state()
-> Result<(), String> {
    assert_options("toast-stack-manager", "toast_stack")?;
    assert_options("notification-toast", "notification_toast")
}

fn assert_options(page: &'static str, prefix: &'static str) -> Result<(), String> {
    for (index, &(setting, expected_value, suffix)) in expected_states().iter().enumerate() {
        let mut state = page_state(page);
        let before = if index == 0 {
            Some(render_state(&state, page))
        } else {
            None
        };
        click_option(&mut state, page, setting)?;

        assert_inspector_option_state(
            &state,
            page,
            setting,
            expected_value,
            state_label(prefix, suffix),
        );
        if let Some(before) = before {
            let after = render_state(&state, page);
            assert!(component_body_pixel_diff(page, &before, &after) > 0);
        }
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("severity", "warning", "severity=warning"),
        ("duration", "custom", "duration=custom"),
        ("action", "visible", "action=visible"),
        ("dismiss", "true", "dismiss=true"),
    ]
}

fn state_label(prefix: &str, suffix: &str) -> &'static str {
    match (prefix, suffix) {
        ("toast_stack", "severity=warning") => "toast_stack.severity=warning",
        ("toast_stack", "duration=custom") => "toast_stack.duration=custom",
        ("toast_stack", "action=visible") => "toast_stack.action=visible",
        ("toast_stack", "dismiss=true") => "toast_stack.dismiss=true",
        ("notification_toast", "severity=warning") => "notification_toast.severity=warning",
        ("notification_toast", "duration=custom") => "notification_toast.duration=custom",
        ("notification_toast", "action=visible") => "notification_toast.action=visible",
        ("notification_toast", "dismiss=true") => "notification_toast.dismiss=true",
        _ => "",
    }
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
