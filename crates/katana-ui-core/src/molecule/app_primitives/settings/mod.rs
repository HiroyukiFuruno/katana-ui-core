mod component_action;
mod query;
mod render;
mod state;
mod types;

use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub use query::SettingsVisibleSection;
pub use types::{
    SettingsControl, SettingsControlKind, SettingsControlOption, SettingsDirtyVisualization,
    SettingsField, SettingsKeyboardInput, SettingsListAction, SettingsListDensity,
    SettingsListEvent, SettingsSection, SettingsValue,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsList {
    label: String,
    state_id: UiStateId,
    sections: Vec<SettingsSection>,
    query: Option<String>,
    density: SettingsListDensity,
    dirty_visualization: SettingsDirtyVisualization,
    collapsed_section_ids: BTreeSet<String>,
    dirty_field_ids: BTreeSet<String>,
    focused_field_id: Option<String>,
    callback_log: Vec<SettingsListEvent>,
    last_event: Option<SettingsListEvent>,
}

impl SettingsList {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::SettingsList),
            sections: Vec::new(),
            query: None,
            density: SettingsListDensity::Default,
            dirty_visualization: SettingsDirtyVisualization::None,
            collapsed_section_ids: BTreeSet::new(),
            dirty_field_ids: BTreeSet::new(),
            focused_field_id: None,
            callback_log: Vec::new(),
            last_event: None,
        }
    }

    #[must_use]
    pub fn section(mut self, section: SettingsSection) -> Self {
        self.register_section_state(&section);
        self.sections.push(section);
        self
    }

    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    #[must_use]
    pub const fn density(mut self, value: SettingsListDensity) -> Self {
        self.density = value;
        self
    }

    #[must_use]
    pub fn dirty_visualization(mut self, value: SettingsDirtyVisualization) -> Self {
        self.dirty_visualization = value;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn sections(&self) -> &[SettingsSection] {
        &self.sections
    }

    #[must_use]
    pub fn visible_fields(&self) -> Vec<&SettingsField> {
        self.visible_sections()
            .into_iter()
            .flat_map(|section| section.fields)
            .collect()
    }

    #[must_use]
    pub fn visible_sections(&self) -> Vec<SettingsVisibleSection<'_>> {
        self.sections
            .iter()
            .filter_map(|section| query::visible_section(self, section))
            .collect()
    }

    #[must_use]
    pub fn last_event(&self) -> Option<&SettingsListEvent> {
        self.last_event.as_ref()
    }

    #[must_use]
    pub fn collapsed_section_ids(&self) -> &BTreeSet<String> {
        &self.collapsed_section_ids
    }

    #[must_use]
    pub fn dirty_field_ids(&self) -> &BTreeSet<String> {
        &self.dirty_field_ids
    }

    #[must_use]
    pub fn focused_field_id(&self) -> Option<&str> {
        self.focused_field_id.as_deref()
    }

    #[must_use]
    pub fn callback_log(&self) -> &[SettingsListEvent] {
        &self.callback_log
    }

    pub fn apply_settings_action(&mut self, action: SettingsListAction) -> Vec<SettingsListEvent> {
        state::apply(self, action)
    }

    fn register_section_state(&mut self, section: &SettingsSection) {
        if section.default_collapsed {
            self.collapsed_section_ids.insert(section.id.clone());
        }
        self.dirty_field_ids
            .extend(section.fields.iter().filter_map(SettingsField::dirty_id));
    }
}

impl From<SettingsList> for UiNode {
    fn from(value: SettingsList) -> Self {
        render::render(value)
    }
}
