use super::super::gutter_interaction::gutter_pointer_events;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceFrameRecord, TextSurfaceLayout,
    TextSurfaceLayoutAction, TextSurfacePoint,
};

pub(super) fn focus_events(
    ui: &egui::Ui,
    surface: &mut TextSurface,
    response: &egui::Response,
    frame: &TextSurfaceFrameRecord,
    pending_focus_request: Option<bool>,
    retain_pointer_focus: bool,
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
    let physical_primary_input = retain_pointer_focus
        && response.contains_pointer()
        && ui.input(|input| {
            input.pointer.button_pressed(egui::PointerButton::Primary)
                || input.pointer.button_released(egui::PointerButton::Primary)
        });
    if response.is_pointer_button_down_on()
        || response.clicked()
        || response.drag_started()
        || physical_primary_input
    {
        response.request_focus();
        return set_focus(surface, true);
    }
    if response.has_focus() && !surface.state().text_area.focused {
        return set_focus(surface, true);
    }
    if response.lost_focus() && surface.state().text_area.focused {
        return set_focus(surface, false);
    }
    if surface.state().text_area.focused
        && !response.has_focus()
        && ui.ctx().memory(|memory| {
            memory
                .focused()
                .is_none_or(|focused| focused == response.id)
        })
    {
        response.request_focus();
    }
    Vec::new()
}

fn set_focus(surface: &mut TextSurface, focused: bool) -> Vec<TextSurfaceEvent> {
    surface
        .apply_action(TextSurfaceAction::SetFocus(focused))
        .events
}

pub(super) fn pointer_events(
    ui: &egui::Ui,
    response: &egui::Response,
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    frame: &TextSurfaceFrameRecord,
) -> Vec<TextSurfaceEvent> {
    let Some(point) = response.interact_pointer_pos().map(surface_point) else {
        return Vec::new();
    };
    if secondary_pointer_hit(ui, response).is_some() {
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

fn secondary_pointer_hit(ui: &egui::Ui, response: &egui::Response) -> Option<egui::Pos2> {
    ui.input(|input| {
        let pointer = &input.pointer;
        let position = pointer.interact_pos().or_else(|| pointer.latest_pos())?;
        ((pointer.secondary_clicked()
            || pointer.secondary_pressed()
            || pointer.secondary_released())
            && response.rect.contains(position))
        .then_some(position)
    })
}

fn contains(bounds: katana_ui_core::render_model::UiRect, point: TextSurfacePoint) -> bool {
    point.x >= bounds.x
        && point.x <= bounds.x.saturating_add(bounds.width as i32)
        && point.y >= bounds.y
        && point.y <= bounds.y.saturating_add(bounds.height as i32)
}

fn surface_point(point: egui::Pos2) -> TextSurfacePoint {
    TextSurfacePoint::new(point.x.round() as i32, point.y.round() as i32)
}
