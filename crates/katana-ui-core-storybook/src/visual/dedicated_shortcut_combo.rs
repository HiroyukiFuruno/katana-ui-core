use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_shortcut_combo_style::{
    key_fill, modifier_label, platform_label, separator_fill, separator_label, status_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const SHORTCUT_X: usize = 42;
pub(super) const SHORTCUT_Y: usize = 52;
const KEY_WIDTH: usize = 64;
const KEY_HEIGHT: usize = 28;
const KEY_GAP: usize = 84;
const SEP_X: usize = 116;
const LABEL_X: usize = 42;
const LABEL_Y: usize = 96;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 10;
const TEXT_Y_OFFSET: usize = 9;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 6;
const LABEL_COUNT: usize = 7;

pub(super) fn shortcut_combo(
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
        "Shortcut combo",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            SHORTCUT_X,
            SHORTCUT_Y,
            KEY_WIDTH,
            KEY_HEIGHT,
            key_fill(palette, scenario),
        ),
        Block::outlined(
            SHORTCUT_X + KEY_GAP,
            SHORTCUT_Y,
            KEY_WIDTH,
            KEY_HEIGHT,
            key_fill(palette, scenario),
        ),
        Block::outlined(
            SEP_X,
            SHORTCUT_Y + m::PX_8,
            m::PX_18,
            m::PX_12,
            separator_fill(palette, scenario),
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
            SHORTCUT_X,
            SHORTCUT_Y + KEY_HEIGHT + m::PX_10,
            KEY_WIDTH + KEY_GAP,
            m::PX_4,
            palette.accent,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            SHORTCUT_X + TEXT_X_OFFSET,
            SHORTCUT_Y + TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            modifier_label(scenario),
        ),
        TextSpec::new(
            SHORTCUT_X + KEY_GAP + TEXT_X_OFFSET,
            SHORTCUT_Y + TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            "K",
        ),
        TextSpec::new(
            SEP_X + m::PX_6,
            SHORTCUT_Y + TEXT_Y_OFFSET,
            m::FONT_8,
            palette.muted,
            separator_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            LABEL_Y,
            m::FONT_8,
            palette.muted,
            platform_label(scenario),
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
            LABEL_X,
            LABEL_Y + m::PX_16,
            m::FONT_7,
            palette.muted,
            "Open command palette",
        ),
    ]
}
