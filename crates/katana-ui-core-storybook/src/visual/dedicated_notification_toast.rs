use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_notification_toast_labels;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const TOAST_X: usize = 46;
const TOAST_Y: usize = 36;
const TOAST_WIDTH: usize = 248;
const TOAST_HEIGHT: usize = 48;
pub(super) const STACK_X: usize = 68;
pub(super) const STACK_Y: usize = 88;
const STACK_WIDTH: usize = 218;
const STACK_HEIGHT: usize = 18;
const BADGE_X: usize = TOAST_X + 12;
const BADGE_Y: usize = TOAST_Y + 12;
const BADGE_SIZE: usize = 24;
pub(super) const CLOSE_X: usize = TOAST_X + 212;
pub(super) const CLOSE_Y: usize = TOAST_Y + 14;
const CLOSE_SIZE: usize = 18;
pub(super) const STATUS_X: usize = 46;
pub(super) const STATUS_Y: usize = 108;
const STATUS_WIDTH: usize = 104;
const STATUS_HEIGHT: usize = 18;
pub(super) const STATUS_TEXT_X: usize = 7;
pub(super) const STATUS_TEXT_Y: usize = 5;
pub(super) const TEXT_X: usize = TOAST_X + 48;
pub(super) const TITLE_Y: usize = TOAST_Y + 11;
pub(super) const BODY_Y: usize = TOAST_Y + 28;
const BLOCK_COUNT: usize = 5;
pub(super) const DISMISS_PRESET_INDEX: usize = 1;
pub(super) const STACK_PRESET_INDEX: usize = 2;
pub(super) const THEME_PRESET_INDEX: usize = 3;

pub(super) fn notification_toast(
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
        "Notification toast",
        &blocks(palette, scenario),
        &dedicated_notification_toast_labels::labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            TOAST_X,
            TOAST_Y,
            TOAST_WIDTH,
            TOAST_HEIGHT,
            toast_fill(palette, scenario),
        ),
        Block::new(
            BADGE_X,
            BADGE_Y,
            BADGE_SIZE,
            BADGE_SIZE,
            badge_fill(scenario),
        ),
        Block::outlined(
            CLOSE_X,
            CLOSE_Y,
            CLOSE_SIZE,
            CLOSE_SIZE,
            close_fill(palette, scenario),
        ),
        Block::outlined(
            STACK_X,
            STACK_Y,
            STACK_WIDTH,
            STACK_HEIGHT,
            stack_fill(palette, scenario),
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

fn toast_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return palette.panel;
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.surface
}

fn badge_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return common::DANGER;
    }
    common::SUCCESS
}

fn close_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return common::DANGER;
    }
    palette.panel
}

fn stack_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == STACK_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

pub(super) fn toast_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() {
        return palette.background;
    }
    palette.text
}

pub(super) fn close_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return palette.background;
    }
    palette.muted
}

pub(super) fn title_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == DISMISS_PRESET_INDEX {
        return "Dismissed";
    }
    "Notification"
}

pub(super) fn body_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return "Theme toast token";
    }
    "Message"
}

pub(super) fn stack_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == STACK_PRESET_INDEX {
        return "ToastStackManager linked";
    }
    "single toast"
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "visible=true";
    }
    scenario.screen_state.state_label
}
