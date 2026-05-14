use super::helpers::{ActivePop, overlay_for};
use super::types::{SideMenuItem, SideMenuPopMode};
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use crate::theme::Theme;
use floem::ViewId;
use floem::peniko::kurbo::Point;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_effect};
use std::cell::Cell;
use std::rc::Rc;

pub(super) struct SideMenuOverlayEffectArgs {
    pub(super) active: RwSignal<Option<ActivePop>>,
    pub(super) anchor: RwSignal<(f32, f32)>,
    pub(super) items: Rc<Vec<SideMenuItem>>,
    pub(super) overlay_id: RwSignal<Option<ViewId>>,
    pub(super) theme: Theme,
    pub(super) close_overlay: Rc<dyn Fn()>,
    pub(super) clear: Rc<dyn Fn()>,
    pub(super) overlay_lifetime: OverlayLifetime,
}

pub(super) fn bind_overlay_effect(args: SideMenuOverlayEffectArgs) {
    let SideMenuOverlayEffectArgs {
        active,
        anchor,
        items,
        overlay_id,
        theme,
        close_overlay,
        clear,
        overlay_lifetime,
    } = args;
    let overlay_generation = Rc::new(Cell::new(0_u64));
    create_effect(move |_| {
        let generation = overlay_generation.get().wrapping_add(1);
        overlay_generation.set(generation);
        let Some(current) = active.try_get().flatten() else {
            close_overlay();
            return;
        };
        if matches!(current.mode, SideMenuPopMode::Expand) {
            close_overlay();
            return;
        }
        let Some(item) = items.get(current.index).and_then(|item| item.pop.as_ref()) else {
            close_overlay();
            return;
        };

        close_overlay();
        let active_for_callback = active;
        let overlay_id_for_callback = overlay_id;
        let overlay_generation_for_callback = Rc::clone(&overlay_generation);
        let popup = overlay_for(
            item,
            current.mode,
            anchor.try_get().unwrap_or((0.0, 0.0)),
            Rc::clone(&clear),
            theme.clone(),
        );
        let overlay_lifetime_for_added = overlay_lifetime.clone();
        OverlayLifecycle::add_overlay_next_tick(
            &overlay_lifetime,
            Point::new(0.0, 0.0),
            move |_| popup,
            move |id| {
                let current_still_active = overlay_generation_for_callback.get() == generation
                    && active_for_callback.try_get().flatten() == Some(current)
                    && overlay_id_for_callback.try_get().flatten().is_none();
                if current_still_active {
                    overlay_id_for_callback.set(Some(id));
                } else {
                    OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime_for_added, id);
                }
            },
        );
    });
}
