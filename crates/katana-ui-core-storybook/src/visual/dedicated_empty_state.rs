use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_empty_state_style::{
    alignment_marker_x, body_label, heading_label, illustration_fill, panel_fill, primary_fill,
    secondary_fill, status_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const EMPTY_STATE_X: usize = 32;
pub(super) const EMPTY_STATE_Y: usize = 30;
const EMPTY_STATE_WIDTH: usize = 286;
const EMPTY_STATE_HEIGHT: usize = 86;
const ILLUSTRATION_X: usize = 142;
const ILLUSTRATION_Y: usize = 44;
const ILLUSTRATION_SIZE: usize = 28;
const HEADING_X: usize = 86;
const HEADING_Y: usize = 78;
const BODY_X: usize = 82;
const BODY_Y: usize = 94;
const PRIMARY_X: usize = 82;
const PRIMARY_Y: usize = 124;
const PRIMARY_WIDTH: usize = 70;
const SECONDARY_X: usize = 162;
const SECONDARY_Y: usize = 124;
const SECONDARY_WIDTH: usize = 76;
const ACTION_HEIGHT: usize = 20;
const ALIGNMENT_MARKER_Y: usize = 112;
const ALIGNMENT_MARKER_WIDTH: usize = 84;
const ALIGNMENT_MARKER_HEIGHT: usize = 4;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const ACTION_TEXT_Y_OFFSET: usize = 6;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 8;
const LABEL_COUNT: usize = 7;

pub(super) fn empty_state(
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
        "Empty state",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            EMPTY_STATE_X,
            EMPTY_STATE_Y,
            EMPTY_STATE_WIDTH,
            EMPTY_STATE_HEIGHT,
            panel_fill(palette, scenario),
        ),
        Block::outlined(
            ILLUSTRATION_X,
            ILLUSTRATION_Y,
            ILLUSTRATION_SIZE,
            ILLUSTRATION_SIZE,
            illustration_fill(palette, scenario),
        ),
        Block::new(
            alignment_marker_x(scenario),
            ALIGNMENT_MARKER_Y,
            ALIGNMENT_MARKER_WIDTH,
            ALIGNMENT_MARKER_HEIGHT,
            illustration_fill(palette, scenario),
        ),
        Block::outlined(
            PRIMARY_X,
            PRIMARY_Y,
            PRIMARY_WIDTH,
            ACTION_HEIGHT,
            primary_fill(palette, scenario),
        ),
        Block::outlined(
            SECONDARY_X,
            SECONDARY_Y,
            SECONDARY_WIDTH,
            ACTION_HEIGHT,
            secondary_fill(palette, scenario),
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
            ILLUSTRATION_X + m::PX_8,
            ILLUSTRATION_Y + m::PX_8,
            m::PX_12,
            m::PX_12,
            palette.background,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            HEADING_X,
            HEADING_Y,
            m::FONT_10,
            palette.text,
            heading_label(scenario),
        ),
        TextSpec::new(
            BODY_X,
            BODY_Y,
            m::FONT_8,
            palette.muted,
            body_label(scenario),
        ),
        TextSpec::new(
            PRIMARY_X + TEXT_X_OFFSET,
            PRIMARY_Y + ACTION_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "reload",
        ),
        TextSpec::new(
            SECONDARY_X + TEXT_X_OFFSET,
            SECONDARY_Y + ACTION_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "docs",
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
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            status_label(scenario),
        ),
        TextSpec::new(
            EMPTY_STATE_X + TEXT_X_OFFSET,
            EMPTY_STATE_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "empty content contract",
        ),
    ]
}
