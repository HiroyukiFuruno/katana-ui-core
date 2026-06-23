use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartupState {
    Idle,
    Loading {
        progress: Option<u8>,
        label: Option<String>,
    },
    Error {
        message: String,
        retry: bool,
        cancel: bool,
    },
}

impl StartupState {
    #[must_use]
    pub fn loading(progress: Option<u8>, label: Option<impl Into<String>>) -> Self {
        Self::Loading {
            progress,
            label: label.map(Into::into),
        }
    }

    #[must_use]
    pub fn error(message: impl Into<String>, retry: bool, cancel: bool) -> Self {
        Self::Error {
            message: message.into(),
            retry,
            cancel,
        }
    }

    #[must_use]
    pub fn accessibility_role(&self) -> &'static str {
        match self {
            Self::Error { .. } => "alert",
            Self::Idle | Self::Loading { .. } => "status",
        }
    }

    #[must_use]
    pub fn progress_percent(&self) -> Option<u8> {
        match self {
            Self::Loading { progress, .. } => progress.map(|value| value.min(Self::MAX_PROGRESS)),
            Self::Idle | Self::Error { .. } => None,
        }
    }

    const MAX_PROGRESS: u8 = 100;
}
