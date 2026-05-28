use super::canvas::Canvas;
use super::layout_metrics::{LayoutRect, PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT, PRESET_TEXT_X_OFFSET};
use super::preset_tab_scroll;
use super::render_context::{RenderContext, ScenarioContext};
use super::text::{TextRenderer, TextVerticalBox};
use crate::catalog::StoryPresetLabels;

const KATANA_TAB_BORDER_WIDTH: usize = 1;
const PRESET_TEXT_SIZE: f32 = 12.0;
const PRESET_TEXT_MIN_SIZE: f32 = 9.0;
const PRESET_TEXT_RIGHT_PADDING: usize = 8;
const TAB_CORNER_SIZE: usize = 2;
const TAB_INACTIVE_OVERLAP_Y: usize = 1;

pub(super) fn draw(canvas: &mut Canvas, render: RenderContext<'_>, scenario: ScenarioContext<'_>) {
    let labels = StoryPresetLabels::for_page(scenario.selected_page);
    let active_index = scenario.preset_index;
    let visible_range = preset_tab_scroll::visible_index_range(
        scenario.selected_page,
        scenario.preset_tab_scroll_x,
    );
    let viewport = preset_tab_scroll::viewport_rect();
    canvas.with_clip(
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        |canvas| {
            for index in visible_range.clone() {
                draw_tab(
                    canvas,
                    render,
                    labels[index],
                    index == active_index,
                    index,
                    index + 1 == visible_range.end,
                    scenario,
                );
            }
        },
    );
}

fn draw_tab(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    label: &str,
    active: bool,
    index: usize,
    last: bool,
    scenario: ScenarioContext<'_>,
) {
    let Some(rect) = preset_tab_scroll::visual_rect_for_index(
        scenario.selected_page,
        index,
        active,
        scenario.preset_tab_scroll_x,
    ) else {
        return;
    };
    let fill = if active {
        render.palette.surface
    } else {
        render.palette.code_background
    };
    draw_katana_tab_shape(canvas, render, rect, fill, active, last);
    draw_tab_label(canvas, render, rect, label, active);
}

fn draw_katana_tab_shape(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    rect: LayoutRect,
    fill: u32,
    active: bool,
    last: bool,
) {
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, fill);
    draw_tab_corners(canvas, render, rect, active);
    canvas.fill_rect(
        rect.x,
        rect.y,
        rect.width,
        KATANA_TAB_BORDER_WIDTH,
        render.palette.border,
    );
    if !active {
        canvas.fill_rect(
            rect.x,
            rect.bottom() - TAB_INACTIVE_OVERLAP_Y,
            rect.width,
            KATANA_TAB_BORDER_WIDTH,
            render.palette.border,
        );
    }
    canvas.fill_rect(
        rect.x,
        rect.y,
        KATANA_TAB_BORDER_WIDTH,
        rect.height,
        render.palette.border,
    );
    if last {
        canvas.fill_rect(
            rect.right() - KATANA_TAB_BORDER_WIDTH,
            rect.y,
            KATANA_TAB_BORDER_WIDTH,
            rect.height,
            render.palette.border,
        );
    }
    if active {
        draw_active_bottom_accent(canvas, render, rect);
    }
}

fn draw_tab_corners(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    rect: LayoutRect,
    active: bool,
) {
    let corner_color = if active {
        render.palette.background
    } else {
        render.palette.surface
    };
    for offset in 0..TAB_CORNER_SIZE {
        canvas.set(
            rect.x + offset,
            rect.y + TAB_CORNER_SIZE - offset - 1,
            corner_color,
        );
        canvas.set(
            rect.right() - offset - 1,
            rect.y + TAB_CORNER_SIZE - offset - 1,
            corner_color,
        );
    }
}

fn draw_active_bottom_accent(canvas: &mut Canvas, render: RenderContext<'_>, rect: LayoutRect) {
    canvas.fill_rect(
        rect.x,
        rect.bottom() - PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT,
        rect.width,
        PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT,
        render.palette.accent,
    );
}

fn draw_tab_label(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    rect: LayoutRect,
    label: &str,
    active: bool,
) {
    let color = if active {
        render.palette.text
    } else {
        render.palette.muted
    };
    let text_x = rect.x + PRESET_TEXT_X_OFFSET;
    let clip_width = tab_label_clip_width(rect);
    let text_size = tab_label_size(render.text, rect, label);
    canvas.with_clip(text_x, rect.y, clip_width, rect.height, |canvas| {
        render.text.draw_centered(
            canvas,
            label,
            text_x,
            TextVerticalBox::new(rect.y, rect.height as f32),
            text_size,
            color,
        );
    });
}

fn tab_label_size(text: &TextRenderer, rect: LayoutRect, label: &str) -> f32 {
    let mut text_size = PRESET_TEXT_SIZE;
    let clip_width = tab_label_clip_width(rect);
    while text_size > PRESET_TEXT_MIN_SIZE && text.measure_width(label, text_size) > clip_width {
        text_size -= 1.0;
    }
    text_size
}

fn tab_label_clip_width(rect: LayoutRect) -> usize {
    rect.width
        .saturating_sub(PRESET_TEXT_X_OFFSET + PRESET_TEXT_RIGHT_PADDING)
}

#[cfg(test)]
pub(super) fn tab_label_widths_for_test(
    text: &TextRenderer,
    rect: LayoutRect,
    label: &str,
) -> (usize, usize) {
    let text_size = tab_label_size(text, rect, label);
    (
        text.measure_width(label, text_size),
        tab_label_clip_width(rect),
    )
}
