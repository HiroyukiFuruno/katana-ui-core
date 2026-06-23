use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelMode {
    Expanded,
    IconOnly,
    Collapsed,
    FloatingOverlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelSide {
    Leading,
    Trailing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResizableWidth {
    pub min: u16,
    pub max: u16,
    pub default: u16,
    pub current: u16,
    pub persist_id: Option<String>,
}

impl ResizableWidth {
    #[must_use]
    pub fn new(
        min: u16,
        max: u16,
        default: u16,
        current: u16,
        persist_id: Option<impl Into<String>>,
    ) -> Self {
        let normalized_max = max.max(min);
        Self {
            min,
            max: normalized_max,
            default: default.clamp(min, normalized_max),
            current: current.clamp(min, normalized_max),
            persist_id: persist_id.map(Into::into),
        }
    }

    #[must_use]
    pub fn clamped(&self, value: u16) -> u16 {
        value.clamp(self.min, self.max)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollapsiblePanelOptions {
    pub side: PanelSide,
    pub pinned: bool,
    pub expand_on_hover: bool,
    pub resize_handle: bool,
}

impl Default for CollapsiblePanelOptions {
    fn default() -> Self {
        Self {
            side: PanelSide::Leading,
            pinned: true,
            expand_on_hover: false,
            resize_handle: false,
        }
    }
}
