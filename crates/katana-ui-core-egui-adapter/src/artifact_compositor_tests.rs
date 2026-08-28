use super::*;
use crate::command_chrome::{
    CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
    EguiCommandChromeDrawLayer,
};
use crate::text_surface::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintPlan, TextSurfacePaintTexture,
};
use katana_ui_core::render_model::UiRect;

const CANVAS_X: i32 = 10;
const CANVAS_Y: i32 = 10;
const OVERLAY_X: i32 = 11;
const DRAW_START_X: i32 = 9;
const SURFACE_WIDTH: u32 = 2;
const SURFACE_HEIGHT: u32 = 2;
const DRAW_WIDTH: u32 = 3;
const ONE_PIXEL: u32 = 1;

fn text_plan(kind: TextSurfacePaintOperationKind) -> TextSurfacePaintPlan {
    TextSurfacePaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        viewport_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![TextSurfacePaintOperation {
            layer: EguiTextSurfaceDrawLayer::Background,
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn chrome_plan(kind: CommandChromePaintOperationKind) -> CommandChromePaintPlan {
    CommandChromePaintPlan {
        surface_bounds: UiRect::new(OVERLAY_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        operations: vec![CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: UiRect::new(OVERLAY_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            kind,
        }],
    }
}

#[test]
fn public_api_preserves_mixed_order_clips_and_repeats_hashes() {
    let text = text_plan(TextSurfacePaintOperationKind::Fill {
        bounds: UiRect::new(DRAW_START_X, CANVAS_Y, DRAW_WIDTH, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    let chrome = chrome_plan(CommandChromePaintOperationKind::Fill {
        bounds: UiRect::new(OVERLAY_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [0, 0, 255, 128],
    });
    let plans = [
        ArtifactPaintPlanRef::TextSurface(&text),
        ArtifactPaintPlanRef::CommandChrome(&chrome),
    ];
    let request = ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    };
    let first = ArtifactCompositor::compose(request.clone()).expect("valid plans compose");
    let second = ArtifactCompositor::compose(request).expect("repeat plans compose");
    assert_eq!(first, second);
    assert_eq!(first.non_transparent_pixel_count, 2);
    assert_eq!(&first.rgba_pixels[0..4], &[255, 0, 0, 255]);
    assert_eq!(&first.rgba_pixels[4..8], &[127, 0, 128, 255]);
}

#[test]
fn public_api_rejects_malformed_texture_and_zero_canvas() {
    let texture = TextSurfacePaintTexture {
        identity: "star-vs16".to_owned(),
        width: 2,
        height: 1,
        rgba_pixels: vec![255, 255, 0, 255],
    };
    let text = text_plan(TextSurfacePaintOperationKind::Texture {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, ONE_PIXEL),
        texture,
    });
    let plans = [ArtifactPaintPlanRef::TextSurface(&text)];
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(
                CANVAS_X,
                CANVAS_Y,
                SURFACE_WIDTH,
                SURFACE_HEIGHT,
            )),
            plans: &plans,
        }),
        Err(ArtifactCompositeError::TextureByteLength { .. })
    ));
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 0, ONE_PIXEL)),
            plans: &plans,
        }),
        Err(ArtifactCompositeError::ZeroCanvas)
    ));
}

#[test]
fn public_api_rejects_canvas_edge_overflow() {
    let text = text_plan(TextSurfacePaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [0, 0, 0, 0],
    });
    let plans = [ArtifactPaintPlanRef::TextSurface(&text)];
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(i32::MAX, 0, 1, 1)),
            plans: &plans,
        }),
        Err(ArtifactCompositeError::Overflow { .. })
    ));
}
