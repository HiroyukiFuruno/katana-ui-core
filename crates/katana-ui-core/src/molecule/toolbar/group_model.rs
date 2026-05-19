use super::identifiers::ToolbarGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarGroup {
    id: ToolbarGroupId,
    label: Option<String>,
    divider: bool,
}

impl ToolbarGroup {
    #[must_use]
    pub fn new(id: impl Into<ToolbarGroupId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            divider: true,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &ToolbarGroupId {
        &self.id
    }

    #[must_use]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    #[must_use]
    pub const fn label_model(&self) -> Option<&String> {
        self.label.as_ref()
    }

    #[must_use]
    pub fn divider(mut self, value: bool) -> Self {
        self.divider = value;
        self
    }

    #[must_use]
    pub const fn divider_model(&self) -> bool {
        self.divider
    }
}
