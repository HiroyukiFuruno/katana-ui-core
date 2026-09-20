use super::{Canvas, CanvasBlitRequest, RgbaBlitRequest, source_has_retina_pixels};
use crate::raster_host::ui_tree_canvas_types::{RgbaSourceRect, UiTreeRenderArea};

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

#[test]
fn canvas_text_and_blit_cover_roles_clipping_and_source_bounds() {
    let mut source = Canvas::new(4, 4, BACKGROUND);
    source.fill_rect(0, 0, 4, 4, 0x123456);
    source.record_text_run("outside", 0, 0, 4, 1);
    let mut target = Canvas::new(4, 4, BACKGROUND);

    target.draw_text(0, 0, "body", 0xffffff);
    target.draw_text_with_role("code", 0, 1, "code", 0xffffff);
    assert!(target.text_width_with_role("body", "body") > 0);
    assert!(target.text_width_with_role("code", "code") > 0);

    target.with_clip(1, 1, 2, 2, &mut |canvas| {
        canvas.blit_canvas(
            &source,
            CanvasBlitRequest {
                dest_x: 0,
                dest_y: 0,
                width: 8,
                height: 8,
                source_y: 2,
            },
        );
    });
    assert!(target.non_background_pixels(BACKGROUND) > 0);
}

#[test]
fn rgba_blit_handles_empty_retina_and_outside_physical_targets() {
    let mut canvas = Canvas::new_scaled(2, 2, 2.0, BACKGROUND);
    let empty = RgbaBlitRequest {
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
    };
    assert!(!source_has_retina_pixels(&empty));
    canvas.blit_rgba(empty);

    let rgba = vec![255, 0, 0, 255, 0, 0, 255, 255];
    let retina = RgbaBlitRequest {
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
    };
    assert!(source_has_retina_pixels(&retina));
    canvas.blit_rgba(retina);
    assert_ne!(BACKGROUND, canvas.pixels()[0]);

    let outside = RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 1,
        source: RgbaSourceRect::full(2, 1),
        area: UiTreeRenderArea {
            x: 8,
            y: 8,
            width: 1,
            height: 1,
            scroll_y: 0.0,
        },
    };
    canvas.blit_rgba(outside);
}

#[test]
fn rgba_blit_stops_when_scaled_source_region_exceeds_source_pixels() {
    let rgba = vec![
        255, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 255,
    ];
    let source = RgbaSourceRect {
        x: 0.0,
        y: 0.0,
        width: 4.0,
        height: 4.0,
    };

    let mut logical = Canvas::new(4, 4, BACKGROUND);
    logical.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 2,
        source,
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 4,
            height: 4,
            scroll_y: 0.0,
        },
    });
    assert_ne!(BACKGROUND, logical.pixels()[0]);
    assert_eq!(BACKGROUND, logical.pixels()[3]);
    assert_eq!(BACKGROUND, logical.pixels()[logical.width() * 2]);

    let mut retina = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);
    retina.blit_rgba(RgbaBlitRequest {
        rgba: &rgba,
        width: 2,
        height: 2,
        source,
        area: UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 4,
            height: 4,
            scroll_y: 0.0,
        },
    });
    assert_ne!(BACKGROUND, retina.pixels()[0]);
    assert_ne!(BACKGROUND, retina.pixels()[3]);
    assert_eq!(BACKGROUND, retina.pixels()[4]);
    assert_eq!(BACKGROUND, retina.pixels()[retina.width() * 4]);
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
