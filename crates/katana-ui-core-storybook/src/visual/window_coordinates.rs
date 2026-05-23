#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct WindowPoint {
    pub(super) x: f32,
    pub(super) y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CanvasPoint {
    pub(super) x: usize,
    pub(super) y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SurfaceSize {
    pub(super) width: usize,
    pub(super) height: usize,
}

impl WindowPoint {
    pub(super) const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl SurfaceSize {
    pub(super) const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

pub(super) fn window_point_to_canvas_point(
    point: WindowPoint,
    window: SurfaceSize,
    canvas: SurfaceSize,
) -> Option<CanvasPoint> {
    if window.width == 0 || window.height == 0 || canvas.width == 0 || canvas.height == 0 {
        return None;
    }
    let rect = rendered_canvas_rect(window, canvas);
    if point.x < rect.x
        || point.y < rect.y
        || point.x >= rect.x + rect.width
        || point.y >= rect.y + rect.height
    {
        return None;
    }
    let x = ((point.x - rect.x) * canvas.width as f32 / rect.width).floor() as usize;
    let y = ((point.y - rect.y) * canvas.height as f32 / rect.height).floor() as usize;
    Some(CanvasPoint {
        x: x.min(canvas.width - 1),
        y: y.min(canvas.height - 1),
    })
}

fn rendered_canvas_rect(window: SurfaceSize, canvas: SurfaceSize) -> RenderedCanvasRect {
    let window_width = window.width as f32;
    let window_height = window.height as f32;
    let canvas_aspect = canvas.width as f32 / canvas.height as f32;
    let window_aspect = window_width / window_height;
    if canvas_aspect > window_aspect {
        let height = window_width / canvas_aspect;
        return RenderedCanvasRect {
            x: 0.0,
            y: (window_height - height) / 2.0,
            width: window_width,
            height,
        };
    }
    let width = window_height * canvas_aspect;
    RenderedCanvasRect {
        x: (window_width - width) / 2.0,
        y: 0.0,
        width,
        height: window_height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RenderedCanvasRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[cfg(test)]
mod tests {
    use super::{
        CanvasPoint, SurfaceSize, WindowPoint, rendered_canvas_rect, window_point_to_canvas_point,
    };

    const RETINA_WINDOW_WIDTH: usize = 2879;
    const RETINA_WINDOW_HEIGHT: usize = 1728;
    const CANVAS_WIDTH: usize = 1440;
    const CANVAS_HEIGHT: usize = 920;

    #[test]
    fn scaled_window_point_maps_back_to_canvas_point() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(2160, 1380);
        let point = WindowPoint::new(465.0, 156.0);

        assert_eq!(
            Some(CanvasPoint { x: 310, y: 104 }),
            window_point_to_canvas_point(point, window, canvas)
        );
    }

    #[test]
    fn non_integer_scale_mapping_keeps_canvas_point_center() {
        let canvas = SurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
        let window = SurfaceSize::new(RETINA_WINDOW_WIDTH, RETINA_WINDOW_HEIGHT);
        let rect = rendered_canvas_rect(window, canvas);

        let sample = CanvasPoint { x: 310, y: 120 };
        let point = WindowPoint::new(
            rect.x + (sample.x as f32 + 0.5) * rect.width / canvas.width as f32,
            rect.y + (sample.y as f32 + 0.5) * rect.height / canvas.height as f32,
        );

        assert_eq!(
            Some(sample),
            window_point_to_canvas_point(point, window, canvas),
        );
    }

    #[test]
    fn non_integer_scale_mapping_keeps_canvas_bottom_row_center() {
        let canvas = SurfaceSize::new(CANVAS_WIDTH, CANVAS_HEIGHT);
        let window = SurfaceSize::new(RETINA_WINDOW_WIDTH, RETINA_WINDOW_HEIGHT);
        let rect = rendered_canvas_rect(window, canvas);

        let sample = CanvasPoint {
            x: CANVAS_WIDTH - 1,
            y: CANVAS_HEIGHT - 1,
        };
        let point = WindowPoint::new(rect.x + rect.width - 0.5, rect.y + rect.height - 0.5);

        assert_eq!(
            Some(sample),
            window_point_to_canvas_point(point, window, canvas),
        );
    }

    #[test]
    fn mouse_point_and_window_surface_must_share_the_same_coordinate_space() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(2160, 1380);
        let surface_space_point = WindowPoint::new(465.0, 156.0);
        let scaled_mouse_point = WindowPoint::new(310.0, 104.0);

        assert_eq!(
            Some(CanvasPoint { x: 310, y: 104 }),
            window_point_to_canvas_point(surface_space_point, window, canvas)
        );
        assert_ne!(
            Some(CanvasPoint { x: 310, y: 104 }),
            window_point_to_canvas_point(scaled_mouse_point, window, canvas)
        );
    }

    #[test]
    fn letterboxed_window_point_removes_margin_before_mapping() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(2000, 1200);
        let point = WindowPoint::new(465.3, 135.7);

        assert_eq!(
            Some(CanvasPoint { x: 310, y: 104 }),
            window_point_to_canvas_point(point, window, canvas)
        );
    }

    #[test]
    fn point_outside_rendered_canvas_returns_none() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(2000, 1200);
        let point = WindowPoint::new(20.0, 100.0);

        assert_eq!(None, window_point_to_canvas_point(point, window, canvas));
    }
}
