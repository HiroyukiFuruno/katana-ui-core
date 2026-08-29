mod actions;
mod events;
mod options;
mod planner;
mod render;
mod state;
mod types;

use crate::interaction::{VirtualRange, VirtualizationConfig};
use crate::molecule::virtualization::MoleculeVirtualization;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

pub use actions::{DiagnosticKeyboardInput, DiagnosticsListAction};
pub use events::{BulkFixSkipReason, DiagnosticsListEvent};
pub use options::{DiagnosticsGroupBy, DiagnosticsListOptions, DiagnosticsSortBy};
pub use planner::{DiagnosticsGroup, DiagnosticsListPlanner, DiagnosticsVisibleSnapshot};
pub use state::DiagnosticsListState;
pub use types::{
    DiagnosticAction, DiagnosticFixPreview, DiagnosticId, DiagnosticItem, DiagnosticLocation,
    DiagnosticScopeInput, DiagnosticScopeKey, DiagnosticSeverity,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsList {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) options: DiagnosticsListOptions,
    pub(super) state: DiagnosticsListState,
    pub(super) items: Vec<DiagnosticItem>,
    pub(super) scopes: Vec<DiagnosticScopeInput>,
    pub(super) empty_slot: Option<UiNode>,
    pub(super) loading_slot: Option<UiNode>,
    pub(super) bulk_preview: Option<UiNode>,
}

/// Immutable generic read-model used by retained adapter implementations.
///
/// It intentionally contains no host path resolution, linter type, URL, or
/// action target. Adapters return only core `DiagnosticsListEvent` values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsListRenderSnapshot {
    pub label: String,
    pub state_id: UiStateId,
    pub options: DiagnosticsListOptions,
    pub state: DiagnosticsListState,
    pub items: Vec<DiagnosticItem>,
    pub scopes: Vec<DiagnosticScopeInput>,
    pub visible: DiagnosticsVisibleSnapshot,
}

impl DiagnosticsList {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::DiagnosticsList),
            options: DiagnosticsListOptions::default(),
            state: DiagnosticsListState::default(),
            items: Vec::new(),
            scopes: Vec::new(),
            empty_slot: None,
            loading_slot: None,
            bulk_preview: None,
        }
    }

    #[must_use]
    pub fn option(mut self, value: DiagnosticsListOptions) -> Self {
        self.options = value;
        self
    }

    #[must_use]
    pub fn item(mut self, value: DiagnosticItem) -> Self {
        self.items.push(value);
        self
    }

    #[must_use]
    pub fn scope(
        mut self,
        key: impl Into<String>,
        label: impl Into<String>,
        accessible_label: impl Into<String>,
    ) -> Self {
        self.scopes
            .push(DiagnosticScopeInput::new(key, label, accessible_label));
        self.state.reconcile_scope_selection(&self.scopes);
        self
    }

    pub fn set_scopes(&mut self, values: impl IntoIterator<Item = (String, String, String)>) {
        let values = values
            .into_iter()
            .map(|(key, label, accessible_label)| {
                DiagnosticScopeInput::new(key, label, accessible_label)
            })
            .collect();
        self.scopes = values;
        self.state.reconcile_scope_selection(&self.scopes);
    }

    #[must_use]
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.options.virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        let visible = DiagnosticsListPlanner::visible_items_for_scope(
            &self.items,
            &self.options,
            self.state.selected_scope_key.as_ref(),
        );
        MoleculeVirtualization::range(&self.options.virtualization, visible.len())
    }

    #[must_use]
    pub fn empty_slot(mut self, value: impl Into<UiNode>) -> Self {
        self.empty_slot = Some(value.into());
        self
    }

    #[must_use]
    pub fn loading_slot(mut self, value: impl Into<UiNode>) -> Self {
        self.loading_slot = Some(value.into());
        self
    }

    #[must_use]
    pub fn bulk_preview(mut self, value: impl Into<UiNode>) -> Self {
        self.bulk_preview = Some(value.into());
        self
    }

    #[must_use]
    pub fn loading(mut self, value: bool) -> Self {
        self.state.loading = value;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn render_snapshot(&self) -> DiagnosticsListRenderSnapshot {
        DiagnosticsListRenderSnapshot {
            label: self.label.clone(),
            state_id: self.state_id.clone(),
            options: self.options.clone(),
            state: self.state.clone(),
            items: self.items.clone(),
            scopes: self.scopes.clone(),
            visible: DiagnosticsListPlanner::snapshot_for_scope(
                &self.items,
                &self.options,
                self.state.selected_scope_key.as_ref(),
            ),
        }
    }

    pub fn apply_action(&mut self, action: DiagnosticsListAction) -> Vec<DiagnosticsListEvent> {
        self.state
            .apply_action(action, &self.items, &self.scopes, &self.options)
    }
}
