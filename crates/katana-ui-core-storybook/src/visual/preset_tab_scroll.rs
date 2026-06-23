use super::layout_metrics::{
    LayoutRect, PRESET_ACTIVE_HEIGHT, PRESET_ACTIVE_Y, PRESET_GAP, PRESET_INACTIVE_HEIGHT,
    PRESET_INACTIVE_Y, PRESET_WIDTH, PREVIEW_X,
};
use super::preview_detail;
use crate::catalog::StoryPresetLabels;

pub(super) fn viewport_rect() -> LayoutRect {
    LayoutRect::new(
        PREVIEW_X,
        PRESET_ACTIVE_Y,
        viewport_width(),
        PRESET_ACTIVE_HEIGHT,
    )
}

pub(super) fn max_scroll_x_for_page(page: &str) -> usize {
    content_width_for_page(page).saturating_sub(viewport_width())
}

pub(super) fn ensure_index_visible(page: &str, index: usize, current: usize) -> usize {
    let labels = StoryPresetLabels::for_page(page);
    if labels.is_empty() {
        return 0;
    }
    let target = index.min(labels.len() - 1);
    let offset = clamp_offset(page, current);
    let tab_left = target * tab_step();
    let tab_right = tab_left + PRESET_WIDTH;
    if tab_left < offset {
        return clamp_offset(page, tab_left);
    }
    if tab_right > offset + viewport_width() {
        return clamp_offset(page, tab_right - viewport_width());
    }
    offset
}

pub(super) fn scroll_delta(page: &str, current: usize, delta: f32) -> usize {
    let offset = clamp_offset(page, current);
    let max_offset = max_scroll_x_for_page(page);
    if delta == 0.0 || max_offset == 0 {
        return offset;
    }
    if delta < 0.0 {
        return (offset + tab_step()).min(max_offset);
    }
    offset.saturating_sub(tab_step())
}

pub(super) fn hit_index_at(page: &str, x: usize, y: usize, scroll_x: usize) -> Option<usize> {
    let viewport = viewport_rect();
    if !viewport.contains(x, y) {
        return None;
    }
    let local_x = x - viewport.x + clamp_offset(page, scroll_x);
    let index = local_x / tab_step();
    if index >= StoryPresetLabels::for_page(page).len() {
        return None;
    }
    visual_rect_for_index(page, index, false, scroll_x)
        .filter(|rect| rect.contains(x, y))
        .map(|_| index)
}

pub(super) fn visible_index_range(page: &str, scroll_x: usize) -> std::ops::Range<usize> {
    let labels = StoryPresetLabels::for_page(page);
    let offset = clamp_offset(page, scroll_x);
    let first = offset.div_ceil(tab_step());
    let right = offset + viewport_width();
    let end = right
        .saturating_sub(PRESET_WIDTH)
        .checked_div(tab_step())
        .map_or(first, |index| index + 1)
        .min(labels.len());
    first..end
}

pub(super) fn visual_rect_for_index(
    page: &str,
    index: usize,
    active: bool,
    scroll_x: usize,
) -> Option<LayoutRect> {
    if !visible_index_range(page, scroll_x).contains(&index) {
        return None;
    }
    let offset = clamp_offset(page, scroll_x);
    let y = if active {
        PRESET_ACTIVE_Y
    } else {
        PRESET_INACTIVE_Y
    };
    let height = if active {
        PRESET_ACTIVE_HEIGHT
    } else {
        PRESET_INACTIVE_HEIGHT
    };
    Some(LayoutRect::new(
        PREVIEW_X + index * tab_step() - offset,
        y,
        PRESET_WIDTH,
        height,
    ))
}

pub(super) fn active_index_scroll_x(page: &str, index: usize) -> usize {
    ensure_index_visible(page, index, 0)
}

fn clamp_offset(page: &str, offset: usize) -> usize {
    offset.min(max_scroll_x_for_page(page))
}

fn content_width_for_page(page: &str) -> usize {
    let len = StoryPresetLabels::for_page(page).len();
    if len == 0 {
        return 0;
    }
    len * PRESET_WIDTH + (len - 1) * PRESET_GAP
}

fn viewport_width() -> usize {
    let (_, _, width, _) = preview_detail::selected_hero_rect();
    width
}

fn tab_step() -> usize {
    PRESET_WIDTH + PRESET_GAP
}
