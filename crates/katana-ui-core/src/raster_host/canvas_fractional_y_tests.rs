use super::Canvas;

const BACKGROUND: u32 = 0x000000;
const FILL: u32 = 0xffffff;
const BLEND: u32 = 0xff0000;

#[test]
fn fractional_y_fill_preserves_physical_boundaries_and_canvas_clipping() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.fill_rect_at_logical_y(0, 1.5, 4, 1.0, FILL);
    canvas.fill_rect_at_logical_y(0, f32::NAN, 4, 1.0, BLEND);
    canvas.fill_rect_at_logical_y(0, 0.0, 4, 0.0, BLEND);
    canvas.fill_rect_at_logical_y(0, 0.0, 0, 1.0, BLEND);
    canvas.fill_rect_at_logical_y(4, 0.0, 1, 1.0, BLEND);
    canvas.fill_rect_at_logical_y(0, 4.0, 1, 1.0, BLEND);
    canvas.fill_rect_at_logical_y(0, 3.5, 1, 1.0, BLEND);
    canvas.fill_rect_at_logical_y(0, -1.0, 1, 0.5, BLEND);
    canvas.with_clip(1, 2, 2, 1, &mut |canvas| {
        canvas.fill_rect_at_logical_y(0, 2.0, 4, 1.0, BLEND);
    });

    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 0, 2));
    assert_eq!(Some(FILL), pixel_at(&canvas, 0, 3));
    assert_eq!(Some(FILL), pixel_at(&canvas, 0, 4));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 0, 5));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 2, 4));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 5, 5));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 0, 6));
}

#[test]
fn fill_rect_at_logical_y_skips_fully_clipped_negative_rectangles() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.fill_rect_at_logical_y(0, -1.0, 4, 1.0, FILL);

    assert!(canvas.pixels().iter().all(|pixel| *pixel == BACKGROUND));

    canvas.fill_rect_at_logical_y(0, 0.0, 4, 1.0, FILL);
    assert_eq!(Some(FILL), pixel_at(&canvas, 0, 0));
}

#[test]
fn logical_y_clip_skips_invalid_bounds_and_intersects_parent_clip() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);
    let mut invalid_clip_drew = false;

    canvas.with_clip_at_logical_y(0, f32::NAN, 1, 1.0, &mut |_| {
        invalid_clip_drew = true;
    });
    assert!(!invalid_clip_drew);

    canvas.with_clip(1, 1, 2, 2, &mut |canvas| {
        canvas.with_clip_at_logical_y(0, 1.5, 4, 1.0, &mut |canvas| {
            canvas.fill_rect(0, 0, 4, 4, FILL);
        });
    });

    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 1, 3));
    assert_eq!(Some(FILL), pixel_at(&canvas, 2, 3));
    assert_eq!(Some(FILL), pixel_at(&canvas, 5, 4));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 6, 4));
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas
        .pixels()
        .get(y.checked_mul(canvas.width())?.checked_add(x)?)
        .copied()
}
