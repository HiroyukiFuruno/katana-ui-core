use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const STATUS_X: usize = 26;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 92;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;

pub(super) fn tooltip(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let tooltip_fill = if scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.surface
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Tooltip",
        &[
            Block::outlined(m::PX_112, m::PX_34, m::PX_118, m::PX_24, tooltip_fill),
            Block::outlined(m::PX_222, m::PX_62, m::PX_96, m::PX_24, palette.surface),
            Block::outlined(m::PX_22, m::PX_62, m::PX_96, m::PX_24, palette.surface),
        ],
        &[
            TextSpec::new(m::PX_142, m::PX_77, m::FONT_9, palette.background, "anchor"),
            TextSpec::new(m::PX_120, m::PX_41, m::FONT_9, palette.muted, "top hover"),
            TextSpec::new(m::PX_230, m::PX_69, m::FONT_9, palette.muted, "right"),
            TextSpec::new(m::PX_30, m::PX_69, m::FONT_9, palette.muted, "focus"),
            TextSpec::new(
                m::PX_26,
                m::PX_94,
                m::FONT_9,
                palette.muted,
                "delay / flip / close lifecycle",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
    common::chip(
        canvas,
        text,
        palette,
        Rect::new(x + m::PX_134, y + m::PX_70, m::PX_74, m::PX_22),
        "anchor",
        palette.accent,
    );
}
pub(super) fn popover(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let panel_fill = if scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.surface
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Popover",
        &[
            Block::outlined(m::PX_116, m::PX_34, m::PX_188, m::PX_60, panel_fill),
            Block::new(m::PX_108, m::PX_78, m::PX_8, m::PX_8, palette.accent),
        ],
        &[
            TextSpec::new(
                m::PX_128,
                m::PX_42,
                m::FONT_9,
                palette.muted,
                "placement: right + offset 12",
            ),
            TextSpec::new(
                m::PX_128,
                m::PX_58,
                m::FONT_9,
                palette.muted,
                "outside click -> close",
            ),
            TextSpec::new(
                m::PX_128,
                m::PX_74,
                m::FONT_9,
                palette.muted,
                "Esc / content select log",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
    common::chip(
        canvas,
        text,
        palette,
        Rect::new(x + m::PX_28, y + m::PX_72, m::PX_76, m::PX_22),
        "anchor",
        palette.accent,
    );
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rows = [
        action_label(scenario),
        event_label(scenario),
        state_label(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_x = x + STATUS_X + index * (STATUS_WIDTH + STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "open=false";
    }
    scenario.screen_state.state_label
}
