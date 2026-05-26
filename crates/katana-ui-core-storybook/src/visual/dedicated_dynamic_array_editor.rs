use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_dynamic_array_editor_style::{
    control_fill, reorder_fill, row_fill, row_label, status_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const ARRAY_EDITOR_X: usize = 30;
pub(super) const ARRAY_EDITOR_Y: usize = 30;
const EDITOR_WIDTH: usize = 300;
const EDITOR_HEIGHT: usize = 98;
const ROW_X: usize = 44;
const ROW_Y: usize = 54;
const ROW_WIDTH: usize = 190;
const ROW_HEIGHT: usize = 18;
const ROW_GAP: usize = 22;
const DRAG_X: usize = 48;
const DRAG_WIDTH: usize = 4;
const ACTION_X: usize = 246;
const ACTION_WIDTH: usize = 58;
const ACTION_HEIGHT: usize = 20;
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
const BLOCK_COUNT: usize = 11;
const LABEL_COUNT: usize = 7;

pub(super) fn dynamic_array_editor(
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
        "Dynamic array",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            ARRAY_EDITOR_X,
            ARRAY_EDITOR_Y,
            EDITOR_WIDTH,
            EDITOR_HEIGHT,
            palette.panel,
        ),
        row_block(palette, scenario, m::PX_0),
        row_block(palette, scenario, m::PX_1),
        row_block(palette, scenario, m::PX_2),
        drag_block(palette, scenario, m::PX_0),
        drag_block(palette, scenario, m::PX_1),
        drag_block(palette, scenario, m::PX_2),
        Block::outlined(
            ACTION_X,
            ROW_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            control_fill(palette, scenario),
        ),
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
            ACTION_X + m::PX_8,
            ROW_Y + m::PX_8,
            m::PX_42,
            m::PX_4,
            palette.accent,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        row_text(palette, scenario, m::PX_0),
        row_text(palette, scenario, m::PX_1),
        row_text(palette, scenario, m::PX_2),
        TextSpec::new(
            ACTION_X + TEXT_X_OFFSET,
            ROW_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "add",
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
            ARRAY_EDITOR_X + TEXT_X_OFFSET,
            ARRAY_EDITOR_Y + m::PX_8,
            m::FONT_7,
            palette.muted,
            "array editor contract",
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

fn drag_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> Block {
    Block::new(
        DRAG_X,
        row_y(row) + m::PX_4,
        DRAG_WIDTH,
        ROW_HEIGHT - m::PX_8,
        reorder_fill(palette, scenario),
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
