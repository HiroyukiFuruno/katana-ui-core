use super::dedicated_dod_common::TextSpec;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;

const INVALID_PRESET: usize = 1;
const HELPER_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const LABEL_COUNT: usize = 4;

pub(super) fn labels(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            m::PX_24,
            m::PX_26,
            m::FONT_9,
            palette.muted,
            label_text(scenario),
        ),
        TextSpec::new(
            m::PX_32,
            m::PX_48,
            m::FONT_9,
            palette.text,
            value_text(scenario),
        ),
        TextSpec::new(
            m::PX_24,
            m::PX_90,
            m::FONT_8,
            helper_text_color(palette, scenario),
            helper_text(scenario),
        ),
        TextSpec::new(
            m::PX_284,
            m::PX_58,
            m::FONT_8,
            palette.background,
            state_text(scenario),
        ),
    ]
}

fn helper_text_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == INVALID_PRESET {
        return palette.background;
    }
    palette.muted
}

fn label_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == THEME_PRESET {
        return "Theme field";
    }
    "Repository name"
}

fn value_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "katana-ui-core";
    }
    "katana"
}

fn helper_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == INVALID_PRESET {
        return "required";
    }
    if scenario.preset_index == HELPER_PRESET {
        return "Used for release notes and package metadata";
    }
    "Visible helper text"
}

fn state_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == INVALID_PRESET || scenario.screen_state.has_widget_action() {
        return "invalid";
    }
    "valid"
}
