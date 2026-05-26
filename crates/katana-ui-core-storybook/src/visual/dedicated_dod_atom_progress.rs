use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PROGRESS_CHANGE_PRESET: usize = 1;
const PROGRESS_EMPTY_PRESET: usize = 2;
const PROGRESS_THEME_PRESET: usize = 3;

pub(super) fn progress(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let progress_width = progress_width(scenario);
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "ProgressBar",
        &[
            Block::outlined(
                m::PX_20,
                m::PX_44,
                m::PX_244,
                m::PX_18,
                track_fill(palette, scenario),
            ),
            Block::new(
                m::PX_22,
                m::PX_46,
                progress_width,
                m::PX_14,
                progress_fill(palette, scenario),
            ),
            Block::new(
                m::PX_22,
                m::PX_72,
                m::PX_244,
                m::PX_8,
                track_fill(palette, scenario),
            ),
        ],
        &[
            TextSpec::new(
                m::PX_278,
                m::PX_46,
                m::FONT_9,
                palette.muted,
                percent_label(scenario),
            ),
            TextSpec::new(
                m::PX_20,
                m::PX_90,
                m::FONT_9,
                palette.muted,
                state_label(scenario),
            ),
        ],
    );
}

fn progress_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return m::PX_0;
    }
    if scenario.screen_state.has_settings_override()
        || scenario.screen_state.has_widget_action()
        || scenario.preset_index == PROGRESS_CHANGE_PRESET
    {
        return m::PX_204;
    }
    if scenario.preset_index == PROGRESS_THEME_PRESET {
        return m::PX_198;
    }
    m::PX_164
}

fn progress_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PROGRESS_THEME_PRESET {
        return common::TOKEN;
    }
    palette.accent
}

fn track_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PROGRESS_THEME_PRESET {
        return palette.panel;
    }
    palette.surface
}

fn percent_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return "0%";
    }
    if scenario.preset_index == PROGRESS_CHANGE_PRESET || scenario.screen_state.has_widget_action()
    {
        return "82%";
    }
    "65%"
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return "empty progress";
    }
    if scenario.preset_index == PROGRESS_THEME_PRESET {
        return "theme track token";
    }
    "determinate / indeterminate"
}
