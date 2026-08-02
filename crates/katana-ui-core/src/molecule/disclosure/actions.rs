use super::types::DisclosureTypedModel;
use crate::interaction::{UiAction, UiActionResult, UiActionSource};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNodeKind, UiStateId};

pub(super) fn apply_disclosure_action(
    state: &mut MoleculeState,
    model: &DisclosureTypedModel,
    kind: UiNodeKind,
    action: &UiAction,
) -> UiActionResult {
    let before = state.interaction();
    if action.target() != &state.state_id || state.disabled {
        return UiActionResult::ignored(state.state_id.clone(), before);
    }
    if apply_kind_action(state, model, kind, action) {
        return handled(state.state_id.clone(), action, before, state.interaction());
    }
    if kind == UiNodeKind::Accordion && is_accordion_trigger_action(action) {
        return UiActionResult::ignored(state.state_id.clone(), before);
    }
    if kind == UiNodeKind::Modal && is_modal_lifecycle_action(action) {
        return UiActionResult::ignored(state.state_id.clone(), before);
    }
    state.apply_action(action, false)
}

fn apply_kind_action(
    state: &mut MoleculeState,
    model: &DisclosureTypedModel,
    kind: UiNodeKind,
    action: &UiAction,
) -> bool {
    match kind {
        UiNodeKind::Tooltip => apply_tooltip_action(state, model, action),
        UiNodeKind::Accordion => apply_toggle_action(state, model, action),
        UiNodeKind::Modal => apply_modal_action(state, model, action),
        UiNodeKind::Popover => apply_popover_action(state, model, action),
        _ => false,
    }
}

fn apply_tooltip_action(
    state: &mut MoleculeState,
    model: &DisclosureTypedModel,
    action: &UiAction,
) -> bool {
    match action {
        UiAction::SetHover { hovered, .. } if model.hover_trigger => {
            state.transient.hovered = *hovered;
            state.open = *hovered;
            true
        }
        UiAction::SetFocus { focused, .. } if model.focus_trigger => {
            state.transient.focused = *focused;
            state.open = *focused;
            true
        }
        UiAction::Press {
            source: UiActionSource::Tooltip,
            ..
        } => toggle_open(state),
        _ => false,
    }
}

fn apply_toggle_action(
    state: &mut MoleculeState,
    model: &DisclosureTypedModel,
    action: &UiAction,
) -> bool {
    match action {
        UiAction::Press { source, .. } if accepts_accordion_trigger(model, *source) => {
            if model.controlled {
                state.value = format!("requested_open={}", !state.open);
                return true;
            }
            toggle_open(state)
        }
        UiAction::Press { source, .. } if is_accordion_trigger_source(*source) => false,
        _ => false,
    }
}

pub(super) fn is_accordion_trigger_action(action: &UiAction) -> bool {
    matches!(
        action,
        UiAction::Press { source, .. } if is_accordion_trigger_source(*source)
    )
}

fn is_modal_lifecycle_action(action: &UiAction) -> bool {
    matches!(
        action,
        UiAction::Press {
            source: UiActionSource::ModalEscape | UiActionSource::ModalBackdrop,
            ..
        }
    )
}

fn is_accordion_trigger_source(source: UiActionSource) -> bool {
    matches!(
        source,
        UiActionSource::Accordion
            | UiActionSource::Click
            | UiActionSource::Generic
            | UiActionSource::AccordionRow
            | UiActionSource::AccordionIcon
            | UiActionSource::AccordionText
    )
}

fn accepts_accordion_trigger(model: &DisclosureTypedModel, source: UiActionSource) -> bool {
    match source {
        UiActionSource::Accordion => true,
        UiActionSource::Click | UiActionSource::Generic | UiActionSource::AccordionRow => {
            model.trigger_area == crate::molecule::DisclosureTriggerArea::WholeElement
        }
        UiActionSource::AccordionIcon => match model.trigger_area {
            crate::molecule::DisclosureTriggerArea::IconOnly
            | crate::molecule::DisclosureTriggerArea::IconAndText
            | crate::molecule::DisclosureTriggerArea::WholeElement => true,
            crate::molecule::DisclosureTriggerArea::TextOnly => false,
        },
        UiActionSource::AccordionText => match model.trigger_area {
            crate::molecule::DisclosureTriggerArea::IconAndText
            | crate::molecule::DisclosureTriggerArea::WholeElement
            | crate::molecule::DisclosureTriggerArea::TextOnly => true,
            crate::molecule::DisclosureTriggerArea::IconOnly => false,
        },
        _ => false,
    }
}

fn apply_modal_action(
    state: &mut MoleculeState,
    model: &DisclosureTypedModel,
    action: &UiAction,
) -> bool {
    match action {
        UiAction::Press {
            source: UiActionSource::ModalEscape,
            ..
        } => dismiss_if_allowed(
            state,
            model.escape_dismiss,
            "escape",
            focus_return_value(&model.focus_return),
        ),
        UiAction::Press {
            source: UiActionSource::ModalBackdrop,
            ..
        } => dismiss_if_allowed(
            state,
            model.outside_click_dismiss,
            "backdrop",
            focus_return_value(&model.focus_return),
        ),
        _ => false,
    }
}

fn apply_popover_action(
    state: &mut MoleculeState,
    model: &DisclosureTypedModel,
    action: &UiAction,
) -> bool {
    match action {
        UiAction::Press {
            source: UiActionSource::Popover,
            ..
        } => toggle_open(state),
        UiAction::SetFocus { focused, .. } if model.keep_open_on_inner_focus => {
            state.transient.focused = *focused;
            if *focused {
                state.open = true;
            }
            true
        }
        UiAction::Dismiss { .. } if model.keep_open_on_inner_focus && state.transient.focused => {
            true
        }
        UiAction::Dismiss { .. } => {
            dismiss_if_allowed(state, true, "dismiss", popover_focus_return(model))
        }
        UiAction::Press {
            source: UiActionSource::ModalBackdrop,
            ..
        } => dismiss_if_allowed(
            state,
            model.outside_click_dismiss,
            "outside",
            popover_focus_return(model),
        ),
        UiAction::Press {
            source: UiActionSource::ModalEscape,
            ..
        } => dismiss_if_allowed(
            state,
            model.escape_dismiss,
            "escape",
            popover_focus_return(model),
        ),
        _ => false,
    }
}

fn toggle_open(state: &mut MoleculeState) -> bool {
    state.open = !state.open;
    true
}

fn focus_return_value(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

fn popover_focus_return(model: &DisclosureTypedModel) -> Option<String> {
    model
        .focus_return_target
        .as_ref()
        .map(|target| target.as_str().to_string())
        .or_else(|| focus_return_value(&model.focus_return))
}

fn dismiss_if_allowed(
    state: &mut MoleculeState,
    allowed: bool,
    reason: &str,
    focus_return: Option<String>,
) -> bool {
    if !allowed {
        return false;
    }
    state.open = false;
    state.transient.dismiss_reason = reason.to_string();
    if let Some(focus_return) = focus_return {
        state.value = format!("focus_return={focus_return}");
    }
    true
}

fn handled(
    state_id: UiStateId,
    action: &UiAction,
    before: crate::render_model::UiInteractionState,
    after: crate::render_model::UiInteractionState,
) -> UiActionResult {
    UiActionResult::handled(state_id, action, before, after)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrelated_actions_and_sources_are_not_disclosure_triggers() {
        let mut state = MoleculeState::new(UiNodeKind::Tooltip);
        let action = UiAction::set_value(state.state_id.clone(), "value");
        let model = DisclosureTypedModel::default();

        assert!(!apply_tooltip_action(&mut state, &model, &action));
        assert!(!is_modal_lifecycle_action(&action));
        assert!(!is_accordion_trigger_source(UiActionSource::Tooltip));
        assert!(!accepts_accordion_trigger(&model, UiActionSource::Tooltip));
    }
}
