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
            Block::outlined(m::PX_98, m::PX_32, m::PX_46, m::PX_24, mac_fill),
            Block::outlined(m::PX_152, m::PX_32, m::PX_46, m::PX_24, mac_fill),
            Block::outlined(m::PX_206, m::PX_32, m::PX_46, m::PX_24, mac_fill),
            Block::outlined(m::PX_98, m::PX_62, m::PX_46, m::PX_24, win_fill),
            Block::outlined(m::PX_152, m::PX_62, m::PX_46, m::PX_24, win_fill),
            Block::outlined(m::PX_206, m::PX_62, m::PX_46, m::PX_24, win_fill),
        ],
        &[
            TextSpec::new(m::PX_18, m::PX_38, m::FONT_8, palette.muted, "macOS"),
            TextSpec::new(m::PX_110, m::PX_39, m::FONT_9, palette.text, "⌘"),
            TextSpec::new(m::PX_164, m::PX_39, m::FONT_9, palette.text, "⇧"),
            TextSpec::new(m::PX_218, m::PX_39, m::FONT_9, palette.text, "K"),
            TextSpec::new(
                m::PX_18,
                m::PX_68,
                m::FONT_8,
                palette.muted,
                "Windows/Linux",
            ),
            TextSpec::new(m::PX_108, m::PX_69, m::FONT_9, palette.text, "Ctrl"),
            TextSpec::new(m::PX_160, m::PX_69, m::FONT_9, palette.text, "Shift"),
            TextSpec::new(m::PX_218, m::PX_69, m::FONT_9, palette.text, "K"),
        ],
    );
    draw_status(canvas, text, palette, scenario, x, y);
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
