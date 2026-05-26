use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_status_bar_style::{
    center_label, progress_fill, progress_width, segment_fill, status_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const STATUS_BAR_X: usize = 30;
pub(super) const STATUS_BAR_Y: usize = 62;
const BAR_WIDTH: usize = 300;
const BAR_HEIGHT: usize = 28;
const SEGMENT_X: usize = 42;
const SEGMENT_Y: usize = 66;
const SEGMENT_WIDTH: usize = 72;
const SEGMENT_HEIGHT: usize = 20;
const SEGMENT_GAP: usize = 78;
const PROGRESS_X: usize = 184;
const PROGRESS_Y: usize = 96;
const PROGRESS_TRACK_WIDTH: usize = 132;
const PROGRESS_HEIGHT: usize = 6;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 9;
const LABEL_COUNT: usize = 6;

pub(super) fn status_bar(
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
        "Status bar",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            STATUS_BAR_X,
            STATUS_BAR_Y,
            BAR_WIDTH,
            BAR_HEIGHT,
            palette.panel,
        ),
        segment_block(palette, scenario, m::PX_0),
        segment_block(palette, scenario, m::PX_1),
        segment_block(palette, scenario, m::PX_2),
        Block::outlined(
            PROGRESS_X,
            PROGRESS_Y,
            PROGRESS_TRACK_WIDTH,
            PROGRESS_HEIGHT,
            palette.panel,
        ),
        Block::new(
            PROGRESS_X,
            PROGRESS_Y,
            progress_width(scenario),
            PROGRESS_HEIGHT,
            progress_fill(palette, scenario),
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
            SEGMENT_X + m::PX_8,
            SEGMENT_Y + m::PX_8,
            m::PX_6,
            m::PX_6,
            palette.accent,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            SEGMENT_X + TEXT_X_OFFSET,
            SEGMENT_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "main",
        ),
        TextSpec::new(
            SEGMENT_X + SEGMENT_GAP + TEXT_X_OFFSET,
            SEGMENT_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            center_label(scenario),
        ),
        TextSpec::new(
            SEGMENT_X + (SEGMENT_GAP * m::PX_2) + TEXT_X_OFFSET,
            SEGMENT_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "Indexing",
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
            STATUS_BAR_X + TEXT_X_OFFSET,
            STATUS_BAR_Y - m::PX_18,
            m::FONT_7,
            palette.muted,
            "status segments",
        ),
    ]
}

fn segment_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, index: usize) -> Block {
    Block::outlined(
        SEGMENT_X + (SEGMENT_GAP * index),
        SEGMENT_Y,
        SEGMENT_WIDTH,
        SEGMENT_HEIGHT,
        segment_fill(palette, scenario, index),
    )
}
