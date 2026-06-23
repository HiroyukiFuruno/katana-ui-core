use super::visual_interaction_test_support::{
    assert_settings_page_changes_body, component_body_pixel_diff, pixel_at, require_some,
};
use super::{StorybookVisual, preview_detail};

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const ROW_PAGE: &str = "row";
const COLUMN_PAGE: &str = "column";
const STACK_PAGE: &str = "stack";
const GRID_PAGE: &str = "grid";
const ALIGN_CENTER_PAGE: &str = "align-center";
const SCROLL_AREA_PAGE: &str = "scroll-area";
const SPLIT_PANE_PAGE: &str = "split-pane";
const DEFAULT_PRESET: usize = 0;
const ALIGN_PRESET: usize = 1;
const OVERFLOW_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const ROW_DIFF_THRESHOLD: usize = 80;
const ROW_SURFACE_SAMPLE_X_OFFSET: usize = 20;
const ROW_SURFACE_SAMPLE_Y_OFFSET: usize = 42;
const LIGHT_LUMINANCE_THRESHOLD: u32 = 180;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const BLUE_SHIFT: u32 = 0;
const CHANNEL_MASK: u32 = 0xff;
const LUMINANCE_RED_WEIGHT: u32 = 299;
const LUMINANCE_GREEN_WEIGHT: u32 = 587;
const LUMINANCE_BLUE_WEIGHT: u32 = 114;
const LUMINANCE_SCALE: u32 = 1000;

#[test]
fn row_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(ROW_PAGE);
}

#[test]
fn column_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(COLUMN_PAGE);
}

#[test]
fn stack_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(STACK_PAGE);
}

#[test]
fn grid_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(GRID_PAGE);
}

#[test]
fn align_center_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(ALIGN_CENTER_PAGE);
}

#[test]
fn scroll_area_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(SCROLL_AREA_PAGE);
}

#[test]
fn split_pane_presets_render_distinct_layout_bodies() {
    assert_layout_presets_render_distinct_bodies(SPLIT_PANE_PAGE);
}

#[test]
fn settings_change_updates_row_layout_preview_body() {
    assert_settings_page_changes_body(ROW_PAGE);
}

#[test]
fn settings_change_updates_column_layout_preview_body() {
    assert_settings_page_changes_body(COLUMN_PAGE);
}

#[test]
fn settings_change_updates_stack_layout_preview_body() {
    assert_settings_page_changes_body(STACK_PAGE);
}

#[test]
fn settings_change_updates_grid_layout_preview_body() {
    assert_settings_page_changes_body(GRID_PAGE);
}

#[test]
fn settings_change_updates_align_center_layout_preview_body() {
    assert_settings_page_changes_body(ALIGN_CENTER_PAGE);
}

#[test]
fn settings_change_updates_scroll_area_layout_preview_body() {
    assert_settings_page_changes_body(SCROLL_AREA_PAGE);
}

#[test]
fn settings_change_updates_split_pane_layout_preview_body() {
    assert_settings_page_changes_body(SPLIT_PANE_PAGE);
}

#[test]
fn row_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(ROW_PAGE)
}

#[test]
fn column_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(COLUMN_PAGE)
}

#[test]
fn stack_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(STACK_PAGE)
}

#[test]
fn grid_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(GRID_PAGE)
}

#[test]
fn align_center_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(ALIGN_CENTER_PAGE)
}

#[test]
fn scroll_area_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(SCROLL_AREA_PAGE)
}

#[test]
fn split_pane_light_theme_uses_light_surface_token() -> Result<(), String> {
    assert_light_theme_uses_light_surface_token(SPLIT_PANE_PAGE)
}

fn assert_layout_presets_render_distinct_bodies(page: &str) {
    let first = StorybookVisual.render_preset(DARK_THEME, page, DEFAULT_PRESET, 0);
    let second = StorybookVisual.render_preset(DARK_THEME, page, ALIGN_PRESET, 0);
    let third = StorybookVisual.render_preset(DARK_THEME, page, OVERFLOW_PRESET, 0);
    let fourth = StorybookVisual.render_preset(DARK_THEME, page, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(page, &first, &second) > ROW_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(page, &second, &third) > ROW_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(page, &third, &fourth) > ROW_DIFF_THRESHOLD);
}

fn assert_light_theme_uses_light_surface_token(page: &str) -> Result<(), String> {
    let canvas = StorybookVisual.render_preset(LIGHT_THEME, page, DEFAULT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(page);
    let sample = require_some(
        pixel_at(
            &canvas,
            rect.x + ROW_SURFACE_SAMPLE_X_OFFSET,
            rect.y + ROW_SURFACE_SAMPLE_Y_OFFSET,
        ),
        "row surface sample pixel",
    )?;

    assert!(luminance(sample) > LIGHT_LUMINANCE_THRESHOLD);
    Ok(())
}

fn luminance(color: u32) -> u32 {
    let red = (color >> RED_SHIFT) & CHANNEL_MASK;
    let green = (color >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = (color >> BLUE_SHIFT) & CHANNEL_MASK;
    (red * LUMINANCE_RED_WEIGHT + green * LUMINANCE_GREEN_WEIGHT + blue * LUMINANCE_BLUE_WEIGHT)
        / LUMINANCE_SCALE
}
