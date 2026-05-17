use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::layout::popover::{AnchorRect, ViewAnchor};
use floem::ViewId;
use floem::event::Event;
use floem::reactive::{RwSignal, SignalUpdate};

use super::view::{default_anchor_height, default_anchor_width};

pub(super) struct PointerMoveState {
    pub(super) anchor_id: ViewId,
    pub(super) anchor: RwSignal<AnchorRect>,
    pub(super) parent_origin: RwSignal<(f32, f32)>,
    pub(super) hover_ready: Rc<Cell<bool>>,
    pub(super) hover_token: Rc<Cell<u64>>,
    pub(super) mounted: Rc<Cell<bool>>,
    pub(super) visible: RwSignal<bool>,
    pub(super) delay_ms: u32,
}

pub(super) fn apply_pointer_move(event: &Event, state: PointerMoveState) {
    let Event::PointerMove(_) = event else {
        return;
    };
    if !state.mounted.get() {
        return;
    }

    state.anchor.set(ViewAnchor::rect_for_view(
        state.anchor_id,
        default_anchor_rect(),
    ));
    state
        .parent_origin
        .set(ViewAnchor::parent_origin_for_view(state.anchor_id));
    if state.hover_ready.get() {
        return;
    }

    state.hover_ready.set(true);
    let token = state.hover_token.get().wrapping_add(1);
    state.hover_token.set(token);
    if state.delay_ms == 0 {
        state.visible.set(true);
        return;
    }

    let hover_ready_for_delay = Rc::clone(&state.hover_ready);
    let hover_token_for_delay = Rc::clone(&state.hover_token);
    let mounted_for_delay = Rc::clone(&state.mounted);
    floem::action::exec_after(
        Duration::from_millis(u64::from(state.delay_ms)),
        move |_| {
            if !mounted_for_delay.get() {
                return;
            }
            if hover_token_for_delay.get() != token {
                return;
            }
            if !hover_ready_for_delay.get() {
                return;
            }
            state.visible.set(true);
        },
    );
}

fn default_anchor_rect() -> AnchorRect {
    AnchorRect::new(0.0, 0.0, default_anchor_width(), default_anchor_height())
}
