use super::StorybookWindowState;
use super::drag_and_drop_contract::{
    drag_data, drag_event_name, drag_source, drop_target, source_node_id, target_point, target_rect,
};
use crate::visual::dedicated_drag_and_drop;
use crate::visual::preview_detail;
use katana_ui_core::event::DragEvent;
use katana_ui_core::interaction::drag_and_drop::{
    AutoScrollEngine, AutoScrollPolicy, DndPoint, DndRect, DropEffect, KeyboardDragContext,
    KeyboardDragKey, KeyboardDragState,
};

const AUTOSCROLL_VIEWPORT_WIDTH: f32 = 320.0;
const AUTOSCROLL_VIEWPORT_HEIGHT: f32 = 200.0;
const AUTOSCROLL_EDGE_POINTER_X: f32 = 8.0;
const AUTOSCROLL_EDGE_POINTER_Y: f32 = 8.0;
const RESIZE_TARGET_WIDTH_DELTA: f32 = 20.0;
const RESIZE_TARGET_HEIGHT_DELTA: f32 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum DragAndDropAction {
    StartPointer,
    DropPointer,
    KeyboardCancel,
    HoverTarget,
    FocusSource,
    KeyboardDrop,
    ScrollEdge,
    ResizeTarget,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(in crate::visual) struct DragAndDropScreenState {
    dragging: bool,
    committed: bool,
    keyboard_cancelled: bool,
    focused: bool,
    hovered: bool,
    scroll_requested: bool,
    resized: bool,
}

impl DragAndDropScreenState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: DragAndDropAction,
    ) -> DragAndDropUpdate {
        match action {
            DragAndDropAction::StartPointer => self.start_pointer_drag(),
            DragAndDropAction::DropPointer => self.drop_pointer_drag(),
            DragAndDropAction::KeyboardCancel => self.cancel_keyboard_drag(),
            DragAndDropAction::HoverTarget => self.hover_target(),
            DragAndDropAction::FocusSource => self.focus_source(),
            DragAndDropAction::KeyboardDrop => self.keyboard_drop(),
            DragAndDropAction::ScrollEdge => self.scroll_edge(),
            DragAndDropAction::ResizeTarget => self.resize_target(),
        }
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) -> DragAndDropUpdate {
        self.dragging = true;
        self.committed = false;
        self.keyboard_cancelled = false;
        match setting {
            "drag.accept_policy" => DragAndDropUpdate::new(
                "drag_accept_policy_option",
                "drag_accept_changed",
                "drag.accept_policy=move",
            ),
            "drag.keyboard_draggable" => DragAndDropUpdate::new(
                "drag_keyboard_option",
                "drag_keyboard_changed",
                "drag.keyboard_draggable=true",
            ),
            "drag.drop_indicator" => DragAndDropUpdate::new(
                "drag_indicator_option",
                "drag_indicator_changed",
                "drag.drop_indicator=after",
            ),
            "drag.autoscroll" => DragAndDropUpdate::new(
                "drag_autoscroll_option",
                "drag_autoscroll_changed",
                "drag.autoscroll=edge",
            ),
            _ => DragAndDropUpdate::new(
                "drag_option_unknown",
                "drag_option_ignored",
                "dragging=true",
            ),
        }
    }

    pub(in crate::visual) const fn is_dragging(&self) -> bool {
        self.dragging
    }

    pub(in crate::visual) const fn committed(&self) -> bool {
        self.committed
    }

    pub(in crate::visual) const fn focused(&self) -> bool {
        self.focused
    }

    pub(in crate::visual) const fn hovered(&self) -> bool {
        self.hovered
    }

    pub(in crate::visual) const fn scroll_requested(&self) -> bool {
        self.scroll_requested
    }

    pub(in crate::visual) const fn resized(&self) -> bool {
        self.resized
    }

    fn start_pointer_drag(&mut self) -> DragAndDropUpdate {
        self.dragging = true;
        self.committed = false;
        self.keyboard_cancelled = false;
        let event = DragEvent::DragStart {
            source: source_node_id(),
            data: drag_data(),
        };
        DragAndDropUpdate::new("drag_start", drag_event_name(&event), "dragging=true")
    }

    fn drop_pointer_drag(&mut self) -> DragAndDropUpdate {
        let source = drag_source(false);
        let target = drop_target();
        let acceptance = target.accept(&source.payload, target_point(), target_rect());
        if acceptance.effect() == DropEffect::None {
            self.dragging = false;
            self.committed = false;
            return DragAndDropUpdate::new("drag_drop_reject", "drag_rejected", "committed=false");
        }
        self.dragging = false;
        self.committed = true;
        self.keyboard_cancelled = false;
        DragAndDropUpdate::new("drop", "drag_end(committed=true)", "committed=true")
    }

    fn cancel_keyboard_drag(&mut self) -> DragAndDropUpdate {
        let source = drag_source(true);
        let started = KeyboardDragState::idle().handle_key(
            KeyboardDragKey::Space,
            KeyboardDragContext::focused_source(source),
        );
        let cancelled = started.state.handle_key(
            KeyboardDragKey::Escape,
            KeyboardDragContext::empty(source_node_id()),
        );
        self.dragging = false;
        self.committed = false;
        self.keyboard_cancelled = !cancelled.events.is_empty();
        DragAndDropUpdate::new(
            "drag_keyboard_cancel",
            "drag_end(committed=false)",
            "committed=false",
        )
    }

    fn hover_target(&mut self) -> DragAndDropUpdate {
        let acceptance = drop_target().accept(&drag_data(), target_point(), target_rect());
        assert_ne!(DropEffect::None, acceptance.effect());
        self.hovered = true;
        DragAndDropUpdate::new("drag_hover_target", "drag_enter", "hover=target")
    }

    fn focus_source(&mut self) -> DragAndDropUpdate {
        assert!(drag_source(true).keyboard_draggable);
        self.focused = true;
        DragAndDropUpdate::new("drag_focus_source", "focus", "focus=source")
    }

    fn keyboard_drop(&mut self) -> DragAndDropUpdate {
        let source = drag_source(true);
        let target = super::drag_and_drop_contract::drop_target();
        let focus = katana_ui_core::interaction::drag_and_drop::KeyboardDropTargetFocus::new(
            target,
            target_rect(),
            target_point(),
        );
        let picked_up = KeyboardDragState::idle().handle_key(
            KeyboardDragKey::Space,
            KeyboardDragContext::focused_source(source),
        );
        let moved = picked_up.state.handle_key(
            KeyboardDragKey::ArrowRight,
            KeyboardDragContext::focused_target(focus.clone()),
        );
        let dropped = moved.state.handle_key(
            KeyboardDragKey::Space,
            KeyboardDragContext::focused_target(focus),
        );
        assert!(dropped.events.iter().any(|event| {
            matches!(
                event,
                DragEvent::DragEnd {
                    committed: true,
                    ..
                }
            )
        }));
        self.dragging = false;
        self.committed = true;
        DragAndDropUpdate::new(
            "drag_keyboard_drop",
            "drag_end(committed=true)",
            "keyboard=drop",
        )
    }

    fn scroll_edge(&mut self) -> DragAndDropUpdate {
        let request = AutoScrollEngine::request(
            &AutoScrollPolicy::default(),
            DndRect::new(
                0.0,
                0.0,
                AUTOSCROLL_VIEWPORT_WIDTH,
                AUTOSCROLL_VIEWPORT_HEIGHT,
            ),
            DndPoint::new(AUTOSCROLL_EDGE_POINTER_X, AUTOSCROLL_EDGE_POINTER_Y),
            2,
        );
        assert!(request.is_some());
        self.scroll_requested = true;
        DragAndDropUpdate::new(
            "drag_autoscroll",
            "drag_autoscroll_requested",
            "scroll=edge",
        )
    }

    fn resize_target(&mut self) -> DragAndDropUpdate {
        let base = target_rect();
        let resized = DndRect::new(
            base.x,
            base.y,
            base.width + RESIZE_TARGET_WIDTH_DELTA,
            base.height + RESIZE_TARGET_HEIGHT_DELTA,
        );
        let acceptance = drop_target().accept(&drag_data(), target_point(), resized);
        assert_ne!(DropEffect::None, acceptance.effect());
        self.resized = true;
        DragAndDropUpdate::new("drag_resize_target", "drag_target_resized", "resize=target")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct DragAndDropUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl DragAndDropUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<DragAndDropAction> {
    if state.selected_page != "drag-and-drop" {
        return None;
    }
    let base = preview_detail::component_action_hit_rect(state.selected_page);
    dedicated_drag_and_drop::action_at(base.x, base.y, x, y)
}
