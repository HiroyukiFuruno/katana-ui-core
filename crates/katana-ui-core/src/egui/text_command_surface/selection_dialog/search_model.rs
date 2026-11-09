use super::model::KucDialogTarget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultOption {
    pub target: KucDialogTarget,
    pub label: String,
    pub enabled: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultRow {
    pub target: KucDialogTarget,
    pub label: String,
    pub auxiliary_label: Option<String>,
    pub snippet: Option<String>,
    pub highlights: Vec<(usize, usize)>,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultMode {
    pub target: KucDialogTarget,
    pub label: String,
    pub query: String,
    pub include: Option<String>,
    pub exclude: Option<String>,
    pub options: Vec<SearchResultOption>,
    pub history: Vec<SearchResultRow>,
    pub results: Vec<SearchResultRow>,
    pub searched: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultProjection {
    pub modes: Vec<SearchResultMode>,
    pub active_mode: KucDialogTarget,
}

/// One host-owned semantic validation state for a search filter field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchFilterValidity {
    Valid,
    Invalid,
}

/// Localized, mode-specific presentation for the retained search inputs and history lane.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchModePresentation {
    pub(crate) query_hint: Option<String>,
    pub(crate) include_hint: Option<String>,
    pub(crate) exclude_hint: Option<String>,
    pub(crate) include_validity: Option<SearchFilterValidity>,
    pub(crate) exclude_validity: Option<SearchFilterValidity>,
    pub(crate) history_heading: Option<String>,
}

impl SearchModePresentation {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn query_hint(mut self, value: impl Into<String>) -> Self {
        self.query_hint = Some(value.into());
        self
    }

    #[must_use]
    pub fn include_hint(mut self, value: impl Into<String>) -> Self {
        self.include_hint = Some(value.into());
        self
    }

    #[must_use]
    pub fn exclude_hint(mut self, value: impl Into<String>) -> Self {
        self.exclude_hint = Some(value.into());
        self
    }

    #[must_use]
    pub fn include_validity(mut self, value: SearchFilterValidity) -> Self {
        self.include_validity = Some(value);
        self
    }

    #[must_use]
    pub fn exclude_validity(mut self, value: SearchFilterValidity) -> Self {
        self.exclude_validity = Some(value);
        self
    }

    #[must_use]
    pub fn history_heading(mut self, value: impl Into<String>) -> Self {
        self.history_heading = Some(value.into());
        self
    }
}
