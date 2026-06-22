use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    component_body_pixel_diff, pixel_at, rect_non_background_pixels,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "badge";
const DEFAULT_PRESET: usize = 0;
const PASSIVE_PRESET: usize = 1;
const SMALL_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const LEADING_ICON_PRESET: usize = 4;
const FILLED_VARIANT_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 6;
const REQUIRED_OPTION_COUNT: usize = 6;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 22;
const SAMPLE_Y_OFFSET: usize = 44;
const MIN_BADGE_PIXELS: usize = 80;
const BADGE_CHIP_COUNT: usize = 6;

#[test]
fn badge_exposes_leaf_presets_options_and_passive_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("badge_passive", spec.action);
    assert_eq!("none", spec.event);
    assert_eq!("use Chip for dismiss", spec.state);
}

#[test]
fn badge_presets_render_distinct_passive_bodies() {
    let tone_grid = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let passive = StorybookVisual.render_preset(DARK_THEME, PAGE, PASSIVE_PRESET, 0);
    let small = StorybookVisual.render_preset(DARK_THEME, PAGE, SMALL_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);
    let leading_icon = StorybookVisual.render_preset(DARK_THEME, PAGE, LEADING_ICON_PRESET, 0);
    let filled = StorybookVisual.render_preset(DARK_THEME, PAGE, FILLED_VARIANT_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &tone_grid, &passive) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &passive, &small) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &small, &themed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &themed, &leading_icon) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &leading_icon, &filled) > BODY_DIFF_THRESHOLD);
}

#[test]
fn badge_preview_action_changes_passive_style_evidence() {
    let before = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        PAGE,
        DEFAULT_PRESET,
        0,
        true,
    );

    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn badge_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, DEFAULT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}

#[test]
fn badge_chips_stay_inside_component_bounds_and_small_preset_compacts() {
    let default = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let default_rects = badge_rects(DEFAULT_PRESET);
    let small_rects = badge_rects(SMALL_PRESET);

    for rect in default_rects {
        assert!(component.contains(rect.x, rect.y));
        assert!(component.contains(rect.right() - 1, rect.bottom() - 1));
        assert!(
            rect_non_background_pixels(rect, &default, palette::DEFAULT_BACKGROUND)
                > MIN_BADGE_PIXELS
        );
    }

    assert!(small_rects[0].width < default_rects[0].width);
    assert!(small_rects[0].height < default_rects[0].height);
    assert_eq!(
        Some(colors.panel),
        pixel_at(&default, default_rects[0].x + 1, default_rects[0].y + 1)
    );
}

fn badge_rects(preset_index: usize) -> [super::layout_metrics::LayoutRect; BADGE_CHIP_COUNT] {
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
    let component = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_molecule_badge::badge_chip_rects_for_test(
        scenario,
        component.x,
        component.y,
    )
}
