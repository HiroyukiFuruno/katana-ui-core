mod actions;
mod events;
mod options;
mod planner;
mod render;
mod state;
mod types;

use crate::interaction::{VirtualRange, VirtualizationConfig};
use crate::molecule::virtualization;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

pub use actions::{DiagnosticKeyboardInput, DiagnosticsListAction};
pub use events::{BulkFixSkipReason, DiagnosticsListEvent};
pub use options::{DiagnosticsGroupBy, DiagnosticsListOptions, DiagnosticsSortBy};
pub use planner::{DiagnosticsGroup, DiagnosticsListPlanner, DiagnosticsVisibleSnapshot};
pub use state::DiagnosticsListState;
pub use types::{
    DiagnosticAction, DiagnosticFixPreview, DiagnosticId, DiagnosticItem, DiagnosticLocation,
    DiagnosticSeverity,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsList {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) options: DiagnosticsListOptions,
    pub(super) state: DiagnosticsListState,
    pub(super) items: Vec<DiagnosticItem>,
    pub(super) empty_slot: Option<UiNode>,
    pub(super) loading_slot: Option<UiNode>,
    pub(super) bulk_preview: Option<UiNode>,
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
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.options.virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        let visible = DiagnosticsListPlanner::visible_items(&self.items, &self.options);
        virtualization::range(&self.options.virtualization, visible.len())
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

    pub fn apply_action(&mut self, action: DiagnosticsListAction) -> Vec<DiagnosticsListEvent> {
        self.state.apply_action(action, &self.items, &self.options)
    }
}
