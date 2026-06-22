use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const FLEX_PRESET_INDEX: usize = 1;
const DENSE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const STAGE_X: usize = m::PX_16;
const STAGE_Y: usize = m::PX_36;
const STAGE_WIDTH: usize = m::PX_252;
const STAGE_HEIGHT: usize = m::PX_68;
const ITEM_X: usize = m::PX_40;
const ITEM_Y: usize = m::PX_56;
const ITEM_WIDTH: usize = m::PX_48;
const ITEM_HEIGHT: usize = m::PX_24;
const FIXED_GAP: usize = m::PX_24;
const FLEX_GAP: usize = m::PX_64;
const DENSE_GAP: usize = m::PX_10;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const SPACER_BLOCK_COUNT: usize = 6;
const SPACER_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn spacer(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let gap_color = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        gap_color_for(palette, scenario)
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Spacer",
        &spacer_blocks(palette, scenario, gap_color),
        &spacer_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn spacer_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    gap_color: u32,
) -> [Block; SPACER_BLOCK_COUNT] {
    let gap_rect = gap_rect_for(scenario);
    let right_x = gap_rect.x + gap_rect.width;
    [
        Block::outlined(STAGE_X, STAGE_Y, STAGE_WIDTH, STAGE_HEIGHT, palette.surface),
        Block::new(ITEM_X, ITEM_Y, ITEM_WIDTH, ITEM_HEIGHT, palette.panel),
        Block::outlined(
            gap_rect.x,
            gap_rect.y,
            gap_rect.width,
            gap_rect.height,
            gap_color,
        ),
        Block::new(right_x, ITEM_Y, ITEM_WIDTH, ITEM_HEIGHT, palette.panel),
        Block::new(
            gap_rect.x,
            ITEM_Y + m::PX_10,
            gap_rect.width,
            m::PX_4,
            gap_color,
        ),
        Block::new(
            STAGE_X + STAGE_WIDTH - m::PX_58,
            STAGE_Y + STAGE_HEIGHT - m::PX_20,
            theme_marker_width(scenario),
            m::PX_6,
            common::TOKEN,
        ),
    ]
}

fn gap_rect_for(scenario: ScenarioContext<'_>) -> Rect {
    Rect::new(
        ITEM_X + ITEM_WIDTH,
        ITEM_Y,
        gap_width_for(scenario),
        ITEM_HEIGHT,
    )
}

#[cfg(test)]
pub(super) fn gap_rect_for_test(scenario: ScenarioContext<'_>) -> Rect {
    gap_rect_for(scenario)
}

fn spacer_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; SPACER_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            spacer_preset_label(scenario),
        ),
        TextSpec::new(LABEL_X, m::PX_58, m::FONT_8, palette.muted, "layout gap"),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_52, m::PX_64, m::FONT_8, palette.text, "A"),
    ]
}

fn gap_width_for(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        FLEX_PRESET_INDEX => FLEX_GAP,
        DENSE_PRESET_INDEX => DENSE_GAP,
        _ => FIXED_GAP,
    }
}

fn gap_color_for(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return palette.accent;
    }
    common::WARN
}

fn theme_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return m::PX_42;
    }
    m::PX_14
}

fn spacer_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        FLEX_PRESET_INDEX => "flex gap",
        DENSE_PRESET_INDEX => "dense gap",
        THEME_PRESET_INDEX => "theme gap",
        _ => "fixed gap",
    }
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, label) in status_labels(scenario).into_iter().enumerate() {
        let row_x = x + STAGE_X + index * (STATUS_WIDTH + STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            label,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn status_labels(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_LABEL_COUNT] {
    if scenario.screen_state.has_settings_override() {
        return ["action spacer", "event gap", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
