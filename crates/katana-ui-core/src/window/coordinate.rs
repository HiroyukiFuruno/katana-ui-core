use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowSurfacePoint {
    pub x: f32,
    pub y: f32,
}

impl WindowSurfacePoint {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowSurfaceSize {
    pub width: usize,
    pub height: usize,
}

impl WindowSurfaceSize {
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowCanvasPoint {
    pub x: usize,
    pub y: usize,
}

pub struct WindowInputNormalizer;

impl WindowInputNormalizer {
    #[must_use]
    pub fn canvas_point_for_surface_point(
        point: WindowSurfacePoint,
        surface: WindowSurfaceSize,
        canvas: WindowSurfaceSize,
    ) -> Option<WindowCanvasPoint> {
        if surface.width == 0 || surface.height == 0 || canvas.width == 0 || canvas.height == 0 {
            return None;
        }
        let rect = rendered_canvas_rect(surface, canvas);
        if point.x < rect.x
            || point.y < rect.y
            || point.x >= rect.x + rect.width
            || point.y >= rect.y + rect.height
        {
            return None;
        }
        let x = ((point.x - rect.x) * canvas.width as f32 / rect.width).floor() as usize;
        let y = ((point.y - rect.y) * canvas.height as f32 / rect.height).floor() as usize;
        Some(WindowCanvasPoint {
            x: x.min(canvas.width - 1),
            y: y.min(canvas.height - 1),
        })
    }
}

fn rendered_canvas_rect(
    surface: WindowSurfaceSize,
    canvas: WindowSurfaceSize,
) -> RenderedCanvasRect {
    let surface_width = surface.width as f32;
    let surface_height = surface.height as f32;
    let canvas_aspect = canvas.width as f32 / canvas.height as f32;
    let surface_aspect = surface_width / surface_height;
    if canvas_aspect > surface_aspect {
        let height = surface_width / canvas_aspect;
        return RenderedCanvasRect {
            x: 0.0,
            y: (surface_height - height) / 2.0,
            width: surface_width,
            height,
        };
    }
    let width = surface_height * canvas_aspect;
    RenderedCanvasRect {
        x: (surface_width - width) / 2.0,
        y: 0.0,
        width,
        height: surface_height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RenderedCanvasRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
