use crate::render_model::UiTone;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarMode {
    #[default]
    SingleMessage,
    MultiSegment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarSegmentAlignment {
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarDensity {
    Compact,
    #[default]
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressMeterShape {
    Linear,
    Ring,
    Pie,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressMeterSpec {
    pub(super) percent: u8,
    pub(super) label: String,
    pub(super) tone: UiTone,
    pub(super) tooltip: String,
    pub(super) shape: ProgressMeterShape,
}

impl ProgressMeterSpec {
    #[must_use]
    pub fn new(shape: ProgressMeterShape, percent: u8) -> Self {
        Self {
            shape,
            percent: percent.min(100),
            label: String::new(),
            tone: UiTone::Neutral,
            tooltip: String::new(),
        }
    }

    #[must_use]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = value.into();
        self
    }

    #[must_use]
    pub fn tone(mut self, value: UiTone) -> Self {
        self.tone = value;
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = value.into();
        self
    }

    #[must_use]
    pub const fn percent(&self) -> u8 {
        self.percent
    }

    #[must_use]
    pub const fn shape(&self) -> ProgressMeterShape {
        self.shape
    }

    #[must_use]
    pub fn label_text(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn tone_value(&self) -> UiTone {
        self.tone
    }

    #[must_use]
    pub fn tooltip_text(&self) -> &str {
        &self.tooltip
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarPopoverSpec {
    title: String,
    body: String,
}

impl StatusBarPopoverSpec {
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarSegment {
    pub(super) id: String,
    pub(super) label: String,
    pub(super) icon: Option<String>,
    pub(super) tone: UiTone,
    pub(super) alignment: StatusBarSegmentAlignment,
    pub(super) tooltip: Option<String>,
    pub(super) interactive: bool,
    pub(super) popover: Option<StatusBarPopoverSpec>,
    pub(super) progress: Option<ProgressMeterSpec>,
    pub(super) accessibility_label: String,
}

impl StatusBarSegment {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            id: id.into(),
            accessibility_label: label.clone(),
            label,
            icon: None,
            tone: UiTone::Neutral,
            alignment: StatusBarSegmentAlignment::Leading,
            tooltip: None,
            interactive: false,
            popover: None,
            progress: None,
        }
    }

    #[must_use]
    pub fn alignment(mut self, value: StatusBarSegmentAlignment) -> Self {
        self.alignment = value;
        self
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    #[must_use]
    pub fn interactive(mut self, value: bool) -> Self {
        self.interactive = value;
        self
    }

    #[must_use]
    pub fn popover(mut self, value: StatusBarPopoverSpec) -> Self {
        self.popover = Some(value);
        self.interactive = true;
        self
    }

    #[must_use]
    pub fn progress(mut self, value: ProgressMeterSpec) -> Self {
        self.progress = Some(value);
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn icon_name(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    #[must_use]
    pub const fn tone_value(&self) -> UiTone {
        self.tone
    }

    #[must_use]
    pub const fn alignment_value(&self) -> StatusBarSegmentAlignment {
        self.alignment
    }

    #[must_use]
    pub fn tooltip_text(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    #[must_use]
    pub const fn is_interactive(&self) -> bool {
        self.interactive
    }

    #[must_use]
    pub const fn popover_spec(&self) -> Option<&StatusBarPopoverSpec> {
        self.popover.as_ref()
    }

    #[must_use]
    pub fn accessibility_label_text(&self) -> &str {
        &self.accessibility_label
    }

    #[must_use]
    pub const fn progress_spec(&self) -> Option<&ProgressMeterSpec> {
        self.progress.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarContractViolation {
    MultiSegmentHasSingleMessage,
    SingleMessageHasSegments,
}
