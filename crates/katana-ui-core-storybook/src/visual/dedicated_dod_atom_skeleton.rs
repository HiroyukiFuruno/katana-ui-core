use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const AVATAR_PRESET: usize = 1;
const RECT_PRESET: usize = 2;
const WAVE_PRESET: usize = 3;
const REDUCED_MOTION_PRESET: usize = 4;
const TONE_RADIUS_PRESET: usize = 5;
const BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 2;

pub(super) fn skeleton(
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
        "Skeleton loader",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            m::PX_18,
            m::PX_34,
            primary_width(scenario),
            m::PX_18,
            primary_fill(palette, scenario),
        ),
        Block::new(
            m::PX_18,
            m::PX_60,
            secondary_width(scenario),
            m::PX_12,
            secondary_fill(palette, scenario),
        ),
        Block::new(
            m::PX_18,
            m::PX_80,
            tertiary_width(scenario),
            m::PX_10,
            tertiary_fill(palette, scenario),
        ),
        Block::outlined(
            m::PX_258,
            m::PX_34,
            avatar_size(scenario),
            avatar_size(scenario),
            avatar_fill(palette, scenario),
        ),
        Block::new(
            m::PX_258,
            m::PX_88,
            status_width(scenario),
            m::PX_10,
            status_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            m::PX_34,
            m::PX_104,
            m::FONT_9,
            palette.muted,
            shape_label(scenario),
        ),
        TextSpec::new(
            m::PX_198,
            m::PX_104,
            m::FONT_9,
            palette.muted,
            motion_label(scenario),
        ),
    ]
}

fn primary_width(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        AVATAR_PRESET => m::PX_134,
        RECT_PRESET => m::PX_230,
        WAVE_PRESET => m::PX_188,
        _ => m::PX_214,
    }
}

fn secondary_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == RECT_PRESET {
        return m::PX_198;
    }
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return m::PX_150;
    }
    m::PX_174
}

fn tertiary_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == AVATAR_PRESET {
        return m::PX_104;
    }
    if scenario.preset_index == TONE_RADIUS_PRESET {
        return m::PX_188;
    }
    m::PX_142
}

fn avatar_size(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == AVATAR_PRESET {
        return m::PX_52;
    }
    m::PX_44
}

fn status_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == WAVE_PRESET {
        return m::PX_128;
    }
    m::PX_110
}

fn primary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() || scenario.screen_state.has_widget_action() {
        return common::SUCCESS;
    }
    if scenario.preset_index == TONE_RADIUS_PRESET {
        return common::WARN;
    }
    palette.surface
}

fn secondary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return palette.panel;
    }
    if scenario.preset_index == WAVE_PRESET {
        return common::TOKEN;
    }
    palette.surface
}

fn tertiary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == RECT_PRESET {
        return common::PURPLE;
    }
    palette.surface
}

fn avatar_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == AVATAR_PRESET {
        return palette.accent;
    }
    palette.panel
}

fn status_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == TONE_RADIUS_PRESET {
        return common::TOKEN;
    }
    palette.accent
}

fn shape_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        AVATAR_PRESET => "shape=avatar",
        RECT_PRESET => "shape=rect",
        WAVE_PRESET => "shape=line wave",
        _ => "shape=text lines",
    }
}

fn motion_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return "reduced motion=true";
    }
    if scenario.preset_index == TONE_RADIUS_PRESET {
        return "tone=warning radius=14";
    }
    "animation=shimmer"
}
