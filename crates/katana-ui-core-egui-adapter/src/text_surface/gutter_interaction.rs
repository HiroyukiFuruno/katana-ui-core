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
    let action = gutter_action(
        gutter.marker_id.as_deref(),
        gutter.marker_bounds,
        gutter.logical_row,
        point,
    );
    Some(surface.apply_action(action).events)
}

fn gutter_action(
    marker_id: Option<&str>,
    marker_bounds: Option<katana_ui_core::render_model::UiRect>,
    logical_row: usize,
    point: TextSurfacePoint,
) -> TextSurfaceAction {
    match (marker_id, marker_bounds) {
        (Some(marker_id), Some(bounds)) if contains(bounds, point) => {
            TextSurfaceAction::ActivateGutterMarker {
                logical_row,
                marker_id: marker_id.to_owned(),
            }
        }
        (Some(marker_id), None) => TextSurfaceAction::ActivateGutterMarker {
            logical_row,
            marker_id: marker_id.to_owned(),
        },
        _ => TextSurfaceAction::ActivateGutterRow { logical_row },
    }
}

fn contains(bounds: katana_ui_core::render_model::UiRect, point: TextSurfacePoint) -> bool {
    point.x >= bounds.x
        && point.x <= bounds.x.saturating_add(bounds.width as i32)
        && point.y >= bounds.y
        && point.y <= bounds.y.saturating_add(bounds.height as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn legacy_marker_without_dedicated_bounds_uses_the_row_hit_area() {
        assert!(matches!(
            gutter_action(
                Some("marker"),
                None,
                7,
                TextSurfacePoint { x: 10, y: 10 }
            ),
            TextSurfaceAction::ActivateGutterMarker {
                logical_row: 7,
                marker_id
            } if marker_id == "marker"
        ));
        assert!(matches!(
            gutter_action(
                Some("marker"),
                Some(UiRect::new(5, 5, 10, 10)),
                7,
                TextSurfacePoint { x: 10, y: 10 }
            ),
            TextSurfaceAction::ActivateGutterMarker {
                logical_row: 7,
                marker_id
            } if marker_id == "marker"
        ));
        assert!(contains(
            UiRect::new(5, 5, 10, 10),
            TextSurfacePoint { x: 5, y: 5 }
        ));
        assert!(!contains(
            UiRect::new(5, 5, 10, 10),
            TextSurfacePoint { x: 4, y: 4 }
        ));
        assert!(matches!(
            gutter_action(
                Some("marker"),
                Some(UiRect::new(20, 20, 5, 5)),
                7,
                TextSurfacePoint { x: 10, y: 10 }
            ),
            TextSurfaceAction::ActivateGutterRow { logical_row: 7 }
        ));
    }
}
