use crate::component::ComponentAction;
pub use crate::interaction::{RowHeightProvider, VirtualRange, VirtualizationConfig};
use crate::interaction::{UiAction, UiActionResult, VirtualizationPlanner};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VirtualizedEvent {
    None,
    Scrolled(VirtualRange),
    FocusKept(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualizedList {
    label: String,
    state_id: UiStateId,
    config: VirtualizationConfig,
    last_event: VirtualizedEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualizedTree {
    label: String,
    state_id: UiStateId,
    config: VirtualizationConfig,
    expanded_node_ids: Vec<String>,
}

impl VirtualizationConfig {
    #[must_use]
    pub fn visible_range(&self) -> VirtualRange {
        VirtualizationPlanner::compute_visible_range(self)
    }
}

impl VirtualizedList {
    #[must_use]
    pub fn new(label: impl Into<String>, config: VirtualizationConfig) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::VirtualizedList),
            config,
            last_event: VirtualizedEvent::None,
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn visible_range(&self) -> VirtualRange {
        self.config.visible_range()
    }

    #[must_use]
    pub fn last_event(&self) -> &VirtualizedEvent {
        &self.last_event
    }
}

impl ComponentAction for VirtualizedList {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(&self.config);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetValue { value, .. } => {
                self.config.viewport_offset =
                    value.parse::<u32>().unwrap_or(self.config.viewport_offset);
                self.last_event = VirtualizedEvent::Scrolled(self.visible_range());
            }
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.config.focused_index = Some(*selected_index);
                self.last_event = VirtualizedEvent::FocusKept(*selected_index);
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(&self.config))
    }
}

impl From<VirtualizedList> for UiNode {
    fn from(value: VirtualizedList) -> Self {
        UiNode::from_state(UiNodeKind::VirtualizedList, value.label, value.state_id)
            .interaction(state(&value.config))
    }
}

impl VirtualizedTree {
    #[must_use]
    pub fn new(label: impl Into<String>, config: VirtualizationConfig) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::VirtualizedTree),
            config,
            expanded_node_ids: Vec::new(),
        }
    }

    #[must_use]
    pub fn expanded_node(mut self, id: impl Into<String>) -> Self {
        self.expanded_node_ids.push(id.into());
        self
    }

    #[must_use]
    pub fn visible_range(&self) -> VirtualRange {
        self.config.visible_range()
    }
}

impl From<VirtualizedTree> for UiNode {
    fn from(value: VirtualizedTree) -> Self {
        UiNode::from_state(UiNodeKind::VirtualizedTree, value.label, value.state_id)
            .interaction(state(&value.config))
    }
}

fn state(config: &VirtualizationConfig) -> UiInteractionState {
    let range = config.visible_range();
    UiInteractionState {
        selected_index: range.start,
        item_count: range.end.saturating_sub(range.start),
        value: config.viewport_offset.to_string(),
        has_selection: config.focused_index.is_some(),
        ..UiInteractionState::default()
    }
}
