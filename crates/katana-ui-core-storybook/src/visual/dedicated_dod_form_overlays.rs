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
const BASE_PANEL_X: usize = m::PX_116;
const PLACEMENT_PANEL_X: usize = m::PX_96;
const BASE_PANEL_Y: usize = m::PX_34;
const FLIPPED_PANEL_Y: usize = m::PX_24;
const PANEL_WIDTH: usize = m::PX_188;
const WIDE_PANEL_WIDTH: usize = m::PX_230;
const PANEL_HEIGHT: usize = m::PX_60;
const POINTER_X_OFFSET: usize = m::PX_8;
const POINTER_Y: usize = m::PX_78;
const POINTER_SIZE: usize = m::PX_8;
const ANCHOR_X: usize = m::PX_28;
const ANCHOR_Y: usize = m::PX_72;
const ANCHOR_WIDTH: usize = m::PX_76;
const ANCHOR_HEIGHT: usize = m::PX_22;
const LABEL_X_OFFSET: usize = m::PX_12;
const FIRST_LABEL_Y_OFFSET: usize = m::PX_8;
const LABEL_GAP: usize = m::PX_16;
const BLOCK_COUNT: usize = 2;
const LABEL_COUNT: usize = 3;
const PLACEMENT_PRESET_INDEX: usize = 1;
const AUTO_FLIP_PRESET_INDEX: usize = 2;
const OFFSET_WIDTH_PRESET_INDEX: usize = 3;

pub(super) fn popover(
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
        "Popover",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
    common::chip(
        canvas,
        text,
        palette,
        Rect::new(x + ANCHOR_X, y + ANCHOR_Y, ANCHOR_WIDTH, ANCHOR_HEIGHT),
        "anchor",
        palette.accent,
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            panel_x(scenario),
            panel_y(scenario),
            panel_width(scenario),
            PANEL_HEIGHT,
            panel_fill(palette, scenario),
        ),
        Block::new(
            panel_x(scenario) - POINTER_X_OFFSET,
            POINTER_Y,
            POINTER_SIZE,
            POINTER_SIZE,
            pointer_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            panel_x(scenario) + LABEL_X_OFFSET,
            panel_y(scenario) + FIRST_LABEL_Y_OFFSET,
            m::FONT_9,
            label_color(palette, scenario),
            placement_label(scenario),
        ),
        TextSpec::new(
            panel_x(scenario) + LABEL_X_OFFSET,
            panel_y(scenario) + FIRST_LABEL_Y_OFFSET + LABEL_GAP,
            m::FONT_9,
            label_color(palette, scenario),
            close_label(scenario),
        ),
        TextSpec::new(
            panel_x(scenario) + LABEL_X_OFFSET,
            panel_y(scenario) + FIRST_LABEL_Y_OFFSET + LABEL_GAP + LABEL_GAP,
            m::FONT_9,
            label_color(palette, scenario),
            slot_label(scenario),
        ),
    ]
}

fn panel_x(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return PLACEMENT_PANEL_X;
    }
    BASE_PANEL_X
}

fn panel_y(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == AUTO_FLIP_PRESET_INDEX {
        return FLIPPED_PANEL_Y;
    }
    BASE_PANEL_Y
}

fn panel_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == OFFSET_WIDTH_PRESET_INDEX {
        return WIDE_PANEL_WIDTH;
    }
    PANEL_WIDTH
}

fn panel_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.preset_index == AUTO_FLIP_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

fn pointer_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return common::TOKEN;
    }
    panel_fill(palette, scenario)
}

fn label_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() {
        return palette.background;
    }
    palette.muted
}

fn placement_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PLACEMENT_PRESET_INDEX {
        return "placement: bottom-start";
    }
    "placement: right + offset 12"
}

fn close_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == AUTO_FLIP_PRESET_INDEX {
        return "auto flip -> top-start";
    }
    "outside click -> close"
}

fn slot_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == OFFSET_WIDTH_PRESET_INDEX {
        return "width 320px + slot action";
    }
    "Esc / content select log"
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
