use crate::interaction::{UiAction, UiActionSource};
use crate::render_model::UiNodeKind;

pub(super) struct AtomActionPolicy;

impl AtomActionPolicy {
    pub(super) fn blocks(
        kind: UiNodeKind,
        action: &UiAction,
        disabled: bool,
        loading: bool,
        readonly: bool,
    ) -> bool {
        if disabled || Self::loading_blocks(kind, action, loading) {
            return true;
        }
        if Self::readonly_blocks(action, readonly) {
            return true;
        }
        !Self::kind_accepts_action(kind, action)
    }

    fn loading_blocks(kind: UiNodeKind, action: &UiAction, loading: bool) -> bool {
        loading && Self::is_button_like(kind) && matches!(action, UiAction::Press { .. })
    }

    fn readonly_blocks(action: &UiAction, readonly: bool) -> bool {
        readonly
            && matches!(
                action,
                UiAction::SetValue { .. }
                    | UiAction::ClearValue { .. }
                    | UiAction::SetCursorSelection { .. }
            )
    }

    fn kind_accepts_action(kind: UiNodeKind, action: &UiAction) -> bool {
        match kind {
            UiNodeKind::Button
            | UiNodeKind::SvgButton
            | UiNodeKind::TextButton
            | UiNodeKind::IconTextButton => Self::is_button_action(action),
            UiNodeKind::Input => Self::is_input_action(action),
            UiNodeKind::Checkbox | UiNodeKind::Radio | UiNodeKind::Toggle => {
                Self::is_selection_action(action)
            }
            UiNodeKind::ProgressBar => Self::is_progress_action(action),
            UiNodeKind::ColorSwatch => Self::is_color_action(action),
            UiNodeKind::Badge => Self::is_passive_status_action(action),
            UiNodeKind::LoadingDots | UiNodeKind::Spinner => Self::is_loading_action(action),
            UiNodeKind::SlideControl => Self::is_slide_action(action),
            _ => false,
        }
    }

    fn is_button_like(kind: UiNodeKind) -> bool {
        matches!(
            kind,
            UiNodeKind::Button
                | UiNodeKind::SvgButton
                | UiNodeKind::TextButton
                | UiNodeKind::IconTextButton
        )
    }

    fn is_button_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::Press { .. }
                | UiAction::SetFocus { .. }
                | UiAction::SetHover { .. }
                | UiAction::SetActive { .. }
        )
    }

    fn is_input_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::SetValue { .. }
                | UiAction::ClearValue { .. }
                | UiAction::SetFocus { .. }
                | UiAction::SetCursorSelection { .. }
                | UiAction::Press {
                    source: UiActionSource::InputSubmit,
                    ..
                }
        )
    }

    fn is_selection_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::SetSelectedIndex { .. } | UiAction::SetFocus { .. } | UiAction::Press { .. }
        )
    }

    fn is_progress_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::SetValue {
                progress: Some(_),
                ..
            }
        )
    }

    fn is_color_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::SetValue {
                color_drag: Some(_),
                ..
            } | UiAction::SetFocus { .. }
        )
    }

    fn is_passive_status_action(action: &UiAction) -> bool {
        matches!(action, UiAction::SetFocus { .. })
    }

    fn is_loading_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::AnimationTick { .. } | UiAction::SetReducedMotion { .. }
        )
    }

    fn is_slide_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::SetValue {
                source: UiActionSource::SlideControl,
                ..
            } | UiAction::SetFocus { .. }
        )
    }
}
