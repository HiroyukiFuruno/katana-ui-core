use serde::{Deserialize, Serialize};

/// Opaque idempotency key supplied by a controlled consumer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextSurfaceFocusRequestToken(String);

impl TextSurfaceFocusRequestToken {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A one-shot request for the adapter-owned native focus target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceFocusRequest {
    pub token: TextSurfaceFocusRequestToken,
    pub focused: bool,
}

impl TextSurfaceFocusRequest {
    #[must_use]
    pub fn new(token: TextSurfaceFocusRequestToken, focused: bool) -> Self {
        Self { token, focused }
    }
}

/// Records that the adapter issued a focus request; it does not assert actual focus state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceFocusRequestAcknowledgement {
    pub token: TextSurfaceFocusRequestToken,
    pub focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceFocusRequestResult {
    Acknowledged(TextSurfaceFocusRequestAcknowledgement),
}
