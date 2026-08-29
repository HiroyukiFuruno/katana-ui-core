use katana_ui_core::render_model::UiIconProps;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizedCommandCapabilityRejection {
    Missing,
    AlreadyConsumed,
    Reentrant,
    CallbackRejected,
}

pub(super) type CommandCapability =
    Rc<RefCell<Option<Box<dyn FnOnce() -> Result<(), SanitizedCommandCapabilityRejection>>>>>;

/// Opaque command target supplied by the host.
pub struct SanitizedCommandTarget {
    opaque: Box<[u8]>,
    capability: Option<CommandCapability>,
}

impl SanitizedCommandTarget {
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
            callback().map_err(|_| SanitizedCommandCapabilityRejection::CallbackRejected)
        })))));
        self
    }

    #[must_use]
    pub(super) fn stable_fingerprint(&self) -> String {
        SanitizedProjectionHash::sha256_signature_hex(&self.opaque)
    }

    pub(super) fn capability(&self) -> Option<CommandCapability> {
        self.capability.clone()
    }
}

impl std::fmt::Debug for SanitizedCommandTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.opaque.len();
        formatter.write_str("SanitizedCommandTarget(..)")
    }
}

/// Generic command projection retained by the KUC root.
#[derive(Debug, Default)]
pub struct SanitizedCommandProjection {
    groups: Vec<SanitizedCommandGroup>,
}

impl SanitizedCommandProjection {
    #[must_use]
    pub fn new(groups: impl Into<Vec<SanitizedCommandGroup>>) -> Self {
        Self {
            groups: groups.into(),
        }
    }

    #[must_use]
    pub(super) fn groups(&self) -> &[SanitizedCommandGroup] {
        &self.groups
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update(b"kuc.sanitized-command-projection/v1\0");

        for group in &self.groups {
            digest.update(b"group");
            digest.update(group.order.to_le_bytes());
            hash_text(&mut digest, group.label.as_bytes());
            hash_optional_text(&mut digest, group.tooltip.as_deref());
            hash_optional_text(&mut digest, group.accessibility_label.as_deref());
            hash_optional_icon(&mut digest, group.icon.as_ref());
            digest.update([u8::from(group.enabled)]);
            digest.update([u8::from(group.visible)]);

            for item in &group.items {
                digest.update(b"item");
                digest.update(item.order.to_le_bytes());
                digest.update(item.target.stable_fingerprint().as_bytes());
                hash_text(&mut digest, item.label.as_bytes());
                hash_optional_text(&mut digest, item.tooltip.as_deref());
                hash_optional_text(&mut digest, item.accessibility_label.as_deref());
                hash_optional_icon(&mut digest, item.icon.as_ref());
                digest.update([u8::from(item.enabled)]);
                digest.update([u8::from(item.visible)]);

                for dropdown in &item.dropdown_items {
                    digest.update(b"dropdown");
                    digest.update(dropdown.order.to_le_bytes());
                    digest.update(dropdown.target.stable_fingerprint().as_bytes());
                    hash_text(&mut digest, dropdown.label.as_bytes());
                    hash_optional_text(&mut digest, dropdown.tooltip.as_deref());
                    hash_optional_text(&mut digest, dropdown.accessibility_label.as_deref());
                    hash_optional_icon(&mut digest, dropdown.icon.as_ref());
                    digest.update([u8::from(dropdown.enabled)]);
                    digest.update([u8::from(dropdown.visible)]);
                }
            }
        }

        hex::encode(digest.finalize())
    }
}

/// Generic command group projection.
#[derive(Debug)]
pub struct SanitizedCommandGroup {
    order: u32,
    label: String,
    tooltip: Option<String>,
    accessibility_label: Option<String>,
    icon: Option<UiIconProps>,
    enabled: bool,
    visible: bool,
    items: Vec<SanitizedCommandItem>,
}

fn hash_optional_icon(digest: &mut Sha256, icon: Option<&UiIconProps>) {
    match icon {
        Some(icon) => {
            digest.update(b"icon");
            let rendered = format!("{icon:?}");
            hash_text(digest, rendered.as_bytes());
        }
        None => {
            digest.update(b"icon-none");
        }
    }
}

fn hash_optional_text(digest: &mut Sha256, text: Option<&str>) {
    match text {
        Some(text) => {
            digest.update(b"some");
            hash_text(digest, text.as_bytes());
        }
        None => {
            digest.update(b"none");
        }
    }
}

fn hash_text(digest: &mut Sha256, value: &[u8]) {
    SanitizedProjectionHash::hash_len_prefixed_bytes(digest, value);
}

impl SanitizedCommandGroup {
    #[must_use]
    pub fn new(order: u32, label: impl Into<String>) -> Self {
        Self {
            order,
            label: label.into(),
            tooltip: None,
            accessibility_label: None,
            icon: None,
            enabled: true,
            visible: true,
            items: Vec::new(),
        }
    }

    #[must_use]
    pub(super) const fn order(&self) -> u32 {
        self.order
    }

    #[must_use]
    pub(super) fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub(super) fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    #[must_use]
    pub(super) fn accessibility_label(&self) -> Option<&str> {
        self.accessibility_label.as_deref()
    }

    #[must_use]
    pub(super) const fn icon(&self) -> Option<&UiIconProps> {
        self.icon.as_ref()
    }

    #[must_use]
    pub(super) const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub(super) const fn visible(&self) -> bool {
        self.visible
    }

    #[must_use]
    pub(super) fn items(&self) -> &[SanitizedCommandItem] {
        &self.items
    }

    #[must_use]
    pub fn tooltip_text(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
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
    pub const fn visible_state(mut self, value: bool) -> Self {
        self.visible = value;
        self
    }

    #[must_use]
    pub fn item(mut self, value: SanitizedCommandItem) -> Self {
        self.items.push(value);
        self
    }
}

/// Generic command item projection.
#[derive(Debug)]
pub struct SanitizedCommandItem {
    target: SanitizedCommandTarget,
    order: u32,
    label: String,
    tooltip: Option<String>,
    accessibility_label: Option<String>,
    icon: Option<UiIconProps>,
    enabled: bool,
    visible: bool,
    dropdown_items: Vec<SanitizedCommandDropdownItem>,
}

impl SanitizedCommandItem {
    #[must_use]
    pub fn new(target: SanitizedCommandTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            tooltip: None,
            accessibility_label: None,
            icon: None,
            enabled: true,
            visible: true,
            dropdown_items: Vec::new(),
        }
    }

    #[must_use]
    pub(super) const fn target(&self) -> &SanitizedCommandTarget {
        &self.target
    }

    #[must_use]
    pub(super) const fn order(&self) -> u32 {
        self.order
    }

    #[must_use]
    pub(super) fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub(super) fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    #[must_use]
    pub(super) fn accessibility_label(&self) -> Option<&str> {
        self.accessibility_label.as_deref()
    }

    #[must_use]
    pub(super) const fn icon(&self) -> Option<&UiIconProps> {
        self.icon.as_ref()
    }

    #[must_use]
    pub(super) const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub(super) const fn visible(&self) -> bool {
        self.visible
    }

    #[must_use]
    pub(super) fn dropdown_items(&self) -> &[SanitizedCommandDropdownItem] {
        &self.dropdown_items
    }

    #[must_use]
    pub fn tooltip_text(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
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
    pub const fn visible_state(mut self, value: bool) -> Self {
        self.visible = value;
        self
    }

    #[must_use]
    pub fn dropdown_item(mut self, value: SanitizedCommandDropdownItem) -> Self {
        self.dropdown_items.push(value);
        self
    }
}

/// Generic dropdown child item projection.
#[derive(Debug)]
pub struct SanitizedCommandDropdownItem {
    target: SanitizedCommandTarget,
    order: u32,
    label: String,
    tooltip: Option<String>,
    accessibility_label: Option<String>,
    icon: Option<UiIconProps>,
    enabled: bool,
    visible: bool,
}

impl SanitizedCommandDropdownItem {
    #[must_use]
    pub fn new(target: SanitizedCommandTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            tooltip: None,
            accessibility_label: None,
            icon: None,
            enabled: true,
            visible: true,
        }
    }

    #[must_use]
    pub(super) const fn target(&self) -> &SanitizedCommandTarget {
        &self.target
    }

    #[must_use]
    pub(super) const fn order(&self) -> u32 {
        self.order
    }

    #[must_use]
    pub(super) fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub(super) fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    #[must_use]
    pub(super) fn accessibility_label(&self) -> Option<&str> {
        self.accessibility_label.as_deref()
    }

    #[must_use]
    pub(super) const fn icon(&self) -> Option<&UiIconProps> {
        self.icon.as_ref()
    }

    #[must_use]
    pub(super) const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub(super) const fn visible(&self) -> bool {
        self.visible
    }

    #[must_use]
    pub fn tooltip_text(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
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
    pub const fn visible_state(mut self, value: bool) -> Self {
        self.visible = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
        SanitizedCommandProjection, SanitizedCommandTarget,
    };
    use katana_ui_core::render_model::UiIconProps;

    #[test]
    fn generic_projection_preserves_contract_fields() {
        let child = SanitizedCommandDropdownItem::new(
            SanitizedCommandTarget::from_opaque_bytes([3, 4]),
            7,
            "子項目",
        )
        .tooltip_text("子項目の説明")
        .accessibility_label_text("子項目")
        .with_icon(UiIconProps::new("<svg>child</svg>"))
        .visible_state(false)
        .enabled_state(false);
        let item =
            SanitizedCommandItem::new(SanitizedCommandTarget::from_opaque_bytes([1, 2]), 5, "編集")
                .tooltip_text("編集操作")
                .accessibility_label_text("編集")
                .with_icon(UiIconProps::new("<svg/>"))
                .visible_state(false)
                .dropdown_item(child);
        let projection = SanitizedCommandProjection::new([
            SanitizedCommandGroup::new(2, "操作")
                .tooltip_text("操作一覧")
                .accessibility_label_text("操作")
                .with_icon(UiIconProps::new("<svg>group</svg>"))
                .enabled_state(false)
                .visible_state(false)
                .item(item),
        ]);

        let group = &projection.groups()[0];
        let item = &group.items()[0];
        assert_eq!(group.order(), 2);
        assert_eq!(group.label(), "操作");
        assert_eq!(group.tooltip(), Some("操作一覧"));
        assert_eq!(group.accessibility_label(), Some("操作"));
        assert!(group.icon().is_some());
        assert!(!group.enabled());
        assert!(!group.visible());
        assert_eq!(item.order(), 5);
        assert_eq!(item.label(), "編集");
        assert_eq!(item.tooltip(), Some("編集操作"));
        assert_eq!(item.accessibility_label(), Some("編集"));
        assert!(item.icon().is_some());
        assert!(!item.visible());
        assert_eq!(item.dropdown_items().len(), 1);
        let dropdown = &item.dropdown_items()[0];
        assert_eq!(
            dropdown.target().stable_fingerprint(),
            SanitizedCommandTarget::from_opaque_bytes([3, 4]).stable_fingerprint()
        );
        assert_eq!(dropdown.order(), 7);
        assert_eq!(dropdown.label(), "子項目");
        assert_eq!(dropdown.tooltip(), Some("子項目の説明"));
        assert_eq!(dropdown.accessibility_label(), Some("子項目"));
        assert!(dropdown.icon().is_some());
        assert!(!dropdown.enabled());
        assert!(!dropdown.visible());
        assert_eq!(projection.stable_fingerprint().len(), 64);
    }

    #[test]
    fn opaque_target_debug_does_not_reveal_payload() {
        let target = SanitizedCommandTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(format!("{target:?}"), "SanitizedCommandTarget(..)");
    }

    #[test]
    fn opaque_target_fingerprint_is_stable_for_equal_bytes_and_distinct_for_other_bytes() {
        let first = SanitizedCommandTarget::from_opaque_bytes([1, 2, 3]);
        let same = SanitizedCommandTarget::from_opaque_bytes([1, 2, 3]);
        let different = SanitizedCommandTarget::from_opaque_bytes([1, 2, 4]);

        assert_eq!(first.stable_fingerprint(), same.stable_fingerprint());
        assert_ne!(first.stable_fingerprint(), different.stable_fingerprint());
    }

    #[test]
    fn projection_read_accessors_are_not_public_api() {
        let source = include_str!("sanitized_command_projection_body.inc");
        let api_source = source.split("#[cfg(test)]").next().unwrap_or(source);
        let read_accessors = [
            "groups",
            "target",
            "order",
            "label",
            "tooltip",
            "accessibility_label",
            "icon",
            "enabled",
            "visible",
            "items",
            "dropdown_items",
        ];

        for accessor in read_accessors {
            let mut declarations = api_source.lines().filter(|line| {
                let declaration = line.trim_start();
                declaration.contains(&format!("fn {accessor}(&"))
                    || declaration.contains(&format!("fn {accessor}(&self"))
            });
            assert!(
                declarations.clone().count() > 0,
                "missing accessor declaration for {accessor}"
            );
            assert!(
                declarations.all(|line| {
                    let declaration = line.trim_start();
                    declaration.starts_with("pub(super) ") || declaration.starts_with("pub(crate) ")
                }),
                "read accessor {accessor} must remain KUC-internal"
            );
        }

        let public_methods = api_source.lines().filter_map(|line| {
            let declaration = line.trim_start();
            (declaration.starts_with("pub fn ") || declaration.starts_with("pub const fn "))
                .then_some(declaration)
        });
        let allowed_public_methods = [
            "from_opaque_bytes",
            "with_unit_capability",
            "new",
            "tooltip_text",
            "accessibility_label_text",
            "with_icon",
            "enabled_state",
            "visible_state",
            "item",
            "dropdown_item",
        ];
        for declaration in public_methods {
            assert!(
                allowed_public_methods
                    .iter()
                    .any(|name| declaration.contains(&format!("fn {name}"))),
                "unexpected public method: {declaration}"
            );
        }

        assert!(!api_source.contains(&["derive(", "Clone"].concat()));
        assert!(!api_source.contains(&["derive(", "Serial", "ize"].concat()));
        assert!(!api_source.contains(&["pub fn ", "bytes("].concat()));
        assert!(!api_source.contains(&["pub fn ", "opaque_bytes("].concat()));
    }
}
