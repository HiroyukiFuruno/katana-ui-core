use super::{FIELD_X, TEXT_AREA_HEIGHT, TEXT_AREA_WIDTH, TEXT_AREA_Y};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::screen_state::StorybookScreenState;
#[cfg(test)]
use crate::visual::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE;

const RESIZE_GRIP_HIT_SIZE: usize = 16;
const RESIZE_MAX_WIDTH_DELTA: usize = 80;
const RESIZE_MAX_HEIGHT_DELTA: usize = 8;
const TEXT_AREA_STATUS_GAP: usize = 18;
const TEXT_AREA_STATUS_Y: usize = 36;
const TEXT_AREA_STATUS_WIDTH: usize = 96;
const TEXT_AREA_STATUS_HEIGHT: usize = 20;
const TEXT_AREA_STATUS_STEP: usize = 28;
const TEXT_AREA_STATUS_ROW_COUNT: usize = 3;

pub(super) const RESIZE_PRESET_INDEX: usize = 3;
pub(super) const AUTO_GROW_PRESET_INDEX: usize = 4;
pub(super) const VERTICAL_SCROLL_PRESET_INDEX: usize = 5;
pub(super) const HORIZONTAL_SCROLL_PRESET_INDEX: usize = 6;
pub(super) const TAB_BEHAVIOR_PRESET_INDEX: usize = 7;
pub(super) const VERTICAL_SCROLLBAR_PRESET_INDEX: usize = 8;
pub(super) const HORIZONTAL_SCROLLBAR_PRESET_INDEX: usize = 9;

#[cfg(test)]
pub(in crate::visual) fn text_area_rect_for_state(
    x: usize,
    y: usize,
    screen_state: &StorybookScreenState,
) -> LayoutRect {
    text_area_rect_for_instance(x, y, screen_state, DEFAULT_TEXT_AREA_INSTANCE)
}

pub(in crate::visual) fn text_area_rect_for_instance(
    x: usize,
    y: usize,
    screen_state: &StorybookScreenState,
    instance: &'static str,
) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X,
        y + TEXT_AREA_Y,
        TEXT_AREA_WIDTH + screen_state.text_area_resize_width_delta_for(instance),
        TEXT_AREA_HEIGHT + screen_state.text_area_resize_height_delta_for(instance),
    )
}

pub(super) fn text_area_status_rects_for_instance(
    x: usize,
    y: usize,
    screen_state: &StorybookScreenState,
    instance: &'static str,
) -> [LayoutRect; TEXT_AREA_STATUS_ROW_COUNT] {
    let field = text_area_rect_for_instance(x, y, screen_state, instance);
    let left = field.right() + TEXT_AREA_STATUS_GAP;
    [
        text_area_status_rect(left, y, 0),
        text_area_status_rect(left, y, 1),
        text_area_status_rect(left, y, 2),
    ]
}

pub(in crate::visual) fn text_area_resize_grip_rect_for_instance(
    x: usize,
    y: usize,
    preset_index: usize,
    screen_state: &StorybookScreenState,
    instance: &'static str,
) -> Option<LayoutRect> {
    if !resize_enabled_for_instance(preset_index, screen_state, instance) {
        return None;
    }
    let rect = text_area_rect_for_instance(x, y, screen_state, instance);
    Some(LayoutRect::new(
        rect.right().saturating_sub(RESIZE_GRIP_HIT_SIZE),
        rect.bottom().saturating_sub(RESIZE_GRIP_HIT_SIZE),
        RESIZE_GRIP_HIT_SIZE,
        RESIZE_GRIP_HIT_SIZE,
    ))
}

pub(in crate::visual) fn text_area_resize_delta_for_pointer(
    x: usize,
    y: usize,
    pointer_x: usize,
    pointer_y: usize,
) -> (usize, usize) {
    let base = LayoutRect::new(
        x + FIELD_X,
        y + TEXT_AREA_Y,
        TEXT_AREA_WIDTH,
        TEXT_AREA_HEIGHT,
    );
    (
        pointer_x
            .saturating_sub(base.right())
            .min(RESIZE_MAX_WIDTH_DELTA),
        pointer_y
            .saturating_sub(base.bottom())
            .min(RESIZE_MAX_HEIGHT_DELTA),
    )
}

pub(in crate::visual) fn vertical_scroll_enabled_for_instance(
    preset_index: usize,
    screen_state: &StorybookScreenState,
    instance: &'static str,
) -> bool {
    screen_state.text_area_vertical_scroll_enabled_for(instance)
        || preset_index == VERTICAL_SCROLL_PRESET_INDEX
        || preset_index == VERTICAL_SCROLLBAR_PRESET_INDEX
}

pub(in crate::visual) fn horizontal_scroll_enabled_for_instance(
    preset_index: usize,
    screen_state: &StorybookScreenState,
    instance: &'static str,
) -> bool {
    screen_state.text_area_horizontal_scroll_enabled_for(instance)
        || preset_index == HORIZONTAL_SCROLL_PRESET_INDEX
        || preset_index == HORIZONTAL_SCROLLBAR_PRESET_INDEX
}

pub(in crate::visual) fn resize_enabled_for_instance(
    preset_index: usize,
    screen_state: &StorybookScreenState,
    instance: &'static str,
) -> bool {
    screen_state.text_area_resize_enabled_for(instance) || preset_index == RESIZE_PRESET_INDEX
}

fn text_area_status_rect(left: usize, y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        left,
        y + TEXT_AREA_STATUS_Y + index * TEXT_AREA_STATUS_STEP,
        TEXT_AREA_STATUS_WIDTH,
        TEXT_AREA_STATUS_HEIGHT,
    )
}
