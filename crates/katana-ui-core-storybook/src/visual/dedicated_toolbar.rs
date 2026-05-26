use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::dedicated_toolbar_style::{
    accelerator_fill, accelerator_label, action_fill, action_text, bar_fill, density_fill,
    more_fill, split_fill, state_label,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const BAR_X: usize = 44;
pub(super) const BAR_Y: usize = 42;
pub(super) const BAR_WIDTH: usize = 390;
const BAR_HEIGHT: usize = 40;
const SAVE_X: usize = 58;
const SEARCH_X: usize = 160;
const EXPORT_X: usize = 240;
const MORE_X: usize = 334;
const ACTION_Y: usize = 50;
const ACTION_WIDTH: usize = 70;
const ACTION_HEIGHT: usize = 22;
const SPLIT_X: usize = 128;
const SPLIT_WIDTH: usize = 20;
const MORE_WIDTH: usize = 64;
const KEYCAP_X: usize = 58;
const KEYCAP_Y: usize = 94;
const KEYCAP_WIDTH: usize = 72;
const KEYCAP_HEIGHT: usize = 20;
const STATUS_X: usize = 148;
const STATUS_Y: usize = 94;
const STATUS_WIDTH: usize = 180;
const STATUS_HEIGHT: usize = 20;
const DENSITY_X: usize = 344;
const DENSITY_Y: usize = 94;
const DENSITY_WIDTH: usize = 76;
const DENSITY_HEIGHT: usize = 20;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 7;
const STATUS_TEXT_Y_OFFSET: usize = 6;
const BLOCK_COUNT: usize = 9;
const LABEL_COUNT: usize = 8;

pub(super) fn toolbar(
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
        "Toolbar",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            BAR_X,
            BAR_Y,
            BAR_WIDTH,
            BAR_HEIGHT,
            bar_fill(palette, scenario),
        ),
        Block::outlined(
            SAVE_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
        ),
        Block::outlined(
            SPLIT_X,
            ACTION_Y,
            SPLIT_WIDTH,
            ACTION_HEIGHT,
            split_fill(palette, scenario),
        ),
        Block::outlined(
            SEARCH_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
        ),
        Block::outlined(
            EXPORT_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
        ),
        Block::outlined(
            MORE_X,
            ACTION_Y,
            MORE_WIDTH,
            ACTION_HEIGHT,
            more_fill(palette, scenario),
        ),
        Block::outlined(
            KEYCAP_X,
            KEYCAP_Y,
            KEYCAP_WIDTH,
            KEYCAP_HEIGHT,
            accelerator_fill(palette, scenario),
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
        Block::outlined(
            DENSITY_X,
            DENSITY_Y,
            DENSITY_WIDTH,
            DENSITY_HEIGHT,
            density_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            SAVE_X + LABEL_X_OFFSET,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            action_text(palette, scenario),
            "Save",
        ),
        TextSpec::new(
            SPLIT_X + m::PX_6,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "v",
        ),
        TextSpec::new(
            SEARCH_X + LABEL_X_OFFSET,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            action_text(palette, scenario),
            "Search",
        ),
        TextSpec::new(
            EXPORT_X + LABEL_X_OFFSET,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            action_text(palette, scenario),
            "Export",
        ),
        TextSpec::new(
            MORE_X + LABEL_X_OFFSET,
            ACTION_Y + LABEL_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "More",
        ),
        TextSpec::new(
            KEYCAP_X + LABEL_X_OFFSET,
            KEYCAP_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            accelerator_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + LABEL_X_OFFSET,
            STATUS_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
        TextSpec::new(
            DENSITY_X + LABEL_X_OFFSET,
            DENSITY_Y + STATUS_TEXT_Y_OFFSET,
            m::FONT_7,
            palette.muted,
            "density",
        ),
    ]
}
