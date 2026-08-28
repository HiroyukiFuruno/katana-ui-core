use super::ArtifactCompositeError;
use katana_ui_core::render_model::UiRect;

pub(super) fn validate_canvas(canvas: UiRect) -> Result<(), ArtifactCompositeError> {
    if canvas.width == 0 || canvas.height == 0 {
        return Err(ArtifactCompositeError::ZeroCanvas);
    }
    rect_edges(canvas)?;
    Ok(())
}

pub(super) fn clip_rect(
    canvas: UiRect,
    surface: UiRect,
    clip: UiRect,
) -> Result<Option<UiRect>, ArtifactCompositeError> {
    intersect(canvas, surface)?.map_or(Ok(None), |visible| intersect(visible, clip))
}

pub(super) fn intersect(
    left: UiRect,
    right: UiRect,
) -> Result<Option<UiRect>, ArtifactCompositeError> {
    let (left_right, left_bottom) = rect_edges(left)?;
    let (right_right, right_bottom) = rect_edges(right)?;
    let x = left.x.max(right.x);
    let y = left.y.max(right.y);
    let end_x = left_right.min(right_right);
    let end_y = left_bottom.min(right_bottom);
    if end_x <= x || end_y <= y {
        return Ok(None);
    }
    Ok(Some(UiRect::new(
        x,
        y,
        (end_x - x) as u32,
        (end_y - y) as u32,
    )))
}

pub(super) fn rect_edges(rect: UiRect) -> Result<(i32, i32), ArtifactCompositeError> {
    Ok((
        rect.x
            .checked_add_unsigned(rect.width)
            .ok_or(ArtifactCompositeError::Overflow {
                context: "computing rectangle right edge",
            })?,
        rect.y
            .checked_add_unsigned(rect.height)
            .ok_or(ArtifactCompositeError::Overflow {
                context: "computing rectangle bottom edge",
            })?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn validate_canvas_accepts_positive_size_and_rejects_zero() {
        let canvas = UiRect::new(0, 0, 1, 1);
        assert!(validate_canvas(canvas).is_ok());
        assert!(matches!(
            validate_canvas(UiRect::new(0, 0, 0, 1)),
            Err(ArtifactCompositeError::ZeroCanvas)
        ));
        assert!(matches!(
            validate_canvas(UiRect::new(0, 0, 1, 0)),
            Err(ArtifactCompositeError::ZeroCanvas)
        ));
    }

    #[test]
    fn clip_and_intersect_cover_disjoint_and_nested_rectangles() {
        let canvas = UiRect::new(0, 0, 10, 10);
        let surface = UiRect::new(1, 1, 4, 4);
        let clip = UiRect::new(2, 2, 4, 4);

        assert_eq!(
            clip_rect(canvas, surface, clip),
            Ok(Some(UiRect::new(2, 2, 3, 3)))
        );
        assert_eq!(
            clip_rect(canvas, surface, UiRect::new(20, 20, 1, 1)),
            Ok(None)
        );
    }

    #[test]
    fn intersections_fail_on_unordered_coordinates_and_overflow_edges() {
        let left = UiRect::new(i32::MAX, 0, 1, 1);
        let right = UiRect::new(0, 0, 1, 1);
        assert!(matches!(
            intersect(left, right),
            Err(ArtifactCompositeError::Overflow { .. })
        ));

        let overflow = UiRect::new(i32::MAX, 0, u32::MAX, 1);
        assert!(matches!(
            rect_edges(overflow),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
    }

    #[test]
    fn intersect_returns_none_for_edges_that_only_touch() {
        let left = UiRect::new(0, 0, 1, 1);
        let right = UiRect::new(1, 1, 1, 1);
        assert_eq!(intersect(left, right), Ok(None));
    }
}
