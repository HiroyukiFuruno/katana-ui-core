use floem::action::exec_after;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use std::time::Duration;

pub(super) fn schedule_next_step(
    active_step: RwSignal<usize>,
    mounted: RwSignal<bool>,
    dot_count: usize,
    speed_ms: u64,
) {
    exec_after(Duration::from_millis(speed_ms), move |_| {
        if !mounted.try_get_untracked().unwrap_or(false) {
            return;
        }

        if active_step
            .try_update(|frame| {
                *frame = (*frame + 1) % dot_count;
            })
            .is_none()
        {
            return;
        }

        schedule_next_step(active_step, mounted, dot_count, speed_ms);
    });
}
