use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const SHAPE_PRESET: usize = 0;
const TEXT_LINES_PRESET: usize = 1;
const LAST_LINE_RATIO_PRESET: usize = 2;
const LINE_THICKNESS_PRESET: usize = 3;
const SIZE_PRESET: usize = 4;
const ANIMATION_PRESET: usize = 5;
const TONE_PRESET: usize = 6;
const RADIUS_PRESET: usize = 7;
const REDUCED_MOTION_PRESET: usize = 8;
const A11Y_PRESET: usize = 9;
const ASPECT_RATIO_PRESET: usize = 10;
const BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 2;
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SkeletonBlockSnapshot {
    pub(super) rect: Rect,
    pub(super) fill: u32,
}

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
            primary_height(scenario),
            primary_fill(palette, scenario),
        ),
        Block::new(
            m::PX_18,
            m::PX_60,
            secondary_width(scenario),
            secondary_height(scenario),
            secondary_fill(palette, scenario),
        ),
        Block::new(
            m::PX_18,
            m::PX_80,
            tertiary_width(scenario),
            tertiary_height(scenario),
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

#[cfg(test)]
pub(super) fn skeleton_blocks_for_test(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [SkeletonBlockSnapshot; BLOCK_COUNT] {
    [
        SkeletonBlockSnapshot {
            rect: Rect::new(
                m::PX_18,
                m::PX_34,
                primary_width(scenario),
                primary_height(scenario),
            ),
            fill: primary_fill(palette, scenario),
        },
        SkeletonBlockSnapshot {
            rect: Rect::new(
                m::PX_18,
                m::PX_60,
                secondary_width(scenario),
                secondary_height(scenario),
            ),
            fill: secondary_fill(palette, scenario),
        },
        SkeletonBlockSnapshot {
            rect: Rect::new(
                m::PX_18,
                m::PX_80,
                tertiary_width(scenario),
                tertiary_height(scenario),
            ),
            fill: tertiary_fill(palette, scenario),
        },
        SkeletonBlockSnapshot {
            rect: Rect::new(
                m::PX_258,
                m::PX_34,
                avatar_size(scenario),
                avatar_size(scenario),
            ),
            fill: avatar_fill(palette, scenario),
        },
        SkeletonBlockSnapshot {
            rect: Rect::new(m::PX_258, m::PX_88, status_width(scenario), m::PX_10),
            fill: status_fill(palette, scenario),
        },
    ]
}

#[cfg(test)]
pub(super) fn skeleton_motion_label_for_test(scenario: ScenarioContext<'_>) -> &'static str {
    motion_label(scenario)
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
        SHAPE_PRESET => m::PX_134,
        LAST_LINE_RATIO_PRESET => m::PX_166,
        SIZE_PRESET => m::PX_258,
        ASPECT_RATIO_PRESET => m::PX_188,
        _ => m::PX_214,
    }
}

fn primary_height(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        LINE_THICKNESS_PRESET => m::PX_24,
        ASPECT_RATIO_PRESET => m::PX_28,
        _ => m::PX_18,
    }
}

fn secondary_width(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        TEXT_LINES_PRESET => m::PX_198,
        LAST_LINE_RATIO_PRESET => m::PX_118,
        REDUCED_MOTION_PRESET => m::PX_150,
        A11Y_PRESET => m::PX_206,
        _ => m::PX_174,
    }
}

fn secondary_height(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == LINE_THICKNESS_PRESET {
        return m::PX_18;
    }
    m::PX_12
}

fn tertiary_width(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        SHAPE_PRESET => m::PX_104,
        LAST_LINE_RATIO_PRESET => m::PX_88,
        SIZE_PRESET => m::PX_214,
        RADIUS_PRESET => m::PX_188,
        _ => m::PX_142,
    }
}

fn tertiary_height(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == LINE_THICKNESS_PRESET {
        return m::PX_16;
    }
    m::PX_10
}

fn avatar_size(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        SHAPE_PRESET => m::PX_52,
        RADIUS_PRESET => m::PX_60,
        _ => m::PX_44,
    }
}

fn status_width(scenario: ScenarioContext<'_>) -> usize {
    match scenario.preset_index {
        ANIMATION_PRESET => m::PX_128,
        A11Y_PRESET => m::PX_150,
        ASPECT_RATIO_PRESET => m::PX_92,
        _ => m::PX_110,
    }
}

fn primary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.screen_state.has_settings_override() || scenario.screen_state.has_widget_action() {
        return common::SUCCESS;
    }
    match scenario.preset_index {
        TONE_PRESET => palette.accent,
        RADIUS_PRESET => common::WARN,
        ASPECT_RATIO_PRESET => common::PURPLE,
        _ => palette.surface,
    }
}

fn secondary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == REDUCED_MOTION_PRESET {
        return palette.panel;
    }
    if scenario.preset_index == ANIMATION_PRESET {
        return common::TOKEN;
    }
    if scenario.preset_index == A11Y_PRESET {
        return common::SUCCESS;
    }
    palette.surface
}

fn tertiary_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == TEXT_LINES_PRESET {
        return common::PURPLE;
    }
    if scenario.preset_index == LINE_THICKNESS_PRESET {
        return common::WARN;
    }
    palette.surface
}

fn avatar_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == SHAPE_PRESET {
        return palette.accent;
    }
    palette.panel
}

fn status_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    match scenario.preset_index {
        TONE_PRESET | RADIUS_PRESET => common::TOKEN,
        A11Y_PRESET => common::SUCCESS,
        _ => palette.accent,
    }
}

fn shape_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        SHAPE_PRESET => "shape=circle",
        TEXT_LINES_PRESET => "text lines=3",
        LAST_LINE_RATIO_PRESET => "last line=62%",
        LINE_THICKNESS_PRESET => "line thickness=12",
        SIZE_PRESET => "size=fill",
        ASPECT_RATIO_PRESET => "aspect=16:9",
        _ => "shape=text",
    }
}

fn motion_label(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.preset_index {
        ANIMATION_PRESET => "animation=wave",
        TONE_PRESET => "tone=accent",
        RADIUS_PRESET => "radius=14",
        REDUCED_MOTION_PRESET => "reduced motion=true",
        A11Y_PRESET => "a11y label set",
        _ => "animation=shimmer",
    }
}
