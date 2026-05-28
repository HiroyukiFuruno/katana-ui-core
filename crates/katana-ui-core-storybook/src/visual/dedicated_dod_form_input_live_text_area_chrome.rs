use super::{
    Canvas, FIELD_X, ScenarioContext, TEXT_AREA_HEIGHT, TEXT_AREA_WIDTH, TEXT_AREA_Y,
    VisualPalette, m,
};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::screen_state::StorybookScreenState;

const SCROLLBAR_THUMB_X_OFFSET: usize = 5;
const SCROLLBAR_TRACK_SIZE: usize = 2;
const SCROLLBAR_THUMB_SIZE: usize = 4;
const RESIZE_GRIP_SIZE: usize = 10;
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
pub(super) const IME_PRESET_INDEX: usize = 7;
pub(super) const EMOJI_PRESET_INDEX: usize = 8;

pub(super) fn draw_vertical_scrollbar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !vertical_scrollbar_visible(scenario) {
        return;
    }
    let rect = text_area_rect_for_state(x, y, scenario.screen_state);
    canvas.fill_rect(
        rect.right() - m::PX_4,
        rect.y + m::PX_8,
        SCROLLBAR_TRACK_SIZE,
        rect.height - m::PX_16,
        palette.panel,
    );
    canvas.fill_rect(
        rect.right() - SCROLLBAR_THUMB_X_OFFSET,
        rect.y + thumb_y(scenario),
        SCROLLBAR_THUMB_SIZE,
        m::PX_24,
        palette.accent,
    );
}

pub(super) fn draw_horizontal_scrollbar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !horizontal_scrollbar_visible(scenario) {
        return;
    }
    let rect = text_area_rect_for_state(x, y, scenario.screen_state);
    canvas.fill_rect(
        rect.x + m::PX_8,
        rect.bottom() - m::PX_4,
        rect.width - m::PX_16,
        SCROLLBAR_TRACK_SIZE,
        palette.panel,
    );
    canvas.fill_rect(
        rect.x + m::PX_12,
        rect.bottom() - SCROLLBAR_THUMB_SIZE,
        m::PX_36,
        SCROLLBAR_THUMB_SIZE,
        palette.accent,
    );
}

pub(super) fn draw_resize_grip(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !resize_enabled(scenario) {
        return;
    }
    let rect = text_area_rect_for_state(x, y, scenario.screen_state);
    let right = rect.right() - m::PX_4;
    let bottom = rect.bottom() - m::PX_4;
    for offset in [0, m::PX_4, m::PX_8] {
        canvas.fill_rect(
            right - offset,
            bottom - (RESIZE_GRIP_SIZE - offset),
            m::PX_2,
            m::PX_2,
            palette.accent,
        );
    }
}

pub(super) fn horizontal_scroll_offset(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.text_area_wrap_enabled()
        && scenario.preset_index != HORIZONTAL_SCROLL_PRESET_INDEX
    {
        return 0;
    }
    scenario.screen_state.text_area_scroll_x_offset()
}

pub(in crate::visual) fn text_area_rect_for_state(
    x: usize,
    y: usize,
    screen_state: &StorybookScreenState,
) -> LayoutRect {
    LayoutRect::new(
        x + FIELD_X,
        y + TEXT_AREA_Y,
        TEXT_AREA_WIDTH + screen_state.text_area_resize_width_delta(),
        TEXT_AREA_HEIGHT + screen_state.text_area_resize_height_delta(),
    )
}

pub(super) fn text_area_status_rects_for_state(
    x: usize,
    y: usize,
    screen_state: &StorybookScreenState,
) -> [LayoutRect; TEXT_AREA_STATUS_ROW_COUNT] {
    let field = text_area_rect_for_state(x, y, screen_state);
    let left = field.right() + TEXT_AREA_STATUS_GAP;
    [
        text_area_status_rect(left, y, 0),
        text_area_status_rect(left, y, 1),
        text_area_status_rect(left, y, 2),
    ]
}

pub(in crate::visual) fn text_area_resize_grip_rect_for(
    x: usize,
    y: usize,
    preset_index: usize,
    screen_state: &StorybookScreenState,
) -> Option<LayoutRect> {
    if !resize_enabled_for(preset_index, screen_state) {
        return None;
    }
    let rect = text_area_rect_for_state(x, y, screen_state);
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

pub(super) fn vertical_scroll_offset(scenario: ScenarioContext<'_>) -> usize {
    if vertical_scroll_enabled(scenario) {
        return scenario.screen_state.text_area_scroll_offset();
    }
    0
}

pub(super) fn vertical_scroll_enabled(scenario: ScenarioContext<'_>) -> bool {
    vertical_scroll_enabled_for(scenario.preset_index, scenario.screen_state)
}

pub(super) fn horizontal_scroll_enabled(scenario: ScenarioContext<'_>) -> bool {
    horizontal_scroll_enabled_for(scenario.preset_index, scenario.screen_state)
}

pub(in crate::visual) fn vertical_scroll_enabled_for(
    preset_index: usize,
    screen_state: &StorybookScreenState,
) -> bool {
    screen_state.text_area_vertical_scroll_enabled() || preset_index == VERTICAL_SCROLL_PRESET_INDEX
}

pub(in crate::visual) fn horizontal_scroll_enabled_for(
    preset_index: usize,
    screen_state: &StorybookScreenState,
) -> bool {
    screen_state.text_area_horizontal_scroll_enabled()
        || preset_index == HORIZONTAL_SCROLL_PRESET_INDEX
}

pub(in crate::visual) fn resize_enabled_for(
    preset_index: usize,
    screen_state: &StorybookScreenState,
) -> bool {
    screen_state.text_area_resize_enabled() || preset_index == RESIZE_PRESET_INDEX
}

fn resize_enabled(scenario: ScenarioContext<'_>) -> bool {
    resize_enabled_for(scenario.preset_index, scenario.screen_state)
}

fn vertical_scrollbar_visible(scenario: ScenarioContext<'_>) -> bool {
    vertical_scroll_enabled(scenario)
        && (scenario.screen_state.text_area_vertical_scrollbar_visible()
            || scenario.preset_index == VERTICAL_SCROLL_PRESET_INDEX)
}

fn horizontal_scrollbar_visible(scenario: ScenarioContext<'_>) -> bool {
    horizontal_scroll_enabled(scenario)
        && (scenario
            .screen_state
            .text_area_horizontal_scrollbar_visible()
            || scenario.preset_index == HORIZONTAL_SCROLL_PRESET_INDEX)
}

fn thumb_y(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == VERTICAL_SCROLL_PRESET_INDEX {
        return m::PX_34;
    }
    m::PX_12
}

fn text_area_status_rect(left: usize, y: usize, index: usize) -> LayoutRect {
    LayoutRect::new(
        left,
        y + TEXT_AREA_STATUS_Y + index * TEXT_AREA_STATUS_STEP,
        TEXT_AREA_STATUS_WIDTH,
        TEXT_AREA_STATUS_HEIGHT,
    )
}
