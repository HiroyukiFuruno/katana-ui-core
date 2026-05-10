use super::types::ModalProps;

/// Returns whether a backdrop click should close the modal.
pub(super) fn should_dismiss_on_backdrop(props: &ModalProps) -> bool {
    props.dismiss_on_backdrop
}

/// Returns whether an Esc key press should close the modal.
pub(super) fn should_dismiss_on_esc(props: &ModalProps) -> bool {
    props.dismiss_on_esc
}
