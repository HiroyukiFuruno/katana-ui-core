use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const GRID_SPAN_PRESET_INDEX: usize = 1;
const GRID_OVERFLOW_PRESET_INDEX: usize = 2;
const GRID_THEME_PRESET_INDEX: usize = 3;
const STAGE_X: usize = m::PX_16;
const STAGE_Y: usize = m::PX_36;
const STAGE_WIDTH: usize = m::PX_252;
const STAGE_HEIGHT: usize = m::PX_74;
const CELL_X: usize = m::PX_32;
const CELL_Y: usize = m::PX_44;
const CELL_WIDTH: usize = m::PX_48;
const CELL_HEIGHT: usize = m::PX_22;
const DEFAULT_GAP: usize = m::PX_8;
const THEME_GAP: usize = m::PX_14;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const GRID_BLOCK_COUNT: usize = 8;
const GRID_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn grid(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Grid layout",
        &grid_blocks(palette, scenario, accent),
        &grid_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn grid_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; GRID_BLOCK_COUNT] {
    let gap = if scenario.preset_index == GRID_THEME_PRESET_INDEX {
        THEME_GAP
    } else {
        DEFAULT_GAP
    };
    [
        Block::outlined(STAGE_X, STAGE_Y, STAGE_WIDTH, STAGE_HEIGHT, palette.surface),
        Block::new(CELL_X, CELL_Y, CELL_WIDTH, CELL_HEIGHT, accent),
        Block::new(
            cell_x(1, gap),
            CELL_Y,
            second_cell_width(scenario, gap),
            CELL_HEIGHT,
            palette.panel,
        ),
        Block::new(
            cell_x(2, gap),
            CELL_Y,
            CELL_WIDTH,
            CELL_HEIGHT,
            common::TOKEN,
        ),
        Block::new(
            CELL_X,
            CELL_Y + CELL_HEIGHT + gap,
            CELL_WIDTH,
            CELL_HEIGHT,
            palette.panel,
        ),
        Block::new(
            cell_x(1, gap),
            CELL_Y + CELL_HEIGHT + gap,
            CELL_WIDTH,
            CELL_HEIGHT,
            common::WARN,
        ),
        Block::new(
            cell_x(2, gap),
            CELL_Y + CELL_HEIGHT + gap,
            CELL_WIDTH,
            CELL_HEIGHT,
            overflow_cell_color(scenario, palette),
        ),
        Block::new(
            STAGE_X + STAGE_WIDTH - m::PX_28,
            STAGE_Y + m::PX_12,
            overflow_marker_width(scenario),
            STAGE_HEIGHT - m::PX_24,
            common::DANGER,
        ),
    ]
}

fn grid_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; GRID_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            grid_preset_label(scenario),
        ),
        TextSpec::new(LABEL_X, m::PX_58, m::FONT_8, palette.muted, "2 x 3 cells"),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_48, m::PX_56, m::FONT_8, palette.background, "1"),
    ]
}

fn cell_x(column: usize, gap: usize) -> usize {
    CELL_X + column * (CELL_WIDTH + gap)
}

fn second_cell_width(scenario: ScenarioContext<'_>, gap: usize) -> usize {
    if scenario.preset_index == GRID_SPAN_PRESET_INDEX {
        return CELL_WIDTH * 2 + gap;
    }
    CELL_WIDTH
}

fn overflow_cell_color(scenario: ScenarioContext<'_>, palette: &VisualPalette) -> u32 {
    if scenario.preset_index == GRID_OVERFLOW_PRESET_INDEX {
        return common::DANGER;
    }
    palette.panel
}

fn overflow_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == GRID_OVERFLOW_PRESET_INDEX {
        return m::PX_18;
    }
    m::PX_6
}

fn grid_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        GRID_SPAN_PRESET_INDEX => "span 2 columns",
        GRID_OVERFLOW_PRESET_INDEX => "overflow marker",
        GRID_THEME_PRESET_INDEX => "theme gap=14",
        _ => "grid 2x3",
    }
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, label) in status_labels(scenario).into_iter().enumerate() {
        let row_x = x + STAGE_X + index * (STATUS_WIDTH + STATUS_GAP);
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
            label,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn status_labels(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_LABEL_COUNT] {
    if scenario.screen_state.has_settings_override() {
        return ["action grid", "event cell", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
