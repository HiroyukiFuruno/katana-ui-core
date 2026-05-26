use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_tabs_style::{
    active_index, overflow_fill, panel_fill, panel_label, state_label, tab_fill, tab_text,
    theme_line_fill,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const TAB_X: usize = 42;
pub(super) const TAB_Y: usize = 38;
const TAB_WIDTH: usize = 92;
const TAB_HEIGHT: usize = 28;
const SECOND_TAB_X: usize = TAB_X + TAB_WIDTH;
const THIRD_TAB_X: usize = SECOND_TAB_X + TAB_WIDTH;
const OVERFLOW_X: usize = THIRD_TAB_X + TAB_WIDTH + 12;
const OVERFLOW_Y: usize = TAB_Y + 5;
const OVERFLOW_WIDTH: usize = 46;
const OVERFLOW_HEIGHT: usize = 18;
pub(super) const PANEL_X: usize = 42;
pub(super) const PANEL_Y: usize = 70;
pub(super) const PANEL_WIDTH: usize = 390;
const PANEL_HEIGHT: usize = 48;
const STATUS_X: usize = 328;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 92;
const STATUS_HEIGHT: usize = 18;
const LINE_Y: usize = TAB_Y + TAB_HEIGHT - 3;
const LINE_HEIGHT: usize = 3;
const LABEL_X_OFFSET: usize = 10;
const LABEL_Y_OFFSET: usize = 9;
const OVERFLOW_LABEL_Y_OFFSET: usize = 5;
const PANEL_TEXT_X: usize = PANEL_X + 14;
const PANEL_TEXT_Y: usize = PANEL_Y + 17;
const STATUS_TEXT_X: usize = STATUS_X + 7;
const STATUS_TEXT_Y: usize = STATUS_Y + 5;
const BLOCK_COUNT: usize = 8;
const LABEL_COUNT: usize = 6;

pub(super) fn tabs(
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
        "Tabs",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let active = active_index(scenario);
    [
        Block::outlined(
            PANEL_X,
            PANEL_Y,
            PANEL_WIDTH,
            PANEL_HEIGHT,
            panel_fill(palette, scenario),
        ),
        Block::outlined(
            TAB_X,
            TAB_Y,
            TAB_WIDTH,
            TAB_HEIGHT,
            tab_fill(palette, active, m::PX_0),
        ),
        Block::outlined(
            SECOND_TAB_X,
            TAB_Y,
            TAB_WIDTH,
            TAB_HEIGHT,
            tab_fill(palette, active, m::PX_1),
        ),
        Block::outlined(
            THIRD_TAB_X,
            TAB_Y,
            TAB_WIDTH,
            TAB_HEIGHT,
            tab_fill(palette, active, m::PX_2),
        ),
        Block::outlined(
            OVERFLOW_X,
            OVERFLOW_Y,
            OVERFLOW_WIDTH,
            OVERFLOW_HEIGHT,
            overflow_fill(palette, scenario),
        ),
        Block::new(
            active_line_x(active),
            LINE_Y,
            TAB_WIDTH,
            LINE_HEIGHT,
            theme_line_fill(palette, scenario),
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
        Block::new(
            PANEL_X,
            PANEL_Y,
            PANEL_WIDTH,
            LINE_HEIGHT,
            theme_line_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    let active = active_index(scenario);
    [
        TextSpec::new(
            TAB_X + LABEL_X_OFFSET,
            TAB_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            tab_text(palette, active, m::PX_0),
            "Preview",
        ),
        TextSpec::new(
            SECOND_TAB_X + LABEL_X_OFFSET,
            TAB_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            tab_text(palette, active, m::PX_1),
            "Output",
        ),
        TextSpec::new(
            THIRD_TAB_X + LABEL_X_OFFSET,
            TAB_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            tab_text(palette, active, m::PX_2),
            "Settings",
        ),
        TextSpec::new(
            OVERFLOW_X + LABEL_X_OFFSET,
            OVERFLOW_Y + OVERFLOW_LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "more",
        ),
        TextSpec::new(
            PANEL_TEXT_X,
            PANEL_TEXT_Y,
            m::FONT_9,
            tab_text(palette, active, active),
            panel_label(scenario),
        ),
        TextSpec::new(
            STATUS_TEXT_X,
            STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}

fn active_line_x(active: usize) -> usize {
    TAB_X + (TAB_WIDTH * active)
}
