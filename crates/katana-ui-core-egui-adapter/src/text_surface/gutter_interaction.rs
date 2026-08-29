use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceFrameRecord, TextSurfacePoint,
};

pub(super) fn gutter_pointer_events(
    surface: &mut TextSurface,
    frame: &TextSurfaceFrameRecord,
    point: TextSurfacePoint,
) -> Option<Vec<TextSurfaceEvent>> {
    let gutter = frame
        .gutter
        .iter()
        .find(|gutter| contains(gutter.bounds, point))?;
    let action = match (&gutter.marker_id, gutter.marker_bounds) {
        (Some(marker_id), Some(bounds)) if contains(bounds, point) => {
            TextSurfaceAction::ActivateGutterMarker {
                logical_row: gutter.logical_row,
                marker_id: marker_id.clone(),
            }
        }
        (Some(marker_id), None) => TextSurfaceAction::ActivateGutterMarker {
            logical_row: gutter.logical_row,
            marker_id: marker_id.clone(),
        },
        _ => TextSurfaceAction::ActivateGutterRow {
            logical_row: gutter.logical_row,
        },
    };
    Some(surface.apply_action(action).events)
}

fn contains(bounds: katana_ui_core::render_model::UiRect, point: TextSurfacePoint) -> bool {
    point.x >= bounds.x
        && point.x <= bounds.x.saturating_add(bounds.width as i32)
        && point.y >= bounds.y
        && point.y <= bounds.y.saturating_add(bounds.height as i32)
}
