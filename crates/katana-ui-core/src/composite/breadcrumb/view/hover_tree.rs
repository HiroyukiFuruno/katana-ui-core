use crate::composite::breadcrumb::BreadcrumbCrumb;
use crate::composite::tree_view::{TreeView, TreeViewExpandTrigger, TreeViewNode};
use crate::floem_view::FloemColor;
use crate::layout::popover::{AnchorRect, ViewAnchor};
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use crate::theme::Theme;
use floem::ViewId;
use floem::action::exec_after;
use floem::event::EventListener;
use floem::peniko::kurbo::Point;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, container};
use floem::{IntoView, View};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

const HOVER_CLOSE_DELAY_MS: u64 = 120;
const HOVER_TREE_WIDTH: f32 = 220.0;
const HOVER_TREE_OFFSET_Y: f32 = 4.0;
const HOVER_TREE_PADDING: f32 = 8.0;
const HOVER_TREE_RADIUS: f32 = 8.0;
const HOVER_TREE_BORDER: f32 = 1.0;
const HOVER_TREE_FALLBACK_SIZE: f32 = 1.0;

struct HoverOverlayPanelArgs {
    children: Vec<BreadcrumbCrumb>,
    theme: Theme,
    anchor: AnchorRect,
    trigger_hover: RwSignal<bool>,
    overlay_hover: RwSignal<bool>,
    open: RwSignal<bool>,
    hover_token: Rc<Cell<u64>>,
    mounted: Rc<Cell<bool>>,
}

pub(crate) struct BreadcrumbHoverTree;

impl BreadcrumbHoverTree {
    pub(crate) fn view(
        trigger: Box<dyn View>,
        children: Vec<BreadcrumbCrumb>,
        theme: Theme,
    ) -> Box<dyn View> {
        let open = create_rw_signal(false);
        let trigger_hover = create_rw_signal(false);
        let overlay_hover = create_rw_signal(false);
        let overlay_id = create_rw_signal::<Option<ViewId>>(None);
        let anchor = create_rw_signal(default_anchor());
        let hover_token = Rc::new(Cell::new(0_u64));
        let overlay_pending = Rc::new(Cell::new(false));
        let mounted = Rc::new(Cell::new(true));
        let overlay_lifetime = OverlayLifetime::new();
        let trigger = container(trigger);
        let trigger_id = trigger.id();

        create_effect({
            let children = children.clone();
            let theme = theme.clone();
            let hover_token = Rc::clone(&hover_token);
            let overlay_pending = Rc::clone(&overlay_pending);
            let overlay_lifetime = overlay_lifetime.clone();
            let mounted = Rc::clone(&mounted);
            move |_| {
                if !open.try_get().unwrap_or(false) {
                    overlay_pending.set(false);
                    remove_hover_tree_overlay(overlay_id, &overlay_lifetime);
                    return;
                }

                if overlay_id.try_get().unwrap_or(None).is_some() || overlay_pending.get() {
                    return;
                }
                overlay_pending.set(true);

                let current_anchor = anchor.try_get().unwrap_or(default_anchor());
                let panel_theme = theme.clone();
                let panel_children = children.clone();
                let panel_token = Rc::clone(&hover_token);
                let panel_mounted = Rc::clone(&mounted);
                let overlay_lifetime_for_added = overlay_lifetime.clone();
                OverlayLifecycle::add_overlay_next_tick(
                    &overlay_lifetime,
                    Point::new(0.0, 0.0),
                    move |_| {
                        overlay_panel(HoverOverlayPanelArgs {
                            children: panel_children.clone(),
                            theme: panel_theme.clone(),
                            anchor: current_anchor,
                            trigger_hover,
                            overlay_hover,
                            open,
                            hover_token: Rc::clone(&panel_token),
                            mounted: Rc::clone(&panel_mounted),
                        })
                    },
                    {
                        let overlay_pending = Rc::clone(&overlay_pending);
                        move |view_id| {
                            overlay_pending.set(false);
                            if open.try_get().unwrap_or(false)
                                && overlay_id.try_get().unwrap_or(None).is_none()
                            {
                                overlay_id.set(Some(view_id));
                            } else {
                                OverlayLifecycle::remove_overlay_next_tick(
                                    &overlay_lifetime_for_added,
                                    view_id,
                                );
                            }
                        }
                    },
                );
            }
        });

        trigger
            .on_event_cont(EventListener::PointerEnter, move |_| {
                trigger_hover.set(true);
                anchor.set(ViewAnchor::rect_for_view(trigger_id, default_anchor()));
                open.set(true);
            })
            .on_event_cont(EventListener::PointerLeave, {
                let hover_token = Rc::clone(&hover_token);
                let mounted = Rc::clone(&mounted);
                move |_| {
                    trigger_hover.set(false);
                    schedule_close(
                        trigger_hover,
                        overlay_hover,
                        open,
                        Rc::clone(&hover_token),
                        Rc::clone(&mounted),
                    );
                }
            })
            .on_cleanup(move || {
                mounted.set(false);
                overlay_lifetime.dispose();
                remove_hover_tree_overlay(overlay_id, &overlay_lifetime);
            })
            .into_any()
    }
}

fn overlay_panel(args: HoverOverlayPanelArgs) -> Box<dyn View> {
    let HoverOverlayPanelArgs {
        children,
        theme,
        anchor,
        trigger_hover,
        overlay_hover,
        open,
        hover_token,
        mounted,
    } = args;
    let nodes = children.into_iter().map(crumb_to_node).collect::<Vec<_>>();
    let top = anchor.y + anchor.height + HOVER_TREE_OFFSET_Y;
    let panel = TreeView::from_nodes(nodes)
        .expand_trigger(TreeViewExpandTrigger::IconAndLabel)
        .show_expand_controls(true)
        .show_indent_lines(true)
        .view(theme.clone());

    container(panel)
        .style(move |style| {
            style
                .absolute()
                .inset_left(anchor.x)
                .inset_top(top)
                .width(HOVER_TREE_WIDTH)
                .padding(HOVER_TREE_PADDING)
                .border(HOVER_TREE_BORDER)
                .border_color(FloemColor::from_token(theme.color.border))
                .border_radius(HOVER_TREE_RADIUS)
                .background(FloemColor::from_token(theme.color.surface))
        })
        .on_event_cont(EventListener::PointerEnter, move |_| {
            overlay_hover.set(true);
        })
        .on_event_cont(EventListener::PointerLeave, move |_| {
            overlay_hover.set(false);
            schedule_close(
                trigger_hover,
                overlay_hover,
                open,
                Rc::clone(&hover_token),
                Rc::clone(&mounted),
            );
        })
        .into_any()
}

fn crumb_to_node(crumb: BreadcrumbCrumb) -> TreeViewNode {
    let mut node = TreeViewNode::new(crumb.label.clone(), crumb.label)
        .expanded(true)
        .disabled(crumb.disabled);
    if let Some(icon) = crumb.icon {
        node = node.icon(icon);
    }
    if let Some(on_click) = crumb.on_click {
        node = node.on_select(move || (on_click)());
    }
    node.children(crumb.children.into_iter().map(crumb_to_node).collect())
}

fn schedule_close(
    trigger_hover: RwSignal<bool>,
    overlay_hover: RwSignal<bool>,
    open: RwSignal<bool>,
    hover_token: Rc<Cell<u64>>,
    mounted: Rc<Cell<bool>>,
) {
    let next_token = hover_token.get().wrapping_add(1);
    hover_token.set(next_token);
    exec_after(Duration::from_millis(HOVER_CLOSE_DELAY_MS), move |_| {
        if !mounted.get() {
            return;
        }
        if hover_token.get() == next_token && !trigger_hover.get() && !overlay_hover.get() {
            open.set(false);
        }
    });
}

fn remove_hover_tree_overlay(overlay_id: RwSignal<Option<ViewId>>, lifetime: &OverlayLifetime) {
    if let Some(id) = overlay_id.try_update(|current| current.take()).flatten() {
        OverlayLifecycle::remove_overlay_next_tick(lifetime, id);
    }
}

fn default_anchor() -> AnchorRect {
    AnchorRect::new(0.0, 0.0, HOVER_TREE_FALLBACK_SIZE, HOVER_TREE_FALLBACK_SIZE)
}
