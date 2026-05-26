use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::{TextRenderer, TextVerticalBox};

const DISABLED: u32 = 0x2d2d30;
const SEGMENT_X: usize = 18;
const SEGMENT_Y: usize = 44;
const SEGMENT_WIDTH: usize = 92;
const SEGMENT_HEIGHT: usize = 28;
const SEGMENT_COUNT: usize = 3;
const DEFAULT_SEGMENT_INDEX: usize = 0;
const SELECTED_SEGMENT_INDEX: usize = 1;
const DISABLED_SEGMENT_INDEX: usize = SEGMENT_COUNT - 1;
const SEGMENT_MARKER_HEIGHT: usize = 3;
const STATUS_X: usize = 204;
const STATUS_Y: usize = 36;
const STATUS_WIDTH: usize = 120;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const TEXT_X: usize = 10;
const TEXT_Y: usize = 6;

#[derive(Clone, Copy)]
struct SegmentRender<'a> {
    label: &'a str,
    index: usize,
    selected: usize,
    disabled: bool,
    themed: bool,
    origin_x: usize,
    origin_y: usize,
}

pub(super) fn segmented(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SegmentedToggle");
    draw_segments(canvas, text, palette, scenario, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_segments(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let selected = selected_segment(scenario);
    let disabled_index = if scenario.preset_index == m::PX_2 {
        Some(DISABLED_SEGMENT_INDEX)
    } else {
        None
    };
    let themed = scenario.preset_index == m::PX_3;
    for (index, label) in ["Preview", "Code", "Diff"].into_iter().enumerate() {
        draw_segment(
            canvas,
            text,
            palette,
            SegmentRender {
                label,
                index,
                selected,
                disabled: disabled_index == Some(index),
                themed,
                origin_x: x,
                origin_y: y,
            },
        );
    }
}

fn selected_segment(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == m::PX_1 {
        return SELECTED_SEGMENT_INDEX;
    }
    DEFAULT_SEGMENT_INDEX
}

fn draw_segment(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    segment: SegmentRender<'_>,
) {
    let row_x = segment.origin_x + SEGMENT_X + segment.index * SEGMENT_WIDTH;
    let active = segment.index == segment.selected;
    canvas.fill_rect(
        row_x,
        segment.origin_y + SEGMENT_Y,
        SEGMENT_WIDTH,
        SEGMENT_HEIGHT,
        segment_fill(palette, active, segment.disabled),
    );
    let border = if segment.themed {
        palette.accent
    } else {
        palette.border
    };
    canvas.stroke_rect(
        row_x,
        segment.origin_y + SEGMENT_Y,
        SEGMENT_WIDTH,
        SEGMENT_HEIGHT,
        border,
    );
    if active {
        let marker = if segment.themed {
            palette.accent
        } else {
            common::WARN
        };
        canvas.fill_rect(
            row_x,
            segment.origin_y + SEGMENT_Y + SEGMENT_HEIGHT - SEGMENT_MARKER_HEIGHT,
            SEGMENT_WIDTH,
            SEGMENT_MARKER_HEIGHT,
            marker,
        );
    }
    let text_color = if segment.disabled {
        palette.muted
    } else {
        palette.text
    };
    text.draw_centered(
        canvas,
        segment.label,
        row_x + TEXT_X,
        TextVerticalBox::new(segment.origin_y + SEGMENT_Y, SEGMENT_HEIGHT as f32),
        m::FONT_9,
        text_color,
    );
}

fn segment_fill(palette: &VisualPalette, active: bool, disabled: bool) -> u32 {
    if disabled {
        return DISABLED;
    }
    if active {
        return palette.accent;
    }
    palette.panel
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rows = [
        status_action(scenario),
        status_event(scenario),
        status_state(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_y = y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP);
        canvas.fill_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            x + STATUS_X + TEXT_X,
            row_y + TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn status_action(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn status_event(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn status_state(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "selected=none";
    }
    scenario.screen_state.state_label
}
