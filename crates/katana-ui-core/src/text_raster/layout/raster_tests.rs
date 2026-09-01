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
