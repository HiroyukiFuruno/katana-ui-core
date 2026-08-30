//! KUC-owned retained interaction state for a generic full-surface scenario.

use super::scenario::{
    self, FullTextCommandSurfaceScenarioError, FullTextCommandSurfaceScenarioId,
};
use super::{
    EguiTextCommandSurfaceHostProjectionLease, KucOpaqueHostEffectBatch, KucRootEventBatchContext,
};
use katana_ui_core::atom::TextAreaEvent;
use katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent;
use katana_ui_core::molecule::structured::SearchControlStripEvent;
use katana_ui_core::text_surface::TextSurfaceEvent;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod types;

pub use self::types::FullTextCommandSurfaceScenarioSession;
use self::types::{ScenarioSessionState, ScenarioSessionUpdate};

impl FullTextCommandSurfaceScenarioSession {
    #[must_use]
    pub fn new(id: FullTextCommandSurfaceScenarioId) -> Self {
        Self {
            id,
            state: Rc::new(RefCell::new(ScenarioSessionState::default())),
            next_revision: Cell::new(1),
        }
    }

    /// Issues the initial opaque root lease.
    pub fn retain_lease(
        &self,
    ) -> Result<EguiTextCommandSurfaceHostProjectionLease, FullTextCommandSurfaceScenarioError>
    {
        self.issue_lease()
    }

    /// Issues the current opaque root lease after accepted one-shot input dispatch.
    pub fn synchronize_lease(
        &self,
    ) -> Result<EguiTextCommandSurfaceHostProjectionLease, FullTextCommandSurfaceScenarioError>
    {
        self.issue_lease()
    }

    fn issue_lease(
        &self,
    ) -> Result<EguiTextCommandSurfaceHostProjectionLease, FullTextCommandSurfaceScenarioError>
    {
        let presentation = self.state.borrow().presentation(self.id);
        let projected_text = presentation.text.value.clone();
        let revision = self.next_revision()?;
        let state = Rc::clone(&self.state);
        scenario::issue_lease_at_revision(self.id, presentation, revision, move |context| {
            let update = ScenarioSessionUpdate::from_context(&context);
            if update.is_empty() {
                return Ok(None);
            }
            let state = Rc::clone(&state);
            let projected_text = projected_text.clone();
            Ok(Some(KucOpaqueHostEffectBatch::from_handler(move || {
                state.borrow_mut().apply(update, &projected_text);
                Ok(())
            })))
        })
    }

    fn next_revision(&self) -> Result<u64, FullTextCommandSurfaceScenarioError> {
        let revision = self.next_revision.get();
        let next = revision
            .checked_add(1)
            .ok_or(FullTextCommandSurfaceScenarioError::RevisionExhausted)?;
        self.next_revision.set(next);
        Ok(revision)
    }
}

impl ScenarioSessionState {
    fn presentation(
        &self,
        id: FullTextCommandSurfaceScenarioId,
    ) -> super::EguiTextCommandSurfacePresentation {
        let mut presentation = scenario::presentation(id);
        if let Some(text) = &self.text {
            presentation.text.value.clone_from(text);
            presentation.text.annotations.clear();
        }
        if let Some((start, end)) = self.selection
            && valid_selection(&presentation.text.value, start, end)
        {
            presentation.text.selection_start = start;
            presentation.text.selection_end = end;
        }
        if let Some(search) = &mut presentation.search {
            if let Some(query) = &self.search_query {
                search.value.query.clone_from(query);
            }
            if let Some(value) = &self.replace_value {
                search.value.replace_value.clone_from(value);
            }
        }
        presentation
    }

    fn apply(&mut self, update: ScenarioSessionUpdate, projected_text: &str) {
        let selection_source = update.text.as_deref().unwrap_or(projected_text);
        let accepted_selection = update
            .selection
            .filter(|selection| valid_selection(selection_source, selection.0, selection.1));
        if let Some(text) = update.text {
            self.text = Some(text);
        }
        if let Some(selection) = accepted_selection {
            self.selection = Some(selection);
        }
        if let Some(query) = update.search_query {
            self.search_query = Some(query);
        }
        if let Some(value) = update.replace_value {
            self.replace_value = Some(value);
        }
    }
}

impl ScenarioSessionUpdate {
    fn from_context(context: &KucRootEventBatchContext) -> Self {
        let mut update = Self::default();
        for event in context.text_events() {
            match event {
                TextSurfaceEvent::TextArea(TextAreaEvent::Change(value)) => {
                    update.text = Some(value.clone());
                }
                TextSurfaceEvent::SelectionChanged {
                    selection_start,
                    selection_end,
                } => {
                    update.selection = Some((*selection_start, *selection_end));
                }
                _ => {}
            }
        }
        for event in context.search_events() {
            let CommandChromeSearchEvent::Strip { event } = event else {
                continue;
            };
            match event {
                SearchControlStripEvent::SearchQueryChanged(value) => {
                    update.search_query = Some(value.clone());
                }
                SearchControlStripEvent::ReplaceValueChanged(value) => {
                    update.replace_value = Some(value.clone());
                }
                _ => {}
            }
        }
        update
    }

    const fn is_empty(&self) -> bool {
        self.text.is_none()
            && self.selection.is_none()
            && self.search_query.is_none()
            && self.replace_value.is_none()
    }
}

fn valid_selection(value: &str, start: usize, end: usize) -> bool {
    start <= end
        && end <= value.len()
        && value.is_char_boundary(start)
        && value.is_char_boundary(end)
}

#[cfg(test)]
#[path = "scenario_session_tests.rs"]
mod tests;
