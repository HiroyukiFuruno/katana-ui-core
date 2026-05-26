use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PHASE_PRESET: usize = 1;
const REDUCED_MOTION_PRESET: usize = 2;
const THEME_LABEL_PRESET: usize = 3;
const DOT_BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 2;

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
    let second_y = if scenario.preset_index == PHASE_PRESET {
        m::PX_48
    } else {
        m::PX_45
    };
    let third_fill = if scenario.preset_index == REDUCED_MOTION_PRESET {
        palette.panel
    } else {
        common::PURPLE
    };

    [
        Block::new(
            m::PX_32,
            m::PX_48,
            first_size,
            first_size,
            active_color(palette, scenario),
        ),
        Block::new(m::PX_56, second_y, m::PX_8, m::PX_8, common::TOKEN),
        Block::new(m::PX_80, m::PX_48, m::PX_10, m::PX_10, third_fill),
        Block::new(m::PX_104, m::PX_45, m::PX_8, m::PX_8, common::WARN),
        Block::outlined(
            m::PX_188,
            m::PX_38,
            m::PX_96,
            m::PX_20,
            reduced_motion_fill(palette, scenario),
        ),
    ]
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

fn first_dot_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PHASE_PRESET {
        return m::PX_10;
    }
    m::PX_6
}

fn active_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() || scenario.screen_state.has_widget_action() {
        return common::SUCCESS;
    }
    if scenario.preset_index == THEME_LABEL_PRESET {
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
    "phase=3 speed=fast label=Loading"
}
