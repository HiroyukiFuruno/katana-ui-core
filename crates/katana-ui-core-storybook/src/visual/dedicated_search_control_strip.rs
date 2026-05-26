use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_search_control_strip_style::{
    navigation_fill, option_fill, query_fill, query_label, replace_fill, replace_label,
    result_label, status_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const SEARCH_CONTROL_X: usize = 30;
pub(super) const SEARCH_CONTROL_Y: usize = 34;
const STRIP_WIDTH: usize = 300;
const STRIP_HEIGHT: usize = 86;
const QUERY_X: usize = 42;
const QUERY_Y: usize = 54;
const QUERY_WIDTH: usize = 120;
const FIELD_HEIGHT: usize = 22;
const COUNT_X: usize = 170;
const COUNT_WIDTH: usize = 42;
const NAV_X: usize = 220;
const NAV_WIDTH: usize = 26;
const NAV_GAP: usize = 30;
const OPTION_X: usize = 42;
const OPTION_Y: usize = 88;
const OPTION_WIDTH: usize = 42;
const OPTION_GAP: usize = 48;
const REPLACE_X: usize = 194;
const REPLACE_WIDTH: usize = 92;
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
const BLOCK_COUNT: usize = 12;
const LABEL_COUNT: usize = 10;

pub(super) fn search_control_strip(
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
        "Search control strip",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            SEARCH_CONTROL_X,
            SEARCH_CONTROL_Y,
            STRIP_WIDTH,
            STRIP_HEIGHT,
            palette.panel,
        ),
        Block::outlined(
            QUERY_X,
            QUERY_Y,
            QUERY_WIDTH,
            FIELD_HEIGHT,
            query_fill(palette, scenario),
        ),
        Block::outlined(COUNT_X, QUERY_Y, COUNT_WIDTH, FIELD_HEIGHT, palette.surface),
        nav_block(palette, scenario, m::PX_0),
        nav_block(palette, scenario, m::PX_1),
        option_block(palette, scenario, m::PX_0),
        option_block(palette, scenario, m::PX_1),
        option_block(palette, scenario, m::PX_2),
        Block::outlined(
            REPLACE_X,
            OPTION_Y,
            REPLACE_WIDTH,
            FIELD_HEIGHT,
            replace_fill(palette, scenario),
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
            QUERY_X + QUERY_WIDTH - m::PX_10,
            QUERY_Y + m::PX_6,
            m::PX_2,
            m::PX_12,
            palette.accent,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            QUERY_X + TEXT_X_OFFSET,
            QUERY_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            query_label(scenario),
        ),
        TextSpec::new(
            COUNT_X + m::PX_6,
            QUERY_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            result_label(scenario),
        ),
        TextSpec::new(
            NAV_X + m::PX_8,
            QUERY_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "<",
        ),
        TextSpec::new(
            NAV_X + NAV_GAP + m::PX_8,
            QUERY_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            ">",
        ),
        TextSpec::new(
            OPTION_X + TEXT_X_OFFSET,
            OPTION_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "Aa",
        ),
        TextSpec::new(
            OPTION_X + OPTION_GAP + TEXT_X_OFFSET,
            OPTION_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "W",
        ),
        TextSpec::new(
            OPTION_X + (OPTION_GAP * m::PX_2) + TEXT_X_OFFSET,
            OPTION_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            ".*",
        ),
        TextSpec::new(
            REPLACE_X + TEXT_X_OFFSET,
            OPTION_Y + TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            replace_label(scenario),
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
    ]
}

fn nav_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, index: usize) -> Block {
    Block::outlined(
        NAV_X + (NAV_GAP * index),
        QUERY_Y,
        NAV_WIDTH,
        FIELD_HEIGHT,
        navigation_fill(palette, scenario),
    )
}

fn option_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, index: usize) -> Block {
    Block::outlined(
        OPTION_X + (OPTION_GAP * index),
        OPTION_Y,
        OPTION_WIDTH,
        FIELD_HEIGHT,
        option_fill(palette, scenario),
    )
}
