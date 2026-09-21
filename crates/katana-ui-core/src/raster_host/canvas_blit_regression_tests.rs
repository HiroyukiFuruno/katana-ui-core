use super::{Canvas, CanvasBlitRequest};

const BACKGROUND: u32 = 0x000000;

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
fn public_canvas_blit_resamples_logical_pixels_across_scales() {
    let mut source = Canvas::new(2, 2, BACKGROUND);
    source.set(0, 0, 0x112233);
    source.set(1, 0, 0x445566);
    source.set(0, 1, 0x778899);
    source.set(1, 1, 0xaabbcc);
    let mut target = Canvas::new_scaled(2, 2, 2.0, BACKGROUND);

    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: 2,
            height: 2,
            source_y: 0,
        },
    );

    assert_eq!(0x112233, target.pixels()[0]);
    assert_eq!(0x112233, target.pixels()[1]);
    assert_eq!(0x445566, target.pixels()[2]);
    assert_eq!(0x778899, target.pixels()[2 * target.width()]);
    assert_eq!(0xaabbcc, target.pixels()[3 * target.width() + 3]);
}

#[test]
fn public_canvas_blit_preserves_mixed_scale_physical_source_offset() {
    let mut source = Canvas::new_scaled(2, 2, 2.0, BACKGROUND);
    for (y, color) in [0x112233, 0x445566, 0x778899, 0xaabbcc]
        .into_iter()
        .enumerate()
    {
        for x in 0..source.width() {
            source.set_physical(x, y, color);
        }
    }
    let mut target = Canvas::new(2, 2, BACKGROUND);

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

    assert_eq!(0x445566, target.pixels()[0]);
    assert_eq!(0xaabbcc, target.pixels()[target.width()]);
}

#[test]
fn public_canvas_blit_advances_from_fractional_scale_crop_origin() {
    let mut source = Canvas::new_scaled(2, 3, 1.25, BACKGROUND);
    for (y, color) in [0x112233, 0x445566, 0x778899, 0xaabbcc]
        .into_iter()
        .enumerate()
    {
        for x in 0..source.width() {
            source.set_physical(x, y, color);
        }
    }
    let mut target = Canvas::new(2, 2, BACKGROUND);

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

    assert_eq!(0x445566, target.pixels()[0]);
    assert_eq!(0xaabbcc, target.pixels()[target.width()]);
}

#[test]
fn public_canvas_blit_resamples_equal_scale_when_logical_phase_differs() {
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
fn canvas_blit_preserves_selectable_text_run_phase_at_fractional_scale() {
    let mut source = Canvas::new_scaled_with_logical_phase(8, 4, 1.25, 0, 1.0, BACKGROUND);
    source.record_text_run("phase", 0, 2, 4, 1);
    let mut target = Canvas::new(8, 4, BACKGROUND);

    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: source.width(),
            height: 2,
            source_y: 2,
        },
    );

    let run = &target.text_runs()[0];
    assert_eq!(
        1,
        run.y(),
        "phase-aware physical source offset must retain the original logical selection offset"
    );
}

#[test]
fn canvas_blit_rounds_fractional_source_offset_for_selectable_text_runs() {
    let mut source = Canvas::new_scaled(8, 6, 2.0, BACKGROUND);
    source.record_text_run("first", 0, 2, 4, 1);
    source.record_text_run("second", 0, 4, 4, 1);
    let mut target = Canvas::new(8, 10, BACKGROUND);

    target.blit_canvas(
        &source,
        CanvasBlitRequest {
            dest_x: 0,
            dest_y: 0,
            width: source.width(),
            height: source.height().saturating_sub(2),
            source_y: 2,
        },
    );

    assert_eq!(2, target.text_runs().len());
    assert_eq!(1, target.text_runs()[0].y());
    assert_eq!(3, target.text_runs()[1].y());
}

#[test]
fn scaled_canvas_blit_omits_text_runs_outside_the_source_crop() {
    let mut source = Canvas::new_scaled(8, 4, 1.25, BACKGROUND);
    source.record_text_run("before", 0, 0, 4, 1);
    source.record_text_run("visible", 0, 2, 4, 1);
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
    assert_eq!("visible", target.text_runs()[0].text());
    assert_eq!(1, target.text_runs()[0].y());
}
