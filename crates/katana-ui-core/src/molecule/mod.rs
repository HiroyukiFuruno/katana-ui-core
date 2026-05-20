mod app_primitives;
mod attachment_chip;
mod basic;
mod card;
mod chip_group;
mod color;
mod diff;
mod disclosure;
mod disclosure_foundation;
mod drag_preview;
mod empty_state;
mod segmented_toggle;
pub mod selection;
pub mod shortcut_cheatsheet;
mod skeleton_cluster;
mod state;
pub mod status_bar;
pub mod structured;
mod toast_stack_manager;
pub mod toolbar;
mod virtualization;

pub use app_primitives::{
    CollapsibleSidebar, MotionPrimitive, MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy,
    ResizableWidth, RowHeightProvider, SettingsControl, SettingsControlKind, SettingsControlOption,
    SettingsDirtyVisualization, SettingsField, SettingsKeyboardInput, SettingsList,
    SettingsListAction, SettingsListDensity, SettingsListEvent, SettingsSection, SettingsValue,
    SidebarEvent, SidebarMode, VirtualRange, VirtualizationConfig, VirtualizedEvent,
    VirtualizedList, VirtualizedTree,
};
pub use attachment_chip::{
    AttachmentChip, AttachmentChipAction, AttachmentChipEvent, AttachmentKind, AttachmentMeta,
    AttachmentProgress, AttachmentStatus, AttachmentThumbnail,
};
pub use basic::{FormField, List, Menu, MoleculeEventRouting, Toolbar};
pub use card::Card;
pub use chip_group::{
    ChipGroup, ChipGroupAction, ChipGroupEvent, ChipGroupFocusTarget, ChipGroupLayout,
    ChipGroupOverflow, MeasuredChip,
};
pub use color::{ColorBlendingMode, ColorPicker, RgbaColor};
pub use diff::{
    CodeDiff, CodeDiffBuildError, CodeDiffDirection, CodeDiffLine, CodeDiffLineHighlight,
    CodeDiffLineKind, CodeDiffMode, CodeDiffSide, CodeDiffSource, CodeDiffSummary,
    CodeDiffTextSource, CodeDiffWhitespace, CollapsedBlock, HighlightRange,
};
pub use disclosure::{
    Accordion, Banner, BannerAccessibilityRole, BannerAction, BannerActionKind, BannerCommand,
    BannerDensity, BannerEvent, BannerLiveRegion, BannerPlacementHint, BannerSeverity, BannerState,
    BannerVisualContract, HoverCard, HoverCardAction, HoverCardDelayState, HoverCardEvent, Modal,
    ModalOverlay, NotificationToast, Popover, PopoverActionSlot, PopoverArrowSpec,
    PopoverFocusManagement, PopoverSlots, SearchBox, SlideControl, Tooltip,
};
pub use disclosure_foundation::DisclosureTriggerArea;
pub use drag_preview::DragPreview;
pub use empty_state::{
    EmptyState, EmptyStateAction, EmptyStateActionId, EmptyStateAlignment,
    EmptyStateContractViolation, EmptyStateEvent, EmptyStateLayoutRect, EmptyStateLayoutSnapshot,
    EmptyStateSize, EmptyStateTone,
};
pub use segmented_toggle::SegmentedToggle;
pub use selection::{
    Breadcrumb, ChoiceItem, ComboBox, ContextMenu, ContextMenuAction, ContextMenuAnchor,
    ContextMenuCloseReason, ContextMenuDividerTone, ContextMenuEvent, ContextMenuItem,
    ContextMenuItemKind, ContextMenuKeyboardInput, ContextMenuKeyboardIntent,
    ContextMenuKeyboardNavigator, ContextMenuPlacement, ContextMenuPlacementResolver,
    ContextMenuPlacementResult, ContextMenuRect, ContextMenuSize, ContextMenuTypeAheadBuffer,
    ContextMenuViewport, MenuButton, SelectBox, SelectionList, SideMenu, Tabs,
    WindowControlButtonGroup, WindowControlButtonGroupAction, WindowControlButtonGroupEvent,
    WindowControlButtonGroupOptions, WindowControlButtonGroupState, WindowControlKind,
    WindowControlSize, WindowControlVisibility, WindowControlsPosition,
};
pub use shortcut_cheatsheet::{
    ShortcutCheatsheet, ShortcutCheatsheetAction, ShortcutCheatsheetEvent, ShortcutCheatsheetGroup,
    ShortcutCheatsheetItem, ShortcutCheatsheetLayout,
};
pub use skeleton_cluster::{SkeletonCluster, SkeletonClusterPreset};
pub use status_bar::{
    ProgressMeterShape, ProgressMeterSpec, StatusBar, StatusBarAction, StatusBarContractViolation,
    StatusBarDensity, StatusBarEvent, StatusBarMode, StatusBarPopoverSpec, StatusBarSegment,
    StatusBarSegmentAlignment, StatusBarState,
};
pub use structured::{
    ArrayEditorItem, BulkFixSkipReason, CLOSEABLE_TAB_DRAG_TAG, CloseableTab,
    CloseableTabChildState, CloseableTabContextCommand, CloseableTabContextMenu,
    CloseableTabDropPosition, CloseableTabDropRules, CloseableTabGroup,
    CloseableTabGroupContextCommand, CloseableTabGroupId, CloseableTabGroupTarget, CloseableTabId,
    CloseableTabKey, CloseableTabKeyboardController, CloseableTabKeyboardInput,
    CloseableTabKeyboardShortcut, CloseableTabOverflowConfig, CloseableTabOverflowPlan,
    CloseableTabOverflowPlanner, CloseableTabStrip, CloseableTabStripAction,
    CloseableTabStripEvent, CloseableTabStripOptions, CloseableTabStripState, CloseableTabTone,
    CollapsiblePanel, CollapsiblePanelAction, CollapsiblePanelEvent, CollapsiblePanelOptions,
    CollapsiblePanelState, CollapsiblePanelWidth, CommandItem, CommandKeyboardInput,
    CommandLauncherAction, CommandLauncherEvent, CommandPalette, CommandResultRow,
    CommandResultRows, DiagnosticAction, DiagnosticFixPreview, DiagnosticId, DiagnosticItem,
    DiagnosticKeyboardInput, DiagnosticLocation, DiagnosticSeverity, DiagnosticsGroup,
    DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListAction, DiagnosticsListEvent,
    DiagnosticsListOptions, DiagnosticsListPlanner, DiagnosticsListState, DiagnosticsSortBy,
    DiagnosticsVisibleSnapshot, DynamicArrayEditor, HighlightMove, MeasuredCloseableTab, PanelMode,
    PanelSide, ReplaceMode, SearchControlStrip, SearchControlStripAction, SearchControlStripEvent,
    SearchNavigationDirection, SearchOptionKind, SearchOptions, SearchReplaceScope, StartupState,
    StartupStatePanel, StartupStatePanelAction, StartupStatePanelEvent, StartupStatePanelOptions,
    TabGroup, TabGroupId, TabGroupTarget, TabId, TreeLineStyle, TreeNode, TreeNodeKind, TreeView,
};
pub use toast_stack_manager::{
    ActiveToast, ToastAction, ToastActionKind, ToastDedupStrategy, ToastDismissReason,
    ToastPayload, ToastPosition, ToastReplaceKind, ToastStackAction, ToastStackDirection,
    ToastStackEvent, ToastStackManager, ToastStackOptions, ToastStackState,
    ToastStackVisualContract,
};

#[cfg(test)]
mod tests {
    use super::MoleculeEventRouting;
    use super::{Card, Toolbar};
    use crate::atom::Button;
    use crate::render_model::{UiNodeId, UiNodeKind, UiTree};

    #[test]
    fn molecule_snapshot_keeps_children() {
        let tree = UiTree::new(Toolbar::new("main").child(Button::new("Save")));
        assert_eq!(1, tree.root().children().len());
    }

    #[test]
    fn card_uses_molecule_kind() {
        let tree = UiTree::new(Card::new("summary"));
        assert_eq!(UiNodeKind::Card, tree.root().kind());
    }

    #[test]
    fn molecule_event_routing_visits_nested_target_then_parents() {
        let route = MoleculeEventRouting::bubble_nested(
            UiNodeId::new("button"),
            UiNodeId::new("toolbar"),
            UiNodeId::new("root"),
            false,
        );
        let actual: Vec<&str> = route.order().iter().map(UiNodeId::as_str).collect();
        assert_eq!(["button", "toolbar", "root"], actual.as_slice());
    }
}
