use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, ChipSpec, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL: u32 = 0x20242c;
const STATUS_X: usize = 18;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;

pub(super) fn badge(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let neutral_fill = if scenario.screen_state.has_widget_action() {
        common::DANGER
    } else {
        palette.panel
    };
    common::frame(canvas, text, palette, x, y, "Badge tone grid");
    common::draw_chips(
        canvas,
        text,
        palette,
        x,
        y,
        &[
            ChipSpec::new(
                m::PX_14,
                m::PX_36,
                m::PX_94,
                m::PX_20,
                "neutral",
                neutral_fill,
            ),
            ChipSpec::new(
                m::PX_118,
                m::PX_36,
                m::PX_94,
                m::PX_20,
                "accent",
                palette.accent,
            ),
            ChipSpec::new(
                m::PX_222,
                m::PX_36,
                m::PX_94,
                m::PX_20,
                "danger",
                common::DANGER,
            ),
            ChipSpec::new(
                m::PX_14,
                m::PX_64,
                m::PX_94,
                m::PX_20,
                "warning",
                common::WARN,
            ),
            ChipSpec::new(
                m::PX_118,
                m::PX_64,
                m::PX_94,
                m::PX_20,
                "success",
                common::SUCCESS,
            ),
            ChipSpec::new(
                m::PX_222,
                m::PX_64,
                m::PX_94,
                m::PX_20,
                "● icon",
                common::PURPLE,
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
}
pub(super) fn card(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let card_fill = if scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.surface
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Card slots",
        &[
            Block::outlined(m::PX_18, m::PX_32, m::PX_252, m::PX_70, card_fill),
            Block::new(m::PX_18, m::PX_32, m::PX_252, m::PX_18, PANEL),
            Block::outlined(m::PX_32, m::PX_60, m::PX_92, m::PX_18, palette.panel),
        ],
        &[
            TextSpec::new(
                m::PX_28,
                m::PX_37,
                m::FONT_9,
                palette.text,
                "Header + Badge",
            ),
            TextSpec::new(m::PX_40, m::PX_65, m::FONT_8, palette.muted, "TextInput"),
            TextSpec::new(
                m::PX_284,
                m::PX_50,
                m::FONT_9,
                palette.muted,
                "child state isolated",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
    common::draw_chips(
        canvas,
        text,
        palette,
        x,
        y,
        &[
            ChipSpec::new(
                m::PX_136,
                m::PX_60,
                m::PX_58,
                m::PX_18,
                "new",
                common::SUCCESS,
            ),
            ChipSpec::new(
                m::PX_202,
                m::PX_76,
                m::PX_56,
                m::PX_20,
                "Save",
                palette.accent,
            ),
        ],
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
