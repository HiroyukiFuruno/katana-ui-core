mod actions;
mod render;
mod types;

pub use actions::{
    SearchControlStripAction, SearchControlStripEvent, SearchNavigationDirection,
    SearchReplaceScope,
};
pub use types::{ReplaceMode, SearchOptionKind, SearchOptions};

use crate::render_model::{UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchControlStrip {
    label: String,
    state_id: UiStateId,
    query: String,
    options: SearchOptions,
    result_count: Option<usize>,
    active_index: Option<usize>,
    replace_mode: ReplaceMode,
    replace_value: String,
}

impl SearchControlStrip {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::SearchControlStrip),
            query: String::new(),
            options: SearchOptions::default(),
            result_count: None,
            active_index: None,
            replace_mode: ReplaceMode::Hidden,
            replace_value: String::new(),
        }
    }

    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = value.into();
        self
    }

    #[must_use]
    pub fn options(mut self, value: SearchOptions) -> Self {
        self.options = value;
        self
    }

    #[must_use]
    pub fn result_position(mut self, count: usize, active_index: Option<usize>) -> Self {
        self.result_count = Some(count);
        self.active_index = active_index;
        self
    }

    #[must_use]
    pub fn replace_mode(mut self, value: ReplaceMode) -> Self {
        self.replace_mode = value;
        self
    }

    #[must_use]
    pub fn replace_value(mut self, value: impl Into<String>) -> Self {
        self.replace_value = value.into();
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn query_model(&self) -> &str {
        &self.query
    }

    #[must_use]
    pub const fn options_model(&self) -> &SearchOptions {
        &self.options
    }

    #[must_use]
    pub const fn replace_mode_model(&self) -> ReplaceMode {
        self.replace_mode
    }

    #[must_use]
    pub fn replace_value_model(&self) -> &str {
        &self.replace_value
    }

    #[must_use]
    pub fn result_summary_model(&self) -> String {
        types::result_summary(self.result_count, self.active_index)
    }

    pub fn apply_action(
        &mut self,
        action: SearchControlStripAction,
    ) -> Vec<SearchControlStripEvent> {
        actions::apply(self, action)
    }
}

impl From<SearchControlStrip> for crate::render_model::UiNode {
    fn from(value: SearchControlStrip) -> Self {
        render::render(value)
    }
}
