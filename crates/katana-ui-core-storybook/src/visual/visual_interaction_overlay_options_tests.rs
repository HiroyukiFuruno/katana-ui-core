use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn tooltip_inspector_options_mutate_overlay_semantic_state() -> Result<(), String> {
    assert_overlay_options(
        "tooltip",
        &[
            ("open", "tooltip.open=true"),
            ("placement", "tooltip.placement=edge"),
            ("focus", "tooltip.focus=first"),
            ("dismiss", "tooltip.dismiss=outside"),
        ],
    )
}

#[test]
fn popover_inspector_options_mutate_overlay_semantic_state() -> Result<(), String> {
    assert_overlay_options(
        "popover",
        &[
            ("open", "popover.open=true"),
            ("placement", "popover.placement=edge"),
            ("focus", "popover.focus=first"),
            ("dismiss", "popover.dismiss=outside"),
        ],
    )
}

#[test]
fn modal_inspector_options_mutate_overlay_semantic_state() -> Result<(), String> {
    assert_overlay_options(
        "modal",
        &[
            ("open", "modal.open=true"),
            ("placement", "modal.placement=edge"),
            ("focus", "modal.focus=first"),
            ("dismiss", "modal.dismiss=outside"),
        ],
    )
}

#[test]
fn modal_overlay_inspector_options_mutate_overlay_semantic_state() -> Result<(), String> {
    assert_overlay_options(
        "modal-overlay",
        &[
            ("open", "modal_overlay.open=true"),
            ("placement", "modal_overlay.placement=edge"),
            ("focus", "modal_overlay.focus=first"),
            ("dismiss", "modal_overlay.dismiss=outside"),
        ],
    )
}

fn assert_overlay_options(
    page: &'static str,
    expected_states: &'static [(&'static str, &'static str)],
) -> Result<(), String> {
    for &(setting, expected_state) in expected_states {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_inspector_option_state(
            &state,
            page,
            setting,
            expected_value(setting)?,
            expected_state,
        );
        assert!(
            component_body_pixel_diff(page, &before, &after) > 0,
            "{page} option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn expected_value(setting: &str) -> Result<&'static str, String> {
    match setting {
        "open" => Ok("true"),
        "placement" => Ok("edge"),
        "focus" => Ok("first"),
        "dismiss" => Ok("outside"),
        _ => Err(format!("unhandled overlay option `{setting}`")),
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
