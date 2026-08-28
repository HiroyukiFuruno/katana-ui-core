use katana_ui_core::render_model::{UiIconProps, UiSvgPaintPolicy};
use sha2::{Digest, Sha256};

const STROKE_AND_FILL_DISCRIMINANT: u8 = 3;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum SanitizedContextMenuCapabilityRejection {
    Missing,
    AlreadyConsumed,
    Reentrant,
    CallbackRejected,
}

pub(super) type ContextMenuCapability =
    Rc<RefCell<Option<Box<dyn FnOnce() -> Result<(), SanitizedContextMenuCapabilityRejection>>>>>;

/// Opaque target supplied by the host for a context-menu item.
pub struct SanitizedContextMenuTarget {
    opaque: Box<[u8]>,
    capability: Option<ContextMenuCapability>,
}

impl SanitizedContextMenuTarget {
    /// Creates a target without assigning meaning to the supplied data.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
            capability: None,
        }
    }

    #[must_use]
    pub fn with_unit_capability<F, E>(mut self, callback: F) -> Self
    where
        F: FnOnce() -> Result<(), E> + 'static,
        E: 'static,
    {
        self.capability = Some(Rc::new(RefCell::new(Some(Box::new(move || {
            callback().map_err(|_| SanitizedContextMenuCapabilityRejection::CallbackRejected)
        })))));
        self
    }

    #[must_use]
    pub(crate) const fn opaque(&self) -> &[u8] {
        &self.opaque
    }

    pub(super) fn capability(&self) -> Option<ContextMenuCapability> {
        self.capability.as_ref().map(std::borrow::ToOwned::to_owned)
    }
}

impl std::fmt::Debug for SanitizedContextMenuTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque().len();
        formatter.write_str("SanitizedContextMenuTarget(..)")
    }
}

/// Generic context-menu projection retained by a future KUC root.
#[derive(Default)]
pub struct SanitizedContextMenuProjection {
    items: Vec<SanitizedContextMenuItem>,
}

impl std::fmt::Debug for SanitizedContextMenuProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedContextMenuProjection")
            .field("item_count", &self.items.len())
            .field("fingerprint", &self.stable_fingerprint())
            .finish()
    }
}

impl SanitizedContextMenuProjection {
    #[must_use]
    pub fn new(items: impl Into<Vec<SanitizedContextMenuItem>>) -> Self {
        Self {
            items: items.into(),
        }
    }

    #[must_use]
    pub(crate) fn items(&self) -> &[SanitizedContextMenuItem] {
        &self.items
    }

    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self.stable_fingerprint() == other.stable_fingerprint()
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update((self.items.len() as u64).to_le_bytes());
        for item in &self.items {
            item.update_fingerprint(&mut digest);
        }
        hex::encode(digest.finalize())
    }
}

/// Consuming builder for a generic context-menu projection.
#[derive(Debug, Default)]
pub struct SanitizedContextMenuProjectionBuilder {
    items: Vec<SanitizedContextMenuItem>,
}

impl SanitizedContextMenuProjectionBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn item(mut self, item: SanitizedContextMenuItem) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn build(self) -> SanitizedContextMenuProjection {
        SanitizedContextMenuProjection::new(self.items)
    }
}

/// One generic context-menu item, optionally containing a submenu.
pub struct SanitizedContextMenuItem {
    target: SanitizedContextMenuTarget,
    order: u32,
    label: String,
    accessibility_label: Option<String>,
    icon: Option<UiIconProps>,
    enabled: bool,
    checked: bool,
    submenu: Vec<Self>,
}

impl std::fmt::Debug for SanitizedContextMenuItem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedContextMenuItem")
            .field("target", &"<opaque>")
            .field("order", &self.order())
            .field("icon", &self.icon().is_some())
            .field("enabled", &self.enabled())
            .field("checked", &self.checked())
            .field("submenu_count", &self.submenu.len())
            .finish()
    }
}

impl SanitizedContextMenuItem {
    #[must_use]
    pub fn new(target: SanitizedContextMenuTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            accessibility_label: None,
            icon: None,
            enabled: true,
            checked: false,
            submenu: Vec::new(),
        }
    }

    #[must_use]
    pub fn accessibility_label_text(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub const fn enabled_state(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    #[must_use]
    pub const fn checked_state(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }

    #[must_use]
    pub fn submenu_item(mut self, item: Self) -> Self {
        self.submenu.push(item);
        self
    }

    #[must_use]
    pub(crate) const fn target(&self) -> &SanitizedContextMenuTarget {
        &self.target
    }

    #[must_use]
    pub(crate) const fn order(&self) -> u32 {
        self.order
    }

    #[must_use]
    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub(crate) fn accessibility_label(&self) -> Option<&str> {
        self.accessibility_label.as_deref()
    }

    #[must_use]
    pub(crate) const fn icon(&self) -> Option<&UiIconProps> {
        self.icon.as_ref()
    }

    #[must_use]
    pub(crate) const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub(crate) const fn checked(&self) -> bool {
        self.checked
    }

    #[must_use]
    pub(crate) fn submenu(&self) -> &[Self] {
        &self.submenu
    }

    fn update_fingerprint(&self, digest: &mut Sha256) {
        digest.update((self.target.opaque.len() as u64).to_le_bytes());
        digest.update(&self.target.opaque);
        digest.update(self.order.to_le_bytes());
        update_string(digest, &self.label);
        match &self.accessibility_label {
            Some(label) => {
                digest.update([1]);
                update_string(digest, label);
            }
            None => digest.update([0]),
        }
        match &self.icon {
            Some(icon) => {
                digest.update([1]);
                update_icon(digest, icon);
            }
            None => digest.update([0]),
        }
        digest.update([self.enabled as u8, self.checked as u8]);
        digest.update((self.submenu.len() as u64).to_le_bytes());
        for item in &self.submenu {
            item.update_fingerprint(digest);
        }
    }
}

fn update_string(digest: &mut Sha256, value: &str) {
    SanitizedProjectionHash::hash_len_prefixed_text(digest, value);
}

fn update_icon(digest: &mut Sha256, icon: &UiIconProps) {
    update_string(digest, &icon.svg_source);
    update_string(digest, &icon.view_box);
    update_string(digest, &icon.path_summary);
    digest.update([match icon.paint_policy {
        UiSvgPaintPolicy::CurrentColor => 0,
        UiSvgPaintPolicy::StrokeOnly => 1,
        UiSvgPaintPolicy::FillOnly => 2,
        UiSvgPaintPolicy::StrokeAndFill => STROKE_AND_FILL_DISCRIMINANT,
    }]);
    update_string(digest, &icon.role);
    update_string(digest, &icon.color_token);
    update_string(digest, &icon.theme_token);
}

#[cfg(test)]
mod tests {
    use super::{
        SanitizedContextMenuItem, SanitizedContextMenuProjectionBuilder, SanitizedContextMenuTarget,
    };
    use katana_ui_core::render_model::{UiIconProps, UiSvgPaintPolicy};

    #[test]
    fn consuming_builder_constructs_a_localized_checked_submenu_tree() {
        let child = SanitizedContextMenuItem::new(
            SanitizedContextMenuTarget::from_opaque_bytes([2, 3]),
            2,
            "子項目",
        )
        .accessibility_label_text("子項目を実行")
        .enabled_state(false);
        let item = SanitizedContextMenuItem::new(
            SanitizedContextMenuTarget::from_opaque_bytes([1]),
            1,
            "表示",
        )
        .accessibility_label_text("表示設定")
        .with_icon(UiIconProps::new("<svg/>"))
        .checked_state(true)
        .submenu_item(child);
        let projection = SanitizedContextMenuProjectionBuilder::new()
            .item(item)
            .build();

        let item = &projection.items()[0];
        assert_eq!(item.order(), 1);
        assert_eq!(item.label(), "表示");
        assert_eq!(item.accessibility_label(), Some("表示設定"));
        assert!(item.icon().is_some());
        assert!(item.enabled());
        assert!(item.checked());
        assert_eq!(item.submenu().len(), 1);
        assert!(!item.submenu()[0].enabled());
        assert_eq!(item.target().opaque(), [1]);
    }

    #[test]
    fn opaque_target_debug_does_not_reveal_bytes() {
        let target = SanitizedContextMenuTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(format!("{target:?}"), "SanitizedContextMenuTarget(..)");
    }

    #[test]
    fn debug_and_fingerprint_cover_optional_fields_and_every_icon_policy() {
        let policies = [
            UiSvgPaintPolicy::CurrentColor,
            UiSvgPaintPolicy::StrokeOnly,
            UiSvgPaintPolicy::FillOnly,
            UiSvgPaintPolicy::StrokeAndFill,
        ];
        let mut builder = SanitizedContextMenuProjectionBuilder::new();
        for (index, policy) in policies.into_iter().enumerate() {
            let icon = UiIconProps::new(format!("<svg>{index}</svg>"))
                .view_box("0 0 16 16")
                .path_summary("path")
                .paint_policy(policy)
                .role("menu")
                .color_token("foreground")
                .theme_token("dark");
            builder = builder.item(
                SanitizedContextMenuItem::new(
                    SanitizedContextMenuTarget::from_opaque_bytes([index as u8]),
                    index as u32,
                    format!("item-{index}"),
                )
                .accessibility_label_text(format!("action-{index}"))
                .with_icon(icon),
            );
        }
        let projection = builder.build();
        let same = SanitizedContextMenuProjectionBuilder::new()
            .item(SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([9]),
                9,
                "different",
            ))
            .build();

        assert_eq!(projection.stable_fingerprint().len(), 64);
        assert!(!projection.same_as(&same));
        assert!(format!("{projection:?}").contains("item_count: 4"));
        assert!(format!("{:?}", projection.items()[0]).contains("<opaque>"));
    }

    #[test]
    fn public_surface_has_only_generic_construction_methods() {
        let source = include_str!("sanitized_context_projection.rs_body.inc");
        let api_source = source.split("#[cfg(test)]").next().unwrap_or(source);
        let forbidden = [
            "semantic",
            "command",
            "markdown",
            "kle",
            "geometry",
            "anchor",
            "range",
            "query",
            "content",
            "serialize",
            "clone",
            "payload",
        ];
        let api_source = api_source.to_ascii_lowercase();
        for term in forbidden {
            assert!(
                !api_source.contains(term),
                "forbidden context projection term: {term}"
            );
        }

        let public_methods = api_source.lines().filter_map(|line| {
            let declaration = line.trim_start();
            (declaration.starts_with("pub fn ") || declaration.starts_with("pub const fn "))
                .then_some(declaration)
        });
        let allowed = [
            "from_opaque_bytes",
            "with_unit_capability",
            "new",
            "item",
            "build",
            "accessibility_label_text",
            "with_icon",
            "enabled_state",
            "checked_state",
            "submenu_item",
        ];
        for declaration in public_methods {
            assert!(
                allowed
                    .iter()
                    .any(|name| declaration.contains(&format!("fn {name}"))),
                "unexpected public method: {declaration}"
            );
        }

        assert!(!api_source.contains("pub fn bytes"));
        assert!(!api_source.contains("pub fn opaque"));
        assert!(!api_source.contains("pub fn stable_fingerprint"));
        assert!(!api_source.contains("pub fn target(&"));
        assert!(!api_source.contains("pub fn label(&"));
        assert!(!api_source.contains("pub fn enabled(&"));
        assert!(!api_source.contains("pub fn checked(&"));
        assert!(!api_source.contains("pub fn submenu(&"));
        assert!(!api_source.contains("derive(Debug, Clone"));
        assert!(!api_source.contains("Serialize"));
    }
}
