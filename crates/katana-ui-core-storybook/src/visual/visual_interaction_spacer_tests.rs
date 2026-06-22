use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{component_body_pixel_diff, pixel_at};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PAGE: &str = "spacer";
const FIXED_PRESET: usize = 0;
const FLEX_PRESET: usize = 1;
const DENSE_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const GAP_SAMPLE_OFFSET: usize = 1;

#[test]
fn spacer_exposes_leaf_presets_options_and_layout_gap_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert_eq!(REQUIRED_PRESET_COUNT, presets.len());
    assert_eq!(REQUIRED_OPTION_COUNT, options.len());
    assert_eq!(options.len(), rows.len());
    assert_eq!("spacer_resize", spec.action);
    assert_eq!("spacer_changed", spec.event);
    assert_eq!("size", spec.option);
    assert_eq!("gap=large", spec.state);
}

#[test]
fn spacer_presets_render_distinct_fixed_flex_dense_and_theme_bodies() {
    let fixed = StorybookVisual.render_preset(DARK_THEME, PAGE, FIXED_PRESET, 0);
    let flex = StorybookVisual.render_preset(DARK_THEME, PAGE, FLEX_PRESET, 0);
    let dense = StorybookVisual.render_preset(DARK_THEME, PAGE, DENSE_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &fixed, &flex) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &fixed, &dense) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &fixed, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn spacer_gap_geometry_covers_fixed_flex_dense_and_parent_bounds() {
    let fixed = gap_rect(FIXED_PRESET);
    let flex = gap_rect(FLEX_PRESET);
    let dense = gap_rect(DENSE_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(flex.width > fixed.width);
    assert!(fixed.width > dense.width);
    assert_eq!(fixed.height, flex.height);
    assert_eq!(fixed.height, dense.height);
    for rect in [fixed, flex, dense] {
        assert!(component.contains(component.x + rect.x, component.y + rect.y));
        assert!(component.contains(
            component.x + rect.x + rect.width - GAP_SAMPLE_OFFSET,
            component.y + rect.y + rect.height - GAP_SAMPLE_OFFSET
        ));
    }
}

#[test]
fn spacer_theme_preset_draws_gap_from_theme_token() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let gap = gap_rect(THEME_PRESET);

    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            rect.x + gap.x + GAP_SAMPLE_OFFSET,
            rect.y + gap.y + GAP_SAMPLE_OFFSET
        )
    );
}

fn gap_rect(preset_index: usize) -> super::dedicated_dod_common::Rect {
    let screen_state = StorybookScreenState::default();
    let scenario = ScenarioContext {
        selected_page: PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state: &screen_state,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    };
    super::dedicated_dod_atom_spacer::gap_rect_for_test(scenario)
}
