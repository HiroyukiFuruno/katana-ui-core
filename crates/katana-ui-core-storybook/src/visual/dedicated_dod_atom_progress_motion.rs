use super::dedicated_dod_atom_progress_props::{
    SEGMENT_COUNT, SPEED_PRESET_MS, core_progress_props,
};
use super::dedicated_dod_metrics as m;
use super::render_context::ScenarioContext;

const DEFAULT_SEGMENT_WIDTH: usize = m::PX_26;
const SPEED_SEGMENT_WIDTH: usize = m::PX_52;
const SEGMENTED_SEGMENT_WIDTH: usize = m::PX_96;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ProgressSegmentMotionSnapshot {
    pub(super) x: usize,
    pub(super) width: usize,
    pub(super) track_x: usize,
    pub(super) track_width: usize,
}

pub(super) fn progress_segment_motion_snapshot(
    scenario: ScenarioContext<'_>,
) -> Option<ProgressSegmentMotionSnapshot> {
    let props = core_progress_props(scenario);
    if props.determinate && !props.loading_indicator.reduced_motion {
        return None;
    }
    let width = segment_width_for_props(
        props.loading_indicator.speed_ms,
        props.loading_indicator.dot_count,
    );
    Some(ProgressSegmentMotionSnapshot {
        x: segment_x_for_props(props.progress_percent, width, props.determinate),
        width,
        track_x: m::PX_22,
        track_width: m::PX_244,
    })
}

fn segment_width_for_props(speed_ms: u16, dot_count: u8) -> usize {
    if dot_count == SEGMENT_COUNT {
        return SEGMENTED_SEGMENT_WIDTH;
    }
    if speed_ms == SPEED_PRESET_MS {
        return SPEED_SEGMENT_WIDTH;
    }
    DEFAULT_SEGMENT_WIDTH
}

fn segment_x_for_props(percent: u8, width: usize, determinate: bool) -> usize {
    if determinate {
        return m::PX_22;
    }
    let travel = m::PX_244.saturating_sub(width);
    m::PX_22 + (travel * usize::from(percent) / 100)
}
