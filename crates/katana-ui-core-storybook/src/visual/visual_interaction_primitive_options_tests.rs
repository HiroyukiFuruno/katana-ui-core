use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn primitive_inspector_options_mutate_variant_tone_size_and_theme_slot_semantic_state()
-> Result<(), String> {
    for &(page, prefix) in pages() {
        assert_options(page, prefix)?;
    }
    Ok(())
}

fn pages() -> &'static [(&'static str, &'static str)] {
    &[
        ("divider", "divider"),
        ("spacer", "spacer"),
        ("color-swatch", "color_swatch"),
        ("slide-control", "slide_control"),
    ]
}

fn assert_options(page: &'static str, prefix: &'static str) -> Result<(), String> {
    for &(setting, expected_value, suffix) in expected_states() {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_inspector_option_state(
            &state,
            page,
            setting,
            expected_value,
            state_label(prefix, suffix),
        );
        assert!(component_body_pixel_diff(page, &before, &after) > 0);
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("variant", "alternate", "variant=alternate"),
        ("tone", "accent", "tone=accent"),
        ("size", "large", "size=large"),
        ("theme.slot", "custom", "theme.slot=custom"),
    ]
}

fn state_label(prefix: &str, suffix: &str) -> &'static str {
    match (prefix, suffix) {
        ("divider", "variant=alternate") => "divider.variant=alternate",
        ("divider", "tone=accent") => "divider.tone=accent",
        ("divider", "size=large") => "divider.size=large",
        ("divider", "theme.slot=custom") => "divider.theme.slot=custom",
        ("spacer", "variant=alternate") => "spacer.variant=alternate",
        ("spacer", "tone=accent") => "spacer.tone=accent",
        ("spacer", "size=large") => "spacer.size=large",
        ("spacer", "theme.slot=custom") => "spacer.theme.slot=custom",
        ("color_swatch", "variant=alternate") => "color_swatch.variant=alternate",
        ("color_swatch", "tone=accent") => "color_swatch.tone=accent",
        ("color_swatch", "size=large") => "color_swatch.size=large",
        ("color_swatch", "theme.slot=custom") => "color_swatch.theme.slot=custom",
        ("slide_control", "variant=alternate") => "slide_control.variant=alternate",
        ("slide_control", "tone=accent") => "slide_control.tone=accent",
        ("slide_control", "size=large") => "slide_control.size=large",
        ("slide_control", "theme.slot=custom") => "slide_control.theme.slot=custom",
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
