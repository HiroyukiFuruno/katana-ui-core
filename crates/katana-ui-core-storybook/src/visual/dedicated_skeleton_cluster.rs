use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_skeleton_cluster_style::{
    line_fill, line_width, media_fill, media_width, preset_label, reduced_motion_label,
    secondary_fill, state_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const MEDIA_X: usize = 42;
const MEDIA_Y: usize = 42;
const MEDIA_HEIGHT: usize = 48;
const LINE_X: usize = 174;
const LINE_Y: usize = 44;
const LINE_HEIGHT: usize = 10;
const LINE_GAP: usize = 18;
const CARD_X: usize = 42;
const CARD_Y: usize = 96;
const CARD_WIDTH: usize = 436;
const CARD_HEIGHT: usize = 18;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const STATUS_X: usize = 42;
const STATUS_Y: usize = 120;
const STATUS_GAP: usize = 126;
const STATE_STATUS_INDEX: usize = 2;
const STATE_STATUS_X: usize = STATUS_X + STATUS_GAP * STATE_STATUS_INDEX;
const BLOCK_COUNT: usize = 7;
const LABEL_COUNT: usize = 5;

pub(super) fn skeleton_cluster(
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
        "Skeleton cluster",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            MEDIA_X,
            MEDIA_Y,
            media_width(scenario),
            MEDIA_HEIGHT,
            media_fill(palette, scenario),
        ),
        Block::new(
            LINE_X,
            LINE_Y,
            line_width(scenario),
            LINE_HEIGHT,
            line_fill(palette, scenario),
        ),
        Block::new(
            LINE_X,
            LINE_Y + LINE_GAP,
            line_width(scenario) - m::PX_34,
            LINE_HEIGHT,
            palette.surface,
        ),
        Block::new(
            LINE_X,
            LINE_Y + LINE_GAP * 2,
            line_width(scenario) - m::PX_68,
            LINE_HEIGHT,
            palette.surface,
        ),
        Block::outlined(
            CARD_X,
            CARD_Y,
            CARD_WIDTH,
            CARD_HEIGHT,
            secondary_fill(palette, scenario),
        ),
        Block::new(
            CARD_X + m::PX_8,
            CARD_Y + m::PX_6,
            CARD_WIDTH - m::PX_16,
            m::PX_6,
            palette.surface,
        ),
        Block::outlined(
            SURFACE_TOKEN_X,
            SURFACE_TOKEN_Y,
            SURFACE_TOKEN_WIDTH,
            SURFACE_TOKEN_HEIGHT,
            palette.surface,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            SURFACE_TOKEN_X + TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            preset_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_GAP,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            "live region",
        ),
        TextSpec::new(
            STATE_STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            STATE_STATUS_X + STATUS_GAP,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            reduced_motion_label(scenario),
        ),
    ]
}
