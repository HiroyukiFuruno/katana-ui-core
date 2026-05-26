use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_modal_labels;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const BACKDROP_X: usize = m::PX_18;
const BACKDROP_Y: usize = m::PX_32;
const BACKDROP_WIDTH: usize = m::PX_148;
const BACKDROP_HEIGHT: usize = m::PX_68;
pub(super) const DIALOG_X: usize = m::PX_38;
pub(super) const DIALOG_Y: usize = m::PX_42;
const DIALOG_WIDTH: usize = m::PX_108;
const DIALOG_HEIGHT: usize = m::PX_46;
pub(super) const NATIVE_X: usize = m::PX_188;
pub(super) const NATIVE_Y: usize = m::PX_38;
pub(super) const NATIVE_WIDTH: usize = m::PX_118;
const NATIVE_HEIGHT: usize = m::PX_54;
const CLOSE_X: usize = m::PX_58;
const CLOSE_Y: usize = m::PX_66;
const CLOSE_WIDTH: usize = m::PX_58;
const CLOSE_HEIGHT: usize = m::PX_18;
pub(super) const STATUS_X: usize = m::PX_20;
pub(super) const STATUS_Y: usize = m::PX_96;
pub(super) const STATUS_WIDTH: usize = m::PX_96;
pub(super) const STATUS_GAP: usize = m::PX_8;
pub(super) const STATUS_TEXT_X: usize = 7;
pub(super) const STATUS_TEXT_Y: usize = 5;
pub(super) const LABEL_X_OFFSET: usize = m::PX_10;
pub(super) const FIRST_LABEL_Y_OFFSET: usize = m::PX_8;
pub(super) const LABEL_GAP: usize = m::PX_16;
const BLOCK_COUNT: usize = 4;
const ESCAPE_PRESET_INDEX: usize = 1;
pub(super) const FOCUS_PRESET_INDEX: usize = 2;
const PARENT_BLOCK_PRESET_INDEX: usize = 3;
const TITLE_SIZE_PRESET_INDEX: usize = 4;

pub(super) fn modal(
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
        "Modal / Overlay",
        &blocks(palette, scenario),
        &dedicated_modal_labels::labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::new(
            BACKDROP_X,
            BACKDROP_Y,
            BACKDROP_WIDTH,
            BACKDROP_HEIGHT,
            backdrop_fill(scenario),
        ),
        Block::outlined(
            DIALOG_X,
            DIALOG_Y,
            dialog_width(scenario),
            DIALOG_HEIGHT,
            dialog_fill(palette, scenario),
        ),
        Block::outlined(
            NATIVE_X,
            NATIVE_Y,
            NATIVE_WIDTH,
            NATIVE_HEIGHT,
            native_fill(palette, scenario),
        ),
        Block::outlined(
            CLOSE_X,
            CLOSE_Y,
            CLOSE_WIDTH,
            CLOSE_HEIGHT,
            close_fill(palette, scenario),
        ),
    ]
}

fn backdrop_fill(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PARENT_BLOCK_PRESET_INDEX {
        return common::PURPLE;
    }
    m::COLOR_MODAL_BACKDROP
}

fn dialog_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == TITLE_SIZE_PRESET_INDEX {
        return NATIVE_WIDTH;
    }
    DIALOG_WIDTH
}

fn dialog_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::WARN;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.accent;
    }
    if scenario.preset_index == ESCAPE_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

fn native_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FOCUS_PRESET_INDEX {
        return palette.accent;
    }
    palette.panel
}

fn close_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == ESCAPE_PRESET_INDEX {
        return common::DANGER;
    }
    palette.accent
}

pub(super) fn dialog_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return palette.background;
    }
    palette.text
}

pub(super) fn native_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == FOCUS_PRESET_INDEX {
        return palette.background;
    }
    palette.muted
}

pub(super) fn dialog_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == ESCAPE_PRESET_INDEX {
        return "Esc close";
    }
    "Overlay dialog"
}

pub(super) fn native_body_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == FOCUS_PRESET_INDEX {
        return "focus returns";
    }
    "focus trap / Esc"
}

pub(super) fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

pub(super) fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "open=true";
    }
    scenario.screen_state.state_label
}
