use super::modal_types::ModalParentInteraction;
use super::types::DisclosureTypedModel;
use crate::render_model::{
    UiModalParentInteraction, UiModalPlacement, UiModalPresentation, UiModalProps, UiModalSize,
};

pub(super) fn native_modal_props(model: &DisclosureTypedModel) -> UiModalProps {
    UiModalProps {
        presentation: UiModalPresentation::NativeWindow,
        title: model.title.clone(),
        size: modal_size(model.size.as_str()),
        footer: model.footer.clone(),
        backdrop: model.backdrop.clone(),
        focus_trap: true,
        focus_return: model.focus_return.clone(),
        dismiss_policy: model.dismiss_policy.clone(),
        dismiss_on_escape: model.escape_dismiss,
        dismiss_on_backdrop: model.outside_click_dismiss,
        parent_interaction: parent_interaction(model.parent_interaction),
        placement: UiModalPlacement::Center,
    }
}

pub(super) fn overlay_dialog_props(
    backdrop: &str,
    focus_trap: bool,
    focus_return: &str,
    dismiss_policy: &str,
    dismiss_on_escape: bool,
    dismiss_on_backdrop: bool,
    placement: UiModalPlacement,
) -> UiModalProps {
    UiModalProps {
        presentation: UiModalPresentation::OverlayDialog,
        backdrop: backdrop.to_string(),
        focus_trap,
        focus_return: focus_return.to_string(),
        dismiss_policy: dismiss_policy.to_string(),
        dismiss_on_escape,
        dismiss_on_backdrop,
        placement,
        ..UiModalProps::default()
    }
}

fn parent_interaction(value: ModalParentInteraction) -> UiModalParentInteraction {
    match value {
        ModalParentInteraction::Block => UiModalParentInteraction::Block,
        ModalParentInteraction::Allow => UiModalParentInteraction::Allow,
    }
}

fn modal_size(value: &str) -> UiModalSize {
    match value {
        "small" | "sm" => UiModalSize::Small,
        "large" | "lg" => UiModalSize::Large,
        custom if custom.starts_with("custom:") => custom_size(custom),
        _ => UiModalSize::Medium,
    }
}

fn custom_size(value: &str) -> UiModalSize {
    let width_px = value
        .trim_start_matches("custom:")
        .trim_end_matches("px")
        .parse::<u16>()
        .ok()
        .map_or(0, |parsed| parsed);
    UiModalSize::Custom { width_px }
}
