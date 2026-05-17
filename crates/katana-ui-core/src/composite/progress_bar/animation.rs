use floem::action::exec_after;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use std::time::Duration;

pub(super) fn schedule_next_frame(frame: RwSignal<u64>, mounted: RwSignal<bool>, speed_ms: u64) {
    exec_after(Duration::from_millis(speed_ms), move |_| {
        if !mounted.try_get_untracked().unwrap_or(false) {
            return;
        }

        if frame
            .try_update(|value| {
                *value = value.wrapping_add(1);
            })
            .is_none()
        {
            return;
        }

        schedule_next_frame(frame, mounted, speed_ms);
    });
}
