use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "checkbox";
const UNCHECKED_PRESET: usize = 0;
const CHECKED_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const FOCUS_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_FILL_SAMPLE_X_OFFSET: usize = 4;
const ROW_FILL_SAMPLE_Y_OFFSET: usize = 2;
const ROW_BORDER_SAMPLE_X_OFFSET: usize = 8;
const MARK_FILL_SAMPLE_X_OFFSET: usize = 2;
const MARK_FILL_SAMPLE_Y_OFFSET: usize = 2;
const CHECK_GLYPH_SAMPLE_X_OFFSET: usize = 10;
const CHECK_GLYPH_SAMPLE_Y_OFFSET: usize = 7;
const CHECK_GLYPH_COLOR: u32 = 0xf8fafc;
const MIN_CHECKBOX_MARK_SIZE: usize = 20;
const MIN_CHECKBOX_ROW_HEIGHT: usize = 36;
const MIN_CHECKBOX_ROW_WIDTH: usize = 240;
const MIN_CHECKBOX_LABEL_GAP_AFTER_MARK: usize = 12;
const MIN_CHECKBOX_ROW_STATUS_GAP: usize = 16;
const MIN_CHECKBOX_CONTROL_HEIGHT: usize = 24;
const MIN_CHECKBOX_STATUS_WIDTH: usize = 150;
const MODERN_CHROME_CORNER_INSET: usize = 6;
const MIN_ROUNDED_CHROME_COLOR_COUNT: usize = 3;

#[test]
fn checkbox_exposes_leaf_presets_options_and_checked_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("checkbox_toggle", spec.action);
    assert_eq!("checked_changed", spec.event);
    assert_eq!("checked", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("before=false after=true", spec.state);
    assert!(options.iter().any(|option| option.setting == "checked"));
    assert!(
        !options
            .iter()
            .any(|option| option.setting == "theme.marker")
    );
}

#[test]
fn checkbox_presets_render_distinct_selection_bodies() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, UNCHECKED_PRESET, 0);
    let checked = StorybookVisual.render_preset(DARK_THEME, PAGE, CHECKED_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &unchecked, &checked) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &checked, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &checked, &focused) > BODY_DIFF_THRESHOLD);
}

#[test]
fn checkbox_setting_option_updates_selection_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn checkbox_preview_action_updates_selection_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn checkbox_checked_mark_renders_as_diagonal_check_not_horizontal_dash() {
    let checked = StorybookVisual.render_preset(DARK_THEME, PAGE, CHECKED_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let mark = super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y);
    let (min_x, max_x, min_y, max_y) = color_bounds_in_rect(&checked, mark, CHECK_GLYPH_COLOR)
        .kuc_expect("checked checkbox must render a check glyph");

    assert!(max_x.saturating_sub(min_x) >= 8);
    assert!(max_y.saturating_sub(min_y) >= 6);
    assert!(
        count_color_in_row(&checked, mark, min_y, CHECK_GLYPH_COLOR)
            < count_color_in_rect(&checked, mark, CHECK_GLYPH_COLOR),
        "check glyph must not collapse into a single horizontal dash"
    );
}

#[test]
fn checkbox_control_and_row_meet_modern_hit_target_size() {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let row = super::dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, rect.x, rect.y);
    let mark = super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y);
    let label =
        super::dedicated_dod_form_binary_choice_live::checkbox_label_rect(0, rect.x, rect.y);
    let read = super::dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(
        rect.x, rect.y,
    );
    let status =
        super::dedicated_dod_form_binary_choice_live::checkbox_state_row_rect(rect.x, rect.y);

    assert!(
        mark.width >= MIN_CHECKBOX_MARK_SIZE,
        "checkbox mark width must be at least {MIN_CHECKBOX_MARK_SIZE}: actual={}",
        mark.width
    );
    assert!(
        mark.height >= MIN_CHECKBOX_MARK_SIZE,
        "checkbox mark height must be at least {MIN_CHECKBOX_MARK_SIZE}: actual={}",
        mark.height
    );
    assert!(row.height >= MIN_CHECKBOX_ROW_HEIGHT);
    assert!(row.width >= MIN_CHECKBOX_ROW_WIDTH);
    assert!(
        label.x >= mark.right() + MIN_CHECKBOX_LABEL_GAP_AFTER_MARK,
        "checkbox label must not visually crowd the mark"
    );
    assert!(
        status.x >= row.right() + MIN_CHECKBOX_ROW_STATUS_GAP,
        "checkbox status column must not visually crowd the interactive row"
    );
    assert!(read.height >= MIN_CHECKBOX_CONTROL_HEIGHT);
    assert!(status.width >= MIN_CHECKBOX_STATUS_WIDTH);
}

#[test]
fn checkbox_rows_controls_and_status_use_rounded_modern_chrome() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, UNCHECKED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let row = super::dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, rect.x, rect.y);
    let read = super::dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(
        rect.x, rect.y,
    );
    let status =
        super::dedicated_dod_form_binary_choice_live::checkbox_state_row_rect(rect.x, rect.y);

    assert_rounded_surface(&unchecked, row, colors.surface);
    assert_rounded_surface(&unchecked, read, colors.surface);
    assert_rounded_surface(&unchecked, status, colors.panel);
}

#[test]
fn checkbox_light_and_dark_rows_use_theme_tokens() {
    assert_row_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_row_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

#[test]
fn checkbox_rows_do_not_overlap_action_controls() {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let second_row =
        super::dedicated_dod_form_binary_choice_live::checkbox_row_rect(1, rect.x, rect.y);
    let read = super::dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(
        rect.x, rect.y,
    );
    let toggle =
        super::dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(rect.x, rect.y);
    let reset =
        super::dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(rect.x, rect.y);

    assert!(!second_row.overlaps(read));
    assert!(!second_row.overlaps(toggle));
    assert!(!second_row.overlaps(reset));
}

fn assert_row_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let unchecked = StorybookVisual.render_preset(theme_id, PAGE, UNCHECKED_PRESET, 0);
    let checked = StorybookVisual.render_preset(theme_id, PAGE, CHECKED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let row = super::dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, rect.x, rect.y);
    let mark = super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y);

    assert_eq!(
        Some(colors.border),
        pixel_at(&unchecked, row.x + ROW_BORDER_SAMPLE_X_OFFSET, row.y)
    );
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &unchecked,
            row.x + ROW_FILL_SAMPLE_X_OFFSET,
            row.y + ROW_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &checked,
            mark.x + MARK_FILL_SAMPLE_X_OFFSET,
            mark.y + MARK_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(CHECK_GLYPH_COLOR),
        pixel_at(
            &checked,
            mark.x + CHECK_GLYPH_SAMPLE_X_OFFSET,
            mark.y + CHECK_GLYPH_SAMPLE_Y_OFFSET
        )
    );
}

fn assert_rounded_surface(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    fill: u32,
) {
    assert!(
        color_count_in_rect(canvas, rect) >= MIN_ROUNDED_CHROME_COLOR_COUNT,
        "rounded chrome must include anti-aliased edge colors instead of a flat rectangle"
    );
    assert_eq!(
        Some(fill),
        pixel_at(
            canvas,
            rect.x + MODERN_CHROME_CORNER_INSET,
            rect.y + MODERN_CHROME_CORNER_INSET
        )
    );
}

fn color_count_in_rect(canvas: &super::Canvas, rect: super::layout_metrics::LayoutRect) -> usize {
    let mut colors = std::collections::BTreeSet::new();
    for y in rect.y..rect.bottom() {
        for x in rect.x..rect.right() {
            if let Some(color) = pixel_at(canvas, x, y) {
                colors.insert(color);
            }
        }
    }
    colors.len()
}

fn color_bounds_in_rect(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    color: u32,
) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    let mut found = false;
    for y in rect.y..rect.bottom() {
        for x in rect.x..rect.right() {
            if pixel_at(canvas, x, y) == Some(color) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                found = true;
            }
        }
    }
    found.then_some((min_x, max_x, min_y, max_y))
}

fn count_color_in_row(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    y: usize,
    color: u32,
) -> usize {
    (rect.x..rect.right())
        .filter(|x| pixel_at(canvas, *x, y) == Some(color))
        .count()
}

fn count_color_in_rect(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    color: u32,
) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}
