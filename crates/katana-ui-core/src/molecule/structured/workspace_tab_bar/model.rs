use super::events::WorkspaceTabBarEvent;
use super::options::WorkspaceTabBarOptions;
use super::state::WorkspaceTabBarState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabBar {
    pub(super) label: String,
    pub(super) options: WorkspaceTabBarOptions,
    pub(super) state: WorkspaceTabBarState,
    pub(super) event_log: Vec<WorkspaceTabBarEvent>,
}
