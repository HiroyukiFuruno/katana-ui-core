use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_window_control_button_group_style::{
    button_size, chrome_fill, close_fill, leading_offset, maximize_fill, minimize_fill,
    platform_label, state_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const CHROME_X: usize = 42;
const CHROME_Y: usize = 42;
const CHROME_WIDTH: usize = 436;
const CHROME_HEIGHT: usize = 54;
const BUTTON_Y: usize = 62;
const BUTTON_GAP: usize = 22;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_TOKEN_WIDTH: usize = 140;
const SURFACE_TOKEN_HEIGHT: usize = 18;
const TEXT_X_OFFSET: usize = 8;
const TOKEN_TEXT_Y_OFFSET: usize = 5;
const STATUS_X: usize = 42;
const STATUS_Y: usize = 116;
const STATUS_GAP: usize = 150;
const STATE_STATUS_INDEX: usize = 2;
const STATE_STATUS_X: usize = STATUS_X + STATUS_GAP * STATE_STATUS_INDEX;
const BLOCK_COUNT: usize = 6;
const LABEL_COUNT: usize = 5;

pub(super) fn window_control_button_group(
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
        "Window controls",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let button_x = CHROME_X + leading_offset(scenario);
    let size = button_size(scenario);
    [
        Block::outlined(
            CHROME_X,
            CHROME_Y,
            CHROME_WIDTH,
            CHROME_HEIGHT,
            chrome_fill(palette, scenario),
        ),
        Block::new(button_x, BUTTON_Y, size, size, close_fill(scenario)),
        Block::new(
            button_x + BUTTON_GAP,
            BUTTON_Y,
            size,
            size,
            minimize_fill(palette, scenario),
        ),
        Block::new(
            button_x + BUTTON_GAP * 2,
            BUTTON_Y,
            size,
            size,
            maximize_fill(palette, scenario),
        ),
        Block::new(
            CHROME_X,
            CHROME_Y + CHROME_HEIGHT - m::PX_4,
            CHROME_WIDTH,
            m::PX_4,
            palette.accent,
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
            CHROME_X + m::PX_118,
            CHROME_Y + m::PX_20,
            m::FONT_8,
            palette.muted,
            "Title bar preview",
        ),
        TextSpec::new(
            STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            platform_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_GAP,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            "Close/Min/Max",
        ),
        TextSpec::new(
            STATE_STATUS_X,
            STATUS_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}
