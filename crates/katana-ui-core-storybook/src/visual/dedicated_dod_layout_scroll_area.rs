use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::panel_scroll_state::PanelScrollRegion;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const SCROLL_PRESET_INDEX: usize = 1;
const SCROLLBAR_PRESET_INDEX: usize = 2;
const THEME_SCROLL_PRESET_INDEX: usize = 3;
const VIEWPORT_X: usize = m::PX_16;
const VIEWPORT_Y: usize = m::PX_36;
const VIEWPORT_WIDTH: usize = m::PX_252;
const VIEWPORT_HEIGHT: usize = m::PX_74;
const ROW_X: usize = m::PX_34;
const ROW_Y: usize = m::PX_44;
const ROW_WIDTH: usize = m::PX_190;
const ROW_HEIGHT: usize = m::PX_14;
const ROW_GAP: usize = m::PX_8;
const THEME_ROW_GAP: usize = m::PX_14;
const SECOND_CONTENT_ROW_INDEX: usize = 2;
const THIRD_CONTENT_ROW_INDEX: usize = 3;
const SCROLL_PRESET_OFFSET: usize = m::PX_18;
const MAX_RENDERED_OFFSET: usize = m::PX_30;
const SCROLLBAR_X: usize = m::PX_244;
const SCROLLBAR_Y: usize = m::PX_44;
const SCROLLBAR_WIDTH: usize = m::PX_8;
const SCROLLBAR_HEIGHT: usize = m::PX_54;
const LABEL_X: usize = m::PX_284;
const STATUS_Y: usize = m::PX_88;
const STATUS_WIDTH: usize = m::PX_92;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const SCROLL_BLOCK_COUNT: usize = 8;
const SCROLL_LABEL_COUNT: usize = 4;
const STATUS_LABEL_COUNT: usize = 3;

pub(super) fn scroll_area(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let accent = if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "ScrollArea layout",
        &scroll_blocks(palette, scenario, accent),
        &scroll_labels(palette, scenario),
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

fn scroll_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; SCROLL_BLOCK_COUNT] {
    let gap = row_gap_for(scenario);
    let offset = scroll_offset_for(scenario);
    [
        Block::outlined(
            VIEWPORT_X,
            VIEWPORT_Y,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            palette.surface,
        ),
        Block::new(
            ROW_X,
            ROW_Y.saturating_sub(offset),
            ROW_WIDTH,
            ROW_HEIGHT,
            accent,
        ),
        Block::new(
            ROW_X,
            (ROW_Y + ROW_HEIGHT + gap).saturating_sub(offset),
            ROW_WIDTH,
            ROW_HEIGHT,
            palette.panel,
        ),
        Block::new(
            ROW_X,
            (ROW_Y + (ROW_HEIGHT + gap) * SECOND_CONTENT_ROW_INDEX).saturating_sub(offset),
            ROW_WIDTH,
            ROW_HEIGHT,
            common::TOKEN,
        ),
        Block::new(
            ROW_X,
            (ROW_Y + (ROW_HEIGHT + gap) * THIRD_CONTENT_ROW_INDEX).saturating_sub(offset),
            ROW_WIDTH,
            ROW_HEIGHT,
            common::WARN,
        ),
        Block::new(
            SCROLLBAR_X,
            SCROLLBAR_Y,
            SCROLLBAR_WIDTH,
            SCROLLBAR_HEIGHT,
            palette.panel,
        ),
        Block::new(
            SCROLLBAR_X,
            thumb_y_for(scenario),
            SCROLLBAR_WIDTH,
            thumb_height_for(scenario),
            common::PURPLE,
        ),
        Block::new(
            VIEWPORT_X + m::PX_8,
            VIEWPORT_Y + VIEWPORT_HEIGHT - m::PX_10,
            theme_marker_width(scenario),
            m::PX_4,
            common::DANGER,
        ),
    ]
}

fn scroll_labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; SCROLL_LABEL_COUNT] {
    [
        TextSpec::new(
            LABEL_X,
            m::PX_42,
            m::FONT_9,
            palette.text,
            scroll_preset_label(scenario),
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_58,
            m::FONT_8,
            palette.muted,
            "content clipped",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "state/action via settings",
        ),
        TextSpec::new(m::PX_46, m::PX_50, m::FONT_8, palette.background, "row"),
    ]
}

fn row_gap_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == THEME_SCROLL_PRESET_INDEX {
        return THEME_ROW_GAP;
    }
    ROW_GAP
}

fn scroll_offset_for(scenario: ScenarioContext<'_>) -> usize {
    let scenario_offset = scenario
        .panel_scroll
        .offset(PanelScrollRegion::Preview)
        .min(MAX_RENDERED_OFFSET);
    if scenario.preset_index == SCROLL_PRESET_INDEX {
        return scenario_offset + SCROLL_PRESET_OFFSET;
    }
    scenario_offset
}

fn thumb_y_for(scenario: ScenarioContext<'_>) -> usize {
    let offset = scroll_offset_for(scenario).min(MAX_RENDERED_OFFSET);
    SCROLLBAR_Y + offset
}

fn thumb_height_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == SCROLLBAR_PRESET_INDEX {
        return m::PX_28;
    }
    m::PX_18
}

fn theme_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == THEME_SCROLL_PRESET_INDEX {
        return m::PX_42;
    }
    m::PX_16
}

fn scroll_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        SCROLL_PRESET_INDEX => "scroll offset",
        SCROLLBAR_PRESET_INDEX => "scrollbar thumb",
        THEME_SCROLL_PRESET_INDEX => "theme scroll",
        _ => "viewport",
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
        let row_x = x + VIEWPORT_X + index * (STATUS_WIDTH + STATUS_GAP);
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
        return ["action scroll", "event offset", "state override"];
    }
    ["action ready", "event ready", "state idle"]
}
