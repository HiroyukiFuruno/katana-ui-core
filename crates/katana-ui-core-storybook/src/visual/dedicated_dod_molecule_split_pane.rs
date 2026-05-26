use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const VERTICAL_PRESET_INDEX: usize = 1;
const CLAMP_PRESET_INDEX: usize = 2;
const RESET_PRESET_INDEX: usize = 3;
const PANEL_X: usize = m::PX_18;
const PANEL_Y: usize = m::PX_36;
const PANEL_WIDTH: usize = m::PX_252;
const PANEL_HEIGHT: usize = m::PX_56;
const LEFT_WIDTH: usize = m::PX_92;
const RESET_LEFT_WIDTH: usize = m::PX_116;
const CLAMP_LEFT_WIDTH: usize = m::PX_72;
const HANDLE_WIDTH: usize = m::PX_6;
const HANDLE_HEIGHT: usize = m::PX_56;
const VERTICAL_HANDLE_Y: usize = m::PX_62;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_96;
const STATUS_WIDTH: usize = m::PX_96;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const SPLIT_BLOCK_COUNT: usize = 6;
const SPLIT_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn split_pane(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.has_widget_action()
        || scenario.screen_state.has_settings_override()
    {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "SplitPane",
        &split_blocks(palette, scenario, accent),
        &split_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn split_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; SPLIT_BLOCK_COUNT] {
    if scenario.preset_index == VERTICAL_PRESET_INDEX {
        return vertical_blocks(palette, accent);
    }
    horizontal_blocks(palette, scenario, accent)
}

fn horizontal_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; SPLIT_BLOCK_COUNT] {
    let left_width = left_width_for(scenario);
    let right_x = PANEL_X + left_width + HANDLE_WIDTH;
    [
        Block::outlined(PANEL_X, PANEL_Y, left_width, PANEL_HEIGHT, palette.surface),
        Block::new(
            PANEL_X + left_width,
            PANEL_Y,
            HANDLE_WIDTH,
            HANDLE_HEIGHT,
            accent,
        ),
        Block::outlined(
            right_x,
            PANEL_Y,
            PANEL_WIDTH - left_width - HANDLE_WIDTH,
            PANEL_HEIGHT,
            palette.surface,
        ),
        Block::new(
            PANEL_X + m::PX_10,
            PANEL_Y + m::PX_18,
            m::PX_48,
            m::PX_8,
            palette.panel,
        ),
        Block::new(
            right_x + m::PX_12,
            PANEL_Y + m::PX_18,
            m::PX_58,
            m::PX_8,
            common::TOKEN,
        ),
        Block::new(
            PANEL_X + left_width.saturating_sub(m::PX_12),
            PANEL_Y + m::PX_8,
            clamp_marker_width(scenario),
            PANEL_HEIGHT - m::PX_16,
            common::DANGER,
        ),
    ]
}

fn vertical_blocks(palette: &VisualPalette, accent: u32) -> [Block; SPLIT_BLOCK_COUNT] {
    [
        Block::outlined(PANEL_X, PANEL_Y, PANEL_WIDTH, m::PX_26, palette.surface),
        Block::new(
            PANEL_X,
            VERTICAL_HANDLE_Y,
            PANEL_WIDTH,
            HANDLE_WIDTH,
            accent,
        ),
        Block::outlined(
            PANEL_X,
            VERTICAL_HANDLE_Y + HANDLE_WIDTH,
            PANEL_WIDTH,
            m::PX_24,
            palette.surface,
        ),
        Block::new(
            PANEL_X + m::PX_18,
            PANEL_Y + m::PX_10,
            m::PX_64,
            m::PX_8,
            palette.panel,
        ),
        Block::new(
            PANEL_X + m::PX_18,
            VERTICAL_HANDLE_Y + m::PX_14,
            m::PX_72,
            m::PX_8,
            common::TOKEN,
        ),
        Block::new(
            PANEL_X + PANEL_WIDTH - m::PX_28,
            PANEL_Y + m::PX_8,
            m::PX_12,
            m::PX_40,
            common::DANGER,
        ),
    ]
}

fn split_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; SPLIT_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            split_preset_label(scenario),
        ),
        TextSpec::new(LABEL_X, m::PX_58, m::FONT_8, palette.muted, "resize handle"),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_32, m::PX_54, m::FONT_8, palette.background, "A | B"),
    ]
}

fn left_width_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == CLAMP_PRESET_INDEX {
        return CLAMP_LEFT_WIDTH;
    }
    if scenario.preset_index == RESET_PRESET_INDEX || scenario.screen_state.has_widget_action() {
        return RESET_LEFT_WIDTH;
    }
    LEFT_WIDTH
}

fn clamp_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == CLAMP_PRESET_INDEX {
        return m::PX_22;
    }
    m::PX_8
}

fn split_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        VERTICAL_PRESET_INDEX => "vertical split",
        CLAMP_PRESET_INDEX => "min clamp",
        RESET_PRESET_INDEX => "reset ratio",
        _ => "horizontal split",
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
        let row_x = x + PANEL_X + index * (STATUS_WIDTH + STATUS_GAP);
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
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return ["action resize", "event ratio", "state changed"];
    }
    ["action ready", "event ready", "state idle"]
}
