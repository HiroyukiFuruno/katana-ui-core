use super::{SettingsControl, SettingsValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsField {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub control: SettingsControl,
    pub reset_to_default: Option<SettingsValue>,
}

impl SettingsField {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, control: SettingsControl) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            control,
            reset_to_default: None,
        }
    }

    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    #[must_use]
    pub fn reset_to_default(mut self, value: SettingsValue) -> Self {
        self.reset_to_default = Some(value);
        self
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.reset_to_default
            .as_ref()
            .is_some_and(|value| *value != self.control.value())
    }

    #[must_use]
    pub fn dirty_id(&self) -> Option<String> {
        self.is_dirty().then(|| self.id.clone())
    }

    #[must_use]
    pub fn matches_query(&self, query: Option<&str>) -> bool {
        query.is_none_or(|it| {
            let needle = it.to_lowercase();
            self.label.to_lowercase().contains(&needle)
                || self
                    .description
                    .as_ref()
                    .is_some_and(|description| description.to_lowercase().contains(&needle))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsSection {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub fields: Vec<SettingsField>,
    pub collapsible: bool,
    pub default_collapsed: bool,
}

impl SettingsSection {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            fields: Vec::new(),
            collapsible: false,
            default_collapsed: false,
        }
    }

    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    #[must_use]
    pub fn field(mut self, value: SettingsField) -> Self {
        self.fields.push(value);
        self
    }

    #[must_use]
    pub const fn collapsible(mut self, value: bool) -> Self {
        self.collapsible = value;
        self
    }

    #[must_use]
    pub const fn default_collapsed(mut self, value: bool) -> Self {
        self.default_collapsed = value;
        self
    }
}
