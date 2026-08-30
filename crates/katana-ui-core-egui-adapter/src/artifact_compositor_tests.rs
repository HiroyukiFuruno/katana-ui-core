use super::*;
use crate::command_chrome::{
    CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
    EguiCommandChromeDrawLayer,
};
use crate::context_menu::{
    ContextMenuPaintOperation, ContextMenuPaintOperationKind, ContextMenuPaintPlan,
    ContextMenuPaintTexture,
};
use crate::diagnostics_list::{
    DiagnosticsListPaintOperation, DiagnosticsListPaintOperationKind, DiagnosticsListPaintPlan,
    DiagnosticsListPaintTexture,
};
use crate::source_address_strip::{
    SourceAddressPaintOperation, SourceAddressPaintOperationKind, SourceAddressPaintPlan,
    SourceAddressPaintTexture,
};
use crate::status_bar::{
    StatusBarPaintOperation, StatusBarPaintOperationKind, StatusBarPaintPlan, StatusBarPaintTexture,
};
use crate::tab_strip_paint::{
    TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintPlan, TabStripPaintTexture,
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

fn source_address_plan(kind: SourceAddressPaintOperationKind) -> SourceAddressPaintPlan {
    SourceAddressPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![SourceAddressPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn status_bar_plan(kind: StatusBarPaintOperationKind) -> StatusBarPaintPlan {
    StatusBarPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![StatusBarPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn diagnostics_list_plan(kind: DiagnosticsListPaintOperationKind) -> DiagnosticsListPaintPlan {
    DiagnosticsListPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![DiagnosticsListPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn context_menu_plan(kind: ContextMenuPaintOperationKind) -> ContextMenuPaintPlan {
    ContextMenuPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![ContextMenuPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
            kind,
        }],
    }
}

fn tab_strip_plan(kind: TabStripPaintOperationKind) -> TabStripPaintPlan {
    TabStripPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![TabStripPaintOperation {
            clip_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
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

#[test]
fn public_api_composes_rounded_and_transparent_layers_with_clipping() {
    let rounded = CommandChromePaintPlan {
        surface_bounds: UiRect::new(OVERLAY_X, CANVAS_Y, 3, 3),
        operations: vec![CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: UiRect::new(OVERLAY_X, CANVAS_Y, 3, 3),
            kind: CommandChromePaintOperationKind::RoundedFill {
                bounds: UiRect::new(OVERLAY_X, CANVAS_Y, 3, 3),
                color_rgba: [20, 40, 80, 255],
                radius_px: 2,
            },
        }],
    };
    let transparent = text_plan(TextSurfacePaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        color_rgba: [255, 255, 255, 0],
    });
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(CANVAS_X, CANVAS_Y, 3, 3)),
        plans: &[
            ArtifactPaintPlanRef::CommandChrome(&rounded),
            ArtifactPaintPlanRef::TextSurface(&transparent),
        ],
    })
    .expect("rounded fill with transparent overlay composes");

    assert!(frame.non_transparent_pixel_count > 0);
    assert!(
        frame
            .rgba_pixels
            .chunks_exact(4)
            .any(|pixel| pixel == [0, 0, 0, 0])
    );
    assert!(
        frame
            .rgba_pixels
            .chunks_exact(4)
            .any(|pixel| pixel == [20, 40, 80, 255])
    );
}

#[test]
fn public_api_rejects_zero_dimension_texture_before_sampling() {
    let text = text_plan(TextSurfacePaintOperationKind::Texture {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        texture: TextSurfacePaintTexture {
            identity: "empty".to_owned(),
            width: 0,
            height: 1,
            rgba_pixels: Vec::new(),
        },
    });
    let result = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL)),
        plans: &[ArtifactPaintPlanRef::TextSurface(&text)],
    });
    assert!(
        matches!(result, Err(ArtifactCompositeError::ZeroTexture { identity }) if identity == "empty")
    );
}

#[test]
fn public_api_composes_source_address_plan_input_fill() {
    let plans = [ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
        SourceAddressPaintOperationKind::Input(TextSurfacePaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [10, 20, 30, 40],
        }),
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("input fill composes");
    assert_eq!(frame.rgba_pixels[0..4], [10, 20, 30, 40]);
}

#[test]
fn public_api_composes_source_address_plan_input_texture() {
    let plans = [ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
        SourceAddressPaintOperationKind::Input(TextSurfacePaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: TextSurfacePaintTexture {
                identity: "source-address-input-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![0, 10, 20, 255],
            },
        }),
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("input texture composes");
    assert_eq!(frame.rgba_pixels[0..4], [0, 10, 20, 255]);
}

#[test]
fn public_api_composes_source_address_plan_fill() {
    let plans = [ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
        SourceAddressPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [30, 40, 50, 60],
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("fill composes");
    assert_eq!(frame.rgba_pixels[0..4], [30, 40, 50, 60]);
}

#[test]
fn public_api_composes_source_address_plan_texture() {
    let plans = [ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
        SourceAddressPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: SourceAddressPaintTexture {
                identity: "source-address-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![30, 40, 50, 255],
            },
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("texture composes");
    assert_eq!(frame.rgba_pixels[0..4], [30, 40, 50, 255]);
}

#[test]
fn public_api_composes_status_bar_plan_fill() {
    let plans = [ArtifactPaintPlanRef::StatusBar(&status_bar_plan(
        StatusBarPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [70, 80, 90, 100],
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("status-bar fill composes");
    assert_eq!(frame.rgba_pixels[0..4], [70, 80, 90, 100]);
}

#[test]
fn public_api_composes_status_bar_plan_texture() {
    let plans = [ArtifactPaintPlanRef::StatusBar(&status_bar_plan(
        StatusBarPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: StatusBarPaintTexture {
                identity: "status-bar-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![70, 80, 90, 255],
            },
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("status-bar texture composes");
    assert_eq!(frame.rgba_pixels[0..4], [70, 80, 90, 255]);
}

#[test]
fn public_api_composes_diagnostics_list_plan_fill() {
    let plans = [ArtifactPaintPlanRef::DiagnosticsList(
        &diagnostics_list_plan(DiagnosticsListPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [90, 100, 110, 120],
        }),
    )];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("diagnostics fill composes");
    assert_eq!(frame.rgba_pixels[0..4], [90, 100, 110, 120]);
}

#[test]
fn public_api_composes_diagnostics_list_plan_texture() {
    let plans = [ArtifactPaintPlanRef::DiagnosticsList(
        &diagnostics_list_plan(DiagnosticsListPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: DiagnosticsListPaintTexture {
                identity: "diagnostics-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![90, 100, 110, 255],
            },
        }),
    )];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("diagnostics texture composes");
    assert_eq!(frame.rgba_pixels[0..4], [90, 100, 110, 255]);
}

#[test]
fn public_api_composes_context_menu_plan_fill() {
    let plans = [ArtifactPaintPlanRef::ContextMenu(&context_menu_plan(
        ContextMenuPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [110, 120, 130, 140],
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("context menu fill composes");
    assert_eq!(frame.rgba_pixels[0..4], [110, 120, 130, 140]);
}

#[test]
fn public_api_composes_context_menu_plan_texture() {
    let plans = [ArtifactPaintPlanRef::ContextMenu(&context_menu_plan(
        ContextMenuPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: ContextMenuPaintTexture {
                identity: "context-menu-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![110, 120, 130, 255],
            },
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("context menu texture composes");
    assert_eq!(frame.rgba_pixels[0..4], [110, 120, 130, 255]);
}

#[test]
fn public_api_composes_tab_strip_plan_fill() {
    let plans = [ArtifactPaintPlanRef::TabStrip(&tab_strip_plan(
        TabStripPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [130, 140, 150, 160],
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("tab strip fill composes");
    assert_eq!(frame.rgba_pixels[0..4], [130, 140, 150, 160]);
}

#[test]
fn public_api_composes_tab_strip_plan_texture() {
    let plans = [ArtifactPaintPlanRef::TabStrip(&tab_strip_plan(
        TabStripPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: TabStripPaintTexture {
                identity: "tab-strip-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![130, 140, 150, 255],
            },
        },
    ))];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("tab strip texture composes");
    assert_eq!(frame.rgba_pixels[0..4], [130, 140, 150, 255]);
}

#[test]
fn public_api_composes_source_plan_clip_empty_area_without_failure() {
    let source = SourceAddressPaintPlan {
        surface_bounds: UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT),
        operations: vec![SourceAddressPaintOperation {
            clip_bounds: UiRect::new(100, 100, ONE_PIXEL, ONE_PIXEL),
            kind: SourceAddressPaintOperationKind::Fill {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                color_rgba: [255, 255, 255, 255],
            },
        }],
    };
    let plans = [ArtifactPaintPlanRef::SourceAddress(&source)];
    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &plans,
    })
    .expect("no-op clip should preserve previous pixels");

    assert_eq!(frame.rgba_pixels, vec![0; 16]);
}
#[test]
fn composite_error_messages_preserve_typed_failure_context() {
    let cases = [
        (
            ArtifactCompositeError::ZeroCanvas,
            "artifact canvas must have non-zero dimensions".to_string(),
        ),
        (
            ArtifactCompositeError::Overflow {
                context: "indexing",
            },
            "artifact arithmetic overflow while indexing".to_string(),
        ),
        (
            ArtifactCompositeError::ZeroTexture {
                identity: "texture-a".to_string(),
            },
            "artifact texture `texture-a` has zero dimensions".to_string(),
        ),
        (
            ArtifactCompositeError::TextureByteLength {
                identity: "texture-b".to_string(),
                expected: 8,
                actual: 4,
            },
            "artifact texture `texture-b` has 4 RGBA bytes; expected 8".to_string(),
        ),
        (
            ArtifactCompositeError::TexturePixelRange {
                identity: "texture-c".to_string(),
                start: 4,
                end: 8,
                actual: 4,
            },
            "artifact texture `texture-c` cannot provide RGBA range 4..8 from 4 bytes".to_string(),
        ),
        (
            ArtifactCompositeError::Serialization("unsupported map key".to_string()),
            "artifact plan serialization failed: unsupported map key".to_string(),
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn artifact_plan_serialization_fails_closed_for_unsupported_json_map_keys() {
    let unsupported_json_key = std::collections::BTreeMap::from([((1_u8, 2_u8), 3_u8)]);

    let error = super::artifact_compositor_hash::serialize_value(&unsupported_json_key)
        .expect_err("JSON object keys must not silently accept tuple semantics");

    assert!(matches!(
        error,
        ArtifactCompositeError::Serialization(message)
            if message.contains("key must be a string")
    ));
}

#[test]
fn every_clipped_plan_skips_a_disjoint_operation() {
    let far_clip = UiRect::new(100, 100, ONE_PIXEL, ONE_PIXEL);
    let mut text = text_plan(TextSurfacePaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    text.operations[0].clip_bounds = far_clip;
    let mut source = source_address_plan(SourceAddressPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    source.operations[0].clip_bounds = far_clip;
    let mut status = status_bar_plan(StatusBarPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    status.operations[0].clip_bounds = far_clip;
    let mut diagnostics = diagnostics_list_plan(DiagnosticsListPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    diagnostics.operations[0].clip_bounds = far_clip;
    let mut chrome = chrome_plan(CommandChromePaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    chrome.surface_bounds = UiRect::new(CANVAS_X, CANVAS_Y, SURFACE_WIDTH, SURFACE_HEIGHT);
    chrome.operations[0].clip_bounds = far_clip;
    let mut context = context_menu_plan(ContextMenuPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 0, 0, 255],
    });
    context.operations[0].clip_bounds = far_clip;

    let frame = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &[
            ArtifactPaintPlanRef::TextSurface(&text),
            ArtifactPaintPlanRef::SourceAddress(&source),
            ArtifactPaintPlanRef::StatusBar(&status),
            ArtifactPaintPlanRef::DiagnosticsList(&diagnostics),
            ArtifactPaintPlanRef::CommandChrome(&chrome),
            ArtifactPaintPlanRef::ContextMenu(&context),
        ],
    })
    .expect("disjoint clips must be ignored for every clipped plan");

    assert_eq!(frame.non_transparent_pixel_count, 0);
    assert!(frame.rgba_pixels.iter().all(|value| *value == 0));
}

#[test]
fn every_clipped_plan_propagates_surface_geometry_overflow() {
    let overflow = UiRect::new(i32::MAX, 0, 1, 1);
    let mut text = text_plan(TextSurfacePaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    text.surface_bounds = overflow;
    let mut source = source_address_plan(SourceAddressPaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    source.surface_bounds = overflow;
    let mut status = status_bar_plan(StatusBarPaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    status.surface_bounds = overflow;
    let mut diagnostics = diagnostics_list_plan(DiagnosticsListPaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    diagnostics.surface_bounds = overflow;
    let mut chrome = chrome_plan(CommandChromePaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    chrome.surface_bounds = overflow;
    let mut context = context_menu_plan(ContextMenuPaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    context.surface_bounds = overflow;

    for plans in [
        vec![ArtifactPaintPlanRef::TextSurface(&text)],
        vec![ArtifactPaintPlanRef::SourceAddress(&source)],
        vec![ArtifactPaintPlanRef::StatusBar(&status)],
        vec![ArtifactPaintPlanRef::DiagnosticsList(&diagnostics)],
        vec![ArtifactPaintPlanRef::CommandChrome(&chrome)],
        vec![ArtifactPaintPlanRef::ContextMenu(&context)],
    ] {
        assert!(matches!(
            ArtifactCompositor::compose(ArtifactCompositeRequest {
                canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 4, 4)),
                plans: &plans,
            }),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
    }
}

#[test]
fn tab_strip_and_rounded_chrome_propagate_blend_failures() {
    let overflow = UiRect::new(i32::MAX, 0, 1, 1);
    let tab_fill = tab_strip_plan(TabStripPaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 4, 4)),
            plans: &[ArtifactPaintPlanRef::TabStrip(&tab_fill)],
        }),
        Err(ArtifactCompositeError::Overflow { .. })
    ));

    let tab_texture = tab_strip_plan(TabStripPaintOperationKind::Texture {
        bounds: UiRect::new(0, 0, 1, 1),
        texture: TabStripPaintTexture {
            identity: "zero-tab-texture".to_string(),
            width: 0,
            height: 1,
            rgba_pixels: Vec::new(),
        },
    });
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 4, 4)),
            plans: &[ArtifactPaintPlanRef::TabStrip(&tab_texture)],
        }),
        Err(ArtifactCompositeError::ZeroTexture { .. })
    ));

    let rounded = CommandChromePaintPlan {
        surface_bounds: UiRect::new(0, 0, 4, 4),
        operations: vec![CommandChromePaintOperation {
            layer: EguiCommandChromeDrawLayer::PanelFill,
            clip_bounds: UiRect::new(0, 0, 4, 4),
            kind: CommandChromePaintOperationKind::RoundedFill {
                bounds: overflow,
                color_rgba: [0, 0, 0, 0],
                radius_px: 1,
            },
        }],
    };
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 4, 4)),
            plans: &[ArtifactPaintPlanRef::CommandChrome(&rounded)],
        }),
        Err(ArtifactCompositeError::Overflow { .. })
    ));
}

#[test]
fn canvas_byte_length_overflow_fails_before_allocation() {
    let canvas = UiRect::new(i32::MIN, i32::MIN, u32::MAX, u32::MAX);

    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(canvas),
            plans: &[],
        }),
        Err(ArtifactCompositeError::Overflow {
            context: "sizing canvas RGBA bytes"
        })
    ));
}
