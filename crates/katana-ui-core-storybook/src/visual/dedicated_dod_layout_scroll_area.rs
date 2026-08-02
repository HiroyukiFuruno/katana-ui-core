use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::panel_scroll_state::PanelScrollRegion;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

#[path = "dedicated_dod_layout_scroll_area_geometry.rs"]
mod geometry;
#[path = "dedicated_dod_layout_scroll_area_status.rs"]
mod status;

pub(super) use geometry::{content_clip_rect, frame_rect, resize_handle_rect, scrollbar_drag_rect};
#[cfg(test)]
pub(super) use geometry::{status_rects, viewport_rect};

const SCROLL_PRESET_INDEX: usize = 1;
const SCROLLBAR_PRESET_INDEX: usize = 2;
const THEME_SCROLL_PRESET_INDEX: usize = 3;
pub(super) const SCROLL_AREA_FRAME_HEIGHT: usize = m::PX_156;
const VIEWPORT_X: usize = m::PX_16;
const VIEWPORT_Y: usize = m::PX_36;
const VIEWPORT_WIDTH: usize = m::PX_278;
const VIEWPORT_HEIGHT: usize = m::PX_72;
const CONTENT_X: usize = VIEWPORT_X + m::PX_8;
const CONTENT_Y: usize = VIEWPORT_Y + m::PX_10;
const CONTENT_WIDTH: usize = VIEWPORT_WIDTH - m::PX_42;
const CONTENT_HEIGHT: usize = VIEWPORT_HEIGHT - m::PX_20;
const ROW_X: usize = CONTENT_X + m::PX_6;
const ROW_Y: usize = CONTENT_Y + m::PX_4;
const ROW_WIDTH: usize = m::PX_212;
const ROW_HEIGHT: usize = m::PX_14;
const ROW_GAP: usize = m::PX_10;
const THEME_ROW_GAP: usize = m::PX_14;
const SECOND_CONTENT_ROW_INDEX: usize = 2;
const THIRD_CONTENT_ROW_INDEX: usize = 3;
const SCROLL_PRESET_OFFSET: usize = m::PX_18;
const MAX_RENDERED_OFFSET: usize = m::PX_30;
const SCROLLBAR_X: usize = m::PX_278;
const SCROLLBAR_Y: usize = m::PX_48;
const SCROLLBAR_WIDTH: usize = m::PX_8;
const SCROLLBAR_HEIGHT: usize = m::PX_52;
const LABEL_X: usize = m::PX_326;
const STATUS_Y: usize = m::PX_118;
const STATUS_WIDTH: usize = m::PX_120;
const STATUS_HEIGHT: usize = m::PX_18;
const STATUS_GAP: usize = m::PX_8;
const STATUS_TEXT_X: usize = m::PX_6;
const STATUS_TEXT_Y: usize = m::PX_4;
const SCROLL_CHROME_BLOCK_COUNT: usize = 4;
const SCROLL_CONTENT_BLOCK_COUNT: usize = 5;
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
    let accent = if scenario.screen_state.has_settings_override()
        || scenario.screen_state.scroll_area.hovered()
        || scenario.screen_state.scroll_area.focused()
        || scenario.screen_state.scroll_area.dragging()
        || scenario.screen_state.scroll_area.resized()
        || scenario.screen_state.scroll_area.offset_y() > 0
    {
        common::SUCCESS
    } else {
        palette.accent
    };
    common::frame_with_height(
        canvas,
        text,
        palette,
        x,
        y,
        SCROLL_AREA_FRAME_HEIGHT,
        "ScrollArea layout",
    );
    common::draw_blocks(
        canvas,
        palette,
        x,
        y,
        &scroll_chrome_blocks(palette, scenario),
    );
    let clip = content_clip_rect(x, y);
    canvas.with_clip(clip.x, clip.y, clip.width, clip.height, &mut |canvas| {
        common::draw_blocks(
            canvas,
            palette,
            x,
            y,
            &scroll_content_blocks(palette, scenario, accent),
        );
    });
    common::draw_labels(canvas, text, x, y, &scroll_labels(palette, scenario));
    status::draw(canvas, text, palette, scenario, x, y);
}

fn scroll_chrome_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [Block; SCROLL_CHROME_BLOCK_COUNT] {
    [
        Block::outlined(
            VIEWPORT_X,
            VIEWPORT_Y,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            palette.surface,
        ),
        Block::outlined(
            CONTENT_X,
            CONTENT_Y,
            CONTENT_WIDTH,
            CONTENT_HEIGHT,
            palette.background,
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
    ]
}

fn scroll_content_blocks(
    _palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    accent: u32,
) -> [Block; SCROLL_CONTENT_BLOCK_COUNT] {
    let gap = row_gap_for(scenario);
    let offset = scroll_offset_for(scenario);
    [
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
            common::SUCCESS,
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
            CONTENT_X,
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
            "viewport clips content",
        ),
        TextSpec::new(
            LABEL_X,
            m::PX_74,
            m::FONT_8,
            palette.muted,
            "scrollbar is part of UI",
        ),
        TextSpec::new(m::PX_42, m::PX_54, m::FONT_8, palette.background, "row 01"),
    ]
}

fn row_gap_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == THEME_SCROLL_PRESET_INDEX {
        return THEME_ROW_GAP;
    }
    ROW_GAP
}

fn scroll_offset_for(scenario: ScenarioContext<'_>) -> usize {
    let story_offset = scenario.screen_state.scroll_area.offset_y() as usize;
    let scenario_offset = story_offset
        + scenario
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
    if scenario.screen_state.scroll_area.resized() {
        return m::PX_12;
    }
    if scenario.preset_index == SCROLLBAR_PRESET_INDEX {
        return m::PX_28;
    }
    m::PX_18
}

pub(super) fn theme_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == THEME_SCROLL_PRESET_INDEX {
        return m::PX_42;
    }
    m::PX_16
}

pub(super) fn scroll_preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        SCROLL_PRESET_INDEX => "scroll offset",
        SCROLLBAR_PRESET_INDEX => "scrollbar thumb",
        THEME_SCROLL_PRESET_INDEX => "theme scroll",
        _ => "viewport",
    }
}
