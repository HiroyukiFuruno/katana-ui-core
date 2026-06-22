use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_startup_state_panel_style::{
    action_fill, cancel_fill, headline_label, progress_width, state_label, status_fill,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PANEL_X: usize = 42;
const PANEL_Y: usize = 38;
const PANEL_WIDTH: usize = 436;
const PANEL_HEIGHT: usize = 74;
const STATUS_X: usize = 58;
const STATUS_Y: usize = 54;
const STATUS_WIDTH: usize = 120;
const STATUS_HEIGHT: usize = 22;
const TRACK_X: usize = 58;
const TRACK_Y: usize = 86;
const TRACK_WIDTH: usize = 278;
const TRACK_HEIGHT: usize = 8;
const RETRY_X: usize = 354;
const CANCEL_X: usize = 416;
const ACTION_Y: usize = 82;
const ACTION_WIDTH: usize = 48;
const ACTION_HEIGHT: usize = 18;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const STATUS_TEXT_Y_OFFSET: usize = 8;
const FOOTER_X: usize = 42;
const FOOTER_Y: usize = 120;
const FOOTER_GAP: usize = 134;
const VERSION_FOOTER_INDEX: usize = 3;
const BLOCK_COUNT: usize = 7;
const LABEL_COUNT: usize = 7;

pub(super) fn startup_state_panel(
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
        "Startup state",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(PANEL_X, PANEL_Y, PANEL_WIDTH, PANEL_HEIGHT, palette.panel),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            status_fill(palette, scenario),
        ),
        Block::outlined(TRACK_X, TRACK_Y, TRACK_WIDTH, TRACK_HEIGHT, palette.surface),
        Block::new(
            TRACK_X,
            TRACK_Y,
            progress_width(scenario),
            TRACK_HEIGHT,
            palette.accent,
        ),
        Block::outlined(
            RETRY_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
        ),
        Block::outlined(
            CANCEL_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            cancel_fill(palette, scenario),
        ),
        Block::outlined(
            SURFACE_TOKEN_X,
            SURFACE_TOKEN_Y,
            SURFACE_TOKEN_WIDTH,
            SURFACE_TOKEN_HEIGHT,
            palette.surface,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            SURFACE_TOKEN_X + TEXT_X_OFFSET,
            SURFACE_TOKEN_Y + TOKEN_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "surface token",
        ),
        TextSpec::new(
            STATUS_X + TEXT_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_8,
            palette.text,
            headline_label(scenario),
        ),
        TextSpec::new(
            RETRY_X + TEXT_X_OFFSET,
            ACTION_Y + m::PX_6,
            m::FONT_7,
            palette.text,
            "Retry",
        ),
        TextSpec::new(
            CANCEL_X + TEXT_X_OFFSET,
            ACTION_Y + m::PX_6,
            m::FONT_7,
            palette.text,
            "Cancel",
        ),
        TextSpec::new(
            FOOTER_X,
            FOOTER_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            FOOTER_X + FOOTER_GAP,
            FOOTER_Y,
            m::FONT_7,
            palette.muted,
            "live region",
        ),
        TextSpec::new(
            FOOTER_X + FOOTER_GAP * VERSION_FOOTER_INDEX,
            FOOTER_Y,
            m::FONT_7,
            palette.muted,
            "version=v0.1.0",
        ),
    ]
}
