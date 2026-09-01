use crate::molecule::RgbaColor;

/// An opaque capability target for one projected tab.
pub struct TabStripTabTarget {
    pub(crate) payload: Box<[u8]>,
}

/// An opaque capability target for one projected group.
pub struct TabStripGroupTarget {
    pub(crate) payload: Box<[u8]>,
}

/// An opaque host-issued swatch choice. It never carries a color value.
pub struct TabStripSwatchTarget {
    pub(crate) payload: Box<[u8]>,
}

/// Localized text retained by KUC for presentation only.
pub struct TabStripText {
    pub(crate) value: String,
}

/// Localized presentation for one generic icon-only control.
pub struct TabStripControlPresentation {
    pub(crate) tooltip: TabStripText,
    pub(crate) accessibility_label: TabStripText,
}

/// Localized presentation for the navigation controls of a generic tab strip.
pub struct TabStripNavigationPresentation {
    pub(crate) previous: TabStripControlPresentation,
    pub(crate) next: TabStripControlPresentation,
    pub(crate) overflow: Option<TabStripControlPresentation>,
}

/// Generic retained horizontal-scroll presentation for one tab strip revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabStripScrollPresentation {
    pub(crate) request_active_reveal: bool,
}

/// Generic tab capabilities used by the renderer and route table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabStripTabCapabilities {
    pub(crate) active: bool,
    pub(crate) dirty: bool,
    pub(crate) pinned: bool,
    pub(crate) selectable: bool,
    pub(crate) closeable: bool,
    pub(crate) draggable: bool,
    pub(crate) accepts_tab_drop: bool,
    pub(crate) groupable: bool,
    pub(crate) virtual_tab: bool,
}

/// Generic group capabilities used by the renderer and route table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabStripGroupCapabilities {
    pub(crate) collapsed: bool,
    pub(crate) collapsible: bool,
    pub(crate) menu_available: bool,
    pub(crate) renamable: bool,
    pub(crate) recolorable: bool,
    pub(crate) closeable: bool,
    pub(crate) ungroupable: bool,
    pub(crate) draggable: bool,
    pub(crate) accepts_tab_drop: bool,
}

/// Root-level capabilities for generic tab-strip commands without target data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabStripSurfaceCapabilities {
    pub(crate) previous_available: bool,
    pub(crate) next_available: bool,
    pub(crate) overflow_available: bool,
    pub(crate) restore_available: bool,
    pub(crate) create_group_available: bool,
    pub(crate) tab_drop_at_end_available: bool,
}

/// One host-issued group swatch. KUC maps it to a private visual palette.
pub struct TabStripSwatchDescriptor {
    pub(crate) target: TabStripSwatchTarget,
    pub(crate) display_color: RgbaColor,
    pub(crate) selected: bool,
    pub(crate) accessibility_label: Option<TabStripText>,
}

/// Generic, non-wire operation associated with one tab-strip overlay entry.
/// The parent tab or group supplies the implicit opaque target when the route
/// table is built; cross-target operations carry only the additional target.
pub enum TabStripMenuOperation {
    RequestClose,
    CloseOthers,
    CloseAll,
    CloseToLeft,
    CloseToRight,
    RestoreClosed,
    SetPinned(bool),
    CreateGroup,
    MoveToGroup(TabStripGroupTarget),
    RemoveFromGroup,
    Ungroup,
    CloseGroup,
    Recolor(TabStripSwatchTarget),
}

/// One non-wire retained overlay entry. An entry with children is a submenu;
/// labels and tree position are presentation only and never select an action.
pub struct TabStripMenuEntry {
    pub(crate) label: TabStripText,
    pub(crate) accessibility_label: TabStripText,
    pub(crate) separator: bool,
    pub(crate) enabled: bool,
    pub(crate) checked: bool,
    pub(crate) operation: Option<TabStripMenuOperation>,
    pub(crate) children: Vec<Self>,
}

/// Host-projected context-menu entries for one tab. Absence means no menu route
/// exists for that tab; KUC must not infer menu contents from capabilities.
#[derive(Default)]
pub struct TabStripContextMenuPresentation {
    pub(crate) entries: Vec<TabStripMenuEntry>,
}

/// Host-projected group-popup presentation. Opening, focus, submenu state, and
/// dismissal are retained inside KUC; `rename_placeholder` only authorizes the
/// KUC-private inline rename sub-state for this accepted projection.
pub struct TabStripGroupPopupPresentation {
    pub(crate) rename_placeholder: Option<TabStripText>,
    pub(crate) entries: Vec<TabStripMenuEntry>,
}

/// Generic projected tab descriptor. The owned presentation text is not
/// exposed through Debug or serialization; KUC alone reads it during render.
pub struct TabStripTabDescriptor {
    pub(crate) target: TabStripTabTarget,
    pub(crate) label: TabStripText,
    pub(crate) tooltip: Option<TabStripText>,
    pub(crate) accessibility_label: Option<TabStripText>,
    pub(crate) capabilities: TabStripTabCapabilities,
    pub(crate) trailing_control: Option<TabStripControlPresentation>,
    pub(crate) context_menu: Option<TabStripContextMenuPresentation>,
}

/// Generic nested projected group descriptor.
pub struct TabStripGroupDescriptor {
    pub(crate) target: TabStripGroupTarget,
    pub(crate) label: TabStripText,
    pub(crate) accessibility_label: Option<TabStripText>,
    pub(crate) capabilities: TabStripGroupCapabilities,
    pub(crate) swatches: Vec<TabStripSwatchDescriptor>,
    pub(crate) tabs: Vec<TabStripTabDescriptor>,
    pub(crate) groups: Vec<TabStripGroupDescriptor>,
    pub(crate) popup: Option<TabStripGroupPopupPresentation>,
}

/// Generic tab-strip projection. It is intentionally not serializable.
pub struct TabStripProjection {
    pub(crate) revision: u64,
    pub(crate) correlation: TabStripCorrelation,
    pub(crate) groups: Vec<TabStripGroupDescriptor>,
    pub(crate) tabs: Vec<TabStripTabDescriptor>,
    pub(crate) capabilities: TabStripSurfaceCapabilities,
    pub(crate) navigation: Option<TabStripNavigationPresentation>,
    pub(crate) scroll_presentation: TabStripScrollPresentation,
}

/// Opaque correlation token for one projection revision.
pub struct TabStripCorrelation {
    pub(crate) payload: Box<[u8]>,
}

/// Non-wire lease consumed by the KUC retained root integration.
///
/// Constructing a lease alone does not mount a strip or authorize a state
/// change. Only an `EguiTextCommandSurfaceHostProjectionLease` consumes it.
pub struct TabStripProjectionLease {
    pub(crate) projection: TabStripProjection,
    pub(crate) proposal_port:
        Option<super::super::tab_strip_proposal_port::TabStripProposalPortHandle>,
}
