use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarAction {
    PressSegment { id: String },
    ActivateSegment { id: String },
    ShowTooltip { id: String },
    ClosePopover { id: String },
    Dismiss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusBarEvent {
    SegmentPressed { id: String },
    SegmentPopoverOpened { id: String },
    SegmentPopoverClosed { id: String },
    SegmentTooltipShown { id: String },
    Dismissed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarState {
    pub(super) open_popover: Option<String>,
}

impl StatusBarState {
    #[must_use]
    pub const fn open_popover(&self) -> Option<&String> {
        self.open_popover.as_ref()
    }
}
