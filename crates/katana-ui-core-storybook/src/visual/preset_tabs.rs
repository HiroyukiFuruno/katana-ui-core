use super::canvas::Canvas;
use super::layout_metrics::{
    LayoutRect, PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT, PRESET_TAB_COUNT, PRESET_TEXT_X_OFFSET,
    preset_tab_visual_rect,
};
use super::render_context::{RenderContext, ScenarioContext};

const KATANA_TAB_BORDER_WIDTH: usize = 1;
const PRESET_TEXT_Y_OFFSET: usize = 9;
const PRESET_TEXT_SIZE: f32 = 12.0;

pub(super) fn draw(canvas: &mut Canvas, render: RenderContext<'_>, scenario: ScenarioContext<'_>) {
    let labels = ["Default", "Interactive", "Edge", "Theme"];
    let active_index = scenario.preset_index;
    for (index, label) in labels.iter().enumerate().take(PRESET_TAB_COUNT) {
        draw_tab(canvas, render, label, index == active_index, index);
    }
}

fn draw_tab(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    label: &str,
    active: bool,
    index: usize,
) {
    let rect = preset_tab_visual_rect(index, active);
    let fill = if active {
        render.palette.surface
    } else {
        render.palette.panel
    };
    draw_katana_tab_shape(canvas, render, rect, fill, active, index);
    draw_tab_label(canvas, render, rect, label, active);
}

fn draw_katana_tab_shape(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    rect: LayoutRect,
    fill: u32,
    active: bool,
    index: usize,
) {
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, fill);
    canvas.fill_rect(
        rect.x,
        rect.y,
        rect.width,
        KATANA_TAB_BORDER_WIDTH,
        render.palette.border,
    );
    canvas.fill_rect(
        rect.x,
        rect.bottom() - KATANA_TAB_BORDER_WIDTH,
        rect.width,
        KATANA_TAB_BORDER_WIDTH,
        render.palette.border,
    );
    canvas.fill_rect(
        rect.x,
        rect.y,
        KATANA_TAB_BORDER_WIDTH,
        rect.height,
        render.palette.border,
    );
    if index + 1 == PRESET_TAB_COUNT {
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
    render.text.draw(
        canvas,
        label,
        rect.x + PRESET_TEXT_X_OFFSET,
        rect.y + PRESET_TEXT_Y_OFFSET,
        PRESET_TEXT_SIZE,
        color,
    );
}
