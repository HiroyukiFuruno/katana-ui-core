use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL: u32 = 0x20242c;
const STATUS_X: usize = 20;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;
const SPLIT_ACTIVE_HANDLE_X: usize = 116;
const SPLIT_BASE_HANDLE_X: usize = 92;

pub(super) fn accordion(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let body_fill = if scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.border
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Accordion",
        &[
            Block::outlined(m::PX_18, m::PX_34, m::PX_204, m::PX_24, palette.surface),
            Block::new(m::PX_36, m::PX_58, m::PX_1, m::PX_34, body_fill),
            Block::outlined(m::PX_234, m::PX_34, m::PX_90, m::PX_24, PANEL),
        ],
        &[
            TextSpec::new(
                m::PX_28,
                m::PX_42,
                m::FONT_8,
                palette.text,
                "⌄ full row trigger",
            ),
            TextSpec::new(
                m::PX_50,
                m::PX_68,
                m::FONT_9,
                palette.muted,
                "Body content / reduced motion",
            ),
            TextSpec::new(m::PX_244, m::PX_42, m::FONT_8, palette.text, "› icon"),
            TextSpec::new(m::PX_234, m::PX_72, m::FONT_9, palette.muted, "tree mode"),
            TextSpec::new(
                m::PX_234,
                m::PX_88,
                m::FONT_9,
                palette.muted,
                "single / multiple",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
}
pub(super) fn split_pane(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let handle_x = if scenario.screen_state.has_widget_action() {
        SPLIT_ACTIVE_HANDLE_X
    } else {
        SPLIT_BASE_HANDLE_X
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "SplitPane",
        &[
            Block::outlined(m::PX_18, m::PX_36, m::PX_142, m::PX_48, palette.surface),
            Block::new(handle_x, m::PX_36, m::PX_6, m::PX_48, palette.accent),
            Block::outlined(m::PX_180, m::PX_36, m::PX_112, m::PX_48, palette.surface),
            Block::new(m::PX_180, m::PX_58, m::PX_112, m::PX_6, palette.accent),
        ],
        &[
            TextSpec::new(
                m::PX_20,
                m::PX_92,
                m::FONT_9,
                palette.muted,
                "horizontal ratio=42% min clamp",
            ),
            TextSpec::new(
                m::PX_180,
                m::PX_92,
                m::FONT_9,
                palette.muted,
                "vertical / reset / keyboard",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
}
pub(super) fn modal(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let dialog_fill = if scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.surface
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Modal / Overlay",
        &[
            Block::new(
                m::PX_18,
                m::PX_32,
                m::PX_148,
                m::PX_68,
                m::COLOR_MODAL_BACKDROP,
            ),
            Block::outlined(m::PX_38, m::PX_42, m::PX_108, m::PX_46, dialog_fill),
            Block::outlined(m::PX_188, m::PX_38, m::PX_118, m::PX_54, palette.panel),
        ],
        &[
            TextSpec::new(
                m::PX_48,
                m::PX_50,
                m::FONT_8,
                palette.text,
                "Overlay dialog",
            ),
            TextSpec::new(
                m::PX_198,
                m::PX_48,
                m::FONT_9,
                palette.muted,
                "native modal",
            ),
            TextSpec::new(
                m::PX_198,
                m::PX_64,
                m::FONT_9,
                palette.muted,
                "focus trap / Esc",
            ),
            TextSpec::new(
                m::PX_198,
                m::PX_80,
                m::FONT_9,
                palette.muted,
                "footer / return focus",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
    common::chip(
        canvas,
        text,
        palette,
        Rect::new(x + m::PX_58, y + m::PX_66, m::PX_58, m::PX_18),
        "Close",
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
        return "state=ready";
    }
    scenario.screen_state.state_label
}
