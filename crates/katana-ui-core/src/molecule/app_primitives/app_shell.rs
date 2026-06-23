use crate::render_model::{UiNode, UiNodeKind, UiPanelProps, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppShellSlotKind {
    Top,
    Leading,
    Main,
    Trailing,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppShellSlot {
    pub kind: AppShellSlotKind,
    pub width: u16,
    pub node: UiNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppShell {
    label: String,
    state_id: UiStateId,
    slots: Vec<AppShellSlot>,
    main_available_width: u16,
}

impl AppShell {
    #[must_use]
    pub fn new(label: impl Into<String>, main_available_width: u16) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::AppShell),
            slots: Vec::new(),
            main_available_width,
        }
    }

    #[must_use]
    pub fn slot(mut self, slot: AppShellSlot) -> Self {
        self.slots.push(slot);
        self
    }

    #[must_use]
    pub fn main_available_width(&self) -> u16 {
        let occupied: u16 = self
            .slots
            .iter()
            .filter(|slot| {
                matches!(
                    slot.kind,
                    AppShellSlotKind::Leading | AppShellSlotKind::Trailing
                )
            })
            .map(|slot| slot.width)
            .sum();
        self.main_available_width.saturating_sub(occupied)
    }
}

impl From<AppShell> for UiNode {
    fn from(value: AppShell) -> Self {
        let main_available_width = value.main_available_width();
        value.slots.into_iter().fold(
            UiNode::from_state(UiNodeKind::AppShell, value.label, value.state_id).panel(
                UiPanelProps::vertical_scroll(0, u32::from(main_available_width), 0, false),
            ),
            |node, slot| node.child(slot.node),
        )
    }
}
