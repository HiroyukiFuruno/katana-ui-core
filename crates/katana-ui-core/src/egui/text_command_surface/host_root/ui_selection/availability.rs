use super::KucUiSelectionAvailability;

pub(super) struct SelectionAvailability;

impl SelectionAvailability {
    pub(super) fn resolve(
        state: &crate::atom::TextAreaState,
        visible: bool,
        provisional: bool,
    ) -> KucUiSelectionAvailability {
        if provisional {
            return KucUiSelectionAvailability::Provisional;
        }
        if !visible {
            return KucUiSelectionAvailability::Hidden;
        }
        if state.disabled {
            return KucUiSelectionAvailability::Disabled;
        }
        if state.readonly {
            return KucUiSelectionAvailability::Readonly;
        }
        if state.composition.is_some() {
            return KucUiSelectionAvailability::Composing;
        }
        if !state.focused {
            return KucUiSelectionAvailability::Unfocused;
        }
        KucUiSelectionAvailability::FocusedEditable
    }
}
