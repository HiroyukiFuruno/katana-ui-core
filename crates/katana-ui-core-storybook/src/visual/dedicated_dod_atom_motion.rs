use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PHASE_PRESET: usize = 1;
const PAUSED_PRESET: usize = 2;
const THEME_LABEL_PRESET: usize = 3;
const SPEED_PRESET: usize = 4;
const SEGMENT_COUNT_PRESET: usize = 5;
const TONE_PRESET: usize = 6;
const SIZE_PRESET: usize = 7;
const SPINNER_BLOCK_COUNT: usize = 6;
#[cfg(test)]
const SPINNER_LABEL_COUNT: usize = 3;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SpinnerBlockSnapshot {
    pub(super) rect: Rect,
    pub(super) fill: u32,
}

pub(super) fn spinner(
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
        "Spinner",
        &spinner_blocks(palette, scenario),
        &[
            TextSpec::new(
                m::PX_128,
                m::PX_42,
                m::FONT_9,
                palette.muted,
                tick_label(scenario),
            ),
            TextSpec::new(
                m::PX_128,
                m::PX_60,
                m::FONT_9,
                palette.muted,
                motion_label(scenario),
            ),
            TextSpec::new(
                m::PX_128,
                m::PX_78,
                m::FONT_9,
                palette.muted,
                label_text(scenario),
            ),
        ],
    );
}

fn spinner_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [Block; SPINNER_BLOCK_COUNT] {
    [
        Block::outlined(
            m::PX_34,
            m::PX_38,
            m::PX_72,
            container_height(scenario),
            container_fill(palette, scenario),
        ),
        Block::new(
            m::PX_62,
            m::PX_40,
            segment_width(scenario),
            m::PX_6,
            leading_color(palette, scenario),
        ),
        Block::new(
            m::PX_70,
            m::PX_47,
            segment_width(scenario),
            m::PX_6,
            token_color(palette, scenario),
        ),
        Block::new(
            m::PX_78,
            m::PX_54,
            segment_width(scenario),
            m::PX_6,
            purple_color(palette, scenario),
        ),
        Block::new(
            m::PX_86,
            m::PX_61,
            segment_width(scenario),
            m::PX_6,
            warn_color(scenario),
        ),
        Block::new(
            m::PX_94,
            m::PX_68,
            extra_segment_width(scenario),
            m::PX_6,
            extra_segment_color(palette, scenario),
        ),
    ]
}

#[cfg(test)]
pub(super) fn spinner_blocks_for_test(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [SpinnerBlockSnapshot; SPINNER_BLOCK_COUNT] {
    [
        SpinnerBlockSnapshot {
            rect: Rect::new(m::PX_34, m::PX_38, m::PX_72, container_height(scenario)),
            fill: container_fill(palette, scenario),
        },
        SpinnerBlockSnapshot {
            rect: Rect::new(m::PX_62, m::PX_40, segment_width(scenario), m::PX_6),
            fill: leading_color(palette, scenario),
        },
        SpinnerBlockSnapshot {
            rect: Rect::new(m::PX_70, m::PX_47, segment_width(scenario), m::PX_6),
            fill: token_color(palette, scenario),
        },
        SpinnerBlockSnapshot {
            rect: Rect::new(m::PX_78, m::PX_54, segment_width(scenario), m::PX_6),
            fill: purple_color(palette, scenario),
        },
        SpinnerBlockSnapshot {
            rect: Rect::new(m::PX_86, m::PX_61, segment_width(scenario), m::PX_6),
            fill: warn_color(scenario),
        },
        SpinnerBlockSnapshot {
            rect: Rect::new(m::PX_94, m::PX_68, extra_segment_width(scenario), m::PX_6),
            fill: extra_segment_color(palette, scenario),
        },
    ]
}

#[cfg(test)]
pub(super) fn spinner_labels_for_test(
    scenario: ScenarioContext<'_>,
) -> [&'static str; SPINNER_LABEL_COUNT] {
    [
        tick_label(scenario),
        motion_label(scenario),
        label_text(scenario),
    ]
}

fn container_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAUSED_PRESET || scenario.preset_index == SIZE_PRESET {
        return palette.surface;
    }
    palette.panel
}

fn container_height(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == SIZE_PRESET {
        return m::PX_56;
    }
    m::PX_50
}

fn segment_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == SPEED_PRESET {
        return m::PX_22;
    }
    m::PX_16
}

fn extra_segment_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == SEGMENT_COUNT_PRESET {
        return m::PX_16;
    }
    m::PX_0
}

fn extra_segment_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SEGMENT_COUNT_PRESET {
        return palette.accent;
    }
    palette.panel
}

fn leading_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override()
        || scenario.screen_state.has_widget_action()
        || scenario.preset_index == PHASE_PRESET
        || scenario.preset_index == TONE_PRESET
    {
        return common::SUCCESS;
    }
    if scenario.preset_index == THEME_LABEL_PRESET {
        return common::TOKEN;
    }
    palette.accent
}

fn token_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAUSED_PRESET {
        return palette.panel;
    }
    common::TOKEN
}

fn purple_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_LABEL_PRESET {
        return palette.text;
    }
    common::PURPLE
}

fn warn_color(scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAUSED_PRESET || scenario.preset_index == TONE_PRESET {
        return common::SUCCESS;
    }
    common::WARN
}

fn tick_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PHASE_PRESET || scenario.screen_state.has_widget_action() {
        return "motion tick: 7/12";
    }
    if scenario.preset_index == SPEED_PRESET {
        return "speed=96ms";
    }
    if scenario.preset_index == SEGMENT_COUNT_PRESET {
        return "segments=5";
    }
    "motion tick: 6/12"
}

fn motion_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PAUSED_PRESET {
        return "reduced motion: on";
    }
    if scenario.preset_index == TONE_PRESET {
        return "tone=accent";
    }
    if scenario.preset_index == SIZE_PRESET {
        return "size=large";
    }
    "reduced motion: paused"
}

fn label_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == THEME_LABEL_PRESET {
        return "label: Theme token";
    }
    "label: Saving"
}
