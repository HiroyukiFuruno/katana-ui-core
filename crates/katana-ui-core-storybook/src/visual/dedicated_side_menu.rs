use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_side_menu_style::{
    active_row_index, collapse_fill, row_fill, row_text, state_label, theme_line_fill,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const PANEL_X: usize = 32;
pub(super) const PANEL_Y: usize = 28;
pub(super) const PANEL_WIDTH: usize = 220;
const PANEL_HEIGHT: usize = 96;
const ROW_X: usize = PANEL_X + 12;
const FIRST_ROW_Y: usize = PANEL_Y + 26;
const ROW_WIDTH: usize = PANEL_WIDTH - 24;
const ROW_HEIGHT: usize = 22;
const ROW_GAP: usize = 8;
const COLLAPSE_X: usize = 268;
const COLLAPSE_Y: usize = 28;
const COLLAPSE_WIDTH: usize = 40;
const COLLAPSE_HEIGHT: usize = 96;
const STATUS_X: usize = 326;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 112;
const STATUS_HEIGHT: usize = 20;
const LINE_HEIGHT: usize = 3;
const LABEL_X_OFFSET: usize = 10;
const LABEL_Y_OFFSET: usize = 7;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 6;
const LABEL_COUNT: usize = 5;

pub(super) fn side_menu(
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
        "Side menu",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let active = active_row_index(scenario);
    [
        Block::outlined(PANEL_X, PANEL_Y, PANEL_WIDTH, PANEL_HEIGHT, palette.surface),
        Block::new(
            PANEL_X,
            PANEL_Y,
            PANEL_WIDTH,
            LINE_HEIGHT,
            theme_line_fill(palette, scenario),
        ),
        Block::outlined(
            ROW_X,
            FIRST_ROW_Y,
            ROW_WIDTH,
            ROW_HEIGHT,
            row_fill(palette, scenario, active, m::PX_0),
        ),
        Block::outlined(
            ROW_X,
            FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP,
            ROW_WIDTH,
            ROW_HEIGHT,
            row_fill(palette, scenario, active, m::PX_1),
        ),
        Block::outlined(
            COLLAPSE_X,
            COLLAPSE_Y,
            COLLAPSE_WIDTH,
            COLLAPSE_HEIGHT,
            collapse_fill(palette, scenario),
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
    let active = active_row_index(scenario);
    [
        TextSpec::new(
            ROW_X + LABEL_X_OFFSET,
            FIRST_ROW_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            row_text(palette, active, m::PX_0),
            "Files",
        ),
        TextSpec::new(
            ROW_X + LABEL_X_OFFSET,
            FIRST_ROW_Y + ROW_HEIGHT + ROW_GAP + LABEL_Y_OFFSET,
            m::FONT_8,
            row_text(palette, active, m::PX_1),
            "Settings",
        ),
        TextSpec::new(
            COLLAPSE_X + LABEL_X_OFFSET,
            COLLAPSE_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "collapse",
        ),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            PANEL_X + LABEL_X_OFFSET,
            PANEL_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "nav tree",
        ),
    ]
}
