use super::{
    HistoryTraversal, SearchQueryHistoryDirection, SearchResultMode, SelectionDialogOperation,
};
use crate::atom::TextAreaEvent;
use crate::egui::text_surface::OrderedKeyboardInput;
use crate::text_surface::{TextSurface, TextSurfaceEvent};

pub(super) struct QueryHistoryInput {
    pub(super) traversal: Option<HistoryTraversal>,
    pub(super) mode: SearchResultMode,
    pub(super) operations: Vec<SelectionDialogOperation>,
    pub(super) ime_active: bool,
}

impl OrderedKeyboardInput for QueryHistoryInput {
    fn before(&mut self, surface: &mut TextSurface, event: &egui::Event) -> bool {
        if self.ime_active {
            return false;
        }
        let direction = match event {
            egui::Event::Key {
                key: egui::Key::ArrowUp,
                pressed: true,
                modifiers,
                ..
            } if *modifiers == egui::Modifiers::NONE => SearchQueryHistoryDirection::Older,
            egui::Event::Key {
                key: egui::Key::ArrowDown,
                pressed: true,
                modifiers,
                ..
            } if *modifiers == egui::Modifiers::NONE => SearchQueryHistoryDirection::Newer,
            _ => return false,
        };
        self.mode.query.clone_from(&surface.state().text_area.value);
        self.navigate_history(direction);
        surface.synchronize_value(self.mode.query.clone());
        true
    }
    fn after(&mut self, surface: &TextSurface, events: &[TextSurfaceEvent]) {
        if events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::TextArea(TextAreaEvent::Change(_))))
        {
            self.traversal = None;
            self.operations
                .push(SelectionDialogOperation::QueryChanged {
                    mode: self.mode.target.clone(),
                    value: surface.state().text_area.value.clone(),
                });
        }
    }
}

impl QueryHistoryInput {
    fn navigate_history(&mut self, direction: SearchQueryHistoryDirection) {
        enum Step {
            Select(usize),
            Restore(String),
        }
        let mode = &mut self.mode;
        let eligible = mode
            .history
            .iter()
            .filter(|row| !row.disabled)
            .collect::<Vec<_>>();
        if eligible.is_empty() {
            return;
        }
        let next = match (direction, self.traversal.as_ref()) {
            (SearchQueryHistoryDirection::Older, None) => Step::Select(0),
            (SearchQueryHistoryDirection::Older, Some(cursor))
                if cursor.index + 1 < eligible.len() =>
            {
                Step::Select(cursor.index + 1)
            }
            (SearchQueryHistoryDirection::Newer, Some(cursor)) if cursor.index > 0 => {
                Step::Select(cursor.index - 1)
            }
            (SearchQueryHistoryDirection::Newer, Some(cursor)) => {
                Step::Restore(cursor.draft.clone())
            }
            _ => return,
        };
        let (value, target) = match next {
            Step::Select(index) => {
                let draft = self
                    .traversal
                    .as_ref()
                    .map_or_else(|| mode.query.clone(), |cursor| cursor.draft.clone());
                self.traversal = Some(super::HistoryTraversal { draft, index });
                (
                    eligible[index].label.clone(),
                    Some(eligible[index].target.clone()),
                )
            }
            Step::Restore(draft) => {
                self.traversal = None;
                (draft, None)
            }
        };
        let changed = mode.query != value;
        mode.query.clone_from(&value);
        if !changed {
            return;
        }
        self.operations
            .push(SelectionDialogOperation::QueryHistoryNavigated {
                mode: mode.target.clone(),
                target,
                value,
                direction,
            });
    }
}
