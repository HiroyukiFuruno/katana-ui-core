use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const BUBBLE_X: usize = 112;
const BUBBLE_EDGE_X: usize = 264;
const BUBBLE_Y: usize = 34;
const BUBBLE_WIDTH: usize = 132;
const BUBBLE_HEIGHT: usize = 26;
const ANCHOR_X: usize = 134;
const ANCHOR_Y: usize = 72;
const ANCHOR_WIDTH: usize = 80;
const ANCHOR_HEIGHT: usize = 22;
const EDGE_ANCHOR_X: usize = 314;
const STATUS_X: usize = 26;
const STATUS_Y: usize = 100;
const STATUS_WIDTH: usize = 92;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const LABEL_X_OFFSET: usize = 8;
const LABEL_Y_OFFSET: usize = 7;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;
const POINTER_Y_OFFSET: usize = 6;
const POINTER_WIDTH: usize = 10;
const POINTER_HEIGHT: usize = 6;
const BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 5;
const HOVER_PRESET_INDEX: usize = 1;
const EDGE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;

pub(super) fn tooltip(
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
        "Tooltip",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    let bubble_x = bubble_x(scenario);
    let anchor_x = anchor_x(scenario);
    [
        Block::outlined(
            bubble_x,
            BUBBLE_Y,
            BUBBLE_WIDTH,
            BUBBLE_HEIGHT,
            bubble_fill(palette, scenario),
        ),
        Block::new(
            anchor_x + ANCHOR_WIDTH / 2,
            ANCHOR_Y - POINTER_Y_OFFSET,
            POINTER_WIDTH,
            POINTER_HEIGHT,
            pointer_fill(palette, scenario),
        ),
        Block::outlined(
            anchor_x,
            ANCHOR_Y,
            ANCHOR_WIDTH,
            ANCHOR_HEIGHT,
            anchor_fill(palette, scenario),
        ),
        Block::outlined(
            STATUS_X,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
        Block::outlined(
            STATUS_X + STATUS_WIDTH + STATUS_GAP,
            STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    let bubble_x = bubble_x(scenario);
    let anchor_x = anchor_x(scenario);
    [
        TextSpec::new(
            bubble_x + LABEL_X_OFFSET,
            BUBBLE_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            bubble_text(palette, scenario),
            bubble_label(scenario),
        ),
        TextSpec::new(
            anchor_x + LABEL_X_OFFSET,
            ANCHOR_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            anchor_text(palette, scenario),
            "anchor",
        ),
        TextSpec::new(
            STATUS_X + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            action_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + STATUS_WIDTH + STATUS_GAP + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            event_label(scenario),
        ),
        TextSpec::new(
            STATUS_X + (STATUS_WIDTH + STATUS_GAP) * 2 + STATUS_TEXT_X,
            STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
            state_label(scenario),
        ),
    ]
}

fn bubble_x(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == EDGE_PRESET_INDEX {
        return BUBBLE_EDGE_X;
    }
    BUBBLE_X
}

fn anchor_x(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == EDGE_PRESET_INDEX {
        return EDGE_ANCHOR_X;
    }
    ANCHOR_X
}

fn bubble_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    if is_open(scenario) {
        return palette.accent;
    }
    palette.surface
}

fn pointer_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if is_open(scenario) {
        return palette.accent;
    }
    bubble_fill(palette, scenario)
}

fn anchor_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return palette.accent;
    }
    if scenario.preset_index == EDGE_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

fn bubble_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if is_open(scenario) {
        return palette.background;
    }
    palette.text
}

fn anchor_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return palette.background;
    }
    palette.text
}

fn bubble_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == EDGE_PRESET_INDEX {
        return "edge placement";
    }
    if is_open(scenario) {
        return "hover open";
    }
    "top hover"
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "hover=false";
    }
    scenario.screen_state.state_label
}

fn is_open(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == HOVER_PRESET_INDEX || scenario.screen_state.has_widget_action()
}
