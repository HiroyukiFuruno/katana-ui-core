mod activation;
mod component_action;
mod hit_test;
mod host_action;
mod layout_metrics;
mod list_types;
mod query;
mod render;
mod state;
mod types;

use crate::render_model::{UiCursor, UiNode, UiNodeId, UiNodeKind, UiStateId};
use std::collections::BTreeSet;

pub use hit_test::{
    SettingsListHitRect, SettingsListHitTarget, SettingsListHitTestInput,
    SettingsListHitTestResult, SettingsListInteraction,
};
pub use layout_metrics::SettingsListLayoutMetrics;
pub use list_types::SettingsList;
pub use query::SettingsVisibleSection;
pub use types::{
    SettingsControl, SettingsControlKind, SettingsControlOption, SettingsDirtyVisualization,
    SettingsField, SettingsKeyboardInput, SettingsListAction, SettingsListDensity,
    SettingsListEvent, SettingsSection, SettingsValue,
};

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

    pub fn apply_field_update(
        &mut self,
        field_id: impl Into<String>,
        value: SettingsValue,
    ) -> Vec<SettingsListEvent> {
        self.apply_settings_action(SettingsListAction::UpdateField {
            field_id: field_id.into(),
            value,
        })
    }

    #[must_use]
    pub fn activation_action_for_field(&self, field_id: &str) -> Option<SettingsListAction> {
        activation::field_action(self, field_id)
    }

    #[must_use]
    pub fn action_from_host_plan(
        &self,
        plan: &crate::render_model::UiHostActionPlan,
    ) -> Option<SettingsListAction> {
        host_action::action_from_host_plan(self, plan)
    }

    #[must_use]
    pub fn hit_test(&self, input: SettingsListHitTestInput) -> SettingsListHitTestResult {
        hit_test::hit_test(self, input)
    }

    #[must_use]
    pub fn action_for_hit(&self, input: SettingsListHitTestInput) -> Option<SettingsListAction> {
        hit_test::action_for_hit(self, input)
    }

    #[must_use]
    pub fn cursor_for_hit(&self, input: SettingsListHitTestInput) -> UiCursor {
        hit_test::cursor_for_hit(self, input)
    }

    #[must_use]
    pub fn hit_targets(&self, viewport_width: u32) -> Vec<SettingsListHitTarget> {
        hit_test::hit_targets(self, viewport_width)
    }

    #[must_use]
    pub fn hit_target_for_field(
        &self,
        field_id: &str,
        viewport_width: u32,
    ) -> Option<SettingsListHitTarget> {
        hit_test::hit_target_for_field(self, field_id, viewport_width)
    }

    #[must_use]
    pub fn hit_target_for_section(
        &self,
        section_id: &str,
        viewport_width: u32,
    ) -> Option<SettingsListHitTarget> {
        hit_test::hit_target_for_section(self, section_id, viewport_width)
    }

    #[must_use]
    pub fn hit_target_for_hit(
        &self,
        input: SettingsListHitTestInput,
        viewport_width: u32,
    ) -> Option<SettingsListHitTarget> {
        hit_test::hit_target_for_hit(self, input, viewport_width)
    }

    #[must_use]
    pub fn interaction_for_hit(
        &self,
        input: SettingsListHitTestInput,
        viewport_width: u32,
    ) -> SettingsListInteraction {
        hit_test::interaction_for_hit(self, input, viewport_width)
    }

    #[must_use]
    pub fn content_height(&self) -> u32 {
        hit_test::content_height(self)
    }

    #[must_use]
    pub const fn layout_metrics(&self) -> SettingsListLayoutMetrics {
        SettingsListLayoutMetrics::DEFAULT
    }

    #[must_use]
    pub fn field_node_id(field_id: &str) -> UiNodeId {
        UiNodeId::new(Self::field_interaction_id(field_id))
    }

    #[must_use]
    pub fn control_node_id(field_id: &str) -> UiNodeId {
        UiNodeId::new(Self::control_interaction_id(field_id))
    }

    #[must_use]
    pub fn section_node_id(section_id: &str) -> UiNodeId {
        UiNodeId::new(Self::section_interaction_id(section_id))
    }

    pub(super) fn field_interaction_id(field_id: &str) -> String {
        format!("settings-field:{field_id}")
    }

    pub(super) fn control_interaction_id(field_id: &str) -> String {
        format!("settings-control:{field_id}")
    }

    pub(super) fn section_interaction_id(section_id: &str) -> String {
        format!("settings-section:{section_id}")
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

#[cfg(test)]
#[path = "action_contract_tests.rs"]
mod action_contract_tests;
#[cfg(test)]
#[path = "hit_contract_tests.rs"]
mod hit_contract_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "target_contract_tests.rs"]
mod target_contract_tests;
