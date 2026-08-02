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
        let is_press = matches!(action, UiAction::Press { .. });
        loading && Self::is_button_like(kind) && is_press
    }

    fn readonly_blocks(action: &UiAction, readonly: bool) -> bool {
        let mutates_value = matches!(
            action,
            UiAction::SetValue { .. } | UiAction::ClearValue { .. } | UiAction::PasteText { .. }
        );
        readonly && mutates_value
    }

    fn kind_accepts_action(kind: UiNodeKind, action: &UiAction) -> bool {
        match kind {
            UiNodeKind::Text => matches!(action, UiAction::CopySelection { .. }),
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
                | UiAction::CopySelection { .. }
                | UiAction::PasteText { .. }
                | UiAction::InvokeCallback { .. }
                | UiAction::Press {
                    source: UiActionSource::InputSubmit,
                    ..
                }
        )
    }

    fn is_selection_action(action: &UiAction) -> bool {
        matches!(
            action,
            UiAction::SetSelectedIndex { .. }
                | UiAction::SetFocus { .. }
                | UiAction::SetHover { .. }
                | UiAction::Press { .. }
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
        matches!(
            action,
            UiAction::SetFocus { .. } | UiAction::CopySelection { .. }
        )
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
                | UiAction::SetHover { .. }
                | UiAction::SetDragging { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::RgbaActionValue;
    use crate::render_model::UiStateId;

    #[test]
    fn policy_accepts_each_typed_atom_action_family() {
        let target = UiStateId::new("atom");
        let cases = [
            (UiNodeKind::Text, UiAction::copy_selection(target.clone())),
            (UiNodeKind::Button, UiAction::button_press(target.clone())),
            (UiNodeKind::Input, UiAction::input_submitted(target.clone())),
            (
                UiNodeKind::Checkbox,
                UiAction::checkbox_checked(target.clone(), true),
            ),
            (
                UiNodeKind::ProgressBar,
                UiAction::progress_changed(target.clone(), true, 50),
            ),
            (
                UiNodeKind::ColorSwatch,
                UiAction::color_drag(
                    target.clone(),
                    RgbaActionValue::new(1, 2, 3, 255),
                    120,
                    true,
                ),
            ),
            (UiNodeKind::Badge, UiAction::focus(target.clone())),
            (
                UiNodeKind::LoadingDots,
                UiAction::animation_tick(target.clone(), 1),
            ),
            (
                UiNodeKind::Spinner,
                UiAction::reduced_motion(target.clone(), true),
            ),
            (
                UiNodeKind::SlideControl,
                UiAction::slide_changed(target.clone(), "0.5"),
            ),
        ];

        for (kind, action) in cases {
            assert!(!AtomActionPolicy::blocks(
                kind, &action, false, false, false
            ));
        }
    }

    #[test]
    fn loading_readonly_disabled_and_wrong_kind_are_blocked() {
        let target = UiStateId::new("atom");
        assert!(AtomActionPolicy::blocks(
            UiNodeKind::TextButton,
            &UiAction::button_press(target.clone()),
            false,
            true,
            false,
        ));
        assert!(AtomActionPolicy::blocks(
            UiNodeKind::Input,
            &UiAction::paste_text(target.clone(), "blocked"),
            false,
            false,
            true,
        ));
        assert!(AtomActionPolicy::blocks(
            UiNodeKind::Input,
            &UiAction::focus(target.clone()),
            true,
            false,
            false,
        ));
        assert!(AtomActionPolicy::blocks(
            UiNodeKind::Text,
            &UiAction::focus(target),
            false,
            false,
            false,
        ));
    }

    #[test]
    fn policy_covers_each_button_kind_and_action_family_boundary() {
        let target = UiStateId::new("atom-boundary");
        for kind in [
            UiNodeKind::Button,
            UiNodeKind::SvgButton,
            UiNodeKind::TextButton,
            UiNodeKind::IconTextButton,
        ] {
            assert!(!AtomActionPolicy::blocks(
                kind,
                &UiAction::hover(target.clone(), true),
                false,
                false,
                false,
            ));
            assert!(AtomActionPolicy::blocks(
                kind,
                &UiAction::button_press(target.clone()),
                false,
                true,
                false,
            ));
        }

        for action in [
            UiAction::input_value(target.clone(), "value"),
            UiAction::clear_value(target.clone()),
            UiAction::cursor_selection(target.clone(), 1, 0, 1),
            UiAction::copy_selection(target.clone()),
            UiAction::paste_text(target.clone(), "pasted"),
            UiAction::invoke_callback(target.clone(), "clear"),
        ] {
            assert!(!AtomActionPolicy::blocks(
                UiNodeKind::Input,
                &action,
                false,
                false,
                false,
            ));
        }

        for kind in [UiNodeKind::Checkbox, UiNodeKind::Radio, UiNodeKind::Toggle] {
            assert!(!AtomActionPolicy::blocks(
                kind,
                &UiAction::press(target.clone()),
                false,
                false,
                false,
            ));
        }
        assert!(!AtomActionPolicy::blocks(
            UiNodeKind::ColorSwatch,
            &UiAction::focus(target.clone()),
            false,
            false,
            false,
        ));
        assert!(!AtomActionPolicy::blocks(
            UiNodeKind::Badge,
            &UiAction::copy_selection(target.clone()),
            false,
            false,
            false,
        ));
        assert!(!AtomActionPolicy::blocks(
            UiNodeKind::LoadingDots,
            &UiAction::reduced_motion(target.clone(), true),
            false,
            false,
            false,
        ));
        assert!(!AtomActionPolicy::blocks(
            UiNodeKind::SlideControl,
            &UiAction::dragging(target.clone(), true),
            false,
            false,
            false,
        ));
        assert!(AtomActionPolicy::blocks(
            UiNodeKind::ImageSurface,
            &UiAction::focus(target),
            false,
            false,
            false,
        ));
    }

    #[test]
    fn typed_action_helpers_reject_unrelated_action_families() {
        let target = UiStateId::new("atom-rejection");
        let unrelated = UiAction::animation_tick(target, 1);

        assert!(!AtomActionPolicy::is_button_like(UiNodeKind::Text));
        assert!(!AtomActionPolicy::is_button_action(&unrelated));
        assert!(!AtomActionPolicy::is_input_action(&unrelated));
        assert!(!AtomActionPolicy::is_selection_action(&unrelated));
        assert!(!AtomActionPolicy::is_progress_action(&unrelated));
        assert!(!AtomActionPolicy::is_color_action(&unrelated));
        assert!(!AtomActionPolicy::is_passive_status_action(&unrelated));
        assert!(!AtomActionPolicy::is_loading_action(&UiAction::focus(
            UiStateId::new("loading")
        )));
        assert!(!AtomActionPolicy::is_slide_action(&unrelated));
    }
}
