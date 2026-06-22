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
    if form_field_invalid(scenario) {
        return palette.background;
    }
    palette.muted
}

fn label_text(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == THEME_PRESET {
        return "Theme field";
    }
    if scenario.screen_state.last_setting == "form_field.required" {
        return "Repository name *";
    }
    "Repository name"
}

fn value_text(scenario: ScenarioContext<'_>) -> &'static str {
    if form_field_invalid(scenario) {
        return "katana-ui-core";
    }
    "katana"
}

fn helper_text(scenario: ScenarioContext<'_>) -> &'static str {
    if form_field_invalid(scenario) {
        return "Repository name is required";
    }
    if scenario.preset_index == HELPER_PRESET
        || scenario.screen_state.last_setting == "form_field.helper_text"
    {
        return "Used for release notes and package metadata";
    }
    "Visible helper text"
}

fn state_text(scenario: ScenarioContext<'_>) -> &'static str {
    if form_field_invalid(scenario) {
        return "invalid";
    }
    if scenario.screen_state.last_action == "form_field_focus_link" {
        return "focused";
    }
    if scenario.screen_state.last_setting == "form_field.required" {
        return "required";
    }
    "valid"
}

fn form_field_invalid(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == INVALID_PRESET
        || scenario.screen_state.last_action == "field_validate"
        || scenario.screen_state.last_setting == "form_field.invalid"
}
