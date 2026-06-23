use super::canvas::Canvas;
use super::dedicated_command_palette_style::{
    query_label, row_fill, row_label, search_fill, shortcut_fill, status_label,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const COMMAND_PALETTE_X: usize = 30;
pub(super) const COMMAND_PALETTE_Y: usize = 24;
const PALETTE_WIDTH: usize = 300;
const PALETTE_HEIGHT: usize = 108;
const SEARCH_X: usize = 48;
const SEARCH_Y: usize = 42;
const SEARCH_WIDTH: usize = 238;
const SEARCH_HEIGHT: usize = 22;
const ROW_X: usize = 48;
const ROW_Y: usize = 74;
const ROW_WIDTH: usize = 238;
const ROW_HEIGHT: usize = 16;
const ROW_GAP: usize = 20;
const SHORTCUT_X: usize = 238;
const SHORTCUT_WIDTH: usize = 42;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const TEXT_Y_OFFSET: usize = 5;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 10;
const LABEL_COUNT: usize = 9;

pub(super) fn command_palette(
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
        "Command palette",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            COMMAND_PALETTE_X,
            COMMAND_PALETTE_Y,
            PALETTE_WIDTH,
            PALETTE_HEIGHT,
            palette.panel,
        ),
        Block::outlined(
            SEARCH_X,
            SEARCH_Y,
            SEARCH_WIDTH,
            SEARCH_HEIGHT,
            search_fill(palette, scenario),
        ),
        row_block(palette, scenario, m::PX_0),
        row_block(palette, scenario, m::PX_1),
        row_block(palette, scenario, m::PX_2),
        shortcut_block(palette, scenario, m::PX_0),
        shortcut_block(palette, scenario, m::PX_1),
        Block::outlined(
            SURFACE_TOKEN_X,
            SURFACE_TOKEN_Y,
            SURFACE_TOKEN_WIDTH,
            SURFACE_TOKEN_HEIGHT,
            palette.surface,
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
        Block::new(
            SEARCH_X + SEARCH_WIDTH - m::PX_12,
            SEARCH_Y + m::PX_6,
            m::PX_2,
            m::PX_12,
            palette.accent,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            SEARCH_X + TEXT_X_OFFSET,
            SEARCH_Y + TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            query_label(scenario),
        ),
        row_text(palette, scenario, m::PX_0),
        row_text(palette, scenario, m::PX_1),
        row_text(palette, scenario, m::PX_2),
        TextSpec::new(
            SHORTCUT_X + m::PX_6,
            row_y(m::PX_0) + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "cmd",
        ),
        TextSpec::new(
            SHORTCUT_X + m::PX_6,
            row_y(m::PX_1) + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "cmd",
        ),
        TextSpec::new(
            SURFACE_TOKEN_X + TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X + TEXT_X_OFFSET,
            STATUS_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            status_label(scenario),
        ),
        TextSpec::new(
            COMMAND_PALETTE_X + TEXT_X_OFFSET,
            COMMAND_PALETTE_Y + m::PX_8,
            m::FONT_7,
            palette.muted,
            "modal command surface",
        ),
    ]
}

fn row_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> Block {
    Block::outlined(
        ROW_X,
        row_y(row),
        ROW_WIDTH,
        ROW_HEIGHT,
        row_fill(palette, scenario, row),
    )
}

fn shortcut_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> Block {
    Block::outlined(
        SHORTCUT_X,
        row_y(row),
        SHORTCUT_WIDTH,
        ROW_HEIGHT,
        shortcut_fill(palette, scenario),
    )
}

fn row_text(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> TextSpec {
    TextSpec::new(
        ROW_X + TEXT_X_OFFSET,
        row_y(row) + TEXT_Y_OFFSET,
        m::FONT_7,
        palette.text,
        row_label(scenario, row),
    )
}

fn row_y(row: usize) -> usize {
    ROW_Y + (ROW_GAP * row)
}
