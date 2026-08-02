use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

pub mod drag;

pub use drag::{DragEvent, DragEventRouteNode, DragEventRouting};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerEvent {
    pub target: UiNodeId,
    pub x: f32,
    pub y: f32,
    pub kind: PointerEventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointerEventKind {
    Down,
    Up,
    Click,
    Move,
    Enter,
    Leave,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClickEvent {
    pub target: UiNodeId,
    pub x: f32,
    pub y: f32,
    pub source: ClickEventSource,
}

impl ClickEvent {
    #[must_use]
    pub fn new(target: UiNodeId, x: f32, y: f32, source: ClickEventSource) -> Self {
        Self {
            target,
            x,
            y,
            source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClickEventSource {
    Pointer,
    Keyboard,
    Programmatic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardEvent {
    pub target: UiNodeId,
    pub key: String,
    pub modifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusEvent {
    pub target: UiNodeId,
    pub focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEvent {
    pub target: UiNodeId,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UiEvent {
    Pointer(PointerEvent),
    Click(ClickEvent),
    Keyboard(KeyboardEvent),
    Focus(FocusEvent),
    Command(CommandEvent),
    Drag(DragEvent),
    Scroll(crate::layout::ScrollAreaEvent),
    SplitPane(crate::layout::SplitPaneEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventRoute {
    order: Vec<UiNodeId>,
    stopped: bool,
}

impl EventRoute {
    #[must_use]
    pub fn bubble(target: UiNodeId, parents: Vec<UiNodeId>, disabled: bool) -> Self {
        let mut order = vec![target];
        if !disabled {
            order.extend(parents);
        }
        Self {
            order,
            stopped: disabled,
        }
    }

    #[must_use]
    pub fn capture(root_to_target: Vec<UiNodeId>, cancelled: bool) -> Self {
        Self {
            order: root_to_target,
            stopped: cancelled,
        }
    }

    #[must_use]
    pub fn order(&self) -> &[UiNodeId] {
        &self.order
    }

    #[must_use]
    pub fn stopped(&self) -> bool {
        self.stopped
    }
}

#[cfg(test)]
mod tests {
    use super::{ClickEvent, ClickEventSource, EventRoute, UiEvent};
    use crate::render_model::UiNodeId;

    #[test]
    fn bubbling_visits_target_then_parents() {
        let route = EventRoute::bubble(
            UiNodeId::new("button"),
            vec![UiNodeId::new("toolbar"), UiNodeId::new("root")],
            false,
        );
        let actual: Vec<&str> = route.order().iter().map(UiNodeId::as_str).collect();
        assert_eq!(["button", "toolbar", "root"], actual.as_slice());
    }

    #[test]
    fn capture_can_stop_before_target() {
        let route = EventRoute::capture(vec![UiNodeId::new("root")], true);
        assert!(route.stopped());
    }

    #[test]
    fn click_event_is_component_agnostic() {
        let click = ClickEvent::new(
            UiNodeId::new("any-clickable"),
            12.0,
            24.0,
            ClickEventSource::Pointer,
        );
        let event = UiEvent::Click(click.clone());

        assert!(matches!(&event, UiEvent::Click(_)));
        assert_eq!("any-clickable", click.target.as_str());
        assert_eq!(ClickEventSource::Pointer, click.source);
    }
}
