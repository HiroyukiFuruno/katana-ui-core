mod accessors;
pub mod collapsible_panel;
pub mod command_launcher_results;
mod diagnostics_list;
mod dynamic_array_editor;
mod file_tree;
mod items;
mod model;
mod options;
mod options_actions;
mod options_extra;
pub mod search_control_strip;
pub mod source_address_strip;
pub mod startup_state_panel;
mod tree_view_hit_test;
mod types;
mod workspace_tab_bar;

pub use collapsible_panel::{
    CollapsiblePanel, CollapsiblePanelAction, CollapsiblePanelEvent, CollapsiblePanelOptions,
    CollapsiblePanelState, PanelMode, PanelSide, ResizableWidth as CollapsiblePanelWidth,
};
pub use command_launcher_results::{
    CommandKeyboardInput, CommandLauncherAction, CommandLauncherEvent, CommandResultRow,
    CommandResultRows, HighlightMove,
};
pub use diagnostics_list::{
    BulkFixSkipReason, DiagnosticAction, DiagnosticFixPreview, DiagnosticId, DiagnosticItem,
    DiagnosticKeyboardInput, DiagnosticLocation, DiagnosticSeverity, DiagnosticsGroup,
    DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListAction, DiagnosticsListEvent,
    DiagnosticsListOptions, DiagnosticsListPlanner, DiagnosticsListState, DiagnosticsSortBy,
    DiagnosticsVisibleSnapshot,
};
pub use dynamic_array_editor::{DynamicArrayEditorAction, DynamicArrayEditorEvent};
pub use file_tree::{
    FileTree, FileTreeAction, FileTreeHitRect, FileTreeHitTarget, FileTreeHitTestInput,
    FileTreeItem, FileTreeState,
};
pub use items::{ArrayEditorItem, CommandItem, TreeNode, TreeNodeKind};
pub use model::{CommandPalette, DynamicArrayEditor, TreeView};
pub use search_control_strip::{
    ReplaceMode, SearchControlStrip, SearchControlStripAction, SearchControlStripEvent,
    SearchNavigationDirection, SearchOptionKind, SearchOptions, SearchReplaceScope,
};
pub use source_address_strip::{
    SourceAddressAction, SourceAddressEntry, SourceAddressEvent, SourceAddressPresentation,
    SourceAddressStrip, SourceAddressSubmission,
};
pub use startup_state_panel::{
    StartupState, StartupStatePanel, StartupStatePanelAction, StartupStatePanelEvent,
    StartupStatePanelOptions,
};
pub use tree_view_hit_test::{
    TreeViewAction, TreeViewHitRect, TreeViewHitTarget, TreeViewHitTestInput,
};
pub use types::TreeLineStyle;
pub use workspace_tab_bar::{
    CLOSEABLE_TAB_DRAG_TAG, CloseableTab, CloseableTabChildState, CloseableTabClosePresentation,
    CloseableTabContextCommand, CloseableTabContextMenu, CloseableTabDropPosition,
    CloseableTabDropRules, CloseableTabGroup, CloseableTabGroupContextCommand, CloseableTabGroupId,
    CloseableTabGroupTarget, CloseableTabId, CloseableTabKey, CloseableTabKeyboardController,
    CloseableTabKeyboardInput, CloseableTabKeyboardShortcut, CloseableTabOverflowConfig,
    CloseableTabOverflowPlan, CloseableTabOverflowPlanner, CloseableTabScrollConfig,
    CloseableTabScrollPlan, CloseableTabScrollPlanner, CloseableTabStrip, CloseableTabStripAction,
    CloseableTabStripEvent, CloseableTabStripIntent, CloseableTabStripOptions,
    CloseableTabStripState, CloseableTabTone, ClosedTab, MAX_RECENTLY_CLOSED_TABS,
    MeasuredCloseableTab, TabGroup, TabGroupId, TabGroupTarget, TabId,
};

#[cfg(test)]
mod file_tree_hit_item_tests;
#[cfg(test)]
mod file_tree_hit_tests;
#[cfg(test)]
mod file_tree_tests;
