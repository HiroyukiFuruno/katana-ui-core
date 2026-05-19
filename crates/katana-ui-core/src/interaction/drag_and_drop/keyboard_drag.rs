use super::{
    DragAnnouncement, DragSource, DropEffect, KeyboardDragContext, KeyboardDragKey,
    KeyboardDragPhase,
};
use crate::event::drag::{DRAG_CANCEL_REASON_KEYBOARD_ESCAPE, DragEvent};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardDragState {
    session: Option<KeyboardDragSession>,
}

impl KeyboardDragState {
    #[must_use]
    pub const fn idle() -> Self {
        Self { session: None }
    }

    #[must_use]
    pub const fn phase(&self) -> KeyboardDragPhase {
        if self.session.is_some() {
            KeyboardDragPhase::Dragging
        } else {
            KeyboardDragPhase::Idle
        }
    }

    #[must_use]
    pub fn handle_key(
        &self,
        key: KeyboardDragKey,
        context: KeyboardDragContext,
    ) -> KeyboardDragTransition {
        match &self.session {
            None if key.starts_or_drops() => start_drag(context),
            Some(session) if key.moves_focus() => move_focus(self, session, context),
            Some(session) if key.starts_or_drops() => drop_on_target(session, context),
            Some(session) if key == KeyboardDragKey::Escape => cancel_drag(session),
            _ => KeyboardDragTransition::unchanged(self.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KeyboardDragSession {
    source: DragSource,
    focused_target: Option<UiNodeId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardDragTransition {
    pub state: KeyboardDragState,
    pub events: Vec<DragEvent>,
    pub announcement: Option<DragAnnouncement>,
}

impl KeyboardDragTransition {
    fn unchanged(state: KeyboardDragState) -> Self {
        Self {
            state,
            events: Vec::new(),
            announcement: None,
        }
    }
}

fn start_drag(context: KeyboardDragContext) -> KeyboardDragTransition {
    let Some(source) = context.source else {
        return KeyboardDragTransition::unchanged(KeyboardDragState::idle());
    };
    if !source.keyboard_draggable {
        return KeyboardDragTransition::unchanged(KeyboardDragState::idle());
    }
    let label = drag_label(&source).to_string();
    KeyboardDragTransition {
        state: KeyboardDragState {
            session: Some(KeyboardDragSession {
                source: source.clone(),
                focused_target: None,
            }),
        },
        events: vec![DragEvent::DragStart {
            source: source.node_id,
            data: source.payload,
        }],
        announcement: Some(DragAnnouncement {
            message: format!("Picked up {label}"),
        }),
    }
}

fn move_focus(
    state: &KeyboardDragState,
    session: &KeyboardDragSession,
    context: KeyboardDragContext,
) -> KeyboardDragTransition {
    let Some(focus) = context.target else {
        return KeyboardDragTransition::unchanged(state.clone());
    };
    let acceptance = focus
        .target
        .accept(&session.source.payload, focus.position, focus.rect);
    let mut events = Vec::new();
    if session.focused_target.as_ref() != Some(&focus.target.node_id) {
        if let Some(previous_target) = &session.focused_target {
            events.push(DragEvent::DragLeave {
                target: previous_target.clone(),
            });
        }
        events.push(DragEvent::DragEnter {
            target: focus.target.node_id.clone(),
            data: session.source.payload.clone(),
        });
    }
    events.push(DragEvent::DragOver {
        target: focus.target.node_id.clone(),
        position: focus.position,
        acceptance: acceptance.clone(),
    });
    KeyboardDragTransition {
        state: KeyboardDragState {
            session: Some(KeyboardDragSession {
                source: session.source.clone(),
                focused_target: Some(focus.target.node_id),
            }),
        },
        events,
        announcement: Some(DragAnnouncement {
            message: "Moved over drop target".to_string(),
        }),
    }
}

fn drop_on_target(
    session: &KeyboardDragSession,
    context: KeyboardDragContext,
) -> KeyboardDragTransition {
    let Some(focus) = context.target else {
        return KeyboardDragTransition::unchanged(KeyboardDragState {
            session: Some(session.clone()),
        });
    };
    let acceptance = focus
        .target
        .accept(&session.source.payload, focus.position, focus.rect);
    if acceptance.effect() == DropEffect::None || !session.source.allows_effect(acceptance.effect())
    {
        return KeyboardDragTransition::unchanged(KeyboardDragState {
            session: Some(session.clone()),
        });
    }
    KeyboardDragTransition {
        state: KeyboardDragState::idle(),
        events: vec![
            DragEvent::Drop {
                target: focus.target.node_id,
                data: session.source.payload.clone(),
                effect: acceptance.effect(),
            },
            DragEvent::DragEnd {
                source: session.source.node_id.clone(),
                committed: true,
            },
        ],
        announcement: Some(DragAnnouncement {
            message: "Dropped".to_string(),
        }),
    }
}

fn cancel_drag(session: &KeyboardDragSession) -> KeyboardDragTransition {
    KeyboardDragTransition {
        state: KeyboardDragState::idle(),
        events: vec![
            DragEvent::DragCancel {
                source: session.source.node_id.clone(),
                reason: DRAG_CANCEL_REASON_KEYBOARD_ESCAPE.to_string(),
            },
            DragEvent::DragEnd {
                source: session.source.node_id.clone(),
                committed: false,
            },
        ],
        announcement: Some(DragAnnouncement {
            message: "Drag cancelled".to_string(),
        }),
    }
}

fn drag_label(source: &DragSource) -> &str {
    source
        .payload
        .metadata
        .get("label")
        .unwrap_or_else(|| source.node_id.as_str())
}
