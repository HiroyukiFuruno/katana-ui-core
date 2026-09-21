use super::{Canvas, CanvasBlitRequest};

const BACKGROUND: u32 = 0x000000;

#[test]
fn public_canvas_blit_preserves_same_and_mixed_scale_pixel_contracts() {
    let mut same_scale_source = Canvas::new_scaled(2, 2, 2.0, BACKGROUND);
    same_scale_source.set_physical(0, 0, 0x112233);
    same_scale_source.set_physical(1, 0, 0x445566);
    let mut same_scale_target = Canvas::new_scaled(2, 2, 2.0, BACKGROUND);
    same_scale_target.blit_canvas(
        &same_scale_source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 2,
            height: 2,
            source_y: 0,
        },
    );
    assert_eq!(0x112233, same_scale_target.pixels()[0]);
    assert_eq!(0x445566, same_scale_target.pixels()[1]);

    let mut mixed_scale_source = Canvas::new(2, 2, BACKGROUND);
    mixed_scale_source.set(0, 0, 0x223344);
    mixed_scale_source.set(1, 0, 0x556677);
    mixed_scale_source.set(0, 1, 0x8899aa);
    let mut mixed_scale_target = Canvas::new_scaled(2, 2, 2.0, BACKGROUND);
    mixed_scale_target.blit_canvas(
        &mixed_scale_source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 2,
            height: 2,
            source_y: 0,
        },
    );
    assert_eq!(0x223344, mixed_scale_target.pixels()[0]);
    assert_eq!(0x223344, mixed_scale_target.pixels()[1]);
    assert_eq!(0x556677, mixed_scale_target.pixels()[2]);
    assert_eq!(
        0x8899aa,
        mixed_scale_target.pixels()[2 * mixed_scale_target.width()]
    );
}

#[test]
fn scaled_canvas_blit_keeps_fractional_text_phase_and_skips_clipped_runs() {
    let mut source = Canvas::new_scaled_with_logical_phase(8, 4, 1.25, 0, 1.0, BACKGROUND);
    source.record_text_run("before", 0, 0, 4, 1);
    source.record_text_run("phase", 0, 2, 4, 1);
    let mut target = Canvas::new(8, 2, BACKGROUND);

    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 8,
            height: 2,
            source_y: 2,
        },
    );

    assert_eq!(1, target.text_runs().len());
    assert_eq!("phase", target.text_runs()[0].text());
    assert_eq!(1, target.text_runs()[0].y());
}

#[test]
fn canvas_blit_resamples_equal_scale_when_logical_phase_differs() {
    let mut source = Canvas::new_scaled_with_logical_phase(1, 3, 1.25, 0, 1.0, BACKGROUND);
    for (y, color) in [0x112233, 0x112233, 0x445566, 0x778899]
        .into_iter()
        .enumerate()
    {
        source.set_physical(0, y, color);
    }
    let mut target = Canvas::new_scaled_with_logical_phase(1, 3, 1.25, 0, 0.0, BACKGROUND);

    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 1,
            height: 3,
            source_y: 0,
        },
    );

    assert_eq!(&[0x112233, 0x445566, 0x445566, 0x778899], target.pixels());
}

#[test]
fn scaled_canvas_blit_stops_when_source_or_destination_is_exhausted() {
    let source = Canvas::new(1, 1, BACKGROUND);
    let mut target = Canvas::new_scaled(1, 1, 2.0, BACKGROUND);
    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 2,
            height: 2,
            source_y: 1,
        },
    );
    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 2,
            height: 1,
            source_y: 0,
        },
    );
    assert_eq!(BACKGROUND, target.pixels()[0]);
}

#[test]
fn logical_blend_ignores_invalid_or_off_canvas_rectangles() {
    let mut canvas = Canvas::new(2, 2, BACKGROUND);
    canvas.blend_rect_at_logical_y(0, f32::NAN, 1, 1.0, 0xffffff, 255);
    canvas.blend_rect_at_logical_y(3, 0.0, 1, 1.0, 0xffffff, 255);
    canvas.blend_rect_at_logical_y(0, 2.0, 1, 1.0, 0xffffff, 255);
    let mut fractional = Canvas::new_scaled_with_logical_phase(2, 2, 1.25, 0, 0.0, BACKGROUND);
    fractional.blend_rect_at_logical_y(0, 0.1, 1, 0.1, 0xffffff, 255);
    assert_eq!(&[BACKGROUND; 4], canvas.pixels());
    assert!(fractional.pixels().iter().all(|pixel| *pixel == BACKGROUND));
}
