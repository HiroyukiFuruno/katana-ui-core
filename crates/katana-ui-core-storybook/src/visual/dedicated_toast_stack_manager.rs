use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_toast_stack_manager_labels;
use super::dedicated_toast_stack_manager_style::{
    action_fill, lower_toast_fill, middle_toast_fill, pause_fill, position_fill, queue_fill,
    stack_panel_fill, top_toast_fill,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const POSITION_X: usize = 28;
const POSITION_Y: usize = 34;
const POSITION_WIDTH: usize = 132;
const POSITION_HEIGHT: usize = 20;
const PAUSE_X: usize = 28;
const PAUSE_Y: usize = 64;
const PAUSE_WIDTH: usize = 132;
const PAUSE_HEIGHT: usize = 20;
pub(super) const STATUS_X: usize = 28;
pub(super) const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 132;
const STATUS_HEIGHT: usize = 20;
const STACK_PANEL_X: usize = 190;
const STACK_PANEL_Y: usize = 24;
const STACK_PANEL_WIDTH: usize = 292;
const STACK_PANEL_HEIGHT: usize = 104;
pub(super) const TOP_TOAST_X: usize = 232;
pub(super) const TOP_TOAST_Y: usize = 32;
pub(super) const TOAST_WIDTH: usize = 220;
const TOAST_HEIGHT: usize = 24;
const SECOND_TOAST_X: usize = 214;
const SECOND_TOAST_Y: usize = 60;
const THIRD_TOAST_X: usize = 196;
const THIRD_TOAST_Y: usize = 88;
const QUEUE_X: usize = 202;
const QUEUE_Y: usize = 116;
const QUEUE_WIDTH: usize = 144;
const QUEUE_HEIGHT: usize = 10;
pub(super) const ACTION_X: usize = 370;
pub(super) const ACTION_Y: usize = 36;
const ACTION_WIDTH: usize = 66;
const ACTION_HEIGHT: usize = 18;
pub(super) const LABEL_X_OFFSET: usize = 8;
pub(super) const LABEL_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 9;

pub(super) fn toast_stack_manager(
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
        "Toast stack manager",
        &blocks(palette, scenario),
        &dedicated_toast_stack_manager_labels::labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            POSITION_X,
            POSITION_Y,
            POSITION_WIDTH,
            POSITION_HEIGHT,
            position_fill(palette, scenario),
        ),
        Block::outlined(
            PAUSE_X,
            PAUSE_Y,
            PAUSE_WIDTH,
            PAUSE_HEIGHT,
            pause_fill(palette, scenario),
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
        Block::outlined(
            STACK_PANEL_X,
            STACK_PANEL_Y,
            STACK_PANEL_WIDTH,
            STACK_PANEL_HEIGHT,
            stack_panel_fill(palette, scenario),
        ),
        Block::outlined(
            THIRD_TOAST_X,
            THIRD_TOAST_Y,
            TOAST_WIDTH,
            TOAST_HEIGHT,
            lower_toast_fill(palette, scenario),
        ),
        Block::outlined(
            SECOND_TOAST_X,
            SECOND_TOAST_Y,
            TOAST_WIDTH,
            TOAST_HEIGHT,
            middle_toast_fill(palette, scenario),
        ),
        Block::outlined(
            TOP_TOAST_X,
            TOP_TOAST_Y,
            TOAST_WIDTH,
            TOAST_HEIGHT,
            top_toast_fill(palette, scenario),
        ),
        Block::outlined(
            ACTION_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
        ),
        Block::new(
            QUEUE_X,
            QUEUE_Y,
            QUEUE_WIDTH,
            QUEUE_HEIGHT,
            queue_fill(palette, scenario),
        ),
    ]
}
