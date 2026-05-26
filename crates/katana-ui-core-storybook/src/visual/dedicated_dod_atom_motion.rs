use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PHASE_PRESET: usize = 1;
const PAUSED_PRESET: usize = 2;
const THEME_LABEL_PRESET: usize = 3;

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
        &[
            Block::outlined(
                m::PX_34,
                m::PX_38,
                m::PX_72,
                m::PX_50,
                container_fill(palette, scenario),
            ),
            Block::new(
                m::PX_62,
                m::PX_40,
                m::PX_16,
                m::PX_6,
                leading_color(palette, scenario),
            ),
            Block::new(
                m::PX_70,
                m::PX_47,
                m::PX_16,
                m::PX_6,
                token_color(palette, scenario),
            ),
            Block::new(
                m::PX_78,
                m::PX_54,
                m::PX_16,
                m::PX_6,
                purple_color(palette, scenario),
            ),
            Block::new(m::PX_86, m::PX_61, m::PX_16, m::PX_6, warn_color(scenario)),
        ],
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

fn container_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PAUSED_PRESET {
        return palette.surface;
    }
    palette.panel
}

fn leading_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override()
        || scenario.screen_state.has_widget_action()
        || scenario.preset_index == PHASE_PRESET
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
    if scenario.preset_index == PAUSED_PRESET {
        return common::SUCCESS;
    }
    common::WARN
}

fn tick_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PHASE_PRESET || scenario.screen_state.has_widget_action() {
        return "motion tick: 7/12";
    }
    "motion tick: 6/12"
}

fn motion_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PAUSED_PRESET {
        return "reduced motion: on";
    }
    "reduced motion: paused"
}

fn label_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == THEME_LABEL_PRESET {
        return "label: Theme token";
    }
    "label: Saving"
}
