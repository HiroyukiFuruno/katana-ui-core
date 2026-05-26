use super::canvas::Canvas;
use super::dedicated_attachment_chip_style::{
    attachment_fill, file_label, kind_fill, progress_fill, progress_width, retry_fill, retry_label,
    status_label,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const ATTACHMENT_X: usize = 32;
pub(super) const ATTACHMENT_Y: usize = 34;
const ATTACHMENT_WIDTH: usize = 250;
const ATTACHMENT_HEIGHT: usize = 62;
const ICON_X_OFFSET: usize = 14;
const ICON_Y_OFFSET: usize = 14;
const ICON_SIZE: usize = 26;
const LABEL_X_OFFSET: usize = 52;
const LABEL_Y_OFFSET: usize = 14;
const PROGRESS_X_OFFSET: usize = 52;
const PROGRESS_Y_OFFSET: usize = 40;
const PROGRESS_WIDTH: usize = 124;
const PROGRESS_HEIGHT: usize = 6;
const RETRY_X_OFFSET: usize = 188;
const RETRY_Y_OFFSET: usize = 24;
const RETRY_WIDTH: usize = 44;
const RETRY_HEIGHT: usize = 20;
const STATUS_X: usize = 338;
const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 140;
const STATUS_HEIGHT: usize = 20;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const STATUS_TEXT_X_OFFSET: usize = 8;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 7;
const LABEL_COUNT: usize = 5;

pub(super) fn attachment_chip(
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
        "Attachment chip",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            ATTACHMENT_X,
            ATTACHMENT_Y,
            ATTACHMENT_WIDTH,
            ATTACHMENT_HEIGHT,
            attachment_fill(palette, scenario),
        ),
        Block::new(
            ATTACHMENT_X + ICON_X_OFFSET,
            ATTACHMENT_Y + ICON_Y_OFFSET,
            ICON_SIZE,
            ICON_SIZE,
            kind_fill(palette, scenario),
        ),
        Block::outlined(
            ATTACHMENT_X + PROGRESS_X_OFFSET,
            ATTACHMENT_Y + PROGRESS_Y_OFFSET,
            PROGRESS_WIDTH,
            PROGRESS_HEIGHT,
            palette.panel,
        ),
        Block::new(
            ATTACHMENT_X + PROGRESS_X_OFFSET,
            ATTACHMENT_Y + PROGRESS_Y_OFFSET,
            progress_width(scenario),
            PROGRESS_HEIGHT,
            progress_fill(scenario),
        ),
        Block::outlined(
            ATTACHMENT_X + RETRY_X_OFFSET,
            ATTACHMENT_Y + RETRY_Y_OFFSET,
            RETRY_WIDTH,
            RETRY_HEIGHT,
            retry_fill(palette, scenario),
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
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            ATTACHMENT_X + LABEL_X_OFFSET,
            ATTACHMENT_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            palette.text,
            file_label(scenario),
        ),
        TextSpec::new(
            ATTACHMENT_X + RETRY_X_OFFSET + m::PX_8,
            ATTACHMENT_Y + RETRY_Y_OFFSET + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.text,
            retry_label(scenario),
        ),
        TextSpec::new(
            SURFACE_TOKEN_X + STATUS_TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X + STATUS_TEXT_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            status_label(scenario),
        ),
        TextSpec::new(
            ATTACHMENT_X + PROGRESS_X_OFFSET,
            ATTACHMENT_Y + PROGRESS_Y_OFFSET + m::PX_12,
            m::FONT_7,
            palette.muted,
            "upload progress",
        ),
    ]
}
