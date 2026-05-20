use super::identifiers::WorkspaceTabId;
use super::options::WorkspaceTab;
use crate::render_model::UiInteractionState;
use crate::render_model::{UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabChildState {
    pub tab_id: WorkspaceTabId,
    pub state_id: UiStateId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabBarState {
    pub state_id: UiStateId,
    pub active_tab_id: Option<WorkspaceTabId>,
    pub overflow_visible: bool,
    pub drag_in_progress: bool,
    pub dragged_tab_id: Option<WorkspaceTabId>,
    pub pending_close_confirm: Option<WorkspaceTabId>,
    pub child_states: Vec<WorkspaceTabChildState>,
}

impl WorkspaceTabBarState {
    #[must_use]
    pub fn new(tabs: &[WorkspaceTab]) -> Self {
        let state_id = UiStateId::next_for(UiNodeKind::CloseableTabStrip);
        Self {
            child_states: child_states(&state_id, tabs),
            state_id,
            active_tab_id: None,
            overflow_visible: false,
            drag_in_progress: false,
            dragged_tab_id: None,
            pending_close_confirm: None,
        }
    }

    pub fn sync_child_states(&mut self, tabs: &[WorkspaceTab]) {
        self.child_states = child_states(&self.state_id, tabs);
    }

    #[must_use]
    pub fn child_state_id(&self, tab_id: &WorkspaceTabId) -> Option<&UiStateId> {
        self.child_states
            .iter()
            .find(|child| &child.tab_id == tab_id)
            .map(|child| &child.state_id)
    }

    #[must_use]
    pub fn interaction(&self, item_count: usize) -> UiInteractionState {
        UiInteractionState {
            open: self.overflow_visible,
            has_selection: self.active_tab_id.is_some(),
            item_count,
            dragging: self.drag_in_progress,
            value: self
                .active_tab_id
                .as_ref()
                .map_or_else(String::new, |it| it.as_str().to_string()),
            ..UiInteractionState::default()
        }
    }
}

fn child_states(parent_state_id: &UiStateId, tabs: &[WorkspaceTab]) -> Vec<WorkspaceTabChildState> {
    tabs.iter()
        .enumerate()
        .map(|(index, tab)| WorkspaceTabChildState {
            tab_id: tab.id.clone(),
            state_id: UiStateId::new(format!(
                "{}:closeable-tab:{}:{index}",
                parent_state_id.as_str(),
                tab.id.as_str()
            )),
        })
        .collect()
}
