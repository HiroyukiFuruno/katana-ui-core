use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId, UiVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsControlKind {
    Toggle,
    Select,
    Input,
    ColorPicker,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsDirtyVisualization {
    Badge,
    Highlight,
    ResetAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsField {
    pub id: String,
    pub label: String,
    pub control: SettingsControlKind,
    pub value: String,
    pub default_value: String,
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsSection {
    pub id: String,
    pub label: String,
    pub fields: Vec<SettingsField>,
    pub collapsed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsListEvent {
    None,
    QueryChanged(String),
    FieldReset(String),
    SectionCollapsed(String, bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsList {
    label: String,
    state_id: UiStateId,
    sections: Vec<SettingsSection>,
    query: String,
    dirty_visualization: SettingsDirtyVisualization,
    last_event: SettingsListEvent,
}

impl SettingsList {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::SettingsList),
            sections: Vec::new(),
            query: String::new(),
            dirty_visualization: SettingsDirtyVisualization::Badge,
            last_event: SettingsListEvent::None,
        }
    }

    #[must_use]
    pub fn section(mut self, section: SettingsSection) -> Self {
        self.sections.push(section);
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
    pub fn visible_fields(&self) -> Vec<&SettingsField> {
        self.sections
            .iter()
            .filter(|section| !section.collapsed)
            .flat_map(|section| section.fields.iter())
            .filter(|field| field.label.contains(&self.query) || field.value.contains(&self.query))
            .collect()
    }

    #[must_use]
    pub fn last_event(&self) -> &SettingsListEvent {
        &self.last_event
    }
}

impl ComponentAction for SettingsList {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetValue { value, .. } => {
                self.query = value.clone();
                self.last_event = SettingsListEvent::QueryChanged(value.clone());
            }
            UiAction::SetSelectedIndex {
                selected_index,
                selected,
                ..
            } => {
                if let Some(section) = self.sections.get_mut(*selected_index) {
                    section.collapsed = *selected;
                    self.last_event =
                        SettingsListEvent::SectionCollapsed(section.id.clone(), *selected);
                }
            }
            UiAction::ClearValue { .. } => {
                if let Some(field) = self
                    .sections
                    .iter_mut()
                    .flat_map(|it| it.fields.iter_mut())
                    .find(|it| it.dirty)
                {
                    field.value = field.default_value.clone();
                    field.dirty = false;
                    self.last_event = SettingsListEvent::FieldReset(field.id.clone());
                }
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<SettingsList> for UiNode {
    fn from(value: SettingsList) -> Self {
        let state = state(&value);
        UiNode::from_state(UiNodeKind::SettingsList, value.label, value.state_id)
            .interaction(state)
            .variant(match value.dirty_visualization {
                SettingsDirtyVisualization::Badge => UiVariant::Plain,
                SettingsDirtyVisualization::Highlight => UiVariant::Filled,
                SettingsDirtyVisualization::ResetAction => UiVariant::Outline,
            })
    }
}

fn state(value: &SettingsList) -> UiInteractionState {
    UiInteractionState {
        value: value.query.clone(),
        item_count: value.visible_fields().len(),
        has_selection: value.sections.iter().any(|section| section.collapsed),
        ..UiInteractionState::default()
    }
}
