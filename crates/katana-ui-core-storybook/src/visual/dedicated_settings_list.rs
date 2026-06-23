use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_settings_list_style::{
    control_fill, dirty_marker_fill, field_label, query_fill, section_fill, section_label,
    status_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const SETTINGS_X: usize = 30;
pub(super) const SETTINGS_Y: usize = 28;
const LIST_WIDTH: usize = 298;
const LIST_HEIGHT: usize = 104;
const QUERY_X: usize = 42;
const QUERY_Y: usize = 38;
const QUERY_WIDTH: usize = 170;
const QUERY_HEIGHT: usize = 18;
const SECTION_X: usize = 42;
const SECTION_Y: usize = 64;
const SECTION_WIDTH: usize = 86;
const SECTION_HEIGHT: usize = 52;
const SECTION_GAP: usize = 92;
const FIELD_X: usize = 52;
const FIELD_Y: usize = 86;
const CONTROL_X: usize = 250;
const CONTROL_Y: usize = 82;
const CONTROL_WIDTH: usize = 48;
const CONTROL_HEIGHT: usize = 22;
const DIRTY_X: usize = 42;
const DIRTY_Y: usize = 122;
const DIRTY_WIDTH: usize = 222;
const DIRTY_HEIGHT: usize = 4;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const FIELD_TEXT_Y_OFFSET: usize = 6;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 10;
const LABEL_COUNT: usize = 7;

pub(super) fn settings_list(
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
        "Settings list",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            SETTINGS_X,
            SETTINGS_Y,
            LIST_WIDTH,
            LIST_HEIGHT,
            palette.surface,
        ),
        Block::outlined(
            QUERY_X,
            QUERY_Y,
            QUERY_WIDTH,
            QUERY_HEIGHT,
            query_fill(palette, scenario),
        ),
        section_block(palette, scenario, m::PX_0),
        section_block(palette, scenario, m::PX_1),
        section_block(palette, scenario, m::PX_2),
        Block::outlined(
            CONTROL_X,
            CONTROL_Y,
            CONTROL_WIDTH,
            CONTROL_HEIGHT,
            control_fill(palette, scenario),
        ),
        Block::new(
            DIRTY_X,
            DIRTY_Y,
            DIRTY_WIDTH,
            DIRTY_HEIGHT,
            dirty_marker_fill(scenario),
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
            CONTROL_X + m::PX_6,
            CONTROL_Y + m::PX_8,
            m::PX_30,
            m::PX_6,
            palette.background,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            QUERY_X + TEXT_X_OFFSET,
            QUERY_Y + FIELD_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "settings query",
        ),
        TextSpec::new(
            SECTION_X + TEXT_X_OFFSET,
            SECTION_Y + FIELD_TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            section_label(scenario),
        ),
        TextSpec::new(
            FIELD_X,
            FIELD_Y,
            m::FONT_7,
            palette.text,
            field_label(scenario),
        ),
        TextSpec::new(
            CONTROL_X + TEXT_X_OFFSET,
            CONTROL_Y + FIELD_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            "edit",
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
            DIRTY_X + TEXT_X_OFFSET,
            DIRTY_Y + m::PX_6,
            m::FONT_7,
            palette.muted,
            "dirty / reset contract",
        ),
    ]
}

fn section_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, section: usize) -> Block {
    Block::outlined(
        SECTION_X + (SECTION_GAP * section),
        SECTION_Y,
        SECTION_WIDTH,
        SECTION_HEIGHT,
        section_fill(palette, scenario, section),
    )
}
