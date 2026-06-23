use super::canvas::Canvas;
use super::dedicated_chip_group_style::{
    chip_fill, group_fill, overflow_fill, overflow_label, reorder_fill, scroll_thumb_fill,
    state_label,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const GROUP_X: usize = 32;
pub(super) const GROUP_Y: usize = 34;
const GROUP_WIDTH: usize = 270;
const GROUP_HEIGHT: usize = 78;
const CHIP_Y: usize = GROUP_Y + 18;
const CHIP_WIDTH: usize = 58;
const CHIP_HEIGHT: usize = 22;
const CHIP_GAP: usize = 10;
const CHIP_X: usize = GROUP_X + 14;
const SECOND_ROW_Y: usize = CHIP_Y + CHIP_HEIGHT + m::PX_8;
const OVERFLOW_X: usize = GROUP_X + 212;
const OVERFLOW_WIDTH: usize = 38;
const SCROLL_TRACK_X: usize = GROUP_X + 18;
const SCROLL_TRACK_Y: usize = GROUP_Y + 64;
const SCROLL_TRACK_WIDTH: usize = 214;
const SCROLL_TRACK_HEIGHT: usize = 5;
const SCROLL_THUMB_WIDTH: usize = 74;
const REORDER_X: usize = GROUP_X + 86;
const REORDER_Y: usize = GROUP_Y + 14;
const REORDER_WIDTH: usize = 4;
const REORDER_HEIGHT: usize = 44;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const DRAG_LABEL_X_OFFSET: usize = 194;
const DRAG_LABEL_Y_OFFSET: usize = 52;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 7;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 10;
const LABEL_COUNT: usize = 8;

pub(super) fn chip_group(
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
        "Chip group",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            GROUP_X,
            GROUP_Y,
            GROUP_WIDTH,
            GROUP_HEIGHT,
            group_fill(palette),
        ),
        chip_block(palette, scenario, m::PX_0, CHIP_Y),
        chip_block(palette, scenario, m::PX_1, CHIP_Y),
        chip_block(palette, scenario, m::PX_2, SECOND_ROW_Y),
        Block::outlined(
            OVERFLOW_X,
            CHIP_Y,
            OVERFLOW_WIDTH,
            CHIP_HEIGHT,
            overflow_fill(palette, scenario),
        ),
        Block::new(
            SCROLL_TRACK_X,
            SCROLL_TRACK_Y,
            SCROLL_TRACK_WIDTH,
            SCROLL_TRACK_HEIGHT,
            palette.border,
        ),
        Block::new(
            SCROLL_TRACK_X,
            SCROLL_TRACK_Y,
            SCROLL_THUMB_WIDTH,
            SCROLL_TRACK_HEIGHT,
            scroll_thumb_fill(palette, scenario),
        ),
        Block::new(
            REORDER_X,
            REORDER_Y,
            REORDER_WIDTH,
            REORDER_HEIGHT,
            reorder_fill(palette, scenario),
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
        chip_label(palette, m::PX_0, CHIP_Y, "lint"),
        chip_label(palette, m::PX_1, CHIP_Y, "format"),
        chip_label(palette, m::PX_2, SECOND_ROW_Y, "docs"),
        TextSpec::new(
            OVERFLOW_X + LABEL_X_OFFSET,
            CHIP_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.text,
            overflow_label(scenario),
        ),
        TextSpec::new(
            GROUP_X + LABEL_X_OFFSET,
            GROUP_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "wrap / overflow",
        ),
        TextSpec::new(
            SURFACE_TOKEN_X + LABEL_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            GROUP_X + DRAG_LABEL_X_OFFSET,
            GROUP_Y + DRAG_LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "drag",
        ),
    ]
}

fn chip_block(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
    y: usize,
) -> Block {
    Block::outlined(
        CHIP_X + ((CHIP_WIDTH + CHIP_GAP) * index),
        y,
        CHIP_WIDTH,
        CHIP_HEIGHT,
        chip_fill(palette, scenario, index),
    )
}

fn chip_label(palette: &VisualPalette, index: usize, y: usize, label: &'static str) -> TextSpec {
    TextSpec::new(
        CHIP_X + ((CHIP_WIDTH + CHIP_GAP) * index) + LABEL_X_OFFSET,
        y + LABEL_Y_OFFSET,
        m::FONT_7,
        palette.text,
        label,
    )
}
