use super::{Canvas, RgbaBlitRequest};
use crate::raster_host::ui_tree_canvas_types::{RgbaSourceRect, UiTreeRenderArea};

const BACKGROUND: u32 = 0x000000;

#[test]
fn retina_blit_stops_at_physical_source_bounds_and_ignores_fully_transparent_samples() {
    let opaque = [255, 0, 0, 255].repeat(16);
    let mut clipped = Canvas::new_scaled(1, 1, 2.0, BACKGROUND);
    clipped.blit_rgba(RgbaBlitRequest {
        rgba: &opaque,
        width: 4,
        height: 4,
        source: RgbaSourceRect::full(8, 8),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            scroll_y: 0.0,
        },
    });
    assert_ne!(BACKGROUND, clipped.pixels()[0]);
    assert_eq!(BACKGROUND, clipped.pixels()[1]);
    assert_eq!(BACKGROUND, clipped.pixels()[clipped.width()]);

    let transparent = [255, 0, 0, 0].repeat(4);
    let mut transparent_target = Canvas::new_scaled(1, 1, 2.0, BACKGROUND);
    transparent_target.blit_rgba(RgbaBlitRequest {
        rgba: &transparent,
        width: 2,
        height: 2,
        source: RgbaSourceRect::full(2, 2),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            scroll_y: 0.0,
        },
    });
    assert!(
        transparent_target
            .pixels()
            .iter()
            .all(|pixel| *pixel == BACKGROUND)
    );
}
