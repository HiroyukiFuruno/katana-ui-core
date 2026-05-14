use floem::action::{add_overlay, exec_after, remove_overlay};
use floem::peniko::kurbo::Point;
use floem::{View, ViewId};
use std::time::Duration;

const DEFER_OVERLAY_LIFECYCLE_MS: u64 = 1;

pub(crate) struct OverlayLifecycle;

impl OverlayLifecycle {
    pub(crate) fn add_overlay_next_tick<V: View + 'static>(
        position: Point,
        view: impl FnOnce(ViewId) -> V + 'static,
        on_added: impl FnOnce(ViewId) + 'static,
    ) {
        exec_after(
            Duration::from_millis(DEFER_OVERLAY_LIFECYCLE_MS),
            move |_| {
                let id = add_overlay(position, view);
                on_added(id);
            },
        );
    }

    pub(crate) fn remove_overlay_next_tick(id: ViewId) {
        exec_after(
            Duration::from_millis(DEFER_OVERLAY_LIFECYCLE_MS),
            move |_| {
                remove_overlay(id);
            },
        );
    }

    pub(crate) fn request_focus_next_tick(id: ViewId) {
        exec_after(
            Duration::from_millis(DEFER_OVERLAY_LIFECYCLE_MS),
            move |_| {
                id.request_focus();
            },
        );
    }
}
