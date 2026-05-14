use super::types::ModalWindowPlacement;
use floem::WindowIdExt;
use floem::peniko::kurbo::{Point, Rect};

pub(super) fn window_position(
    placement: ModalWindowPlacement,
    width: f64,
    height: f64,
) -> Option<Point> {
    match placement {
        ModalWindowPlacement::SystemDefault => None,
        ModalWindowPlacement::At(position) => Some(position),
        ModalWindowPlacement::SameDisplayAs(parent) => {
            let parent_bounds = parent
                .bounds_on_screen_including_frame()
                .or_else(|| parent.bounds_of_content_on_screen())?;
            Some(same_display_position(parent_bounds, width, height))
        }
    }
}

fn same_display_position(parent_bounds: Rect, width: f64, height: f64) -> Point {
    Point::new(
        centered_axis(parent_bounds.x0, parent_bounds.x1, width),
        centered_axis(parent_bounds.y0, parent_bounds.y1, height),
    )
}

fn centered_axis(parent_start: f64, parent_end: f64, size: f64) -> f64 {
    parent_start + ((parent_end - parent_start - size) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_display_position_centers_inside_parent_window() {
        let parent = Rect::new(100.0, 80.0, 900.0, 680.0);

        let position = same_display_position(parent, 400.0, 240.0);

        assert_eq!(position, Point::new(300.0, 260.0));
    }

    #[test]
    fn same_display_position_preserves_secondary_display_offset() {
        let parent = Rect::new(1800.0, 100.0, 2600.0, 700.0);

        let position = same_display_position(parent, 400.0, 240.0);

        assert_eq!(position, Point::new(2000.0, 280.0));
    }
}
