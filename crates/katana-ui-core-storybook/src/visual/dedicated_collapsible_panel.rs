use super::canvas::Canvas;
use super::dedicated_collapsible_panel_style::{
    active_row_index, handle_fill, panel_fill, panel_width, pin_fill, rail_fill, row_fill,
    row_text, state_label,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const PANEL_X: usize = 34;
pub(super) const PANEL_Y: usize = 30;
const PANEL_HEIGHT: usize = 96;
const RAIL_WIDTH: usize = 36;
const HEADER_HEIGHT: usize = 20;
const ROW_X_OFFSET: usize = 46;
const FIRST_ROW_Y: usize = PANEL_Y + 28;
const ROW_WIDTH: usize = 148;
const ROW_HEIGHT: usize = 18;
const ROW_GAP: usize = 7;
const HANDLE_WIDTH: usize = 8;
const HANDLE_X_GAP: usize = 8;
const STATUS_X: usize = 340;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 142;
const STATUS_HEIGHT: usize = 20;
const PIN_SIZE: usize = 12;
const LINE_HEIGHT: usize = 3;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 5;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 9;
const LABEL_COUNT: usize = 6;

pub(super) fn collapsible_panel(
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
        "Collapsible panel",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let active = active_row_index(scenario);
    let width = panel_width(scenario);
    [
        Block::outlined(
            PANEL_X,
            PANEL_Y,
            width,
            PANEL_HEIGHT,
            panel_fill(palette, scenario),
        ),
        Block::new(PANEL_X, PANEL_Y, width, LINE_HEIGHT, palette.accent),
        Block::outlined(
            PANEL_X,
            PANEL_Y,
            RAIL_WIDTH,
            PANEL_HEIGHT,
            rail_fill(palette, scenario),
        ),
        row_block(palette, active, m::PX_0),
        row_block(palette, active, m::PX_1),
        row_block(palette, active, m::PX_2),
        Block::new(
            handle_x(width),
            PANEL_Y,
            HANDLE_WIDTH,
            PANEL_HEIGHT,
            handle_fill(palette, scenario),
        ),
        Block::new(
            PANEL_X + m::PX_12,
            PANEL_Y + HEADER_HEIGHT + m::PX_2,
            PIN_SIZE,
            PIN_SIZE,
            pin_fill(scenario),
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    let active = active_row_index(scenario);
    [
        TextSpec::new(
            PANEL_X + LABEL_X_OFFSET,
            PANEL_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "rail",
        ),
        row_label(palette, active, m::PX_0, "Files"),
        row_label(palette, active, m::PX_1, "History"),
        row_label(palette, active, m::PX_2, "Outline"),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            handle_x(panel_width(scenario)) + m::PX_2,
            PANEL_Y + m::PX_8,
            m::FONT_7,
            common::TOKEN,
            "||",
        ),
    ]
}

fn row_block(palette: &VisualPalette, active: usize, index: usize) -> Block {
    Block::outlined(
        PANEL_X + ROW_X_OFFSET,
        FIRST_ROW_Y + ((ROW_HEIGHT + ROW_GAP) * index),
        ROW_WIDTH,
        ROW_HEIGHT,
        row_fill(palette, active, index),
    )
}

fn row_label(
    palette: &VisualPalette,
    active: usize,
    index: usize,
    value: &'static str,
) -> TextSpec {
    TextSpec::new(
        PANEL_X + ROW_X_OFFSET + LABEL_X_OFFSET,
        FIRST_ROW_Y + ((ROW_HEIGHT + ROW_GAP) * index) + LABEL_Y_OFFSET,
        m::FONT_7,
        row_text(palette, active, index),
        value,
    )
}

fn handle_x(width: usize) -> usize {
    PANEL_X + width + HANDLE_X_GAP
}
