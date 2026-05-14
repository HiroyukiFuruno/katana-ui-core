use super::types::{ModalOpenError, ModalWindowPlacement};
use floem::peniko::kurbo::Point;

pub(super) fn window_position(
    placement: ModalWindowPlacement,
    _width: f64,
    _height: f64,
) -> Result<Option<Point>, ModalOpenError> {
    match placement {
        ModalWindowPlacement::SystemDefault => Ok(None),
        ModalWindowPlacement::At(position) => validate_position(position).map(Some),
        ModalWindowPlacement::SameDisplayAs(_) => {
            Err(ModalOpenError::SameDisplayPlacementUnavailable)
        }
    }
}

fn validate_position(position: Point) -> Result<Point, ModalOpenError> {
    if position.x.is_finite() && position.y.is_finite() {
        return Ok(position);
    }

    Err(ModalOpenError::InvalidWindowPosition {
        x: position.x,
        y: position.y,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_position_accepts_finite_coordinates() {
        let position = Point::new(120.0, 240.0);

        assert_eq!(validate_position(position), Ok(position));
    }

    #[test]
    fn explicit_position_rejects_nan_coordinates() {
        let error = validate_position(Point::new(f64::NAN, 240.0));

        assert!(matches!(
            error,
            Err(ModalOpenError::InvalidWindowPosition { .. })
        ));
    }
}
