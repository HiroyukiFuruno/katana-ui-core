use super::helpers::{ActivePop, SideMenuSignals};
use super::types::{
    DEFAULT_EXPANDED_PANEL_WIDTH, SIDE_MENU_HOVER_DELAY_MS, SideMenuPopMode, SideMenuSide,
};
use floem::ViewId;
use floem::action::exec_after;
use floem::reactive::{SignalGet, SignalUpdate};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

pub(super) fn schedule_hover_open(
    index: usize,
    mode: Option<SideMenuPopMode>,
    side: SideMenuSide,
    button_id: ViewId,
    signals: SideMenuSignals,
    hover_token: &Rc<Cell<u64>>,
) {
    let Some(mode) = mode else { return };
    if signals.hover_cooldown.try_get().unwrap_or(false) {
        return;
    }
    if signals
        .active
        .try_get()
        .flatten()
        .is_some_and(|current| current.pinned)
    {
        return;
    }

    signals.hovered.set(true);
    signals.anchor.set(anchor_for_button(
        side,
        button_id,
        DEFAULT_EXPANDED_PANEL_WIDTH,
    ));
    let token = hover_token.get().wrapping_add(1);
    hover_token.set(token);
    exec_after(Duration::from_millis(SIDE_MENU_HOVER_DELAY_MS), {
        let hover_token = Rc::clone(hover_token);
        move |_| {
            if hover_token.get() != token {
                return;
            }
            if signals.hover_cooldown.try_get().unwrap_or(false) {
                return;
            }
            if signals.hovered.try_get().unwrap_or(false) {
                signals.active.set(Some(ActivePop {
                    index,
                    mode,
                    pinned: false,
                }));
            }
        }
    });
}

pub(super) fn anchor_for_button(
    side: SideMenuSide,
    button_id: ViewId,
    panel_width: f32,
) -> (f32, f32) {
    let Some(layout) = button_id.get_layout() else {
        return (0.0, 0.0);
    };
    let (x, y) = origin_in_window(button_id);
    (side.expansion_panel_x(x, layout.size.width, panel_width), y)
}

fn origin_in_window(view_id: ViewId) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut current = Some(view_id);
    while let Some(id) = current {
        if let Some(layout) = id.get_layout() {
            x += layout.location.x;
            y += layout.location.y;
        }
        current = id.parent();
    }
    (x, y)
}
