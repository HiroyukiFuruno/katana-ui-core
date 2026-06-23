use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_list_style::{
    list_fill, row_fill, row_label, row_text, scrollbar_fill, state_label, theme_line_fill,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const LIST_X: usize = 34;
pub(super) const LIST_Y: usize = 30;
const LIST_WIDTH: usize = 278;
const LIST_HEIGHT: usize = 86;
const ROW_X: usize = LIST_X + 12;
const FIRST_ROW_Y: usize = LIST_Y + 22;
const ROW_WIDTH: usize = LIST_WIDTH - 36;
const ROW_HEIGHT: usize = 17;
const ROW_GAP: usize = 6;
const SCROLLBAR_X: usize = LIST_X + LIST_WIDTH - 12;
const SCROLLBAR_WIDTH: usize = 5;
const SCROLLBAR_HEIGHT: usize = 38;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 128;
const STATUS_HEIGHT: usize = 20;
const LINE_HEIGHT: usize = 3;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 5;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 7;
const LABEL_COUNT: usize = 5;

pub(super) fn list(
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
        "List",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            LIST_X,
            LIST_Y,
            LIST_WIDTH,
            LIST_HEIGHT,
            list_fill(palette, scenario),
        ),
        Block::new(
            LIST_X,
            LIST_Y,
            LIST_WIDTH,
            LINE_HEIGHT,
            theme_line_fill(palette, scenario),
        ),
        row_block(palette, scenario, m::PX_0),
        row_block(palette, scenario, m::PX_1),
        row_block(palette, scenario, m::PX_2),
        Block::new(
            SCROLLBAR_X,
            FIRST_ROW_Y,
            SCROLLBAR_WIDTH,
            SCROLLBAR_HEIGHT,
            scrollbar_fill(palette, scenario),
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
            LIST_X + LABEL_X_OFFSET,
            LIST_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "collection",
        ),
        row_text_spec(palette, scenario, m::PX_0),
        row_text_spec(palette, scenario, m::PX_1),
        row_text_spec(palette, scenario, m::PX_2),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}

fn row_block(palette: &VisualPalette, scenario: ScenarioContext<'_>, index: usize) -> Block {
    Block::outlined(
        ROW_X,
        FIRST_ROW_Y + ((ROW_HEIGHT + ROW_GAP) * index),
        ROW_WIDTH,
        ROW_HEIGHT,
        row_fill(palette, scenario, index),
    )
}

fn row_text_spec(palette: &VisualPalette, scenario: ScenarioContext<'_>, index: usize) -> TextSpec {
    TextSpec::new(
        ROW_X + LABEL_X_OFFSET,
        FIRST_ROW_Y + ((ROW_HEIGHT + ROW_GAP) * index) + LABEL_Y_OFFSET,
        m::FONT_7,
        row_text(palette, scenario, index),
        row_label(scenario, index),
    )
}
