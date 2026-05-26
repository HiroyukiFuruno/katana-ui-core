use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_virtualization_style::{
    correction_fill, row_fill, row_text, state_label, viewport_fill,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const VIEWPORT_X: usize = 36;
pub(super) const VIEWPORT_Y: usize = 30;
const VIEWPORT_WIDTH: usize = 312;
const VIEWPORT_HEIGHT: usize = 86;
const ROW_X: usize = VIEWPORT_X + 14;
const FIRST_ROW_Y: usize = VIEWPORT_Y + 18;
const ROW_WIDTH: usize = 238;
const ROW_HEIGHT: usize = 16;
const ROW_GAP: usize = 6;
const SCROLLBAR_X: usize = VIEWPORT_X + VIEWPORT_WIDTH - 14;
const SCROLLBAR_WIDTH: usize = 6;
const SCROLLBAR_HEIGHT: usize = 52;
const THUMB_HEIGHT: usize = 20;
const CORRECTION_WIDTH: usize = 44;
const CORRECTION_HEIGHT: usize = 8;
const STATUS_X: usize = 374;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 118;
const STATUS_HEIGHT: usize = 20;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 5;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 8;
const LABEL_COUNT: usize = 5;

pub(super) fn virtualization(
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
        "Virtualization",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            VIEWPORT_X,
            VIEWPORT_Y,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            viewport_fill(palette, scenario),
        ),
        row_block(palette, scenario, m::PX_0),
        row_block(palette, scenario, m::PX_1),
        row_block(palette, scenario, m::PX_2),
        Block::outlined(
            SCROLLBAR_X,
            FIRST_ROW_Y,
            SCROLLBAR_WIDTH,
            SCROLLBAR_HEIGHT,
            palette.border,
        ),
        Block::new(
            SCROLLBAR_X,
            FIRST_ROW_Y + m::PX_18,
            SCROLLBAR_WIDTH,
            THUMB_HEIGHT,
            palette.accent,
        ),
        Block::new(
            ROW_X + ROW_WIDTH - CORRECTION_WIDTH,
            FIRST_ROW_Y + ((ROW_HEIGHT + ROW_GAP) * m::PX_2) + ROW_HEIGHT + m::PX_2,
            CORRECTION_WIDTH,
            CORRECTION_HEIGHT,
            correction_fill(palette, scenario),
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
            VIEWPORT_X + LABEL_X_OFFSET,
            VIEWPORT_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "viewport 1260",
        ),
        row_text_spec(palette, scenario, m::PX_0, "row 40"),
        row_text_spec(palette, scenario, m::PX_1, "row 41"),
        row_text_spec(palette, scenario, m::PX_2, "row 42"),
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

fn row_text_spec(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    index: usize,
    value: &'static str,
) -> TextSpec {
    TextSpec::new(
        ROW_X + LABEL_X_OFFSET,
        FIRST_ROW_Y + ((ROW_HEIGHT + ROW_GAP) * index) + LABEL_Y_OFFSET,
        m::FONT_7,
        row_text(palette, scenario, index),
        value,
    )
}
