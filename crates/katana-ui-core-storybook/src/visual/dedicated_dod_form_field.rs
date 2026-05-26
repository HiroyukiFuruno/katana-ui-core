use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_form_field_labels;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const INVALID_PRESET: usize = 1;
const HELPER_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const BLOCK_COUNT: usize = 5;

pub(super) fn form_field(
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
        "Form field",
        &blocks(palette, scenario),
        &dedicated_dod_form_field_labels::labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            m::PX_18,
            m::PX_38,
            m::PX_252,
            m::PX_34,
            field_fill(palette, scenario),
        ),
        Block::new(
            m::PX_24,
            m::PX_44,
            label_marker_width(scenario),
            m::PX_4,
            label_marker_fill(palette, scenario),
        ),
        Block::new(
            m::PX_24,
            m::PX_64,
            value_width(scenario),
            m::PX_4,
            value_fill(palette, scenario),
        ),
        Block::outlined(
            m::PX_18,
            m::PX_82,
            helper_width(scenario),
            m::PX_22,
            helper_fill(palette, scenario),
        ),
        Block::new(
            m::PX_284,
            m::PX_42,
            m::PX_80,
            m::PX_44,
            status_fill(palette, scenario),
        ),
    ]
}

fn field_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET {
        palette.panel
    } else {
        palette.surface
    }
}

fn label_marker_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == HELPER_PRESET {
        m::PX_120
    } else {
        m::PX_82
    }
}

fn label_marker_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == INVALID_PRESET {
        common::DANGER
    } else {
        palette.accent
    }
}

fn value_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == INVALID_PRESET {
        return m::PX_174;
    }
    if scenario.screen_state.has_widget_action() {
        return m::PX_204;
    }
    m::PX_142
}

fn value_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() {
        common::SUCCESS
    } else {
        palette.text
    }
}

fn helper_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == HELPER_PRESET {
        return m::PX_230;
    }
    m::PX_166
}

fn helper_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == INVALID_PRESET {
        return common::DANGER;
    }
    palette.panel
}

fn status_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == INVALID_PRESET || scenario.screen_state.has_widget_action() {
        return common::DANGER;
    }
    if scenario.preset_index == THEME_PRESET {
        return common::TOKEN;
    }
    palette.accent
}
