use super::keyboard::input_event;
use crate::text_surface::gutter_interaction::gutter_pointer_events;
use crate::text_surface::model::EguiTextSurfaceInputPolicy;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceFrameRecord, TextSurfaceLayout,
    TextSurfaceLayoutAction, TextSurfacePoint,
};

pub(crate) struct TextSurfaceInteraction;

impl TextSurfaceInteraction {
    pub(crate) fn apply_interactions(
        ui: &egui::Ui,
        response: &egui::Response,
        surface: &mut TextSurface,
        layout: &TextSurfaceLayout,
        frame: &TextSurfaceFrameRecord,
        input_policy: &EguiTextSurfaceInputPolicy,
        pending_focus_request: Option<bool>,
    ) -> Vec<TextSurfaceEvent> {
        let accepts_text_input =
            response.has_focus() || ui.ctx().memory(|memory| memory.focused().is_none());
        let mut events = focus_events(surface, response, frame, pending_focus_request);
        events.extend(pointer_events(ui, response, surface, layout, frame));
        if !surface.state().text_area.focused || !accepts_text_input {
            return events;
        }
        for event in ui.input(|input| input.events.clone()) {
            if input_policy.suppresses_event(&event) {
                continue;
            }
            events.extend(input_event(surface, layout, event));
        }
        let scroll = ui.input(|input| input.smooth_scroll_delta());
        if scroll != egui::Vec2::ZERO && (response.hovered() || response.has_focus()) {
            events.extend(
                surface
                    .apply_action(TextSurfaceAction::ScrollBy {
                        delta_x: scroll.x.round() as i32,
                        delta_y: scroll.y.round() as i32,
                    })
                    .events,
            );
        }
        events
    }
}

fn focus_events(
    surface: &mut TextSurface,
    response: &egui::Response,
    frame: &TextSurfaceFrameRecord,
    pending_focus_request: Option<bool>,
) -> Vec<TextSurfaceEvent> {
    if response
        .interact_pointer_pos()
        .map(surface_point)
        .is_some_and(|point| {
            frame
                .gutter
                .iter()
                .any(|gutter| contains(gutter.bounds, point))
        })
    {
        return Vec::new();
    }
    if let Some(focused) = pending_focus_request {
        return set_focus(surface, focused);
    }
    if response.clicked() || response.drag_started() {
        response.request_focus();
        return set_focus(surface, true);
    }
    if response.has_focus() && !surface.state().text_area.focused {
        return set_focus(surface, true);
    }
    if response.lost_focus() && surface.state().text_area.focused {
        return set_focus(surface, false);
    }
    if surface.state().text_area.focused && !response.has_focus() {
        response.request_focus();
    }
    Vec::new()
}

fn set_focus(surface: &mut TextSurface, focused: bool) -> Vec<TextSurfaceEvent> {
    surface
        .apply_action(TextSurfaceAction::SetFocus(focused))
        .events
}

fn pointer_events(
    ui: &egui::Ui,
    response: &egui::Response,
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    frame: &TextSurfaceFrameRecord,
) -> Vec<TextSurfaceEvent> {
    let Some(pointer_position) = response.interact_pointer_pos() else {
        return Vec::new();
    };
    let point = surface_point(pointer_position);
    if secondary_pointer_hit(ui, response, pointer_position) {
        return surface
            .apply_action(TextSurfaceAction::RequestContextTarget {
                selection: frame.selection.range,
            })
            .events;
    }
    if let Some(events) = gutter_pointer_events(surface, frame, point) {
        return events;
    }
    if response.drag_started() || response.clicked() {
        let extend_selection = ui.ctx().input(|input| input.modifiers.shift);
        return surface
            .apply_layout_action(
                layout,
                TextSurfaceLayoutAction::PointerPress {
                    point,
                    extend_selection,
                },
            )
            .events;
    }
    if response.dragged() {
        return surface
            .apply_layout_action(layout, TextSurfaceLayoutAction::PointerDrag { point })
            .events;
    }
    if response.drag_stopped() {
        return surface
            .apply_layout_action(layout, TextSurfaceLayoutAction::PointerRelease)
            .events;
    }
    Vec::new()
}

fn secondary_pointer_hit(
    ui: &egui::Ui,
    response: &egui::Response,
    pointer_position: egui::Pos2,
) -> bool {
    ui.input(|input| {
        let pointer = &input.pointer;
        let secondary = pointer.secondary_clicked()
            || pointer.secondary_pressed()
            || pointer.secondary_released();
        secondary && response.rect.contains(pointer_position)
    })
}

fn contains(bounds: katana_ui_core::render_model::UiRect, point: TextSurfacePoint) -> bool {
    point.x >= bounds.x
        && point.x <= bounds.x.saturating_add(bounds.width as i32)
        && point.y >= bounds.y
        && point.y <= bounds.y.saturating_add(bounds.height as i32)
}

fn surface_point(point: egui::Pos2) -> katana_ui_core::text_surface::TextSurfacePoint {
    TextSurfacePoint::new(point.x.round() as i32, point.y.round() as i32)
}

#[cfg(test)]
#[path = "events_tests.rs"]
mod tests;
