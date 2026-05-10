use super::types::ModalProps;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusTransition {
    None,
    EnterDialog,
    ReturnToTrigger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissReason {
    Backdrop,
    Escape,
}

/// Returns whether the modal should close from backdrop click.
pub(super) fn should_dismiss_on_backdrop(props: &ModalProps) -> bool {
    should_close(props, DismissReason::Backdrop)
}

/// Returns whether the modal should close from Esc key.
pub(super) fn should_dismiss_on_esc(props: &ModalProps) -> bool {
    should_close(props, DismissReason::Escape)
}

/// Returns whether focus should be trapped while the modal is open.
pub(super) fn should_trap_focus(props: &ModalProps) -> bool {
    props.open
}

pub(super) fn should_close(props: &ModalProps, reason: DismissReason) -> bool {
    props.open
        && match reason {
            DismissReason::Backdrop => props.dismiss_on_backdrop,
            DismissReason::Escape => props.dismiss_on_esc,
        }
}

pub(super) fn focus_transition(open_before: bool, open_after: bool) -> FocusTransition {
    match (open_before, open_after) {
        (false, true) => FocusTransition::EnterDialog,
        (true, false) => FocusTransition::ReturnToTrigger,
        _ => FocusTransition::None,
    }
}
