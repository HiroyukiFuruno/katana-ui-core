use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    component_body_pixel_diff, pixel_at, rect_non_background_pixels,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "text";
const ROLE_GRID_PRESET: &str = "role grid";
const MIXED_SCRIPT_PRESET: &str = "mixed script";
const LINE_METRICS_PRESET: &str = "line metrics";
const RICH_SPANS_PRESET: &str = "rich spans";
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const MIN_TEXT_INK_PIXELS: usize = 600;
const EDGE_GUTTER: usize = 2;
const FRAME_SAMPLE_X_OFFSET: usize = 8;
const FRAME_SAMPLE_Y_OFFSET: usize = 8;
const DARK_PANEL: u32 = 0x282828;
const DARK_BORDER: u32 = 0x3c3c3c;
const DARK_SURFACE: u32 = 0x252526;
const DARK_CODE_SWATCH: u32 = 0x2d2d30;
const FIXED_WARN_TEXT: u32 = 0xd7ba7d;
const FIXED_TOKEN_TEXT: u32 = 0x4ec9b0;
const FIXED_MUTED_TEXT: u32 = 0x8f98a8;

#[test]
fn text_exposes_leaf_presets_options_and_style_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("style_apply", spec.action);
    assert_eq!("text_style_changed", spec.event);
    assert_eq!("text.role", spec.option);
    assert_eq!("heading", spec.after);
    assert_eq!("role=heading", spec.state);
    for setting in ["text.role", "text.content", "text.color", "text.wrap"] {
        assert!(options.iter().any(|option| option.setting == setting));
    }
}

#[test]
fn text_presets_render_distinct_typography_bodies() {
    let role_grid = render_preset(ROLE_GRID_PRESET);
    let mixed_script = render_preset(MIXED_SCRIPT_PRESET);
    let line_metrics = render_preset(LINE_METRICS_PRESET);
    let rich_spans = render_preset(RICH_SPANS_PRESET);

    assert!(component_body_pixel_diff(PAGE, &role_grid, &mixed_script) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &role_grid, &line_metrics) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &role_grid, &rich_spans) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_light_and_dark_frames_use_theme_tokens() {
    assert_frame_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_frame_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

#[test]
fn text_light_mixed_script_does_not_keep_dark_code_swatches() {
    let canvas =
        StorybookVisual.render_preset(LIGHT_THEME, PAGE, preset_index(MIXED_SCRIPT_PRESET), 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        0,
        rect_color_pixels(rect, &canvas, DARK_CODE_SWATCH),
        "light text story must not keep dark code swatches"
    );
}

#[test]
fn text_light_role_grid_uses_theme_text_tokens_instead_of_dark_fixed_ink() {
    let canvas =
        StorybookVisual.render_preset(LIGHT_THEME, PAGE, preset_index(ROLE_GRID_PRESET), 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let source = include_str!("dedicated_dod_atom_primitives.rs");
    let text_grid_source = source
        .split("pub(super) fn text_grid")
        .nth(1)
        .kuc_expect("text_grid source must exist");

    for color in [FIXED_WARN_TEXT, FIXED_TOKEN_TEXT, FIXED_MUTED_TEXT] {
        assert_eq!(
            0,
            rect_color_pixels(rect, &canvas, color),
            "light text story must not keep fixed dark ink color {color:#x}"
        );
    }
    for forbidden in ["common::WARN", "common::TOKEN", "MUTED_TEXT"] {
        assert!(
            !text_grid_source.contains(forbidden),
            "text story must not use fixed color `{forbidden}`"
        );
    }
}

#[test]
fn text_ink_stays_inside_component_body_with_gutter() {
    for preset in [
        ROLE_GRID_PRESET,
        MIXED_SCRIPT_PRESET,
        LINE_METRICS_PRESET,
        RICH_SPANS_PRESET,
    ] {
        let canvas = render_preset(preset);
        let rect = preview_detail::component_action_hit_rect(PAGE);
        assert!(rect.inside_content());
        assert!(
            rect_non_background_pixels(rect, &canvas, palette::DEFAULT_BACKGROUND)
                > MIN_TEXT_INK_PIXELS
        );

        for y in rect.y..rect.bottom() {
            let pixel = pixel_at(&canvas, rect.right() - EDGE_GUTTER, y)
                .kuc_expect("right gutter sample must be inside canvas");
            assert!(
                matches!(
                    pixel,
                    palette::DEFAULT_BACKGROUND | DARK_PANEL | DARK_BORDER | DARK_SURFACE
                ),
                "{preset} text ink must not touch the right edge: pixel={pixel:#x}"
            );
        }
    }
}

fn assert_frame_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, preset_index(ROLE_GRID_PRESET), 0);
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
        .kuc_expect("text preset label must exist")
}

fn rect_color_pixels(
    rect: super::layout_metrics::LayoutRect,
    canvas: &super::Canvas,
    color: u32,
) -> usize {
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
