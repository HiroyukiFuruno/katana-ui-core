mod actions;
mod bar;
mod context_menu;
mod dnd;
mod events;
mod identifiers;
mod keyboard;
mod mutations;
mod options;
mod ordering;
mod overflow;
mod state;

pub use actions::{
    CLOSEABLE_TAB_DRAG_TAG, WorkspaceTabBarAction, WorkspaceTabDropPosition, WorkspaceTabDropRules,
    WorkspaceTabGroupTarget,
};
pub use bar::WorkspaceTabBar;
pub use context_menu::{
    WorkspaceGroupContextCommand, WorkspaceTabContextCommand, WorkspaceTabContextMenu,
};
pub use events::WorkspaceTabBarEvent;
pub use identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
pub use keyboard::{
    WorkspaceTabKey, WorkspaceTabKeyboardController, WorkspaceTabKeyboardInput,
    WorkspaceTabKeyboardShortcut,
};
pub use options::{WorkspaceTab, WorkspaceTabBarOptions, WorkspaceTabGroup, WorkspaceTabTone};
pub use overflow::{
    MeasuredWorkspaceTab, WorkspaceTabOverflowConfig, WorkspaceTabOverflowPlan,
    WorkspaceTabOverflowPlanner,
};
pub use state::{WorkspaceTabBarState, WorkspaceTabChildState};

pub type CloseableTab = WorkspaceTab;
pub type CloseableTabChildState = WorkspaceTabChildState;
pub type CloseableTabContextCommand = WorkspaceTabContextCommand;
pub type CloseableTabContextMenu = WorkspaceTabContextMenu;
pub type CloseableTabDropPosition = WorkspaceTabDropPosition;
pub type CloseableTabDropRules = WorkspaceTabDropRules;
pub type CloseableTabGroup = WorkspaceTabGroup;
pub type CloseableTabGroupContextCommand = WorkspaceGroupContextCommand;
pub type CloseableTabGroupId = WorkspaceTabGroupId;
pub type CloseableTabGroupTarget = WorkspaceTabGroupTarget;
pub type CloseableTabId = WorkspaceTabId;
pub type CloseableTabKey = WorkspaceTabKey;
pub type CloseableTabKeyboardController = WorkspaceTabKeyboardController;
pub type CloseableTabKeyboardInput = WorkspaceTabKeyboardInput;
pub type CloseableTabKeyboardShortcut = WorkspaceTabKeyboardShortcut;
pub type CloseableTabOverflowConfig = WorkspaceTabOverflowConfig;
pub type CloseableTabOverflowPlan = WorkspaceTabOverflowPlan;
pub type CloseableTabOverflowPlanner = WorkspaceTabOverflowPlanner;
pub type CloseableTabStrip = WorkspaceTabBar;
pub type CloseableTabStripAction = WorkspaceTabBarAction;
pub type CloseableTabStripEvent = WorkspaceTabBarEvent;
pub type CloseableTabStripOptions = WorkspaceTabBarOptions;
pub type CloseableTabStripState = WorkspaceTabBarState;
pub type CloseableTabTone = WorkspaceTabTone;
pub type MeasuredCloseableTab = MeasuredWorkspaceTab;
pub type TabGroup = WorkspaceTabGroup;
pub type TabGroupId = WorkspaceTabGroupId;
pub type TabGroupTarget = WorkspaceTabGroupTarget;
pub type TabId = WorkspaceTabId;

#[cfg(test)]
mod tests;
