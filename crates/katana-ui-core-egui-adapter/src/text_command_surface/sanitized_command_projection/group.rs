use super::item::SanitizedCommandItem;
use katana_ui_core::render_model::UiIconProps;

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
    pub(crate) const fn order(&self) -> u32 {
        self.order
    }

    #[must_use]
    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub(crate) fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
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
    pub(crate) const fn visible(&self) -> bool {
        self.visible
    }

    #[must_use]
    pub(crate) fn items(&self) -> &[SanitizedCommandItem] {
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
