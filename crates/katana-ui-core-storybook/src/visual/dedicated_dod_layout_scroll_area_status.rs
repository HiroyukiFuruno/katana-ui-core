use super::m;
use crate::visual::canvas::Canvas;
use crate::visual::palette::VisualPalette;
use crate::visual::render_context::ScenarioContext;
use crate::visual::text::TextRenderer;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, label) in labels(scenario).into_iter().enumerate() {
        let row_x = x + super::VIEWPORT_X + index * (super::STATUS_WIDTH + super::STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + super::STATUS_Y,
            super::STATUS_WIDTH,
            super::STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + super::STATUS_Y,
            super::STATUS_WIDTH,
            super::STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            label,
            row_x + super::STATUS_TEXT_X,
            y + super::STATUS_Y + super::STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn labels(scenario: ScenarioContext<'_>) -> [&'static str; super::STATUS_LABEL_COUNT] {
    if scenario.screen_state.scroll_area.dragging() {
        return ["action drag", "event scrolled", "state drag"];
    }
    if scenario.screen_state.scroll_area.focused() {
        return ["action focus", "event focus", "state focus"];
    }
    if scenario.screen_state.scroll_area.hovered() {
        return ["action hover", "event hover", "state hover"];
    }
    if scenario.screen_state.scroll_area.resized() {
        return ["action resize", "event resized", "state viewport"];
    }
    if scenario.screen_state.scroll_area.offset_y() > 0 {
        return ["action scroll", "event scrolled", "state offset"];
    }
    if scenario.screen_state.has_settings_override() {
        return ["action scroll", "event offset", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
