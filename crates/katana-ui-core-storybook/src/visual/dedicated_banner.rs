use super::canvas::Canvas;
use super::dedicated_banner_labels;
use super::dedicated_banner_style::{
    action_fill, banner_fill, details_fill, dismiss_fill, icon_fill, severity_fill,
};
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const BANNER_X: usize = 40;
pub(super) const BANNER_Y: usize = 34;
pub(super) const BANNER_WIDTH: usize = 420;
const BANNER_HEIGHT: usize = 48;
const STRIP_WIDTH: usize = 6;
pub(super) const ICON_X: usize = 54;
pub(super) const ICON_Y: usize = 49;
const ICON_SIZE: usize = 18;
pub(super) const TEXT_X: usize = 82;
pub(super) const TITLE_Y: usize = 42;
pub(super) const BODY_Y: usize = 60;
pub(super) const ACTION_X: usize = 344;
pub(super) const ACTION_Y: usize = 49;
const ACTION_WIDTH: usize = 78;
const ACTION_HEIGHT: usize = 20;
pub(super) const DISMISS_X: usize = 430;
pub(super) const DISMISS_Y: usize = 49;
const DISMISS_SIZE: usize = 20;
pub(super) const DETAILS_X: usize = 40;
pub(super) const DETAILS_Y: usize = 88;
const DETAILS_WIDTH: usize = 420;
const DETAILS_HEIGHT: usize = 30;
pub(super) const LABEL_X_OFFSET: usize = 8;
pub(super) const LABEL_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 6;

pub(super) fn banner(
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
        "Banner",
        &blocks(palette, scenario),
        &dedicated_banner_labels::labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            BANNER_X,
            BANNER_Y,
            BANNER_WIDTH,
            BANNER_HEIGHT,
            banner_fill(palette, scenario),
        ),
        Block::new(
            BANNER_X,
            BANNER_Y,
            STRIP_WIDTH,
            BANNER_HEIGHT,
            severity_fill(palette, scenario),
        ),
        Block::outlined(
            ICON_X,
            ICON_Y,
            ICON_SIZE,
            ICON_SIZE,
            icon_fill(palette, scenario),
        ),
        Block::outlined(
            ACTION_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
        ),
        Block::outlined(
            DISMISS_X,
            DISMISS_Y,
            DISMISS_SIZE,
            DISMISS_SIZE,
            dismiss_fill(palette, scenario),
        ),
        Block::outlined(
            DETAILS_X,
            DETAILS_Y,
            DETAILS_WIDTH,
            DETAILS_HEIGHT,
            details_fill(palette, scenario),
        ),
    ]
}
