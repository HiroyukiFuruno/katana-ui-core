use crate::render_model::UiIconProps;

/// Opaque target supplied by the host for a tab.
pub struct SanitizedTabTarget {
    pub(super) opaque: Box<[u8]>,
}

/// Generic tab state capabilities.
#[derive(Default)]
pub struct SanitizedTabCapabilities {
    pub(super) active: bool,
    pub(super) dirty: bool,
    pub(super) pinned: bool,
    pub(super) close: bool,
}

/// Explicit localized presentation for a tab close affordance.
pub struct SanitizedTabClosePresentation {
    pub(super) visible_label: String,
    pub(super) tooltip: String,
    pub(super) accessibility_label: String,
}

/// Opaque target supplied by the host for a tab group.
pub struct SanitizedTabGroupTarget {
    pub(super) opaque: Box<[u8]>,
}

/// Generic tab group state capabilities.
#[derive(Default)]
pub struct SanitizedTabGroupCapabilities {
    pub(super) collapse: bool,
    pub(super) menu: bool,
    pub(super) rename: bool,
    pub(super) recolor: bool,
    pub(super) close: bool,
    pub(super) ungroup: bool,
    pub(super) drag: bool,
}

/// Generic tab projection with localized presentation data.
pub struct SanitizedTab {
    pub(super) target: SanitizedTabTarget,
    pub(super) order: u32,
    pub(super) label: String,
    pub(super) icon: Option<UiIconProps>,
    pub(super) capabilities: SanitizedTabCapabilities,
    pub(super) close_presentation: Option<SanitizedTabClosePresentation>,
}

/// Generic recursively nestable tab group.
pub struct SanitizedTabGroup {
    pub(super) target: SanitizedTabGroupTarget,
    pub(super) order: u32,
    pub(super) label: String,
    pub(super) icon: Option<UiIconProps>,
    pub(super) capabilities: SanitizedTabGroupCapabilities,
    pub(super) tabs: Vec<SanitizedTab>,
    pub(super) groups: Vec<SanitizedTabGroup>,
}

/// Generic tab projection composed from ordered nested groups.
#[derive(Default)]
pub struct SanitizedTabProjection {
    pub(super) groups: Vec<SanitizedTabGroup>,
}
