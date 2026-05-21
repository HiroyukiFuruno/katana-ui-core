use super::Canvas;

const BACKGROUND: u32 = 0x000000;
const FILL: u32 = 0xffffff;
const BLEND: u32 = 0xff0000;

#[test]
fn clip_prevents_children_from_painting_outside_parent_bounds() {
    let mut canvas = Canvas::new(12, 8, BACKGROUND);

    canvas.with_clip(3, 2, 5, 4, |canvas| {
        canvas.fill_rect(0, 0, 12, 8, FILL);
        canvas.set(1, 1, FILL);
    });

    assert_eq!(Some(FILL), pixel_at(&canvas, 3, 2));
    assert_eq!(Some(FILL), pixel_at(&canvas, 7, 5));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 2, 2));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 8, 5));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 1, 1));
}

#[test]
fn nested_clips_use_the_intersection_of_parent_and_child_bounds() {
    let mut canvas = Canvas::new(12, 8, BACKGROUND);

    canvas.with_clip(2, 1, 7, 5, |canvas| {
        canvas.with_clip(5, 3, 6, 4, |canvas| {
            canvas.fill_rect(0, 0, 12, 8, FILL);
        });
    });

    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 4, 3));
    assert_eq!(Some(FILL), pixel_at(&canvas, 5, 3));
    assert_eq!(Some(FILL), pixel_at(&canvas, 8, 5));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 9, 5));
}

#[test]
fn clip_applies_to_alpha_blending() {
    let mut canvas = Canvas::new(6, 4, BACKGROUND);

    canvas.with_clip(2, 1, 2, 2, |canvas| {
        canvas.blend_rect(0, 0, 6, 4, BLEND, 255);
    });

    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 1, 1));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 2, 1));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 3, 2));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 4, 2));
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
