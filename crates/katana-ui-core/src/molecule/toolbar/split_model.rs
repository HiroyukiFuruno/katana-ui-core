use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitActionPart {
    disabled: bool,
    tooltip: Option<String>,
    accessibility_label: Option<String>,
}

impl SplitActionPart {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            disabled: false,
            tooltip: None,
            accessibility_label: None,
        }
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }

    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }
}

impl Default for SplitActionPart {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitAction {
    primary: SplitActionPart,
    secondary: SplitActionPart,
}

impl SplitAction {
    #[must_use]
    pub const fn new(primary: SplitActionPart, secondary: SplitActionPart) -> Self {
        Self { primary, secondary }
    }

    #[must_use]
    pub const fn primary(&self) -> &SplitActionPart {
        &self.primary
    }

    #[must_use]
    pub const fn secondary(&self) -> &SplitActionPart {
        &self.secondary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarSplitState {
    primary_disabled: bool,
    secondary_disabled: bool,
}

impl ToolbarSplitState {
    #[must_use]
    pub const fn new(primary_disabled: bool, secondary_disabled: bool) -> Self {
        Self {
            primary_disabled,
            secondary_disabled,
        }
    }

    #[must_use]
    pub const fn primary_disabled(self) -> bool {
        self.primary_disabled
    }

    #[must_use]
    pub const fn secondary_disabled(self) -> bool {
        self.secondary_disabled
    }
}
