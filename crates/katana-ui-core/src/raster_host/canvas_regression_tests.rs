use super::canvas_clip::CanvasClip;
use super::{Canvas, CanvasBlitRequest};

const BACKGROUND: u32 = 0x000000;
const FILL: u32 = 0xffffff;
const BLEND: u32 = 0xff0000;
const ROW0: u32 = 0x111111;
const ROW1: u32 = 0x222222;
const ROW2: u32 = 0x333333;
const ROW3: u32 = 0x444444;

#[test]
fn canvas_edge_contracts_cover_empty_clip_blit_selection_and_viewport() {
    assert!(CanvasClip::from_rect(4, 4, 0, 0, 4, 4).is_none());

    let source = Canvas::new(2, 2, FILL);
    let mut target = Canvas::new(2, 2, BACKGROUND);
    let request = CanvasBlitRequest {
        dest_x: 0,
        dest_y: 3,
        width: 2,
        height: 1,
        source_y: 0,
    };
    assert!(target.copy_unclipped_canvas_row(&source, request, 0, 0));
    let zero_width = CanvasBlitRequest {
        dest_y: 0,
        width: 0,
        ..request
    };
    assert!(target.copy_unclipped_canvas_row(&source, zero_width, 0, 0));

    assert_eq!(
        None,
        target.copy_text_in_selection(Some((0, 0)), Some((1, 1)))
    );
    target.record_text_run("", 0, 0, 1, 1);
    assert!(target.text_runs().is_empty());

    let empty = Canvas::new(0, 2, BACKGROUND);
    let viewport = empty.viewport_y(0, 1, FILL);
    assert_eq!(0, viewport.width());
    assert_eq!(1, viewport.logical_height());
}

#[test]
fn clip_prevents_children_from_painting_outside_parent_bounds() {
    let mut canvas = Canvas::new(12, 8, BACKGROUND);

    canvas.with_clip(3, 2, 5, 4, &mut |canvas| {
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

    canvas.with_clip(2, 1, 7, 5, &mut |canvas| {
        canvas.with_clip(5, 3, 6, 4, &mut |canvas| {
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

    canvas.with_clip(2, 1, 2, 2, &mut |canvas| {
        canvas.blend_rect(0, 0, 6, 4, BLEND, 255);
    });

    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 1, 1));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 2, 1));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 3, 2));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 4, 2));
}

#[test]
fn logical_fill_rect_and_set_are_scaled_on_high_dpi_canvas() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.fill_rect(1, 1, 1, 1, FILL);
    canvas.set(0, 0, FILL);

    assert_eq!(Some(FILL), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(FILL), pixel_at(&canvas, 1, 0));
    assert_eq!(Some(FILL), pixel_at(&canvas, 0, 1));
    assert_eq!(Some(FILL), pixel_at(&canvas, 1, 1));
    assert_eq!(Some(FILL), pixel_at(&canvas, 2, 2));
    assert_eq!(Some(FILL), pixel_at(&canvas, 3, 3));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 4, 3));
}

#[test]
fn logical_blend_paints_full_logical_pixel_on_high_dpi_canvas() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.blend(0, 0, BLEND, 255);

    assert_eq!(Some(BLEND), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 1, 0));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 0, 1));
    assert_eq!(Some(BLEND), pixel_at(&canvas, 1, 1));
}

#[test]
fn point_and_rect_drawing_ignore_coordinates_outside_the_canvas() {
    let mut canvas = Canvas::new(2, 2, BACKGROUND);

    canvas.fill_rect(2, 0, 1, 1, FILL);
    canvas.set(2, 0, FILL);
    canvas.set(0, 2, FILL);
    canvas.blend(2, 0, BLEND, 255);
    canvas.blend(0, 2, BLEND, 255);
    canvas.blend_rect(3, 3, 1, 1, BLEND, 255);

    assert!(canvas.pixels().iter().all(|pixel| *pixel == BACKGROUND));
    assert_eq!(None, canvas.physical_span_x(2));
    assert_eq!(None, canvas.physical_span_y(2));
    assert_eq!(None, canvas.to_physical_clip(2, 2, 1, 1));
}

#[test]
fn clips_skip_empty_and_disjoint_regions_without_leaking_clip_state() {
    let mut canvas = Canvas::new(4, 4, BACKGROUND);

    canvas.with_clip(0, 0, 0, 1, &mut |canvas| canvas.fill_rect(0, 0, 4, 4, FILL));
    canvas.with_clip(0, 0, 1, 0, &mut |canvas| canvas.fill_rect(0, 0, 4, 4, FILL));
    canvas.with_clip(4, 0, 1, 1, &mut |canvas| canvas.fill_rect(0, 0, 4, 4, FILL));
    canvas.with_clip(0, 0, 1, 1, &mut |canvas| {
        canvas.with_clip(3, 3, 1, 1, &mut |canvas| canvas.fill_rect(0, 0, 4, 4, FILL));
    });
    canvas.fill_rect(0, 0, 1, 1, FILL);

    assert_eq!(Some(FILL), pixel_at(&canvas, 0, 0));
    assert!(
        canvas
            .pixels()
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != 0)
            .all(|(_, pixel)| *pixel == BACKGROUND)
    );
}

#[test]
fn zero_sized_stroke_is_a_noop() {
    let mut canvas = Canvas::new(2, 2, BACKGROUND);

    canvas.stroke_rect(0, 0, 0, 2, FILL);
    canvas.stroke_rect(0, 0, 2, 0, FILL);

    assert!(canvas.pixels().iter().all(|pixel| *pixel == BACKGROUND));
}

#[test]
fn logical_stroke_rect_uses_logical_border_width() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.stroke_rect(1, 1, 2, 2, FILL);

    assert_eq!(Some(FILL), pixel_at(&canvas, 2, 2));
    assert_eq!(Some(FILL), pixel_at(&canvas, 5, 5));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 6, 6));
}

#[test]
fn with_clip_keeps_logical_coordinates_for_scaled_canvas() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.with_clip(1, 1, 1, 1, &mut |canvas| {
        canvas.fill_rect(0, 0, 4, 4, FILL);
    });

    assert_eq!(Some(FILL), pixel_at(&canvas, 2, 2));
    assert_eq!(Some(FILL), pixel_at(&canvas, 3, 3));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 1, 2));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 4, 3));
}

#[test]
fn with_clip_limits_round_rect_draw_to_clip_area_on_scaled_canvas() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);

    canvas.with_clip(1, 1, 2, 2, &mut |canvas| {
        canvas.fill_round_rect(0, 0, 4, 4, 0, FILL);
    });

    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 1, 1));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 7, 7));
    assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(FILL), pixel_at(&canvas, 2, 2));
}

#[test]
fn scroll_rect_vertically_moves_scaled_logical_rows() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);
    fill_physical_row(&mut canvas, 0, ROW0);
    fill_physical_row(&mut canvas, 1, ROW0);
    fill_physical_row(&mut canvas, 2, ROW1);
    fill_physical_row(&mut canvas, 3, ROW1);
    fill_physical_row(&mut canvas, 4, ROW2);
    fill_physical_row(&mut canvas, 5, ROW2);

    assert!(canvas.scroll_rect_vertically(0, 0, 4, 3, -1));

    assert_eq!(Some(ROW1), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(ROW1), pixel_at(&canvas, 7, 1));
    assert_eq!(Some(ROW2), pixel_at(&canvas, 0, 2));
    assert_eq!(Some(ROW2), pixel_at(&canvas, 7, 3));
}

#[test]
fn scroll_rect_vertically_moves_down_without_overwriting_source_rows() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);
    fill_physical_row(&mut canvas, 0, ROW0);
    fill_physical_row(&mut canvas, 1, ROW0);
    fill_physical_row(&mut canvas, 2, ROW1);
    fill_physical_row(&mut canvas, 3, ROW1);
    fill_physical_row(&mut canvas, 4, ROW2);
    fill_physical_row(&mut canvas, 5, ROW2);

    assert!(canvas.scroll_rect_vertically(0, 0, 4, 3, 1));

    assert_eq!(Some(ROW0), pixel_at(&canvas, 0, 2));
    assert_eq!(Some(ROW0), pixel_at(&canvas, 7, 3));
    assert_eq!(Some(ROW1), pixel_at(&canvas, 0, 4));
    assert_eq!(Some(ROW1), pixel_at(&canvas, 7, 5));
}

#[test]
fn viewport_y_does_not_duplicate_physical_rows_on_scaled_canvas() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);
    let width = canvas.width();
    for x in 0..width {
        canvas.set_physical(x, 0, ROW0);
        canvas.set_physical(x, 1, ROW1);
        canvas.set_physical(x, 2, ROW2);
        canvas.set_physical(x, 3, ROW3);
    }

    let viewport = canvas.viewport_y(0, 2, BACKGROUND);

    assert_eq!(8, viewport.width());
    assert_eq!(4, viewport.height());
    assert_eq!(Some(ROW0), pixel_at(&viewport, 0, 0));
    assert_eq!(Some(ROW1), pixel_at(&viewport, 0, 1));
    assert_eq!(Some(ROW2), pixel_at(&viewport, 0, 2));
    assert_eq!(Some(ROW3), pixel_at(&viewport, 0, 3));
    assert_eq!(Some(ROW0), pixel_at(&viewport, 7, 0));
    assert_eq!(Some(ROW3), pixel_at(&viewport, 7, 3));
}

#[test]
fn viewport_y_maps_logical_rows_with_physical_canvas() {
    let mut canvas = Canvas::new_scaled(4, 4, 2.0, BACKGROUND);
    canvas.fill_rect(0, 0, 4, 1, FILL);

    let viewport = canvas.viewport_y(2, 1, BACKGROUND);

    assert_eq!(8, viewport.width());
    assert_eq!(2, viewport.height());
    for x in 0..viewport.width() {
        assert_eq!(Some(BACKGROUND), pixel_at(&viewport, x, 0));
        assert_eq!(Some(BACKGROUND), pixel_at(&viewport, x, 1));
    }
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn fill_physical_row(canvas: &mut Canvas, y: usize, color: u32) {
    for x in 0..canvas.width() {
        canvas.set_physical(x, y, color);
    }
}
