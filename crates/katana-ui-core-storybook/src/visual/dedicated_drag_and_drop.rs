use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_drag_and_drop_style::{
    indicator_fill, mode_label, payload_label, rail_fill, source_fill, state_label, target_fill,
    target_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const SOURCE_X: usize = 42;
const SOURCE_Y: usize = 42;
const SOURCE_WIDTH: usize = 150;
const SOURCE_HEIGHT: usize = 30;
const TARGET_X: usize = 312;
const TARGET_Y: usize = 42;
const TARGET_WIDTH: usize = 166;
const TARGET_HEIGHT: usize = 54;
const PATH_X: usize = 198;
const PATH_Y: usize = 56;
const PATH_WIDTH: usize = 92;
const PATH_HEIGHT: usize = 4;
const INDICATOR_X: usize = 244;
const INDICATOR_Y: usize = 70;
const INDICATOR_WIDTH: usize = 48;
const INDICATOR_HEIGHT: usize = 5;
const RAIL_X: usize = 42;
const RAIL_Y: usize = 104;
const RAIL_WIDTH: usize = 436;
const RAIL_HEIGHT: usize = 8;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const CARD_TEXT_Y_OFFSET: usize = 11;
const TARGET_TEXT_Y_OFFSET: usize = 18;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const STATUS_X: usize = 42;
const STATUS_Y: usize = 118;
const STATUS_GAP: usize = 118;
const STATE_STATUS_INDEX: usize = 2;
const STATE_STATUS_X: usize = STATUS_X + STATUS_GAP * STATE_STATUS_INDEX;
const BLOCK_COUNT: usize = 6;
const LABEL_COUNT: usize = 8;

pub(super) fn drag_and_drop(
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
        "Drag and drop",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            SOURCE_X,
            SOURCE_Y,
            SOURCE_WIDTH,
            SOURCE_HEIGHT,
            source_fill(palette, scenario),
        ),
        Block::outlined(
            TARGET_X,
            TARGET_Y,
            TARGET_WIDTH,
            TARGET_HEIGHT,
            target_fill(palette, scenario),
        ),
        Block::new(PATH_X, PATH_Y, PATH_WIDTH, PATH_HEIGHT, palette.accent),
        Block::new(
            INDICATOR_X,
            INDICATOR_Y,
            INDICATOR_WIDTH,
            INDICATOR_HEIGHT,
            indicator_fill(palette, scenario),
        ),
        Block::outlined(
            RAIL_X,
            RAIL_Y,
            RAIL_WIDTH,
            RAIL_HEIGHT,
            rail_fill(palette, scenario),
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
            SOURCE_X + TEXT_X_OFFSET,
            SOURCE_Y + CARD_TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            payload_label(scenario),
        ),
        TextSpec::new(
            TARGET_X + TEXT_X_OFFSET,
            TARGET_Y + TARGET_TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            target_label(scenario),
        ),
        TextSpec::new(
            INDICATOR_X,
            INDICATOR_Y + m::PX_8,
            m::FONT_7,
            palette.muted,
            "drop indicator",
        ),
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
            mode_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_GAP,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            "autoscroll=edge",
        ),
        TextSpec::new(
            STATE_STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            TARGET_X + TEXT_X_OFFSET,
            TARGET_Y + TARGET_TEXT_Y_OFFSET + m::PX_16,
            m::FONT_7,
            palette.muted,
            "drop zone",
        ),
    ]
}
