use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_shortcut_cheatsheet_style::{
    filter_fill, filter_label, layout_label, layout_rail_fill, primary_row_fill, result_label,
    secondary_panel_fill, state_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const FILTER_X: usize = 42;
pub(super) const FILTER_Y: usize = 36;
const FILTER_WIDTH: usize = 176;
const FILTER_HEIGHT: usize = 20;
const LEFT_GROUP_X: usize = 42;
const RIGHT_GROUP_X: usize = 270;
const GROUP_Y: usize = 66;
const GROUP_WIDTH: usize = 206;
const GROUP_HEIGHT: usize = 48;
const ROW_X_OFFSET: usize = 8;
const ROW_Y_OFFSET: usize = 24;
const ROW_WIDTH: usize = 188;
const ROW_HEIGHT: usize = 16;
const RAIL_X: usize = 254;
const RAIL_WIDTH: usize = 4;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const FILTER_TEXT_Y_OFFSET: usize = 7;
const GROUP_TITLE_Y_OFFSET: usize = 8;
const ROW_TEXT_Y_OFFSET: usize = 28;
const STATUS_X: usize = 42;
const STATUS_Y: usize = 118;
const STATUS_GAP: usize = 112;
const STATE_STATUS_X: usize = STATUS_X + STATUS_GAP * 2;
const SHORTCUT_STATUS_INDEX: usize = 3;
const SHORTCUT_STATUS_X: usize = STATUS_X + STATUS_GAP * SHORTCUT_STATUS_INDEX;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 7;
const LABEL_COUNT: usize = 10;

pub(super) fn shortcut_cheatsheet(
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
        "Shortcut cheatsheet",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            FILTER_X,
            FILTER_Y,
            FILTER_WIDTH,
            FILTER_HEIGHT,
            filter_fill(palette, scenario),
        ),
        Block::outlined(
            SURFACE_TOKEN_X,
            SURFACE_TOKEN_Y,
            SURFACE_TOKEN_WIDTH,
            SURFACE_TOKEN_HEIGHT,
            palette.surface,
        ),
        Block::outlined(
            LEFT_GROUP_X,
            GROUP_Y,
            GROUP_WIDTH,
            GROUP_HEIGHT,
            palette.panel,
        ),
        Block::outlined(
            RIGHT_GROUP_X,
            GROUP_Y,
            GROUP_WIDTH,
            GROUP_HEIGHT,
            secondary_panel_fill(palette, scenario),
        ),
        Block::new(
            LEFT_GROUP_X + ROW_X_OFFSET,
            GROUP_Y + ROW_Y_OFFSET,
            ROW_WIDTH,
            ROW_HEIGHT,
            primary_row_fill(palette, scenario),
        ),
        Block::new(
            RIGHT_GROUP_X + ROW_X_OFFSET,
            GROUP_Y + ROW_Y_OFFSET,
            ROW_WIDTH,
            ROW_HEIGHT,
            palette.surface,
        ),
        Block::new(
            RAIL_X,
            GROUP_Y,
            RAIL_WIDTH,
            GROUP_HEIGHT,
            layout_rail_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            FILTER_X + TEXT_X_OFFSET,
            FILTER_Y + FILTER_TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            filter_label(scenario),
        ),
        TextSpec::new(
            SURFACE_TOKEN_X + TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            LEFT_GROUP_X + TEXT_X_OFFSET,
            GROUP_Y + GROUP_TITLE_Y_OFFSET,
            m::FONT_8,
            palette.muted,
            "Editing",
        ),
        TextSpec::new(
            LEFT_GROUP_X + TEXT_X_OFFSET,
            GROUP_Y + ROW_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "Format document",
        ),
        TextSpec::new(
            RIGHT_GROUP_X + TEXT_X_OFFSET,
            GROUP_Y + GROUP_TITLE_Y_OFFSET,
            m::FONT_8,
            palette.muted,
            "Navigation",
        ),
        TextSpec::new(
            RIGHT_GROUP_X + TEXT_X_OFFSET,
            GROUP_Y + ROW_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "Command palette",
        ),
        TextSpec::new(
            STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            layout_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_GAP,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            result_label(scenario),
        ),
        TextSpec::new(
            STATE_STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            SHORTCUT_STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            "Cmd+F",
        ),
    ]
}
