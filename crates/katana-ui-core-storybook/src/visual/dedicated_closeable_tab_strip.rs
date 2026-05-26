use super::canvas::Canvas;
use super::dedicated_closeable_tab_strip_style::{
    active_index, dirty_fill, overflow_fill, state_label, strip_fill, tab_fill, tab_text,
    theme_line_fill,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const STRIP_X: usize = 30;
pub(super) const STRIP_Y: usize = 42;
pub(super) const STRIP_WIDTH: usize = 450;
const STRIP_HEIGHT: usize = 36;
const TAB_Y: usize = 48;
const TAB_WIDTH: usize = 78;
const TAB_HEIGHT: usize = 24;
const TAB_GAP: usize = 4;
const FIRST_TAB_X: usize = 42;
const SECOND_TAB_X: usize = FIRST_TAB_X + TAB_WIDTH + TAB_GAP;
const THIRD_TAB_X: usize = SECOND_TAB_X + TAB_WIDTH + TAB_GAP;
const FOURTH_TAB_X: usize = THIRD_TAB_X + TAB_WIDTH + TAB_GAP;
const FIFTH_TAB_X: usize = FOURTH_TAB_X + TAB_WIDTH + TAB_GAP;
const OVERFLOW_X: usize = FIFTH_TAB_X + TAB_WIDTH + 10;
const OVERFLOW_WIDTH: usize = 36;
const DIRTY_SIZE: usize = 7;
const STATUS_X: usize = 42;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 178;
const STATUS_HEIGHT: usize = 20;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 7;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const LINE_HEIGHT: usize = 3;
const BLOCK_COUNT: usize = 10;
const LABEL_COUNT: usize = 7;

pub(super) fn closeable_tab_strip(
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
        "Closeable tab strip",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let active = active_index(scenario);
    [
        Block::outlined(
            STRIP_X,
            STRIP_Y,
            STRIP_WIDTH,
            STRIP_HEIGHT,
            strip_fill(palette, scenario),
        ),
        tab_block(palette, active, m::PX_0, FIRST_TAB_X),
        tab_block(palette, active, m::PX_1, SECOND_TAB_X),
        tab_block(palette, active, m::PX_2, THIRD_TAB_X),
        tab_block(palette, active, m::PX_3, FOURTH_TAB_X),
        tab_block(palette, active, m::PX_4, FIFTH_TAB_X),
        Block::outlined(
            OVERFLOW_X,
            TAB_Y,
            OVERFLOW_WIDTH,
            TAB_HEIGHT,
            overflow_fill(palette, scenario),
        ),
        Block::new(
            FOURTH_TAB_X + TAB_WIDTH - DIRTY_SIZE - m::PX_6,
            TAB_Y + m::PX_6,
            DIRTY_SIZE,
            DIRTY_SIZE,
            dirty_fill(scenario),
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
        Block::new(
            STRIP_X,
            STRIP_Y,
            STRIP_WIDTH,
            LINE_HEIGHT,
            theme_line_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    let active = active_index(scenario);
    [
        tab_label(palette, active, m::PX_0, FIRST_TAB_X, "default"),
        tab_label(palette, active, m::PX_1, SECOND_TAB_X, "pinned"),
        tab_label(palette, active, m::PX_2, THIRD_TAB_X, "groups"),
        tab_label(palette, active, m::PX_3, FOURTH_TAB_X, "dirty"),
        tab_label(palette, active, m::PX_4, FIFTH_TAB_X, "dragging"),
        TextSpec::new(
            OVERFLOW_X + LABEL_X_OFFSET,
            TAB_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "...",
        ),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}

fn tab_block(palette: &VisualPalette, active: usize, index: usize, x: usize) -> Block {
    Block::outlined(
        x,
        TAB_Y,
        TAB_WIDTH,
        TAB_HEIGHT,
        tab_fill(palette, active, index),
    )
}

fn tab_label(
    palette: &VisualPalette,
    active: usize,
    index: usize,
    x: usize,
    value: &'static str,
) -> TextSpec {
    TextSpec::new(
        x + LABEL_X_OFFSET,
        TAB_Y + LABEL_Y_OFFSET,
        m::FONT_7,
        tab_text(palette, active, index),
        value,
    )
}
