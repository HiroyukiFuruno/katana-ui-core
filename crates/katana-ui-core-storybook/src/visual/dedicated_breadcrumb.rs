use super::canvas::Canvas;
use super::dedicated_breadcrumb_style::{
    active_index, bar_fill, crumb_fill, crumb_text, overflow_fill, state_label, theme_line_fill,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const BAR_X: usize = 38;
pub(super) const BAR_Y: usize = 46;
pub(super) const BAR_WIDTH: usize = 404;
const BAR_HEIGHT: usize = 34;
const ROOT_X: usize = 54;
const SRC_X: usize = 150;
const FILE_X: usize = 246;
const CRUMB_Y: usize = 52;
const CRUMB_WIDTH: usize = 78;
const CRUMB_HEIGHT: usize = 22;
const OVERFLOW_X: usize = 344;
const OVERFLOW_Y: usize = 52;
const OVERFLOW_WIDTH: usize = 64;
const OVERFLOW_HEIGHT: usize = 22;
const STATUS_X: usize = 54;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 170;
const STATUS_HEIGHT: usize = 20;
const LINE_Y: usize = CRUMB_Y + CRUMB_HEIGHT - 3;
const LINE_HEIGHT: usize = 3;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 7;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const PATH_LABEL_X_OFFSET: usize = 92;
const BLOCK_COUNT: usize = 8;
const LABEL_COUNT: usize = 6;

pub(super) fn breadcrumb(
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
        "Breadcrumb",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let active = active_index(scenario);
    [
        Block::outlined(
            BAR_X,
            BAR_Y,
            BAR_WIDTH,
            BAR_HEIGHT,
            bar_fill(palette, scenario),
        ),
        Block::outlined(
            ROOT_X,
            CRUMB_Y,
            CRUMB_WIDTH,
            CRUMB_HEIGHT,
            crumb_fill(palette, active, m::PX_0),
        ),
        Block::outlined(
            SRC_X,
            CRUMB_Y,
            CRUMB_WIDTH,
            CRUMB_HEIGHT,
            crumb_fill(palette, active, m::PX_1),
        ),
        Block::outlined(
            FILE_X,
            CRUMB_Y,
            CRUMB_WIDTH,
            CRUMB_HEIGHT,
            crumb_fill(palette, active, m::PX_2),
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
            CRUMB_WIDTH,
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
            BAR_X,
            BAR_Y,
            BAR_WIDTH,
            LINE_HEIGHT,
            theme_line_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    let active = active_index(scenario);
    [
        TextSpec::new(
            ROOT_X + LABEL_X_OFFSET,
            CRUMB_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            crumb_text(palette, active, m::PX_0),
            "Root",
        ),
        TextSpec::new(
            SRC_X + LABEL_X_OFFSET,
            CRUMB_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            crumb_text(palette, active, m::PX_1),
            "src",
        ),
        TextSpec::new(
            FILE_X + LABEL_X_OFFSET,
            CRUMB_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            crumb_text(palette, active, m::PX_2),
            "lib.rs",
        ),
        TextSpec::new(
            OVERFLOW_X + LABEL_X_OFFSET,
            OVERFLOW_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "more",
        ),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            BAR_X + BAR_WIDTH - PATH_LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "path nav",
        ),
    ]
}

fn active_line_x(active: usize) -> usize {
    ROOT_X + (CRUMB_WIDTH + m::PX_18) * active
}
