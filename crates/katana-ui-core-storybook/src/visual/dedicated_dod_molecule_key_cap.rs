use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, ChipSpec, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const STATUS_Y: usize = 92;
const STATUS_WIDTH: usize = 92;
const STATUS_HEIGHT: usize = 16;
const STATUS_GAP: usize = 8;
const COMBO_PRESET: usize = 1;
const NON_MACOS_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const LABEL_MACOS: &str = "macOS";
const LABEL_NON_MACOS: &str = "Windows/Linux";
const KEY_WIDTH: usize = m::PX_46;
const KEY_HEIGHT: usize = m::PX_24;
const KEY_X_0: usize = m::PX_98;
const KEY_X_1: usize = m::PX_152;
const KEY_X_2: usize = m::PX_206;
const MAC_KEY_Y: usize = m::PX_32;
const NON_MAC_KEY_Y: usize = m::PX_62;
const MAC_LABEL_Y: usize = m::PX_39;
const NON_MAC_LABEL_Y: usize = m::PX_69;
const MAC_MODIFIER_X: usize = m::PX_110;
const MAC_SHIFT_X: usize = m::PX_164;
const KEY_LETTER_X: usize = m::PX_218;
const NON_MAC_MODIFIER_X: usize = m::PX_108;
const NON_MAC_SHIFT_X: usize = m::PX_160;
const KEY_TEXT_SIZE: f32 = m::FONT_9;
#[cfg(test)]
const KEY_LABEL_COUNT: usize = 6;
#[cfg(test)]
const KEY_RECT_COUNT: usize = 6;
#[cfg(test)]
const MAC_MODIFIER_INDEX: usize = 0;
#[cfg(test)]
const MAC_SHIFT_INDEX: usize = 1;
#[cfg(test)]
const MAC_KEY_INDEX: usize = 2;
#[cfg(test)]
const NON_MAC_MODIFIER_INDEX: usize = 3;
#[cfg(test)]
const NON_MAC_SHIFT_INDEX: usize = 4;
#[cfg(test)]
const NON_MAC_KEY_INDEX: usize = 5;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct KeyCapLabelFit {
    pub(super) value: &'static str,
    pub(super) rect: Rect,
    pub(super) text_x: usize,
    pub(super) size: f32,
}

pub(super) fn key_cap(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let mac_fill = active_platform_fill(palette, scenario);
    let win_fill = inactive_platform_fill(palette, scenario);
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "KeyCap platform",
        &[
            Block::outlined(KEY_X_0, MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT, mac_fill),
            Block::outlined(KEY_X_1, MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT, mac_fill),
            Block::outlined(KEY_X_2, MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT, mac_fill),
            Block::outlined(KEY_X_0, NON_MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT, win_fill),
            Block::outlined(KEY_X_1, NON_MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT, win_fill),
            Block::outlined(KEY_X_2, NON_MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT, win_fill),
        ],
        &[
            TextSpec::new(m::PX_18, m::PX_38, m::FONT_8, palette.muted, LABEL_MACOS),
            TextSpec::new(
                MAC_MODIFIER_X,
                MAC_LABEL_Y,
                KEY_TEXT_SIZE,
                palette.text,
                "⌘",
            ),
            TextSpec::new(MAC_SHIFT_X, MAC_LABEL_Y, KEY_TEXT_SIZE, palette.text, "⇧"),
            TextSpec::new(KEY_LETTER_X, MAC_LABEL_Y, KEY_TEXT_SIZE, palette.text, "K"),
            TextSpec::new(
                m::PX_18,
                m::PX_68,
                m::FONT_8,
                palette.muted,
                LABEL_NON_MACOS,
            ),
            TextSpec::new(
                NON_MAC_MODIFIER_X,
                NON_MAC_LABEL_Y,
                KEY_TEXT_SIZE,
                palette.text,
                "Ctrl",
            ),
            TextSpec::new(
                NON_MAC_SHIFT_X,
                NON_MAC_LABEL_Y,
                KEY_TEXT_SIZE,
                palette.text,
                "Shift",
            ),
            TextSpec::new(
                KEY_LETTER_X,
                NON_MAC_LABEL_Y,
                KEY_TEXT_SIZE,
                palette.text,
                "K",
            ),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
}

#[cfg(test)]
pub(super) fn key_cap_rects_for_test() -> [Rect; KEY_RECT_COUNT] {
    [
        Rect::new(KEY_X_0, MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT),
        Rect::new(KEY_X_1, MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT),
        Rect::new(KEY_X_2, MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT),
        Rect::new(KEY_X_0, NON_MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT),
        Rect::new(KEY_X_1, NON_MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT),
        Rect::new(KEY_X_2, NON_MAC_KEY_Y, KEY_WIDTH, KEY_HEIGHT),
    ]
}

#[cfg(test)]
pub(super) fn key_cap_label_fits_for_test() -> [KeyCapLabelFit; KEY_LABEL_COUNT] {
    let rects = key_cap_rects_for_test();
    [
        KeyCapLabelFit {
            value: "⌘",
            rect: rects[MAC_MODIFIER_INDEX],
            text_x: MAC_MODIFIER_X,
            size: KEY_TEXT_SIZE,
        },
        KeyCapLabelFit {
            value: "⇧",
            rect: rects[MAC_SHIFT_INDEX],
            text_x: MAC_SHIFT_X,
            size: KEY_TEXT_SIZE,
        },
        KeyCapLabelFit {
            value: "K",
            rect: rects[MAC_KEY_INDEX],
            text_x: KEY_LETTER_X,
            size: KEY_TEXT_SIZE,
        },
        KeyCapLabelFit {
            value: "Ctrl",
            rect: rects[NON_MAC_MODIFIER_INDEX],
            text_x: NON_MAC_MODIFIER_X,
            size: KEY_TEXT_SIZE,
        },
        KeyCapLabelFit {
            value: "Shift",
            rect: rects[NON_MAC_SHIFT_INDEX],
            text_x: NON_MAC_SHIFT_X,
            size: KEY_TEXT_SIZE,
        },
        KeyCapLabelFit {
            value: "K",
            rect: rects[NON_MAC_KEY_INDEX],
            text_x: KEY_LETTER_X,
            size: KEY_TEXT_SIZE,
        },
    ]
}

fn active_platform_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        return common::SUCCESS;
    }
    if scenario.preset_index == NON_MACOS_PRESET {
        return palette.panel;
    }
    if scenario.screen_state.has_widget_action() || scenario.preset_index == COMBO_PRESET {
        return palette.accent;
    }
    if scenario.preset_index == THEME_PRESET {
        return common::TOKEN;
    }
    palette.surface
}

fn inactive_platform_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == NON_MACOS_PRESET {
        return palette.accent;
    }
    if scenario.screen_state.has_widget_action() {
        return palette.panel;
    }
    if scenario.preset_index == THEME_PRESET {
        return palette.text;
    }
    palette.surface
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::draw_chips(
        canvas,
        text,
        palette,
        x,
        y,
        &[
            ChipSpec::new(
                m::PX_18,
                STATUS_Y,
                STATUS_WIDTH,
                STATUS_HEIGHT,
                action_label(scenario),
                common::SUCCESS,
            ),
            ChipSpec::new(
                m::PX_18 + STATUS_WIDTH + STATUS_GAP,
                STATUS_Y,
                STATUS_WIDTH,
                STATUS_HEIGHT,
                event_label(scenario),
                common::TOKEN,
            ),
            ChipSpec::new(
                m::PX_18 + (STATUS_WIDTH + STATUS_GAP) * 2,
                STATUS_Y,
                STATUS_WIDTH,
                STATUS_HEIGHT,
                state_label(scenario),
                palette.accent,
            ),
        ],
    );
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "shortcut ready";
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
    if scenario.preset_index == COMBO_PRESET {
        return "combo=shown";
    }
    if scenario.preset_index == NON_MACOS_PRESET {
        return "platform=nonmac";
    }
    if scenario.preset_index == THEME_PRESET {
        return "theme=key";
    }
    if scenario.screen_state.state_label == "idle" {
        return "platform=ready";
    }
    scenario.screen_state.state_label
}
