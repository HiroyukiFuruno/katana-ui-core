use super::*;

const OVERLAY_EXTENT: u32 = 4;

fn compose_tab_strip_overlay(kind: TabStripPaintOperationKind) -> ArtifactCompositeFrame {
    let mut tab_strip = tab_strip_plan(kind);
    tab_strip.surface_bounds = UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL);
    tab_strip.operations[0].clip_bounds =
        UiRect::new(CANVAS_X - 1, CANVAS_Y - 1, OVERLAY_EXTENT, OVERLAY_EXTENT);

    ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &[ArtifactPaintPlanRef::TabStrip(&tab_strip)],
    })
    .expect("tab-strip overlay should clip to the composite root")
}

#[test]
fn tab_strip_fill_clips_an_overlay_to_the_canvas_and_surface_bounds() {
    let frame = compose_tab_strip_overlay(TabStripPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X - 1, CANVAS_Y - 1, OVERLAY_EXTENT, OVERLAY_EXTENT),
        color_rgba: [255, 0, 0, 255],
    });

    assert_eq!(frame.non_transparent_pixel_count, 1);
    assert_eq!(&frame.rgba_pixels[0..4], &[255, 0, 0, 255]);
    assert_eq!(&frame.rgba_pixels[4..], &[0; 12]);
}

#[test]
fn tab_strip_texture_clips_an_overlay_to_the_canvas_and_surface_bounds() {
    let frame = compose_tab_strip_overlay(TabStripPaintOperationKind::Texture {
        bounds: UiRect::new(CANVAS_X - 1, CANVAS_Y - 1, OVERLAY_EXTENT, OVERLAY_EXTENT),
        texture: TabStripPaintTexture {
            identity: "tab-strip-overlay".to_owned(),
            width: 1,
            height: 1,
            rgba_pixels: vec![0, 255, 0, 255],
        },
    });

    assert_eq!(frame.non_transparent_pixel_count, 1);
    assert_eq!(&frame.rgba_pixels[0..4], &[0, 255, 0, 255]);
    assert_eq!(&frame.rgba_pixels[4..], &[0; 12]);
}

#[test]
fn tab_strip_operation_outside_the_canvas_is_skipped_without_wrapping_pixels() {
    let mut tab_strip = tab_strip_plan(TabStripPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    tab_strip.operations[0].clip_bounds = UiRect::new(
        CANVAS_X + SURFACE_WIDTH as i32,
        CANVAS_Y + SURFACE_HEIGHT as i32,
        ONE_PIXEL,
        ONE_PIXEL,
    );

    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &[ArtifactPaintPlanRef::TabStrip(&tab_strip)],
    })
    .expect("out-of-canvas tab-strip operation should be a safe no-op");

    assert_eq!(frame.non_transparent_pixel_count, 0);
    assert!(frame.rgba_pixels.iter().all(|channel| *channel == 0));
}
