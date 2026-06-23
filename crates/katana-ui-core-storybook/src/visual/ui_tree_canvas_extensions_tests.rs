use super::{Canvas, CanvasBlitRequest, RgbaBlitRequest};
use crate::visual::ui_tree_canvas_types::{RgbaSourceRect, UiTreeRenderArea};

const BACKGROUND: u32 = 0x000000;

#[test]
fn rgba_blit_uses_interpolated_samples_when_scaling_image_surface() {
    let mut canvas = Canvas::new(3, 1, BACKGROUND);
    let rgba = vec![255, 0, 0, 255, 0, 0, 255, 255];

    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 1,
        source: RgbaSourceRect::full(2, 1),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 3,
            height: 1,
            scroll_y: 0.0,
        },
    });

    assert_eq!(0xff0000, canvas.pixels()[0]);
    assert_ne!(0xff0000, canvas.pixels()[1]);
    assert_ne!(0x0000ff, canvas.pixels()[1]);
    assert_eq!(0x800080, canvas.pixels()[1]);
}

#[test]
fn rgba_blit_does_not_darken_transparent_svg_edges() {
    let mut canvas = Canvas::new(3, 1, BACKGROUND);
    let rgba = vec![255, 255, 255, 255, 0, 0, 0, 0];

    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 1,
        source: RgbaSourceRect::full(2, 1),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 3,
            height: 1,
            scroll_y: 0.0,
        },
    });

    assert_eq!(0xffffff, canvas.pixels()[0]);
    assert_eq!(0x7f7f7f, canvas.pixels()[1]);
    assert_eq!(BACKGROUND, canvas.pixels()[2]);
}

#[test]
fn rgba_blit_uses_distinct_source_pixels_on_scaled_canvas() {
    let mut canvas = Canvas::new_scaled(1, 1, 2.0, BACKGROUND);
    let rgba = vec![255, 0, 0, 255, 0, 0, 255, 255];

    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 1,
        source: RgbaSourceRect::full(2, 1),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            scroll_y: 0.0,
        },
    });

    assert_eq!(0xff0000, canvas.pixels()[0]);
    assert_eq!(0x0000ff, canvas.pixels()[1]);
}

#[test]
fn retina_rgba_blit_bottom_clip_preserves_unclipped_source_scale() {
    let mut canvas = Canvas::new_scaled(1, 2, 2.0, BACKGROUND);
    let rgba = vertical_row_rgba(2, 8);

    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 8,
        source: RgbaSourceRect::full(2, 8),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1,
            height: 4,
            scroll_y: 0.0,
        },
    });

    assert_eq!(
        0x3c0000,
        canvas.pixels()[3 * canvas.width()],
        "bottom-clipped retina blit must keep the original target scale and draw the top visible slice"
    );
}

#[test]
fn canvas_blit_preserves_selectable_text_runs() {
    let mut source = Canvas::new(240, 160, BACKGROUND);
    source.record_text_run("Viewer text", 16, 48, 120, 20);
    let mut target = Canvas::new(320, 240, BACKGROUND);

    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 40,
            dest_y: 24,
            width: 200,
            height: 120,
            source_y: 20,
        },
    );

    let run = &target.text_runs()[0];
    assert_eq!(
        Some("Viewer text".to_string()),
        target.copy_text_in_selection(
            Some((run.x(), run.y() + run.height() / 2)),
            Some((run.right(), run.y() + run.height() / 2)),
        )
    );
}

fn vertical_row_rgba(width: u32, height: u32) -> Vec<u8> {
    const ROW_RED_STEP: u32 = 20;
    let mut rgba = Vec::new();
    for y in 0..height {
        for _ in 0..width {
            rgba.push((y * ROW_RED_STEP) as u8);
            rgba.push(0);
            rgba.push(0);
            rgba.push(u8::MAX);
        }
    }
    rgba
}
