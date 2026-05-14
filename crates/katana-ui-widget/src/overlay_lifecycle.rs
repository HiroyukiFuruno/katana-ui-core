use floem::action::{add_overlay, exec_after, remove_overlay};
use floem::peniko::kurbo::Point;
use floem::{View, ViewId};
use std::{cell::Cell, rc::Rc, time::Duration};

const DEFER_OVERLAY_LIFECYCLE_MS: u64 = 1;

#[derive(Clone, Debug)]
pub(crate) struct OverlayLifetime {
    alive: Rc<Cell<bool>>,
    generation: Rc<Cell<u64>>,
}

impl OverlayLifetime {
    pub(crate) fn new() -> Self {
        Self {
            alive: Rc::new(Cell::new(true)),
            generation: Rc::new(Cell::new(0)),
        }
    }

    pub(crate) fn invalidate(&self) {
        self.generation.set(self.generation.get().wrapping_add(1));
    }

    pub(crate) fn dispose(&self) {
        self.alive.set(false);
        self.invalidate();
    }

    fn current_generation(&self) -> u64 {
        self.generation.get()
    }

    fn next_generation(&self) -> u64 {
        self.invalidate();
        self.current_generation()
    }

    fn is_current(&self, generation: u64) -> bool {
        self.alive.get() && self.current_generation() == generation
    }
}

impl Default for OverlayLifetime {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct OverlayLifecycle;

impl OverlayLifecycle {
    pub(crate) fn add_overlay_next_tick<V: View + 'static>(
        lifetime: &OverlayLifetime,
        position: Point,
        view: impl FnOnce(ViewId) -> V + 'static,
        on_added: impl FnOnce(ViewId) + 'static,
    ) {
        let generation = lifetime.next_generation();
        let lifetime = lifetime.clone();
        exec_after(
            Duration::from_millis(DEFER_OVERLAY_LIFECYCLE_MS),
            move |_| {
                if !lifetime.is_current(generation) {
                    return;
                }
                let id = add_overlay(position, view);
                if lifetime.is_current(generation) {
                    on_added(id);
                    return;
                }
                remove_overlay(id);
            },
        );
    }

    pub(crate) fn remove_overlay_next_tick(lifetime: &OverlayLifetime, id: ViewId) {
        lifetime.invalidate();
        exec_after(
            Duration::from_millis(DEFER_OVERLAY_LIFECYCLE_MS),
            move |_| {
                remove_overlay(id);
            },
        );
    }

    pub(crate) fn request_focus_next_tick(lifetime: &OverlayLifetime, id: ViewId) {
        let generation = lifetime.current_generation();
        let lifetime = lifetime.clone();
        exec_after(
            Duration::from_millis(DEFER_OVERLAY_LIFECYCLE_MS),
            move |_| {
                if !lifetime.is_current(generation) {
                    return;
                }
                id.request_focus();
            },
        );
    }
}
