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
const OPEN_PRESET_INDEX: usize = 1;
const DISABLED_PRESET_INDEX: usize = 2;
const CONTROLLED_PRESET_INDEX: usize = 3;
const MULTIPLE_PRESET_INDEX: usize = 4;
const TREE_PRESET_INDEX: usize = 5;
const REDUCED_MOTION_PRESET_INDEX: usize = 6;
const TRIGGER_AREAS_PRESET_INDEX: usize = 7;
pub(super) fn accordion(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Accordion",
        &[
            Block::outlined(
                m::PX_18,
                m::PX_34,
                m::PX_204,
                m::PX_24,
                header_fill(palette, scenario),
            ),
            Block::new(
                m::PX_36,
                m::PX_58,
                body_width(scenario),
                m::PX_34,
                body_fill(palette, scenario),
            ),
            Block::outlined(m::PX_234, m::PX_34, m::PX_90, m::PX_24, mode_fill(scenario)),
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

fn header_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override()
        || scenario.preset_index == CONTROLLED_PRESET_INDEX
    {
        return common::WARN;
    }
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.border;
    }
    if scenario.preset_index == TRIGGER_AREAS_PRESET_INDEX {
        return common::PURPLE;
    }
    palette.surface
}

fn body_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == OPEN_PRESET_INDEX {
        return palette.accent;
    }
    if scenario.preset_index == REDUCED_MOTION_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.border
}

fn mode_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == TREE_PRESET_INDEX {
        return common::SUCCESS;
    }
    if scenario.preset_index == MULTIPLE_PRESET_INDEX {
        return common::PURPLE;
    }
    PANEL
}

fn body_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == OPEN_PRESET_INDEX {
        return m::PX_204;
    }
    if scenario.preset_index == MULTIPLE_PRESET_INDEX {
        return m::PX_90;
    }
    m::PX_1
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
