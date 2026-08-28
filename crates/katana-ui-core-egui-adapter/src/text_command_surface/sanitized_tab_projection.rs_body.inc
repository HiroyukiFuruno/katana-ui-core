use katana_ui_core::render_model::UiIconProps;
use sha2::{Digest, Sha256};

/// Opaque target supplied by the host for a tab.
pub struct SanitizedTabTarget {
    pub(super) opaque: Box<[u8]>,
}

impl SanitizedTabTarget {
    /// Creates a target without assigning meaning to the supplied data.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
        }
    }
}

impl std::fmt::Debug for SanitizedTabTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedTabTarget(..)")
    }
}

/// Generic tab state capabilities.
#[derive(Default)]
pub struct SanitizedTabCapabilities {
    active: bool,
    dirty: bool,
    pinned: bool,
    close: bool,
}

impl std::fmt::Debug for SanitizedTabCapabilities {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedTabCapabilities")
            .field("active", &self.active)
            .field("dirty", &self.dirty)
            .field("pinned", &self.pinned)
            .field("close", &self.close)
            .finish()
    }
}

impl SanitizedTabCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active: false,
            dirty: false,
            pinned: false,
            close: false,
        }
    }

    #[must_use]
    pub const fn active_state(mut self, value: bool) -> Self {
        self.active = value;
        self
    }

    #[must_use]
    pub const fn dirty_state(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    #[must_use]
    pub const fn pinned_state(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    #[must_use]
    pub const fn close_state(mut self, value: bool) -> Self {
        self.close = value;
        self
    }
}

/// Explicit localized presentation for a tab close affordance.
pub struct SanitizedTabClosePresentation {
    visible_label: String,
    tooltip: String,
    accessibility_label: String,
}

impl std::fmt::Debug for SanitizedTabClosePresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SanitizedTabClosePresentation(..)")
    }
}

impl SanitizedTabClosePresentation {
    #[must_use]
    pub fn new(
        visible_label: impl Into<String>,
        tooltip: impl Into<String>,
        accessibility_label: impl Into<String>,
    ) -> Self {
        Self {
            visible_label: visible_label.into(),
            tooltip: tooltip.into(),
            accessibility_label: accessibility_label.into(),
        }
    }
}

/// Opaque target supplied by the host for a tab group.
pub struct SanitizedTabGroupTarget {
    pub(super) opaque: Box<[u8]>,
}

impl SanitizedTabGroupTarget {
    /// Creates a target without assigning meaning to the supplied data.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
        }
    }
}

impl std::fmt::Debug for SanitizedTabGroupTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedTabGroupTarget(..)")
    }
}

/// Generic tab group state capabilities.
#[derive(Default)]
pub struct SanitizedTabGroupCapabilities {
    collapse: bool,
    menu: bool,
    rename: bool,
    recolor: bool,
    close: bool,
    ungroup: bool,
    drag: bool,
}

impl std::fmt::Debug for SanitizedTabGroupCapabilities {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedTabGroupCapabilities")
            .field("collapse", &self.collapse)
            .field("menu", &self.menu)
            .field("rename", &self.rename)
            .field("recolor", &self.recolor)
            .field("close", &self.close)
            .field("ungroup", &self.ungroup)
            .field("drag", &self.drag)
            .finish()
    }
}

impl SanitizedTabGroupCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            collapse: false,
            menu: false,
            rename: false,
            recolor: false,
            close: false,
            ungroup: false,
            drag: false,
        }
    }

    #[must_use]
    pub const fn collapse_state(mut self, value: bool) -> Self {
        self.collapse = value;
        self
    }

    #[must_use]
    pub const fn menu_state(mut self, value: bool) -> Self {
        self.menu = value;
        self
    }

    #[must_use]
    pub const fn rename_state(mut self, value: bool) -> Self {
        self.rename = value;
        self
    }

    #[must_use]
    pub const fn recolor_state(mut self, value: bool) -> Self {
        self.recolor = value;
        self
    }

    #[must_use]
    pub const fn close_state(mut self, value: bool) -> Self {
        self.close = value;
        self
    }

    #[must_use]
    pub const fn ungroup_state(mut self, value: bool) -> Self {
        self.ungroup = value;
        self
    }

    #[must_use]
    pub const fn drag_state(mut self, value: bool) -> Self {
        self.drag = value;
        self
    }
}

/// Generic tab projection with localized presentation data.
pub struct SanitizedTab {
    target: SanitizedTabTarget,
    order: u32,
    label: String,
    icon: Option<UiIconProps>,
    capabilities: SanitizedTabCapabilities,
    close_presentation: Option<SanitizedTabClosePresentation>,
}

impl SanitizedTab {
    #[must_use]
    pub fn new(target: SanitizedTabTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            icon: None,
            capabilities: SanitizedTabCapabilities::new(),
            close_presentation: None,
        }
    }

    #[must_use]
    pub fn with_icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub fn with_capabilities(mut self, value: SanitizedTabCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn with_close_presentation(mut self, value: SanitizedTabClosePresentation) -> Self {
        self.close_presentation = Some(value);
        self
    }
}

/// Generic recursively nestable tab group.
pub struct SanitizedTabGroup {
    target: SanitizedTabGroupTarget,
    order: u32,
    label: String,
    icon: Option<UiIconProps>,
    capabilities: SanitizedTabGroupCapabilities,
    tabs: Vec<SanitizedTab>,
    groups: Vec<SanitizedTabGroup>,
}

impl SanitizedTabGroup {
    #[must_use]
    pub fn new(target: SanitizedTabGroupTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            icon: None,
            capabilities: SanitizedTabGroupCapabilities::new(),
            tabs: Vec::new(),
            groups: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub fn with_capabilities(mut self, value: SanitizedTabGroupCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn tab(mut self, value: SanitizedTab) -> Self {
        self.tabs.push(value);
        self
    }

    #[must_use]
    pub fn group(mut self, value: SanitizedTabGroup) -> Self {
        self.groups.push(value);
        self
    }
}

/// Generic tab projection composed from ordered nested groups.
#[derive(Default)]
pub struct SanitizedTabProjection {
    groups: Vec<SanitizedTabGroup>,
}

impl std::fmt::Debug for SanitizedTab {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedTab")
            .field("target", &self.target)
            .field("order", &self.order)
            .field("label", &self.label)
            .field("icon", &self.icon.is_some())
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl std::fmt::Debug for SanitizedTabGroup {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedTabGroup")
            .field("target", &self.target)
            .field("order", &self.order)
            .field("label", &self.label)
            .field("icon", &self.icon.is_some())
            .field("capabilities", &self.capabilities)
            .field("tabs", &self.tabs)
            .field("groups", &self.groups)
            .finish()
    }
}

impl std::fmt::Debug for SanitizedTabProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedTabProjection")
            .field("groups", &self.groups)
            .finish()
    }
}

impl SanitizedTabProjection {
    #[must_use]
    pub fn new(groups: impl Into<Vec<SanitizedTabGroup>>) -> Self {
        Self {
            groups: groups.into(),
        }
    }

    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self.stable_fingerprint() == other.stable_fingerprint()
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update(b"kuc.sanitized-tab-projection/v1\0");
        digest.update((self.groups.len() as u64).to_le_bytes());
        for group in &self.groups {
            update_group_fingerprint(&mut digest, group);
        }
        hex::encode(digest.finalize())
    }
}

fn update_group_fingerprint(digest: &mut Sha256, group: &SanitizedTabGroup) {
    digest.update(b"group");
    digest.update(Sha256::digest(&group.target.opaque));
    digest.update(group.order.to_le_bytes());
    update_text(digest, &group.label);
    update_optional_icon(digest, group.icon.as_ref());
    digest.update([
        u8::from(group.capabilities.collapse),
        u8::from(group.capabilities.menu),
        u8::from(group.capabilities.rename),
        u8::from(group.capabilities.recolor),
        u8::from(group.capabilities.close),
        u8::from(group.capabilities.ungroup),
        u8::from(group.capabilities.drag),
    ]);
    digest.update((group.tabs.len() as u64).to_le_bytes());
    for tab in &group.tabs {
        digest.update(b"tab");
        digest.update(tab.order.to_le_bytes());
        digest.update(Sha256::digest(&tab.target.opaque));
        update_text(digest, &tab.label);
        update_optional_icon(digest, tab.icon.as_ref());
        digest.update([
            u8::from(tab.capabilities.active),
            u8::from(tab.capabilities.dirty),
            u8::from(tab.capabilities.pinned),
            u8::from(tab.capabilities.close),
        ]);
        update_optional_close_presentation(digest, tab.close_presentation.as_ref());
    }
    digest.update((group.groups.len() as u64).to_le_bytes());
    for child in &group.groups {
        update_group_fingerprint(digest, child);
    }
}

fn update_text(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value.as_bytes());
}

fn update_optional_icon(digest: &mut Sha256, icon: Option<&UiIconProps>) {
    match icon {
        Some(icon) => {
            digest.update(b"icon");
            update_text(digest, &format!("{icon:?}"));
        }
        None => digest.update(b"no-icon"),
    }
}

fn update_optional_close_presentation(
    digest: &mut Sha256,
    presentation: Option<&SanitizedTabClosePresentation>,
) {
    match presentation {
        Some(presentation) => {
            digest.update(b"close-presentation");
            update_text(digest, &presentation.visible_label);
            update_text(digest, &presentation.tooltip);
            update_text(digest, &presentation.accessibility_label);
        }
        None => digest.update(b"no-close-presentation"),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
        SanitizedTabGroupCapabilities, SanitizedTabGroupTarget, SanitizedTabProjection,
        SanitizedTabTarget,
    };
    use katana_ui_core::render_model::UiIconProps;

    #[test]
    fn debug_contracts_cover_composed_tab_projection_without_exposing_targets() {
        let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 1, "Tab")
            .with_capabilities(SanitizedTabCapabilities::new().active_state(true));
        let group = SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([2]),
            2,
            "Group",
        )
        .with_icon(UiIconProps::new("<svg/>"))
        .with_capabilities(SanitizedTabGroupCapabilities::new().menu_state(true))
        .tab(tab);
        let projection = SanitizedTabProjection::new([group]);

        let debug = format!("{projection:?}");
        assert!(debug.contains("SanitizedTabProjection"));
        assert!(debug.contains("SanitizedTabGroup"));
        assert!(debug.contains("SanitizedTab"));
        assert!(debug.contains("active: true"));
        assert!(debug.contains("menu: true"));
        assert!(!debug.contains("[1]"));
        assert!(!debug.contains("[2]"));
        assert!(projection.same_as(&projection));
    }

    #[test]
    fn construction_preserves_localized_nested_order_and_capabilities() {
        let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1, 2]), 7, "編集")
            .with_icon(UiIconProps::new("<svg/>"))
            .with_capabilities(
                SanitizedTabCapabilities::new()
                    .active_state(true)
                    .dirty_state(true)
                    .pinned_state(true)
                    .close_state(true),
            );
        let group_capabilities = SanitizedTabGroupCapabilities::new()
            .collapse_state(true)
            .menu_state(true)
            .rename_state(true)
            .recolor_state(true)
            .close_state(true)
            .ungroup_state(true)
            .drag_state(true);
        assert_eq!(format!("{:?}", tab.target), "SanitizedTabTarget(..)");
        let child = SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([3, 4, 5]),
            3,
            "固定",
        )
        .with_capabilities(group_capabilities)
        .tab(tab);
        let projection = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([9, 9, 1]),
            9,
            "作業",
        )
        .group(child)]);

        let group = &projection.groups[0];
        assert_eq!(group.order, 9);
        assert_eq!(group.label, "作業");
        assert!(group.icon.is_none());
        assert_eq!(format!("{:?}", group.target), "SanitizedTabGroupTarget(..)");
        assert_eq!(group.groups.len(), 1);
        let nested = &group.groups[0];
        assert_eq!(nested.order, 3);
        assert_eq!(nested.label, "固定");
        assert_eq!(
            format!("{:?}", nested.target),
            "SanitizedTabGroupTarget(..)"
        );
        assert_eq!(nested.tabs.len(), 1);
        assert!(nested.capabilities.collapse);
        assert!(nested.capabilities.menu);
        assert!(nested.capabilities.rename);
        assert!(nested.capabilities.recolor);
        assert!(nested.capabilities.close);
        assert!(nested.capabilities.ungroup);
        assert!(nested.capabilities.drag);
        assert_eq!(nested.tabs[0].order, 7);
        assert_eq!(nested.tabs[0].label, "編集");
        assert!(nested.tabs[0].icon.is_some());
        assert!(nested.tabs[0].capabilities.active);
        assert!(nested.tabs[0].capabilities.dirty);
        assert!(nested.tabs[0].capabilities.pinned);
        assert!(nested.tabs[0].capabilities.close);
    }

    #[test]
    fn close_presentation_requires_caller_supplied_localized_labels() {
        let presentation =
            SanitizedTabClosePresentation::new("閉じる", "タブを閉じる", "編集タブを閉じる");
        let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
            .with_close_presentation(presentation);

        let close_presentation = tab
            .close_presentation
            .as_ref()
            .expect("close presentation must be caller supplied");
        assert_eq!(close_presentation.visible_label.as_str(), "閉じる");
        assert_eq!(close_presentation.tooltip.as_str(), "タブを閉じる");
        assert_eq!(
            close_presentation.accessibility_label.as_str(),
            "編集タブを閉じる"
        );
    }

    #[test]
    fn construction_can_represent_sibling_groups_and_tabs() {
        let projection = SanitizedTabProjection::new([
            SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([2]), 2, "右")
                .tab(SanitizedTab::new(
                    SanitizedTabTarget::from_opaque_bytes([2]),
                    2,
                    "B",
                ))
                .tab(SanitizedTab::new(
                    SanitizedTabTarget::from_opaque_bytes([3]),
                    3,
                    "C",
                )),
            SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([1]), 1, "左").tab(
                SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 1, "A"),
            ),
        ]);

        assert_eq!(projection.groups.len(), 2);
        assert_eq!(projection.groups[0].order, 2);
        assert_eq!(projection.groups[0].tabs.len(), 2);
        assert_eq!(projection.groups[1].order, 1);
        assert_eq!(projection.groups[1].tabs[0].label, "A");
    }

    #[test]
    fn opaque_debug_does_not_reveal_target_bytes() {
        let target = SanitizedTabTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(format!("{target:?}"), "SanitizedTabTarget(..)");
    }

    #[test]
    fn group_fingerprint_tracks_target_and_capabilities() {
        let base = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([1]),
            0,
            "root",
        )
        .with_capabilities(SanitizedTabGroupCapabilities::new())
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([11]),
            1,
            "Tab",
        ))]);
        let target_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([2]),
            0,
            "root",
        )
        .with_capabilities(SanitizedTabGroupCapabilities::new())
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([11]),
            1,
            "Tab",
        ))]);
        let capability_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([1]),
            0,
            "root",
        )
        .with_capabilities(
            SanitizedTabGroupCapabilities::new()
                .collapse_state(true)
                .menu_state(true),
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([11]),
            1,
            "Tab",
        ))]);

        assert_ne!(
            base.stable_fingerprint(),
            target_changed.stable_fingerprint()
        );
        assert_ne!(
            base.stable_fingerprint(),
            capability_changed.stable_fingerprint()
        );
    }

    #[test]
    fn nested_group_targets_remain_distinct_without_public_readback() {
        let first = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "root",
        )
        .group(SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([1]),
            0,
            "nested",
        ))]);
        let second = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "root",
        )
        .group(SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([2]),
            0,
            "nested",
        ))]);

        assert_ne!(first.stable_fingerprint(), second.stable_fingerprint());
    }

    #[test]
    fn close_presentation_fingerprint_distinguishes_absent_and_different_values() {
        let base = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "タブ",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([1]),
            0,
            "編集",
        ))]);
        let present = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "タブ",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "閉じる",
                    "タブを閉じる",
                    "編集タブを閉じる",
                )),
        )]);
        let visible_label_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "タブ",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "閉じる別表記",
                    "タブを閉じる",
                    "編集タブを閉じる",
                )),
        )]);
        let tooltip_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "タブ",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "閉じる",
                    "別のツールチップ",
                    "編集タブを閉じる",
                )),
        )]);
        let accessibility_label_changed = SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "タブ",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "編集")
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "閉じる",
                    "タブを閉じる",
                    "別のアクセシビリティラベル",
                )),
        )]);

        let base_fingerprint = base.stable_fingerprint();
        assert_ne!(base_fingerprint, present.stable_fingerprint());
        assert_ne!(
            present.stable_fingerprint(),
            visible_label_changed.stable_fingerprint()
        );
        assert_ne!(
            present.stable_fingerprint(),
            tooltip_changed.stable_fingerprint()
        );
        assert_ne!(
            present.stable_fingerprint(),
            accessibility_label_changed.stable_fingerprint()
        );
    }

    #[test]
    fn opaque_group_target_debug_does_not_reveal_payload() {
        let target = SanitizedTabGroupTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(format!("{target:?}"), "SanitizedTabGroupTarget(..)");
    }

    #[test]
    fn close_presentation_debug_does_not_reveal_localized_labels() {
        let presentation = SanitizedTabClosePresentation::new(
            "可視ラベルの秘密",
            "ツールチップの秘密",
            "アクセシビリティの秘密",
        );
        let debug = format!("{presentation:?}");

        assert_eq!(debug, "SanitizedTabClosePresentation(..)");
        assert!(!debug.contains("可視ラベルの秘密"));
        assert!(!debug.contains("ツールチップの秘密"));
        assert!(!debug.contains("アクセシビリティの秘密"));
    }

    #[test]
    fn public_api_is_constructor_only_and_stays_generic() {
        let source = include_str!("sanitized_tab_projection.rs_body.inc");
        let api_source = source.split("#[cfg(test)]").next().unwrap_or(source);
        let lower = api_source.to_ascii_lowercase();
        for forbidden in [
            "document",
            "path",
            "markdown",
            "katana_language",
            "katana::",
            "kle",
            "geometry",
            "coordinate",
            "serialize",
            "clone",
            "payload",
            "pub fn stable_fingerprint",
            "pub fn target",
            "pub fn groups",
            "pub fn tabs",
            "pub fn capabilities",
            "pub fn visible_label",
            "pub fn tooltip",
            "pub fn accessibility_label",
        ] {
            assert!(
                !lower.contains(forbidden),
                "forbidden tab projection API term: {forbidden}"
            );
        }

        let public_methods = api_source.lines().filter_map(|line| {
            let declaration = line.trim_start();
            (declaration.starts_with("pub fn ") || declaration.starts_with("pub const fn "))
                .then_some(declaration)
        });
        let allowed = [
            "from_opaque_bytes",
            "new",
            "active_state",
            "dirty_state",
            "pinned_state",
            "close_state",
            "collapse_state",
            "menu_state",
            "rename_state",
            "recolor_state",
            "ungroup_state",
            "drag_state",
            "with_icon",
            "with_capabilities",
            "with_close_presentation",
            "tab",
            "group",
        ];
        for declaration in public_methods {
            assert!(
                allowed
                    .iter()
                    .any(|name| declaration.contains(&format!("fn {name}"))),
                "unexpected public tab projection method: {declaration}"
            );
        }

        assert!(api_source.contains("visible_label: impl Into<String>"));
        assert!(api_source.contains("tooltip: impl Into<String>"));
        assert!(api_source.contains("accessibility_label: impl Into<String>"));
        assert!(!api_source.contains("visible: bool"));
        assert!(!api_source.contains("derive(Debug"));
    }
}
