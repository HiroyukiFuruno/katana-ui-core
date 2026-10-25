use super::*;

fn bounds(width: f32, height: f32) -> PlatformTextGraphemeBounds {
    PlatformTextGraphemeBounds {
        byte_start: 0,
        byte_end: 1,
        x: 0.0,
        y: 0.0,
        width,
        height,
    }
}

#[test]
fn duplicate_grapheme_bounds_merge_to_their_geometric_union() {
    let mut merged = BTreeMap::new();
    merge_bounds(&mut merged, bounds(2.0, 3.0));
    let mut overlapping = bounds(4.0, 5.0);
    overlapping.x = -1.0;
    overlapping.y = -2.0;
    merge_bounds(&mut merged, overlapping);

    let result = merged.get(&(0, 1)).expect("merged grapheme bound");
    assert_eq!((result.x, result.y), (-1.0, -2.0));
    assert_eq!((result.width, result.height), (4.0, 5.0));
}

#[test]
fn raster_extent_rejects_axis_and_total_pixel_limits() {
    assert!(matches!(
        raster_extent(&[bounds((MAX_RASTER_DIMENSION + 1) as f32, 1.0)], 1.0),
        Err(PlatformTextRasterError::RasterTooLarge { .. })
    ));
    assert_eq!(
        raster_extent(
            &[bounds(
                MAX_RASTER_DIMENSION as f32,
                MAX_RASTER_DIMENSION as f32
            )],
            1.0,
        ),
        Err(PlatformTextRasterError::RasterTooLarge {
            width: MAX_RASTER_DIMENSION,
            height: MAX_RASTER_DIMENSION,
            max_pixels: MAX_RASTER_PIXELS,
        })
    );
}

#[test]
fn layout_raster_dimension_rejects_non_finite_dimensions() {
    assert_eq!(
        raster_dimension(f32::NAN),
        Err(PlatformTextRasterError::NonFiniteLayoutExtent)
    );
    assert_eq!(
        raster_dimension(f32::INFINITY),
        Err(PlatformTextRasterError::NonFiniteLayoutExtent)
    );
}

#[test]
fn hard_break_uses_the_logical_terminal_for_rtl_and_ltr_lines() {
    let mut rtl_bounds =
        BTreeMap::from([((0, 2), bounds(10.0, 20.0)), ((2, 4), bounds(10.0, 20.0))]);
    rtl_bounds.get_mut(&(0, 2)).unwrap().x = 10.0;
    rtl_bounds.get_mut(&(2, 4)).unwrap().x = 0.0;
    add_hard_break_bounds(
        "אב\nx",
        &mut rtl_bounds,
        &BTreeMap::from([(0, (0.0, 0.0, 20.0, 4))]),
        20.0,
    );
    assert_eq!(rtl_bounds.get(&(4, 5)).unwrap().x, 0.0);

    let mut ltr_bounds =
        BTreeMap::from([((0, 1), bounds(10.0, 20.0)), ((1, 2), bounds(10.0, 20.0))]);
    ltr_bounds.get_mut(&(1, 2)).unwrap().x = 10.0;
    add_hard_break_bounds(
        "ab\nx",
        &mut ltr_bounds,
        &BTreeMap::from([(0, (20.0, 0.0, 20.0, 2))]),
        20.0,
    );
    assert_eq!(ltr_bounds.get(&(2, 3)).unwrap().x, 20.0);
}

#[test]
fn hard_break_empty_rows_do_not_reuse_the_previous_row_terminal() {
    let mut bounds = BTreeMap::from([((0, 1), bounds(10.0, 20.0))]);
    add_hard_break_bounds(
        "a\n\nb",
        &mut bounds,
        &BTreeMap::from([(0, (10.0, 0.0, 20.0, 1)), (1, (0.0, 20.0, 20.0, 2))]),
        20.0,
    );
    assert_eq!(bounds.get(&(1, 2)).unwrap().x, 10.0);
    assert_eq!(bounds.get(&(2, 3)).unwrap().x, 0.0);
}
