use super::{SettingsDirtyVisualization, SettingsListDensity, SettingsListEvent, SettingsSection};
use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsList {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) sections: Vec<SettingsSection>,
    pub(super) query: Option<String>,
    pub(super) density: SettingsListDensity,
    pub(super) dirty_visualization: SettingsDirtyVisualization,
    pub(super) collapsed_section_ids: BTreeSet<String>,
    pub(super) dirty_field_ids: BTreeSet<String>,
    pub(super) focused_field_id: Option<String>,
    pub(super) callback_log: Vec<SettingsListEvent>,
    pub(super) last_event: Option<SettingsListEvent>,
}
