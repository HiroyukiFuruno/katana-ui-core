use super::canvas::Canvas;
use super::dedicated_card_style::{
    active_slot_index, badge_fill, border_line_fill, card_fill, header_fill, slot_fill, slot_text,
    state_label,
};
use super::dedicated_dod_common::{self as common, Block, ChipSpec, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const CARD_X: usize = 22;
pub(super) const CARD_Y: usize = 30;
const CARD_WIDTH: usize = 286;
const CARD_HEIGHT: usize = 78;
const HEADER_HEIGHT: usize = 20;
const SLOT_X: usize = CARD_X + 16;
const SLOT_Y: usize = CARD_Y + 30;
const SLOT_WIDTH: usize = 82;
const SLOT_HEIGHT: usize = 18;
const SLOT_GAP: usize = 8;
const STATUS_X: usize = 330;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 142;
const STATUS_HEIGHT: usize = 20;
const LINE_HEIGHT: usize = 3;
const CHIP_Y: usize = SLOT_Y + 24;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 6;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 7;
const LABEL_COUNT: usize = 6;
const CHIP_COUNT: usize = 2;

pub(super) fn card(
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
        "Card slots",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
    common::draw_chips(canvas, text, palette, x, y, &chips(palette, scenario));
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let active = active_slot_index(scenario);
    [
        Block::outlined(
            CARD_X,
            CARD_Y,
            CARD_WIDTH,
            CARD_HEIGHT,
            card_fill(palette, scenario),
        ),
        Block::new(
            CARD_X,
            CARD_Y,
            CARD_WIDTH,
            HEADER_HEIGHT,
            header_fill(palette, scenario),
        ),
        Block::new(
            CARD_X,
            CARD_Y,
            CARD_WIDTH,
            LINE_HEIGHT,
            border_line_fill(palette, scenario),
        ),
        slot_block(palette, active, m::PX_0),
        slot_block(palette, active, m::PX_1),
        slot_block(palette, active, m::PX_2),
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
    let active = active_slot_index(scenario);
    [
        TextSpec::new(
            CARD_X + LABEL_X_OFFSET,
            CARD_Y + LABEL_Y_OFFSET,
            m::FONT_9,
            palette.text,
            "Header + Badge",
        ),
        slot_label(palette, active, m::PX_0, "title"),
        slot_label(palette, active, m::PX_1, "body"),
        slot_label(palette, active, m::PX_2, "actions"),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            CARD_X + CARD_WIDTH + m::PX_18,
            CARD_Y + m::PX_20,
            m::FONT_8,
            palette.muted,
            "child state isolated",
        ),
    ]
}

fn chips(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [ChipSpec; CHIP_COUNT] {
    [
        ChipSpec::new(
            SLOT_X,
            CHIP_Y,
            m::PX_58,
            m::PX_18,
            "new",
            badge_fill(scenario),
        ),
        ChipSpec::new(
            SLOT_X + m::PX_66,
            CHIP_Y,
            m::PX_56,
            m::PX_20,
            "Save",
            palette.accent,
        ),
    ]
}

fn slot_block(palette: &VisualPalette, active: usize, index: usize) -> Block {
    Block::outlined(
        SLOT_X + ((SLOT_WIDTH + SLOT_GAP) * index),
        SLOT_Y,
        SLOT_WIDTH,
        SLOT_HEIGHT,
        slot_fill(palette, active, index),
    )
}

fn slot_label(
    palette: &VisualPalette,
    active: usize,
    index: usize,
    value: &'static str,
) -> TextSpec {
    TextSpec::new(
        SLOT_X + ((SLOT_WIDTH + SLOT_GAP) * index) + LABEL_X_OFFSET,
        SLOT_Y + LABEL_Y_OFFSET,
        m::FONT_7,
        slot_text(palette, active, index),
        value,
    )
}
