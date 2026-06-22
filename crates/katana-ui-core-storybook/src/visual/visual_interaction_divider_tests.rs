use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{component_body_pixel_diff, pixel_at};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PAGE: &str = "divider";
const HORIZONTAL_PRESET: usize = 0;
const VERTICAL_PRESET: usize = 1;
const INSET_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const LINE_SAMPLE_OFFSET: usize = 1;

#[test]
fn divider_exposes_leaf_presets_options_and_separator_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert_eq!(REQUIRED_PRESET_COUNT, presets.len());
    assert_eq!(REQUIRED_OPTION_COUNT, options.len());
    assert_eq!(options.len(), rows.len());
    assert_eq!("divider_resize", spec.action);
    assert_eq!("divider_changed", spec.event);
    assert_eq!("size", spec.option);
    assert_eq!("inset=true", spec.state);
}

#[test]
fn divider_presets_render_distinct_orientation_inset_and_theme_bodies() {
    let horizontal = StorybookVisual.render_preset(DARK_THEME, PAGE, HORIZONTAL_PRESET, 0);
    let vertical = StorybookVisual.render_preset(DARK_THEME, PAGE, VERTICAL_PRESET, 0);
    let inset = StorybookVisual.render_preset(DARK_THEME, PAGE, INSET_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &horizontal, &vertical) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &horizontal, &inset) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &horizontal, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn divider_line_geometry_covers_orientation_thickness_and_inset() {
    let horizontal = line_rect(HORIZONTAL_PRESET);
    let vertical = line_rect(VERTICAL_PRESET);
    let inset = line_rect(INSET_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(horizontal.width > horizontal.height);
    assert!(vertical.height > vertical.width);
    assert_eq!(horizontal.height, vertical.width);
    assert!(inset.x > horizontal.x);
    assert!(inset.width < horizontal.width);
    for rect in [horizontal, vertical, inset] {
        assert!(component.contains(component.x + rect.x, component.y + rect.y));
        assert!(component.contains(
            component.x + rect.x + rect.width - LINE_SAMPLE_OFFSET,
            component.y + rect.y + rect.height - LINE_SAMPLE_OFFSET
        ));
    }
}

#[test]
fn divider_theme_preset_draws_line_from_theme_token() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let line = line_rect(THEME_PRESET);

    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            rect.x + line.x + LINE_SAMPLE_OFFSET,
            rect.y + line.y + LINE_SAMPLE_OFFSET
        )
    );
}

fn line_rect(preset_index: usize) -> super::dedicated_dod_common::Rect {
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
    super::dedicated_dod_atom_divider::line_rect_for_test(scenario)
}
