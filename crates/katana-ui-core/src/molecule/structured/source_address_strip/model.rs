use serde::{Deserialize, Serialize as DeriveSerialize};
use std::fmt;

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, DeriveSerialize, Deserialize)]
pub struct SourceAddressPresentation {
    visible: String,
    tooltip: String,
    accessibility: String,
}

impl SourceAddressPresentation {
    #[must_use]
    pub fn new(
        visible: impl Into<String>,
        tooltip: impl Into<String>,
        accessibility: impl Into<String>,
    ) -> Self {
        Self {
            visible: visible.into(),
            tooltip: tooltip.into(),
            accessibility: accessibility.into(),
        }
    }

    #[must_use]
    pub fn visible(&self) -> &str {
        &self.visible
    }

    #[must_use]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    #[must_use]
    pub fn accessibility(&self) -> &str {
        &self.accessibility
    }
}

/// host target をKUC内部だけで保持する選択項目。
pub struct SourceAddressEntry {
    presentation: SourceAddressPresentation,
    target: Vec<u8>,
}

impl SourceAddressEntry {
    #[must_use]
    pub fn new(presentation: SourceAddressPresentation, opaque_target: impl Into<Vec<u8>>) -> Self {
        Self {
            presentation,
            target: opaque_target.into(),
        }
    }

    #[must_use]
    pub fn presentation(&self) -> &SourceAddressPresentation {
        &self.presentation
    }

    fn retain_target(&self) {
        let _ = self.target.as_slice();
    }
}

impl fmt::Debug for SourceAddressEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceAddressEntry(..)")
    }
}

pub struct SourceAddressStrip {
    presentation: SourceAddressPresentation,
    draft: String,
    history: Vec<SourceAddressEntry>,
    candidates: Vec<SourceAddressEntry>,
    selected_history: Option<usize>,
    selected_candidate: Option<usize>,
    history_open: bool,
    candidates_open: bool,
    focused: bool,
    enabled: bool,
}

impl SourceAddressStrip {
    #[must_use]
    pub fn new(presentation: SourceAddressPresentation) -> Self {
        Self {
            presentation,
            draft: String::new(),
            history: Vec::new(),
            candidates: Vec::new(),
            selected_history: None,
            selected_candidate: None,
            history_open: false,
            candidates_open: false,
            focused: false,
            enabled: true,
        }
    }

    #[must_use]
    pub fn presentation(&self) -> &SourceAddressPresentation {
        &self.presentation
    }

    #[must_use]
    pub fn draft(&self) -> &str {
        &self.draft
    }

    #[must_use]
    pub fn history(&self) -> &[SourceAddressEntry] {
        &self.history
    }

    #[must_use]
    pub fn candidates(&self) -> &[SourceAddressEntry] {
        &self.candidates
    }

    #[must_use]
    pub const fn selected_history(&self) -> Option<usize> {
        self.selected_history
    }

    #[must_use]
    pub const fn selected_candidate(&self) -> Option<usize> {
        self.selected_candidate
    }

    #[must_use]
    pub const fn history_open(&self) -> bool {
        self.history_open
    }

    #[must_use]
    pub const fn candidates_open(&self) -> bool {
        self.candidates_open
    }

    #[must_use]
    pub const fn focused(&self) -> bool {
        self.focused
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_history(&mut self, history: Vec<SourceAddressEntry>) {
        self.history = history;
        self.selected_history = None;
    }

    pub fn set_candidates(&mut self, candidates: Vec<SourceAddressEntry>) {
        self.candidates = candidates;
        self.selected_candidate = None;
    }

    pub fn apply_action(
        &mut self,
        action: super::actions::SourceAddressAction,
    ) -> Option<super::actions::SourceAddressEvent> {
        super::actions::apply(self, action)
    }

    pub(super) fn set_draft(
        &mut self,
        draft: String,
    ) -> Option<super::actions::SourceAddressEvent> {
        if self.draft == draft {
            return None;
        }
        self.draft = draft;
        Some(super::actions::SourceAddressEvent::DraftChanged)
    }

    pub(super) fn set_enabled(
        &mut self,
        enabled: bool,
    ) -> Option<super::actions::SourceAddressEvent> {
        if self.enabled == enabled {
            return None;
        }
        self.enabled = enabled;
        if !enabled {
            self.focused = false;
            self.history_open = false;
            self.candidates_open = false;
        }
        Some(super::actions::SourceAddressEvent::EnabledChanged)
    }

    pub(super) fn set_focused(
        &mut self,
        focused: bool,
    ) -> Option<super::actions::SourceAddressEvent> {
        if self.focused == focused {
            return None;
        }
        self.focused = focused;
        Some(if focused {
            super::actions::SourceAddressEvent::Focused
        } else {
            super::actions::SourceAddressEvent::Blurred
        })
    }

    pub(super) fn open_history(&mut self) -> Option<super::actions::SourceAddressEvent> {
        if self.history_open {
            return None;
        }
        self.history_open = true;
        self.candidates_open = false;
        Some(super::actions::SourceAddressEvent::HistoryOpened)
    }

    pub(super) fn close_history(&mut self) -> Option<super::actions::SourceAddressEvent> {
        if !self.history_open {
            return None;
        }
        self.history_open = false;
        Some(super::actions::SourceAddressEvent::HistoryClosed)
    }

    pub(super) fn open_candidates(&mut self) -> Option<super::actions::SourceAddressEvent> {
        if self.candidates_open {
            return None;
        }
        self.candidates_open = true;
        self.history_open = false;
        Some(super::actions::SourceAddressEvent::CandidatesOpened)
    }

    pub(super) fn close_candidates(&mut self) -> Option<super::actions::SourceAddressEvent> {
        if !self.candidates_open {
            return None;
        }
        self.candidates_open = false;
        Some(super::actions::SourceAddressEvent::CandidatesClosed)
    }

    pub(super) fn select_history(
        &mut self,
        index: usize,
    ) -> Option<super::actions::SourceAddressEvent> {
        let entry = self.history.get(index)?;
        entry.retain_target();
        self.draft.clone_from(&entry.presentation.visible);
        self.selected_history = Some(index);
        self.history_open = false;
        Some(super::actions::SourceAddressEvent::HistorySelected)
    }

    pub(super) fn select_candidate(
        &mut self,
        index: usize,
    ) -> Option<super::actions::SourceAddressEvent> {
        let entry = self.candidates.get(index)?;
        entry.retain_target();
        self.draft.clone_from(&entry.presentation.visible);
        self.selected_candidate = Some(index);
        self.candidates_open = false;
        Some(super::actions::SourceAddressEvent::CandidateSelected)
    }
}

impl fmt::Debug for SourceAddressStrip {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SourceAddressStrip")
            .field("draft_present", &!self.draft.is_empty())
            .field("history_count", &self.history.len())
            .field("candidate_count", &self.candidates.len())
            .field("selected_history", &self.selected_history)
            .field("selected_candidate", &self.selected_candidate)
            .field("history_open", &self.history_open)
            .field("candidates_open", &self.candidates_open)
            .field("focused", &self.focused)
            .field("enabled", &self.enabled)
            .finish()
    }
}
