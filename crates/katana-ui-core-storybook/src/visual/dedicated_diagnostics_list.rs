use super::canvas::Canvas;
use super::dedicated_diagnostics_list_style::{
    header_label, preview_fill, range_width, row_fill, row_label, severity_fill, status_label,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const DIAGNOSTICS_X: usize = 30;
pub(super) const DIAGNOSTICS_Y: usize = 28;
const LIST_WIDTH: usize = 300;
const LIST_HEIGHT: usize = 102;
const HEADER_HEIGHT: usize = 22;
const HEADER_LABEL_X: usize = 42;
const HEADER_LABEL_Y: usize = 36;
const ROW_X: usize = 42;
const ROW_Y: usize = 58;
const ROW_WIDTH: usize = 236;
const ROW_HEIGHT: usize = 16;
const ROW_GAP: usize = 20;
const SEVERITY_WIDTH: usize = 4;
const PREVIEW_X: usize = 286;
const PREVIEW_Y: usize = 58;
const PREVIEW_WIDTH: usize = 30;
const PREVIEW_HEIGHT: usize = 56;
const RANGE_X: usize = 42;
const RANGE_Y: usize = 118;
const RANGE_HEIGHT: usize = 4;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const ROW_TEXT_Y_OFFSET: usize = 5;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 12;
const LABEL_COUNT: usize = 7;
const ROW_COUNT: usize = 3;

pub(super) fn diagnostics_list(
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
        "Diagnostics list",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            DIAGNOSTICS_X,
            DIAGNOSTICS_Y,
            LIST_WIDTH,
            LIST_HEIGHT,
            palette.surface,
        ),
        Block::new(
            DIAGNOSTICS_X,
            DIAGNOSTICS_Y,
            LIST_WIDTH,
            HEADER_HEIGHT,
            palette.panel,
        ),
        row_block(palette, scenario, m::PX_0),
        row_block(palette, scenario, m::PX_1),
        row_block(palette, scenario, m::PX_2),
        severity_block(scenario, m::PX_0),
        severity_block(scenario, m::PX_1),
        severity_block(scenario, m::PX_2),
        Block::outlined(
            PREVIEW_X,
            PREVIEW_Y,
            PREVIEW_WIDTH,
            PREVIEW_HEIGHT,
            preview_fill(palette, scenario),
        ),
        Block::new(
            RANGE_X,
            RANGE_Y,
            range_width(scenario),
            RANGE_HEIGHT,
            palette.accent,
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
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            HEADER_LABEL_X,
            HEADER_LABEL_Y,
            m::FONT_8,
            palette.text,
            header_label(scenario),
        ),
        row_text(palette, scenario, m::PX_0),
        row_text(palette, scenario, m::PX_1),
        row_text(palette, scenario, m::PX_2),
        TextSpec::new(
            SURFACE_TOKEN_X + TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X + TEXT_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            status_label(scenario),
        ),
        TextSpec::new(
            PREVIEW_X + m::PX_6,
            PREVIEW_Y + m::PX_8,
            m::FONT_7,
            palette.text,
            "fix",
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

fn severity_block(scenario: ScenarioContext<'_>, row: usize) -> Block {
    Block::new(
        ROW_X,
        row_y(row),
        SEVERITY_WIDTH,
        ROW_HEIGHT,
        severity_fill(scenario, row),
    )
}

fn row_text(palette: &VisualPalette, scenario: ScenarioContext<'_>, row: usize) -> TextSpec {
    TextSpec::new(
        ROW_X + TEXT_X_OFFSET,
        row_y(row) + ROW_TEXT_Y_OFFSET,
        m::FONT_7,
        palette.text,
        row_label(scenario, row),
    )
}

fn row_y(row: usize) -> usize {
    ROW_Y + (ROW_GAP * row.min(ROW_COUNT - m::PX_1))
}
