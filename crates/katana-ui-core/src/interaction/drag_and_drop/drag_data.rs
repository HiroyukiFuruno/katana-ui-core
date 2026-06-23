use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const OS_TAG_PREFIX: &str = "os/";
pub const KUC_TAG_PREFIX: &str = "katana-ui-core/";
pub const CONSUMER_TAG_PREFIX: &str = "consumer/";
pub const OS_FILE_LIST_TAG: &str = "os/file-list";
pub const OS_URL_TAG: &str = "os/url";
pub const OS_TEXT_TAG: &str = "os/text";

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragMetadata {
    pub entries: BTreeMap<String, String>,
}

impl DragMetadata {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.entries.insert("label".to_string(), value.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.entries.insert("icon".to_string(), value.into());
        self
    }

    #[must_use]
    pub fn count(mut self, value: usize) -> Self {
        self.entries.insert("count".to_string(), value.to_string());
        self
    }

    #[must_use]
    pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries.insert(key.into(), value.into());
        self
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragData {
    pub tag: String,
    pub payload: serde_json::Value,
    pub metadata: DragMetadata,
}

impl DragData {
    #[must_use]
    pub fn new(tag: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            tag: tag.into(),
            payload,
            metadata: DragMetadata::default(),
        }
    }

    #[must_use]
    pub fn metadata(mut self, value: DragMetadata) -> Self {
        self.metadata = value;
        self
    }

    #[must_use]
    pub fn has_reserved_prefix(&self) -> bool {
        self.tag.starts_with(OS_TAG_PREFIX)
            || self.tag.starts_with(KUC_TAG_PREFIX)
            || self.tag.starts_with(CONSUMER_TAG_PREFIX)
    }
}
