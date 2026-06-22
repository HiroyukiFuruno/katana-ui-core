use super::UiNodeKind;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

const INITIAL_ID_SEQUENCE: u64 = 1;
static NODE_SEQUENCE: AtomicU64 = AtomicU64::new(INITIAL_ID_SEQUENCE);
static STATE_SEQUENCE: AtomicU64 = AtomicU64::new(INITIAL_ID_SEQUENCE);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UiNodeId(String);

impl UiNodeId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn next_for(kind: UiNodeKind) -> Self {
        let sequence = NODE_SEQUENCE.fetch_add(INITIAL_ID_SEQUENCE, Ordering::Relaxed);
        Self::new(format!("{kind:?}:{sequence}"))
    }
}

impl From<&str> for UiNodeId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for UiNodeId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UiStateId(String);

impl UiStateId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn next_for(kind: UiNodeKind) -> Self {
        let sequence = STATE_SEQUENCE.fetch_add(INITIAL_ID_SEQUENCE, Ordering::Relaxed);
        Self::new(format!("state:{kind:?}:{sequence}"))
    }
}

impl From<&str> for UiStateId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for UiStateId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
