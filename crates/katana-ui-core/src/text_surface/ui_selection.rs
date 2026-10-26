use super::TextSurface;
use crate::atom::TextAreaSelection;
use crate::text_selection::UiTextCaretPosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TextSurfaceUiSelection {
    selection: TextAreaSelection,
    affinity: Option<UiTextCaretPosition>,
    preferred_column: Option<i32>,
}

impl TextSurface {
    pub(crate) fn ui_selection(&self) -> TextSurfaceUiSelection {
        TextSurfaceUiSelection {
            selection: self.state.text_area.selection,
            affinity: self.caret_affinity,
            preferred_column: self.preferred_column,
        }
    }

    pub(crate) fn restore_ui_selection(&mut self, value: &TextSurfaceUiSelection) {
        self.props.text_area.synchronize_selection(value.selection);
        self.state.text_area = self.props.text_area.state().clone();
        self.caret_affinity = value.affinity;
        self.preferred_column = value.preferred_column;
        self.state.pointer_anchor = None;
    }

    pub(crate) const fn preferred_column(&self) -> Option<i32> {
        self.preferred_column
    }

    pub(crate) fn set_preferred_column(&mut self, value: Option<i32>) {
        self.preferred_column = value;
    }
}
