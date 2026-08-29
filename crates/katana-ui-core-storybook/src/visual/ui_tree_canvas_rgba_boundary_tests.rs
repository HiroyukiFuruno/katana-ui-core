use super::{Canvas, CanvasBlitRequest, RgbaBlitRequest};
use crate::visual::ui_tree_canvas_types::{RgbaSourceRect, UiTreeRenderArea};

const BACKGROUND: u32 = 0x000000;

#[test]
fn rgba_blit_covers_empty_logical_clipped_and_alpha_samples() {
    let mut canvas = Canvas::new(4, 2, 0x202020);
    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &[],
        width: 0,
        height: 0,
        source: RgbaSourceRect::full(0, 0),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            scroll_y: 0.0,
        },
    });

    let rgba = vec![
        255, 0, 0, 255, 0, 255, 0, 128, 0, 0, 255, 0, 255, 255, 255, 255,
    ];
    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 4,
        height: 1,
        source: RgbaSourceRect::full(4, 1),
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
            scroll_y: 0.0,
        },
    });
    assert_eq!(0xff0000, canvas.pixels()[0]);
    assert_eq!(0x202020, canvas.pixels()[2]);

    canvas.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 4,
        height: 1,
        source: RgbaSourceRect::full(4, 1),
        area: UiTreeRenderArea {
            x: 100,
            y: 100,
            width: 2,
            height: 1,
            scroll_y: 0.0,
        },
    });
}

#[test]
fn canvas_and_rgba_blits_stop_at_every_source_and_destination_boundary() {
    let source = Canvas::new(8, 1, 0x123456);
    let mut target = Canvas::new(4, 1, BACKGROUND);
    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 3,
            dest_y: 0,
            width: 8,
            height: 1,
            source_y: 0,
        },
    );
    assert_eq!(0x123456, target.pixels()[3]);

    let logical_rgba = vec![255; 2 * 2 * 4];
    for source_rect in [
        RgbaSourceRect {
            x: 0.0,
            y: 2.0,
            width: 2.0,
            height: 2.0,
        },
        RgbaSourceRect {
            x: 2.0,
            y: 0.0,
            width: 2.0,
            height: 2.0,
        },
    ] {
        target.blit_rgba(RgbaBlitRequest {
            rgba: &logical_rgba,
            width: 2,
            height: 2,
            source: source_rect,
            area: UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
                scroll_y: 0.0,
            },
        });
    }

    let retina_rgba = vec![255; 4 * 4 * 4];
    for source_rect in [
        RgbaSourceRect {
            x: 0.0,
            y: 4.0,
            width: 4.0,
            height: 4.0,
        },
        RgbaSourceRect {
            x: 4.0,
            y: 0.0,
            width: 4.0,
            height: 4.0,
        },
    ] {
        target.blit_rgba(RgbaBlitRequest {
            rgba: &retina_rgba,
            width: 4,
            height: 4,
            source: source_rect,
            area: UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
                scroll_y: 0.0,
            },
        });
    }

    let transparent_retina = vec![0; 2 * 2 * 4];
    target.blit_rgba(RgbaBlitRequest {
        rgba: &transparent_retina,
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
}
