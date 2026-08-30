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
