use super::canvas::Canvas;
use super::layout_metrics::{
    LayoutRect, PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT, PRESET_TAB_COUNT, PRESET_TEXT_X_OFFSET,
    preset_tab_visual_rect,
};
use super::render_context::{RenderContext, ScenarioContext};
use super::text::TextVerticalBox;
use crate::catalog::StoryPresetLabels;

const KATANA_TAB_BORDER_WIDTH: usize = 1;
const PRESET_TEXT_SIZE: f32 = 12.0;
const TAB_CORNER_SIZE: usize = 2;
const TAB_INACTIVE_OVERLAP_Y: usize = 1;

pub(super) fn draw(canvas: &mut Canvas, render: RenderContext<'_>, scenario: ScenarioContext<'_>) {
    let labels = StoryPresetLabels::for_page(scenario.selected_page);
    let active_index = scenario.preset_index;
    let visible_count = labels.len().min(PRESET_TAB_COUNT);
    for (index, label) in labels.iter().enumerate().take(PRESET_TAB_COUNT) {
        draw_tab(
            canvas,
            render,
            label,
            index == active_index,
            index,
            index + 1 == visible_count,
        );
    }
}

fn draw_tab(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    label: &str,
    active: bool,
    index: usize,
    last: bool,
) {
    let rect = preset_tab_visual_rect(index, active);
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
    render.text.draw_centered(
        canvas,
        label,
        rect.x + PRESET_TEXT_X_OFFSET,
        TextVerticalBox::new(rect.y, rect.height as f32),
        PRESET_TEXT_SIZE,
        color,
    );
}
