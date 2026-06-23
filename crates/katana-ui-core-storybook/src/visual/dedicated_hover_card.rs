use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_hover_card_labels;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const ANCHOR_X: usize = 28;
pub(super) const ANCHOR_Y: usize = 72;
const ANCHOR_WIDTH: usize = 96;
const ANCHOR_HEIGHT: usize = 26;
const CARD_X: usize = 142;
const FOLLOW_CARD_X: usize = 176;
pub(super) const CARD_Y: usize = 28;
const CARD_WIDTH: usize = 218;
const CARD_HEIGHT: usize = 80;
const POINTER_X: usize = 132;
const POINTER_Y: usize = 80;
const POINTER_SIZE: usize = 8;
pub(super) const TITLE_X_OFFSET: usize = 12;
pub(super) const FIRST_LABEL_Y_OFFSET: usize = 10;
pub(super) const LABEL_GAP: usize = 18;
pub(super) const ACTION_X: usize = 272;
pub(super) const ACTION_Y: usize = 78;
const ACTION_WIDTH: usize = 72;
const ACTION_HEIGHT: usize = 20;
pub(super) const STATUS_X: usize = 28;
pub(super) const STATUS_Y: usize = 104;
pub(super) const STATUS_WIDTH: usize = 94;
const STATUS_HEIGHT: usize = 18;
pub(super) const STATUS_TEXT_X: usize = 7;
pub(super) const STATUS_TEXT_Y: usize = 5;
const BLOCK_COUNT: usize = 5;
pub(super) const POINTER_PRESET_INDEX: usize = 1;
pub(super) const FOCUS_PRESET_INDEX: usize = 2;
pub(super) const RICH_PRESET_INDEX: usize = 3;
pub(super) const ACTIONS_PRESET_INDEX: usize = 4;

pub(super) fn hover_card(
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
        "Hover card",
        &blocks(palette, scenario),
        &dedicated_hover_card_labels::labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            ANCHOR_X,
            ANCHOR_Y,
            ANCHOR_WIDTH,
            ANCHOR_HEIGHT,
            anchor_fill(palette, scenario),
        ),
        Block::new(
            POINTER_X,
            POINTER_Y,
            POINTER_SIZE,
            POINTER_SIZE,
            pointer_fill(palette, scenario),
        ),
        Block::outlined(
            card_x(scenario),
            CARD_Y,
            CARD_WIDTH,
            CARD_HEIGHT,
            card_fill(palette, scenario),
        ),
        Block::outlined(
            ACTION_X,
            ACTION_Y,
            ACTION_WIDTH,
            ACTION_HEIGHT,
            action_fill(palette, scenario),
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

pub(super) fn card_x(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == POINTER_PRESET_INDEX {
        return FOLLOW_CARD_X;
    }
    CARD_X
}

fn anchor_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FOCUS_PRESET_INDEX || scenario.screen_state.has_settings_override()
    {
        return palette.accent;
    }
    palette.surface
}

fn pointer_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == POINTER_PRESET_INDEX {
        return common::TOKEN;
    }
    card_fill(palette, scenario)
}

fn card_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() || scenario.preset_index == RICH_PRESET_INDEX {
        return palette.accent;
    }
    palette.surface
}

fn action_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ACTIONS_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn anchor_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FOCUS_PRESET_INDEX || scenario.screen_state.has_settings_override()
    {
        return palette.background;
    }
    palette.text
}

pub(super) fn card_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.preset_index == RICH_PRESET_INDEX {
        return palette.background;
    }
    palette.text
}

pub(super) fn action_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ACTIONS_PRESET_INDEX {
        return palette.background;
    }
    palette.muted
}

pub(super) fn anchor_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == FOCUS_PRESET_INDEX {
        return "Focus";
    }
    "Anchor"
}

pub(super) fn body_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == POINTER_PRESET_INDEX {
        return "Follows pointer";
    }
    "Rich hover content"
}

pub(super) fn footer_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == RICH_PRESET_INDEX {
        return "Card keeps focus";
    }
    "Delayed close"
}

pub(super) fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "open=false";
    }
    scenario.screen_state.state_label
}
