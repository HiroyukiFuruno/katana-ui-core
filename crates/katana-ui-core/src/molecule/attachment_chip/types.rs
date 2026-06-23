use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

const FULL_PROGRESS_BASIS_POINTS: u16 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentKind {
    File,
    Image,
    Url,
    Paste,
    Resource,
}

impl AttachmentKind {
    #[must_use]
    pub const fn default_icon(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Image => "image",
            Self::Url => "link",
            Self::Paste => "clipboard",
            Self::Resource => "resource",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentMeta {
    size_label: String,
    mime_label: String,
    detail_label: String,
}

impl AttachmentMeta {
    #[must_use]
    pub fn new(
        size_label: impl Into<String>,
        mime_label: impl Into<String>,
        detail_label: impl Into<String>,
    ) -> Self {
        Self {
            size_label: size_label.into(),
            mime_label: mime_label.into(),
            detail_label: detail_label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentThumbnail {
    source: String,
    aspect_width: u16,
    aspect_height: u16,
}

impl AttachmentThumbnail {
    #[must_use]
    pub fn new(source: impl Into<String>, aspect_width: u16, aspect_height: u16) -> Self {
        Self {
            source: source.into(),
            aspect_width,
            aspect_height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentProgress {
    basis_points: u16,
}

impl AttachmentProgress {
    #[must_use]
    pub const fn from_basis_points(value: u16) -> Self {
        Self {
            basis_points: if value > FULL_PROGRESS_BASIS_POINTS {
                FULL_PROGRESS_BASIS_POINTS
            } else {
                value
            },
        }
    }

    #[must_use]
    pub const fn percent(self) -> u8 {
        (self.basis_points / 100) as u8
    }

    #[must_use]
    pub const fn basis_points(self) -> u16 {
        self.basis_points
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentStatus {
    Pending,
    Uploading,
    Ready,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentChipAction {
    OpenPreview,
    Dismiss,
    Retry,
    TransitionStatus(AttachmentStatus),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentChipEvent {
    Opened {
        id: UiStateId,
    },
    Dismissed {
        id: UiStateId,
    },
    Retry {
        id: UiStateId,
    },
    StatusChanged {
        id: UiStateId,
        previous: AttachmentStatus,
        current: AttachmentStatus,
    },
}
