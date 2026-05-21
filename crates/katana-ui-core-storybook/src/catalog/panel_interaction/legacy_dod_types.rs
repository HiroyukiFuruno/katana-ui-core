use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsMutationReport {
    pub page: String,
    pub ui_marker: String,
    pub action: String,
    pub event: String,
    pub target_state_id: String,
    pub option: TypedOptionMutationReport,
    pub state: BeforeAfterReport,
    pub preview: BeforeAfterReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedOptionMutationReport {
    pub name: String,
    pub value_type: String,
    pub before_value: String,
    pub after_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeforeAfterReport {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyUiMarkerReport {
    pub page: String,
    pub ui_marker: String,
    pub root_kind: String,
    pub state_id: String,
    pub preview_marker: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresetDifferenceReport {
    pub page: String,
    pub ui_marker: String,
    pub default_marker: String,
    pub interactive_marker: String,
    pub edge_marker: String,
    pub theme_marker: String,
}
