use super::{
    BulkFixSkipReason, DiagnosticId, DiagnosticItem, DiagnosticKeyboardInput,
    DiagnosticsListAction, DiagnosticsListEvent, DiagnosticsListOptions, DiagnosticsListPlanner,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListState {
    pub selected_id: Option<DiagnosticId>,
    pub expanded_ids: BTreeSet<DiagnosticId>,
    pub loading: bool,
    pub bulk_preview_open: bool,
}

impl DiagnosticsListState {
    pub(super) fn apply_action(
        &mut self,
        action: DiagnosticsListAction,
        items: &[DiagnosticItem],
        options: &DiagnosticsListOptions,
    ) -> Vec<DiagnosticsListEvent> {
        match action {
            DiagnosticsListAction::SetGroupBy(_)
            | DiagnosticsListAction::SetSortBy(_)
            | DiagnosticsListAction::SetSeverityFilter(_) => {
                vec![DiagnosticsListEvent::FilterChanged]
            }
            DiagnosticsListAction::Select(id) => self.select(id),
            DiagnosticsListAction::ToggleFixPreview(id) => self.toggle_fix_preview(id),
            DiagnosticsListAction::ApplyFix(id) => apply_fix(items, id),
            DiagnosticsListAction::OpenBulkPreview => self.open_bulk_preview(),
            DiagnosticsListAction::ConfirmBulkApply => bulk_apply(items, options),
            DiagnosticsListAction::Keyboard(input) => self.apply_keyboard(input, items, options),
        }
    }

    fn select(&mut self, id: DiagnosticId) -> Vec<DiagnosticsListEvent> {
        self.selected_id = Some(id.clone());
        vec![DiagnosticsListEvent::DiagnosticSelected { id }]
    }

    fn toggle_fix_preview(&mut self, id: DiagnosticId) -> Vec<DiagnosticsListEvent> {
        let expanded = if self.expanded_ids.contains(&id) {
            self.expanded_ids.remove(&id);
            false
        } else {
            self.expanded_ids.insert(id.clone());
            true
        };
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded }]
    }

    fn open_bulk_preview(&mut self) -> Vec<DiagnosticsListEvent> {
        self.bulk_preview_open = true;
        vec![DiagnosticsListEvent::BulkFixPreviewOpened]
    }

    fn apply_keyboard(
        &mut self,
        input: DiagnosticKeyboardInput,
        items: &[DiagnosticItem],
        options: &DiagnosticsListOptions,
    ) -> Vec<DiagnosticsListEvent> {
        match input {
            DiagnosticKeyboardInput::F8 => self.select_error(items, options, true),
            DiagnosticKeyboardInput::ShiftF8 => self.select_error(items, options, false),
            DiagnosticKeyboardInput::Space => self.apply_selected_fix(items),
            DiagnosticKeyboardInput::Enter => self.navigate_selected(),
            DiagnosticKeyboardInput::ArrowRight => self.toggle_selected_preview(),
            DiagnosticKeyboardInput::ArrowLeft => self.collapse_selected_preview(),
            DiagnosticKeyboardInput::ArrowUp => self.select_visible(items, options, false),
            DiagnosticKeyboardInput::ArrowDown => self.select_visible(items, options, true),
        }
    }

    fn select_visible(
        &mut self,
        items: &[DiagnosticItem],
        options: &DiagnosticsListOptions,
        forward: bool,
    ) -> Vec<DiagnosticsListEvent> {
        let ids = DiagnosticsListPlanner::visible_items(items, options)
            .into_iter()
            .map(|it| it.id.clone())
            .collect::<Vec<_>>();
        let Some(id) = next_id(&ids, self.selected_id.as_ref(), forward, options) else {
            return Vec::new();
        };
        self.select(id)
    }

    fn select_error(
        &mut self,
        items: &[DiagnosticItem],
        options: &DiagnosticsListOptions,
        forward: bool,
    ) -> Vec<DiagnosticsListEvent> {
        let visible = DiagnosticsListPlanner::visible_items(items, options);
        let errors = visible
            .into_iter()
            .filter(|it| it.severity.is_error())
            .map(|it| it.id.clone())
            .collect::<Vec<_>>();
        let Some(id) = next_id(&errors, self.selected_id.as_ref(), forward, options) else {
            return Vec::new();
        };
        self.select(id)
    }

    fn apply_selected_fix(&self, items: &[DiagnosticItem]) -> Vec<DiagnosticsListEvent> {
        self.selected_id
            .clone()
            .map_or_else(Vec::new, |id| apply_fix(items, id))
    }

    fn toggle_selected_preview(&mut self) -> Vec<DiagnosticsListEvent> {
        self.selected_id
            .clone()
            .map_or_else(Vec::new, |id| self.toggle_fix_preview(id))
    }

    fn navigate_selected(&self) -> Vec<DiagnosticsListEvent> {
        self.selected_id.clone().map_or_else(Vec::new, |id| {
            vec![DiagnosticsListEvent::NavigateRequested { id }]
        })
    }

    fn collapse_selected_preview(&mut self) -> Vec<DiagnosticsListEvent> {
        let Some(id) = self.selected_id.clone() else {
            return Vec::new();
        };
        if !self.expanded_ids.remove(&id) {
            return Vec::new();
        }
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id,
            expanded: false,
        }]
    }
}

fn apply_fix(items: &[DiagnosticItem], id: DiagnosticId) -> Vec<DiagnosticsListEvent> {
    items
        .iter()
        .find(|it| it.id == id && it.quickfix.is_some())
        .map_or_else(Vec::new, |_| {
            vec![DiagnosticsListEvent::DiagnosticFixApplied { id }]
        })
}

fn bulk_apply(
    items: &[DiagnosticItem],
    options: &DiagnosticsListOptions,
) -> Vec<DiagnosticsListEvent> {
    let visible = DiagnosticsListPlanner::visible_items(items, options);
    let visible_ids = visible
        .iter()
        .map(|it| it.id.clone())
        .collect::<BTreeSet<_>>();
    let mut applied_ids = Vec::new();
    let mut skipped_ids = Vec::new();
    for item in items {
        if !visible_ids.contains(&item.id) {
            skipped_ids.push((item.id.clone(), BulkFixSkipReason::FilteredOut));
        } else if item.quickfix.is_some() {
            applied_ids.push(item.id.clone());
        } else {
            skipped_ids.push((item.id.clone(), BulkFixSkipReason::NoQuickfix));
        }
    }
    vec![DiagnosticsListEvent::BulkFixApplied {
        applied_ids,
        skipped_ids,
    }]
}

fn next_id(
    ids: &[DiagnosticId],
    current: Option<&DiagnosticId>,
    forward: bool,
    options: &DiagnosticsListOptions,
) -> Option<DiagnosticId> {
    if ids.is_empty() {
        return None;
    }
    let current_index = current.and_then(|id| ids.iter().position(|it| it == id));
    let next_index = match (current_index, forward) {
        (Some(index), true) if index + 1 < ids.len() => index + 1,
        (Some(_), true) if options.wrap_error_navigation => 0,
        (Some(index), false) if index > 0 => index - 1,
        (Some(_), false) if options.wrap_error_navigation => ids.len() - 1,
        (None, _) => 0,
        _ => return None,
    };
    ids.get(next_index).cloned()
}

impl super::DiagnosticSeverity {
    fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}
