mod app_primitives;
mod attachment_chip;
mod basic;
mod card;
mod chip_group;
mod color;
mod diff;
mod disclosure;
mod disclosure_foundation;
mod empty_state;
mod segmented_toggle;
mod selection;
mod state;
mod structured;
pub mod toolbar;

pub use app_primitives::{
    AppShell, AppShellSlot, AppShellSlotKind, CollapsibleSidebar, MotionPrimitive,
    MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy, ResizableWidth, RowHeightProvider,
    SettingsControlKind, SettingsDirtyVisualization, SettingsField, SettingsList,
    SettingsListEvent, SettingsSection, ShortcutCheatsheet, ShortcutCheatsheetEntry,
    ShortcutCheatsheetEvent, ShortcutCombo, ShortcutPlatform, SidebarEvent, SidebarMode, Skeleton,
    SkeletonAnimation, SkeletonCluster, SkeletonShape, SplashBackground, SplashEvent, SplashScreen,
    SplashSize, SplashStatus, TitleBar, TitleBarEvent, TitleBarStyle, VirtualRange,
    VirtualizationConfig, VirtualizedEvent, VirtualizedList, VirtualizedTree, WindowChrome,
    WindowControlKind, WindowControlsPosition,
};
pub use attachment_chip::{
    AttachmentChip, AttachmentChipAction, AttachmentChipEvent, AttachmentKind, AttachmentMeta,
    AttachmentProgress, AttachmentStatus, AttachmentThumbnail,
};
pub use basic::{FormField, List, Menu, MoleculeEventRouting, StatusBar, Toolbar};
pub use card::Card;
pub use chip_group::{
    ChipGroup, ChipGroupAction, ChipGroupEvent, ChipGroupFocusTarget, ChipGroupLayout,
    ChipGroupOverflow, MeasuredChip,
};
pub use color::{ColorBlendingMode, ColorPicker, RgbaColor};
pub use diff::{
    CodeDiff, CodeDiffDirection, CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource,
    CodeDiffWhitespace, CollapsedBlock, HighlightRange,
};
pub use disclosure::{
    Accordion, HoverCard, HoverCardAction, HoverCardDelayState, HoverCardEvent, Modal,
    ModalOverlay, NotificationToast, Popover, PopoverActionSlot, PopoverArrowSpec,
    PopoverFocusManagement, PopoverSlots, SearchBox, SlideControl, Tooltip,
};
pub use disclosure_foundation::DisclosureTriggerArea;
pub use empty_state::{
    EmptyState, EmptyStateAction, EmptyStateActionId, EmptyStateAlignment,
    EmptyStateContractViolation, EmptyStateEvent, EmptyStateLayoutSnapshot, EmptyStateSize,
    EmptyStateTone,
};
pub use segmented_toggle::SegmentedToggle;
pub use selection::{
    Breadcrumb, ChoiceItem, ComboBox, ContextMenu, ContextMenuAction, ContextMenuAnchor,
    ContextMenuCloseReason, ContextMenuEvent, ContextMenuItem, ContextMenuItemKind,
    ContextMenuKeyboardInput, ContextMenuKeyboardIntent, ContextMenuKeyboardNavigator,
    ContextMenuPlacement, ContextMenuPlacementResolver, ContextMenuPlacementResult,
    ContextMenuRect, ContextMenuSize, ContextMenuTypeAheadBuffer, ContextMenuViewport, MenuButton,
    SelectBox, SelectionList, SideMenu, Tabs,
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
    CommandItem, CommandPalette, DiagnosticAction, DiagnosticFixPreview, DiagnosticId,
    DiagnosticItem, DiagnosticKeyboardInput, DiagnosticLocation, DiagnosticSeverity,
    DiagnosticsGroup, DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListAction,
    DiagnosticsListEvent, DiagnosticsListOptions, DiagnosticsListPlanner, DiagnosticsListState,
    DiagnosticsSortBy, DiagnosticsVisibleSnapshot, DynamicArrayEditor, MeasuredCloseableTab,
    TabGroup, TabGroupId, TabGroupTarget, TabId, TreeLineStyle, TreeNode, TreeNodeKind, TreeView,
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
