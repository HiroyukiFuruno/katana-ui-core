use crate::text_surface::{TextSurface, TextSurfaceEvent, TextSurfaceFocusRequestResult};

pub(super) fn synchronize_focus_request(
    response: &egui::Response,
    surface: &mut TextSurface,
) -> Option<TextSurfaceFocusRequestResult> {
    let acknowledgement = surface.issue_controlled_focus_request()?;
    if acknowledgement.focused {
        response.request_focus();
    } else {
        response.surrender_focus();
    }
    Some(TextSurfaceFocusRequestResult::Acknowledged(acknowledgement))
}

pub(super) fn focus_request_event(value: &TextSurfaceFocusRequestResult) -> TextSurfaceEvent {
    match value {
        TextSurfaceFocusRequestResult::Acknowledged(value) => {
            TextSurfaceEvent::FocusRequestAcknowledged(value.clone())
        }
    }
}
