use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const REDUCED_PRESET: usize = 1;
const FORCE_IGNORE_PRESET: usize = 2;
const PER_MOLECULE_PRESET: usize = 3;
const BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 3;

pub(super) fn motion(
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
        "Motion primitives",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::new(
            m::PX_22,
            m::PX_38,
            fade_width(scenario),
            m::PX_12,
            primary_fill(palette, scenario),
        ),
        Block::outlined(
            m::PX_22,
            m::PX_60,
            m::PX_198,
            m::PX_12,
            slide_track(palette, scenario),
        ),
        Block::new(
            slide_offset(scenario),
            m::PX_62,
            m::PX_44,
            m::PX_8,
            primary_fill(palette, scenario),
        ),
        Block::outlined(
            m::PX_258,
            m::PX_38,
            scale_size(scenario),
            scale_size(scenario),
            scale_fill(palette, scenario),
        ),
        Block::new(
            m::PX_258,
            m::PX_92,
            shimmer_width(scenario),
            m::PX_10,
            shimmer_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            m::PX_34,
            m::PX_82,
            m::FONT_9,
            palette.muted,
            primitive_label(scenario),
        ),
        TextSpec::new(
            m::PX_198,
            m::PX_82,
            m::FONT_9,
            palette.muted,
            policy_label(scenario),
        ),
        TextSpec::new(
            m::PX_34,
            m::PX_108,
            m::FONT_9,
            palette.muted,
            state_label(scenario),
        ),
    ]
}

fn fade_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == REDUCED_PRESET {
        return m::PX_80;
    }
    if scenario.preset_index == PER_MOLECULE_PRESET {
        return m::PX_176;
    }
    m::PX_134
}

fn slide_offset(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == FORCE_IGNORE_PRESET {
        return m::PX_118;
    }
    if scenario.screen_state.has_widget_action() {
        return m::PX_142;
    }
    m::PX_54
}

fn scale_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == REDUCED_PRESET {
        return m::PX_34;
    }
    m::PX_44
}

fn shimmer_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PER_MOLECULE_PRESET {
        return m::PX_150;
    }
    m::PX_96
}

fn primary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() || scenario.screen_state.has_widget_action() {
        return common::SUCCESS;
    }
    if scenario.preset_index == FORCE_IGNORE_PRESET {
        return common::WARN;
    }
    palette.accent
}

fn slide_track(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REDUCED_PRESET {
        return palette.panel;
    }
    palette.surface
}

fn scale_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REDUCED_PRESET {
        return palette.panel;
    }
    palette.surface
}

fn shimmer_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PER_MOLECULE_PRESET {
        return common::TOKEN;
    }
    palette.panel
}

fn primitive_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PER_MOLECULE_PRESET {
        return "primitive=shimmer";
    }
    "primitive=fade+slide"
}

fn policy_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == FORCE_IGNORE_PRESET {
        return "policy=ignore";
    }
    "policy=respect"
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == REDUCED_PRESET {
        return "instant=true";
    }
    if scenario.screen_state.has_widget_action() {
        return "phase=tick";
    }
    "duration=200 distance=8"
}
