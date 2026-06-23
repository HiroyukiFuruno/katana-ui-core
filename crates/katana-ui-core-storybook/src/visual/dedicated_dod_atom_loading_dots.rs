use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PHASE_PRESET: usize = 1;
const REDUCED_MOTION_PRESET: usize = 2;
const THEME_LABEL_PRESET: usize = 3;
const SPEED_PRESET: usize = 4;
const DOT_COUNT_PRESET: usize = 5;
const TONE_PRESET: usize = 6;
const SIZE_PRESET: usize = 7;
const DOT_BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 2;
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoadingDotSnapshot {
    pub(super) rect: Rect,
    pub(super) fill: u32,
}

pub(super) fn loading_dots(
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
        "LoadingDots",
        &dot_blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn dot_blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; DOT_BLOCK_COUNT] {
    let first_size = first_dot_size(scenario);

    [
        Block::new(
            m::PX_32,
            m::PX_48,
            first_size,
            first_size,
            active_color(palette, scenario),
        ),
        Block::new(
            m::PX_56,
            second_dot_y(scenario),
            second_size(scenario),
            m::PX_8,
            common::TOKEN,
        ),
        Block::new(
            m::PX_80,
            m::PX_48,
            third_size(scenario),
            third_size(scenario),
            third_color(palette, scenario),
        ),
        Block::new(
            m::PX_104,
            m::PX_45,
            fourth_size(scenario),
            m::PX_8,
            fourth_color(palette, scenario),
        ),
        Block::outlined(
            m::PX_188,
            m::PX_38,
            m::PX_96,
            m::PX_20,
            reduced_motion_fill(palette, scenario),
        ),
    ]
}

#[cfg(test)]
pub(super) fn loading_dot_blocks_for_test(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [LoadingDotSnapshot; DOT_BLOCK_COUNT] {
    [
        LoadingDotSnapshot {
            rect: Rect::new(
                m::PX_32,
                m::PX_48,
                first_dot_size(scenario),
                first_dot_size(scenario),
            ),
            fill: active_color(palette, scenario),
        },
        LoadingDotSnapshot {
            rect: Rect::new(
                m::PX_56,
                second_dot_y(scenario),
                second_size(scenario),
                m::PX_8,
            ),
            fill: common::TOKEN,
        },
        LoadingDotSnapshot {
            rect: Rect::new(
                m::PX_80,
                m::PX_48,
                third_size(scenario),
                third_size(scenario),
            ),
            fill: third_color(palette, scenario),
        },
        LoadingDotSnapshot {
            rect: Rect::new(m::PX_104, m::PX_45, fourth_size(scenario), m::PX_8),
            fill: fourth_color(palette, scenario),
        },
        LoadingDotSnapshot {
            rect: Rect::new(m::PX_188, m::PX_38, m::PX_96, m::PX_20),
            fill: reduced_motion_fill(palette, scenario),
        },
    ]
}

#[cfg(test)]
pub(super) fn loading_dots_phase_label_for_test(scenario: ScenarioContext<'_>) -> &'static str {
    phase_label(scenario)
}

#[cfg(test)]
pub(super) fn loading_dots_motion_label_for_test(scenario: ScenarioContext<'_>) -> &'static str {
    motion_label(scenario)
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            m::PX_198,
            m::PX_43,
            m::FONT_9,
            palette.muted,
            motion_label(scenario),
        ),
        TextSpec::new(
            m::PX_34,
            m::PX_82,
            m::FONT_9,
            palette.muted,
            phase_label(scenario),
        ),
    ]
}

fn second_dot_y(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PHASE_PRESET || scenario.preset_index == SPEED_PRESET {
        return m::PX_48;
    }
    m::PX_45
}

fn third_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return palette.panel;
    }
    common::PURPLE
}

fn first_dot_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PHASE_PRESET || scenario.preset_index == SIZE_PRESET {
        return m::PX_10;
    }
    m::PX_6
}

fn second_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == SPEED_PRESET {
        return m::PX_14;
    }
    m::PX_8
}

fn third_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == DOT_COUNT_PRESET {
        return m::PX_14;
    }
    m::PX_10
}

fn fourth_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == DOT_COUNT_PRESET {
        return m::PX_16;
    }
    m::PX_8
}

fn fourth_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == TONE_PRESET {
        return palette.accent;
    }
    common::WARN
}

fn active_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() || scenario.screen_state.has_widget_action() {
        return common::SUCCESS;
    }
    if scenario.preset_index == THEME_LABEL_PRESET || scenario.preset_index == TONE_PRESET {
        return common::TOKEN;
    }
    palette.accent
}

fn reduced_motion_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return common::SUCCESS;
    }
    palette.surface
}

fn motion_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return "reduced motion on";
    }
    "reduced motion"
}

fn phase_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PHASE_PRESET || scenario.screen_state.has_widget_action() {
        return "phase=4 speed=fast label=Loading";
    }
    if scenario.preset_index == THEME_LABEL_PRESET {
        return "phase=3 speed=fast theme=accent";
    }
    if scenario.preset_index == SPEED_PRESET {
        return "speed=96ms";
    }
    if scenario.preset_index == DOT_COUNT_PRESET {
        return "dot_count=5";
    }
    if scenario.preset_index == TONE_PRESET {
        return "tone=accent";
    }
    if scenario.preset_index == SIZE_PRESET {
        return "size=large";
    }
    "phase=3 speed=fast label=Loading"
}
