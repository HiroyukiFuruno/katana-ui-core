use super::canvas::Canvas;
use super::dedicated_dod_atom_progress_motion::progress_segment_motion_snapshot;
use super::dedicated_dod_atom_progress_props::{
    component_label, core_progress_props, progress_has_segment_count_preset,
    progress_has_speed_preset,
};
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const PROGRESS_EMPTY_PRESET: usize = 2;
const PROGRESS_THEME_PRESET: usize = 3;
const SEGMENT_COUNT_PRESET: usize = 5;
const REDUCED_MOTION_PRESET: usize = 6;
const TONE_PRESET: usize = 7;
const SIZE_PRESET: usize = 8;
const PROGRESS_BLOCK_COUNT: usize = 4;
const PROGRESS_TRACK_RADIUS: usize = 9;
const PROGRESS_EMPTY_PERCENT: u8 = 0;
const PROGRESS_DEFAULT_PERCENT: u8 = 65;
const PROGRESS_CHANGED_PERCENT: u8 = 82;
const PROGRESS_MAX_PERCENT: u8 = 99;
#[cfg(test)]
const PROGRESS_LABEL_COUNT: usize = 3;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ProgressBlockSnapshot {
    pub(super) rect: Rect,
    pub(super) fill: u32,
    pub(super) radius: usize,
}

pub(super) fn progress(
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
        "ProgressBar",
        &progress_blocks(palette, scenario),
        &[
            TextSpec::new(
                m::PX_20,
                m::PX_26,
                m::FONT_9,
                palette.muted,
                component_label(scenario),
            ),
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

fn progress_blocks(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [Block; PROGRESS_BLOCK_COUNT] {
    [
        Block::rounded_outlined(
            m::PX_20,
            m::PX_44,
            m::PX_244,
            m::PX_18,
            PROGRESS_TRACK_RADIUS,
            track_fill(palette, scenario),
        ),
        Block::rounded(
            m::PX_22,
            m::PX_46,
            progress_width(scenario),
            m::PX_14,
            PROGRESS_TRACK_RADIUS.saturating_sub(2),
            progress_fill(palette, scenario),
        ),
        Block::rounded(
            m::PX_22,
            m::PX_46,
            indeterminate_track_width(scenario),
            m::PX_14,
            PROGRESS_TRACK_RADIUS.saturating_sub(2),
            track_fill(palette, scenario),
        ),
        Block::rounded(
            segment_x(scenario),
            m::PX_46,
            segment_width(scenario),
            m::PX_14,
            PROGRESS_TRACK_RADIUS.saturating_sub(2),
            segment_fill(palette, scenario),
        ),
    ]
}

#[cfg(test)]
pub(super) fn progress_blocks_for_test(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
) -> [ProgressBlockSnapshot; PROGRESS_BLOCK_COUNT] {
    [
        ProgressBlockSnapshot {
            rect: Rect::new(m::PX_20, m::PX_44, m::PX_244, m::PX_18),
            fill: track_fill(palette, scenario),
            radius: PROGRESS_TRACK_RADIUS,
        },
        ProgressBlockSnapshot {
            rect: Rect::new(m::PX_22, m::PX_46, progress_width(scenario), m::PX_14),
            fill: progress_fill(palette, scenario),
            radius: PROGRESS_TRACK_RADIUS.saturating_sub(2),
        },
        ProgressBlockSnapshot {
            rect: Rect::new(
                m::PX_22,
                m::PX_46,
                indeterminate_track_width(scenario),
                m::PX_14,
            ),
            fill: track_fill(palette, scenario),
            radius: PROGRESS_TRACK_RADIUS.saturating_sub(2),
        },
        ProgressBlockSnapshot {
            rect: Rect::new(
                segment_x(scenario),
                m::PX_46,
                segment_width(scenario),
                m::PX_14,
            ),
            fill: segment_fill(palette, scenario),
            radius: PROGRESS_TRACK_RADIUS.saturating_sub(2),
        },
    ]
}

#[cfg(test)]
pub(super) fn progress_labels_for_test(
    scenario: ScenarioContext<'_>,
) -> [&'static str; PROGRESS_LABEL_COUNT] {
    [
        component_label(scenario),
        percent_label(scenario),
        state_label(scenario),
    ]
}

fn progress_width(scenario: ScenarioContext<'_>) -> usize {
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return m::PX_0;
    }
    if scenario.preset_index == SIZE_PRESET {
        return m::PX_230;
    }
    let props = core_progress_props(scenario);
    if !props.determinate {
        return m::PX_0;
    }
    progress_width_for_percent(props.progress_percent)
}

fn progress_width_for_percent(percent: u8) -> usize {
    (m::PX_244 * usize::from(percent) / 100).min(m::PX_244)
}

fn progress_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PROGRESS_THEME_PRESET {
        return common::TOKEN;
    }
    if scenario.preset_index == TONE_PRESET {
        return common::SUCCESS;
    }
    palette.accent
}

fn track_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == PROGRESS_THEME_PRESET
        || scenario.preset_index == REDUCED_MOTION_PRESET
    {
        return palette.panel;
    }
    palette.surface
}

fn segment_width(scenario: ScenarioContext<'_>) -> usize {
    progress_segment_motion_snapshot(scenario)
        .map(|it| it.width)
        .unwrap_or(m::PX_0)
}

fn indeterminate_track_width(scenario: ScenarioContext<'_>) -> usize {
    if shows_indeterminate_segment(scenario) {
        return m::PX_244;
    }
    m::PX_0
}

fn segment_x(scenario: ScenarioContext<'_>) -> usize {
    progress_segment_motion_snapshot(scenario)
        .map(|it| it.x)
        .unwrap_or(m::PX_22)
}

fn shows_indeterminate_segment(scenario: ScenarioContext<'_>) -> bool {
    progress_segment_motion_snapshot(scenario).is_some()
}

fn segment_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if core_progress_props(scenario)
        .loading_indicator
        .reduced_motion
    {
        return common::SUCCESS;
    }
    if scenario.preset_index == SEGMENT_COUNT_PRESET {
        return common::PURPLE;
    }
    palette.accent
}

fn percent_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return "0%";
    }
    let props = core_progress_props(scenario);
    if !props.determinate {
        return "indeterminate";
    }
    percent_label_for_value(props.progress_percent)
}

fn percent_label_for_value(percent: u8) -> &'static str {
    match percent {
        PROGRESS_EMPTY_PERCENT => "0%",
        PROGRESS_DEFAULT_PERCENT => "65%",
        PROGRESS_CHANGED_PERCENT => "82%",
        PROGRESS_MAX_PERCENT => "99%",
        _ => "changed%",
    }
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_progress_state() {
        return scenario.screen_state.state_label;
    }
    if scenario.preset_index == PROGRESS_EMPTY_PRESET {
        return "empty progress";
    }
    if scenario.preset_index == PROGRESS_THEME_PRESET {
        return "theme track token";
    }
    let props = core_progress_props(scenario);
    if progress_has_speed_preset(&props) {
        return "speed=96ms";
    }
    if progress_has_segment_count_preset(&props) {
        return "segments=5";
    }
    if props.loading_indicator.reduced_motion {
        return "reduced motion";
    }
    if scenario.preset_index == TONE_PRESET {
        return "tone=accent";
    }
    if scenario.preset_index == SIZE_PRESET {
        return "size=large";
    }
    "determinate / indeterminate"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_unknown_percent_uses_changed_label() {
        assert_eq!("changed%", percent_label_for_value(1));
    }
}
