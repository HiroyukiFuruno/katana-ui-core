use sha2::{Digest, Sha256};

impl std::fmt::Debug for SanitizedTabTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedTabTarget(..)")
    }
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

impl std::fmt::Debug for SanitizedTabClosePresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SanitizedTabClosePresentation(..)")
    }
}

impl std::fmt::Debug for SanitizedTabGroupTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedTabGroupTarget(..)")
    }
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
        format!("{:x}", digest.finalize())
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
