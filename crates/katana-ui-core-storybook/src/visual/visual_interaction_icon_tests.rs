use super::interaction_spec::StorybookInteractionSpec;
use super::layout_metrics::LayoutRect;
use super::visual_interaction_test_support::{component_body_pixel_diff, pixel_at};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "icon";
const CONTENT_PRESET: &str = "content value";
const VISUAL_ROLE_PRESET: &str = "visual role";
const SVG_SOURCE_PRESET: &str = "custom SVG";
const VIEW_BOX_PRESET: &str = "view box";
const PAINT_POLICY_PRESET: &str = "paint policy";
const THEME_TOKEN_PRESET: &str = "theme token";
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ICON_BOX_SIZE: usize = 36;
const ICON_BOX_Y_OFFSET: usize = 36;
const FIRST_ICON_BOX_X_OFFSET: usize = 18;
const ICON_BOX_STEP: usize = 44;
const FRAME_SAMPLE_X_OFFSET: usize = 8;
const FRAME_SAMPLE_Y_OFFSET: usize = 8;
const ACCENT: u32 = 0x569cd6;
const TOKEN: u32 = 0x4ec9b0;
const PURPLE: u32 = 0xc586c0;
const WARN: u32 = 0xd7ba7d;

#[test]
fn icon_exposes_leaf_presets_options_and_svg_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("icon_select", spec.action);
    assert_eq!("icon_changed", spec.event);
    assert_eq!("icon.svg_source", spec.option);
    assert_eq!("custom-svg", spec.after);
    assert_eq!("svg=custom", spec.state);
    for setting in [
        "icon.svg_source",
        "icon.svg_icon",
        "icon.view_box",
        "icon.paint_policy",
        "icon.color_token",
        "icon.theme_token",
    ] {
        assert!(options.iter().any(|option| option.setting == setting));
    }
}

#[test]
fn icon_presets_render_distinct_svg_size_color_and_policy_bodies() {
    let content = render_preset(CONTENT_PRESET);
    let visual_role = render_preset(VISUAL_ROLE_PRESET);
    let svg_source = render_preset(SVG_SOURCE_PRESET);
    let view_box = render_preset(VIEW_BOX_PRESET);
    let paint_policy = render_preset(PAINT_POLICY_PRESET);
    let theme_token = render_preset(THEME_TOKEN_PRESET);

    assert!(component_body_pixel_diff(PAGE, &content, &visual_role) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &content, &svg_source) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &content, &view_box) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &content, &paint_policy) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &content, &theme_token) > BODY_DIFF_THRESHOLD);
}

#[test]
fn icon_default_sizes_are_centered_and_increase_inside_boxes() {
    let canvas = render_preset(CONTENT_PRESET);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    assert!(rect.inside_content());

    let accent = color_count(icon_box(0), &canvas, ACCENT);
    let token = color_count(icon_box(1), &canvas, TOKEN);
    let purple = color_count(icon_box(2), &canvas, PURPLE);
    let warn = color_count(icon_box(3), &canvas, WARN);

    assert!(accent > 0);
    assert!(accent < token);
    assert!(token < purple);
    assert!(purple < warn);
}

#[test]
fn icon_view_box_preset_reorders_sizes_without_leaving_icon_boxes() {
    let canvas = render_preset(VIEW_BOX_PRESET);

    let first = colored_pixels(icon_box(0), &canvas);
    let second = colored_pixels(icon_box(1), &canvas);
    let third = colored_pixels(icon_box(2), &canvas);
    let fourth = colored_pixels(icon_box(3), &canvas);

    assert!(first > second);
    assert!(second > third);
    assert!(third > fourth);
}

#[test]
fn icon_light_and_dark_frames_use_theme_tokens() {
    assert_frame_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_frame_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_frame_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, preset_index(CONTENT_PRESET), 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.panel),
        pixel_at(
            &canvas,
            rect.x + FRAME_SAMPLE_X_OFFSET,
            rect.y + FRAME_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.border),
        pixel_at(&canvas, rect.right() - 1, rect.y)
    );
}

fn render_preset(label: &str) -> super::Canvas {
    StorybookVisual.render_preset(DARK_THEME, PAGE, preset_index(label), 0)
}

fn preset_index(label: &str) -> usize {
    StoryPresetLabels::for_page(PAGE)
        .iter()
        .position(|it| *it == label)
        .kuc_expect("icon preset label must exist")
}

fn icon_box(index: usize) -> LayoutRect {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    LayoutRect::new(
        origin.x + FIRST_ICON_BOX_X_OFFSET + index * ICON_BOX_STEP,
        origin.y + ICON_BOX_Y_OFFSET,
        ICON_BOX_SIZE,
        ICON_BOX_SIZE,
    )
}

fn color_count(rect: LayoutRect, canvas: &super::Canvas, color: u32) -> usize {
    let mut count = 0;
    for y in rect.y..rect.bottom() {
        for x in rect.x..rect.right() {
            if pixel_at(canvas, x, y) == Some(color) {
                count += 1;
            }
        }
    }
    count
}

fn colored_pixels(rect: LayoutRect, canvas: &super::Canvas) -> usize {
    let colors = [ACCENT, TOKEN, PURPLE, WARN];
    colors
        .into_iter()
        .map(|color| color_count(rect, canvas, color))
        .sum()
}
