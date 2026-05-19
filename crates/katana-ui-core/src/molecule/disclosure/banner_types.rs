use crate::render_model::{UiStateId, UiTone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BannerAction {
    pub id: String,
    pub label: String,
    pub kind: BannerActionKind,
    pub tone: UiTone,
    pub disabled: bool,
    pub destructive: bool,
}

impl BannerAction {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: BannerActionKind) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
            tone: UiTone::Accent,
            disabled: false,
            destructive: false,
        }
    }

    #[must_use]
    pub const fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerSeverity {
    Info,
    Success,
    Warning,
    Danger,
    Neutral,
}

impl BannerSeverity {
    #[must_use]
    pub const fn tone(self) -> UiTone {
        match self {
            Self::Info => UiTone::Accent,
            Self::Success => UiTone::Success,
            Self::Warning => UiTone::Warning,
            Self::Danger => UiTone::Danger,
            Self::Neutral => UiTone::Neutral,
        }
    }

    #[must_use]
    pub fn default_icon(self) -> Option<String> {
        match self {
            Self::Info => Some("info".to_string()),
            Self::Success => Some("check".to_string()),
            Self::Warning => Some("alert-triangle".to_string()),
            Self::Danger => Some("alert-octagon".to_string()),
            Self::Neutral => None,
        }
    }

    #[must_use]
    pub const fn role(self) -> BannerAccessibilityRole {
        match self {
            Self::Warning | Self::Danger => BannerAccessibilityRole::Alert,
            Self::Info | Self::Success | Self::Neutral => BannerAccessibilityRole::Status,
        }
    }

    #[must_use]
    pub const fn live_region(self) -> BannerLiveRegion {
        match self {
            Self::Warning | Self::Danger => BannerLiveRegion::Assertive,
            Self::Info | Self::Success | Self::Neutral => BannerLiveRegion::Polite,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerActionKind {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerCommand {
    PressAction(String),
    Dismiss,
    ToggleDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerEvent {
    BannerActioned {
        id: UiStateId,
        action_id: String,
        kind: BannerActionKind,
    },
    BannerDismissed {
        id: UiStateId,
    },
    BannerDetailsToggled {
        id: UiStateId,
        open: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BannerState {
    pub visible: bool,
    pub details_open: bool,
}

impl Default for BannerState {
    fn default() -> Self {
        Self {
            visible: true,
            details_open: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BannerVisualContract {
    pub icon: Option<String>,
    pub tone: UiTone,
    pub role: BannerAccessibilityRole,
    pub live_region: BannerLiveRegion,
    pub density: BannerDensity,
    pub placement_hint: BannerPlacementHint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerAccessibilityRole {
    Status,
    Alert,
}

impl BannerAccessibilityRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Alert => "alert",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerLiveRegion {
    Polite,
    Assertive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerDensity {
    Compact,
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerPlacementHint {
    Inline,
    Sticky,
}
