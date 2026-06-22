use super::canvas::Canvas;
use super::dedicated_chip_style::{
    chip_fill, dismiss_fill, dismiss_label, focus_fill, icon_fill, state_label, tone_fill,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const CHIP_X: usize = 34;
pub(super) const CHIP_Y: usize = 38;
pub(super) const CHIP_WIDTH: usize = 164;
pub(super) const CHIP_HEIGHT: usize = 26;
const ICON_X_OFFSET: usize = 12;
const ICON_Y_OFFSET: usize = 9;
const ICON_SIZE: usize = 8;
const LABEL_X_OFFSET: usize = 28;
const LABEL_Y_OFFSET: usize = 8;
const DISMISS_X_OFFSET: usize = 134;
const DISMISS_Y_OFFSET: usize = 6;
const DISMISS_SIZE: usize = 14;
const FOCUS_Y_OFFSET: usize = 31;
const FOCUS_WIDTH: usize = 154;
const TONE_X: usize = 230;
const TONE_Y: usize = 38;
const TONE_WIDTH: usize = 70;
const TONE_HEIGHT: usize = 20;
const TONE_GAP: usize = 12;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const STATUS_TEXT_X_OFFSET: usize = 8;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const TONE_LABEL_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 9;
const LABEL_COUNT: usize = 7;

pub(super) fn chip(
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
        "Chip",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
    common::cross_icon(
        canvas,
        x + CHIP_X + DISMISS_X_OFFSET,
        y + CHIP_Y + DISMISS_Y_OFFSET,
        DISMISS_SIZE,
        palette.text,
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            CHIP_X,
            CHIP_Y,
            CHIP_WIDTH,
            CHIP_HEIGHT,
            chip_fill(palette, scenario),
        ),
        Block::new(
            CHIP_X + ICON_X_OFFSET,
            CHIP_Y + ICON_Y_OFFSET,
            ICON_SIZE,
            ICON_SIZE,
            icon_fill(palette, scenario),
        ),
        Block::new(
            CHIP_X + m::PX_4,
            CHIP_Y + FOCUS_Y_OFFSET,
            FOCUS_WIDTH,
            m::PX_3,
            focus_fill(palette, scenario),
        ),
        Block::outlined(
            CHIP_X + DISMISS_X_OFFSET - m::PX_4,
            CHIP_Y + m::PX_3,
            DISMISS_SIZE + m::PX_8,
            DISMISS_SIZE + m::PX_4,
            dismiss_fill(palette, scenario),
        ),
        tone_block(palette, scenario, m::PX_0),
        tone_block(palette, scenario, m::PX_1),
        tone_block(palette, scenario, m::PX_2),
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
            CHIP_X + LABEL_X_OFFSET,
            CHIP_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            palette.text,
            "filter: docs",
        ),
        TextSpec::new(
            CHIP_X + DISMISS_X_OFFSET - m::PX_6,
            CHIP_Y + CHIP_HEIGHT + m::PX_10,
            m::FONT_7,
            palette.muted,
            dismiss_label(scenario),
        ),
        tone_label(palette, m::PX_0, "accent"),
        tone_label(palette, m::PX_1, "danger"),
        tone_label(palette, m::PX_2, "muted"),
        TextSpec::new(
            SURFACE_TOKEN_X + STATUS_TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X + STATUS_TEXT_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}

fn tone_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, index: usize) -> Block {
    Block::outlined(
        TONE_X + ((TONE_WIDTH + TONE_GAP) * index),
        TONE_Y,
        TONE_WIDTH,
        TONE_HEIGHT,
        tone_fill(palette, scenario, index),
    )
}

fn tone_label(palette: &VisualPalette, index: usize, label: &'static str) -> TextSpec {
    TextSpec::new(
        TONE_X + ((TONE_WIDTH + TONE_GAP) * index) + m::PX_8,
        TONE_Y + TONE_LABEL_Y_OFFSET,
        m::FONT_7,
        palette.text,
        label,
    )
}
