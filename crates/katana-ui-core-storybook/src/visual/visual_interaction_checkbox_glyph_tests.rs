use super::visual_interaction_test_support::pixel_at;
use super::{StorybookVisual, preview_detail};
use crate::test_assert::KucTestExpect;

const DARK_THEME: &str = "dark";
const PAGE: &str = "checkbox";
const UNCHECKED_PRESET: usize = 0;
const CHECKED_PRESET: usize = 1;
const CHECK_GLYPH_COLOR: u32 = 0xf8fafc;
const MIN_GLYPH_INSET: usize = 4;

#[test]
fn checkbox_unchecked_mark_contains_no_checked_glyph_pixels() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, UNCHECKED_PRESET, 0);
    let mark = checkbox_mark_rect();

    assert_eq!(
        0,
        count_color_in_rect(&unchecked, mark, CHECK_GLYPH_COLOR),
        "unchecked checkbox must not leave checked glyph pixels behind"
    );
}

#[test]
fn checkbox_checked_glyph_stays_inside_mark_with_modern_inset() {
    let checked = StorybookVisual.render_preset(DARK_THEME, PAGE, CHECKED_PRESET, 0);
    let mark = checkbox_mark_rect();
    let (min_x, max_x, min_y, max_y) = color_bounds_in_rect(&checked, mark, CHECK_GLYPH_COLOR)
        .kuc_expect("checked checkbox must render a check glyph");

    assert!(min_x >= mark.x + MIN_GLYPH_INSET);
    assert!(max_x + MIN_GLYPH_INSET < mark.right());
    assert!(min_y >= mark.y + MIN_GLYPH_INSET);
    assert!(max_y + MIN_GLYPH_INSET < mark.bottom());
}

fn checkbox_mark_rect() -> super::layout_metrics::LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y)
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
