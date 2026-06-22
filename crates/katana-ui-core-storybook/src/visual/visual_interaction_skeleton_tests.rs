use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::atom::{Skeleton, SkeletonAnimation, SkeletonShape};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "skeleton";
const TEXT_PRESET: usize = 0;
const LINE_THICKNESS_PRESET: usize = 3;
const SIZE_PRESET: usize = 4;
const REDUCED_MOTION_PRESET: usize = 8;
const ASPECT_RATIO_PRESET: usize = 10;
const REQUIRED_PRESET_COUNT: usize = 11;
const REQUIRED_OPTION_COUNT: usize = 11;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 22;
const SAMPLE_Y_OFFSET: usize = 38;
const SKELETON_BLOCK_COUNT: usize = 5;
const PRIMARY_BLOCK_INDEX: usize = 0;
const SECONDARY_BLOCK_INDEX: usize = 1;
const TERTIARY_BLOCK_INDEX: usize = 2;

#[test]
fn skeleton_exposes_leaf_presets_options_and_animation_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("reduced_motion_toggle", spec.action);
    assert_eq!("skeleton_animation_changed", spec.event);
    assert_eq!("reduced_motion=true", spec.state);
}

#[test]
fn skeleton_presets_render_distinct_placeholder_bodies() {
    let base = StorybookVisual.render_preset(DARK_THEME, PAGE, TEXT_PRESET, 0);
    for preset_index in 1..StoryPresetLabels::for_page(PAGE).len() {
        let candidate = StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index, 0);
        assert!(
            component_body_pixel_diff(PAGE, &base, &candidate) > BODY_DIFF_THRESHOLD,
            "skeleton preset {} did not change the preview body",
            StoryPresetLabels::for_page(PAGE)[preset_index]
        );
    }
}

#[test]
fn skeleton_core_reduced_motion_disables_effective_animation() {
    let skeleton = Skeleton::new("Loading", SkeletonShape::Rect)
        .animation(SkeletonAnimation::Wave)
        .reduced_motion(true);

    assert_eq!(SkeletonAnimation::None, skeleton.effective_animation());
}

#[test]
fn skeleton_geometry_covers_line_thickness_fill_and_aspect_ratio() {
    let text = skeleton_blocks(TEXT_PRESET);
    let line = skeleton_blocks(LINE_THICKNESS_PRESET);
    let fill = skeleton_blocks(SIZE_PRESET);
    let aspect = skeleton_blocks(ASPECT_RATIO_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(line[PRIMARY_BLOCK_INDEX].rect.height > text[PRIMARY_BLOCK_INDEX].rect.height);
    assert!(line[TERTIARY_BLOCK_INDEX].rect.height > text[TERTIARY_BLOCK_INDEX].rect.height);
    assert!(fill[PRIMARY_BLOCK_INDEX].rect.width > text[PRIMARY_BLOCK_INDEX].rect.width);
    assert!(aspect[PRIMARY_BLOCK_INDEX].rect.height > text[PRIMARY_BLOCK_INDEX].rect.height);
    for block in [
        fill[PRIMARY_BLOCK_INDEX],
        fill[TERTIARY_BLOCK_INDEX],
        fill[SECONDARY_BLOCK_INDEX],
    ] {
        assert!(component.contains(component.x + block.rect.x, component.y + block.rect.y));
        assert!(component.contains(
            component.x + block.rect.x + block.rect.width - 1,
            component.y + block.rect.y + block.rect.height - 1
        ));
    }
}

#[test]
fn skeleton_reduced_motion_preset_uses_static_motion_token() {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let reduced = skeleton_blocks(REDUCED_MOTION_PRESET);

    assert_eq!(colors.panel, reduced[SECONDARY_BLOCK_INDEX].fill);
    assert_eq!(
        "reduced motion=true",
        skeleton_motion_label(REDUCED_MOTION_PRESET)
    );
}

#[test]
fn skeleton_setting_option_updates_placeholder_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn skeleton_preview_action_updates_placeholder_style() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn skeleton_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, TEXT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, TEXT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}

fn skeleton_blocks(
    preset_index: usize,
) -> [super::dedicated_dod_atom_skeleton::SkeletonBlockSnapshot; SKELETON_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_skeleton::skeleton_blocks_for_test(&colors, scenario)
}

fn skeleton_motion_label(preset_index: usize) -> &'static str {
    let screen_state = StorybookScreenState::default();
    let scenario = scenario(preset_index, &screen_state);

    super::dedicated_dod_atom_skeleton::skeleton_motion_label_for_test(scenario)
}

fn scenario<'a>(
    preset_index: usize,
    screen_state: &'a StorybookScreenState,
) -> ScenarioContext<'a> {
    ScenarioContext {
        selected_page: PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    }
}
