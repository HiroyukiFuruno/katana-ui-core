use crate::render_model::{UiIconProps, UiSvgPaintPolicy};
use sha2::{Digest, Sha256};

const CONTEXT_ICON_PAINT_POLICY_STROKE_AND_FILL: u8 = 3;

use super::target::SanitizedContextMenuTarget;

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

    pub(super) fn update_fingerprint(&self, digest: &mut Sha256) {
        digest.update((self.target.opaque().len() as u64).to_le_bytes());
        digest.update(self.target.opaque());
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
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value.as_bytes());
}

fn update_icon(digest: &mut Sha256, icon: &UiIconProps) {
    update_string(digest, &icon.svg_source);
    update_string(digest, &icon.view_box);
    update_string(digest, &icon.path_summary);
    digest.update([match icon.paint_policy {
        UiSvgPaintPolicy::CurrentColor => 0,
        UiSvgPaintPolicy::StrokeOnly => 1,
        UiSvgPaintPolicy::FillOnly => 2,
        UiSvgPaintPolicy::StrokeAndFill => CONTEXT_ICON_PAINT_POLICY_STROKE_AND_FILL,
    }]);
    update_string(digest, &icon.role);
    update_string(digest, &icon.color_token);
    update_string(digest, &icon.theme_token);
}
