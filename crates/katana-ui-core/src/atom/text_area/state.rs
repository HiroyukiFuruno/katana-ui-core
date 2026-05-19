use super::autogrow::{TextAreaRowMeasurement, measure_rows};
use super::caret;
use super::{TextAreaCompositionState, TextAreaOptions};
use crate::render_model::{UiInteractionState, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaSelection {
    pub start: usize,
    pub end: usize,
}

impl TextAreaSelection {
    #[must_use]
    pub const fn collapsed(caret: usize) -> Self {
        Self {
            start: caret,
            end: caret,
        }
    }

    pub(super) fn ordered(self) -> (usize, usize) {
        if self.start <= self.end {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaState {
    pub state_id: UiStateId,
    pub value: String,
    pub caret: usize,
    pub selection: TextAreaSelection,
    pub composition: Option<TextAreaCompositionState>,
    pub focused: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub measured_rows: u16,
    pub internal_scroll: bool,
}

impl TextAreaState {
    #[must_use]
    pub fn from_options(options: &TextAreaOptions) -> Self {
        let measurement = measure_rows(
            &options.value,
            options.min_rows,
            options.max_rows,
            options.auto_grow,
        );
        let caret = options.value.len();
        Self {
            state_id: UiStateId::next_for(UiNodeKind::TextArea),
            value: options.value.clone(),
            caret,
            selection: TextAreaSelection::collapsed(caret),
            composition: None,
            focused: false,
            disabled: options.disabled,
            readonly: options.readonly,
            invalid: options.invalid,
            measured_rows: measurement.rows,
            internal_scroll: measurement.internal_scroll,
        }
    }

    #[must_use]
    pub fn interaction_state(&self) -> UiInteractionState {
        UiInteractionState {
            value: self.value.clone(),
            focused: self.focused,
            cursor: self.caret,
            selection_start: self.selection.start,
            selection_end: self.selection.end,
            ..UiInteractionState::default()
        }
    }

    pub(super) fn set_value(&mut self, value: String) {
        self.value = value;
        self.caret = self.value.len();
        self.selection = TextAreaSelection::collapsed(self.caret);
    }

    pub(super) fn set_caret(&mut self, caret: usize) {
        self.caret = caret::clamp_to_char_boundary(&self.value, caret);
        self.selection = TextAreaSelection::collapsed(self.caret);
    }

    pub(super) fn set_selection(&mut self, selection: TextAreaSelection) {
        let start = caret::clamp_to_char_boundary(&self.value, selection.start);
        let end = caret::clamp_to_char_boundary(&self.value, selection.end);
        self.selection = TextAreaSelection { start, end };
        self.caret = end;
    }

    pub(super) fn measure(
        &mut self,
        min_rows: u16,
        max_rows: u16,
        auto_grow: bool,
    ) -> TextAreaRowMeasurement {
        let measurement = measure_rows(&self.value, min_rows, max_rows, auto_grow);
        self.measured_rows = measurement.rows;
        self.internal_scroll = measurement.internal_scroll;
        measurement
    }
}
