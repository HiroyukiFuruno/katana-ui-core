use super::*;

#[test]
fn public_api_composes_source_plan_clip_empty_area_without_failure() {
    let mut source = source_address_plan(SourceAddressPaintOperationKind::Fill {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        color_rgba: [255, 255, 255, 255],
    });
    source.operations[0].clip_bounds = UiRect::new(100, 100, ONE_PIXEL, ONE_PIXEL);
    let Some(frame) = require_ok(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(
                CANVAS_X,
                CANVAS_Y,
                SURFACE_WIDTH,
                SURFACE_HEIGHT,
            )),
            plans: &[ArtifactPaintPlanRef::SourceAddress(&source)],
        }),
        "no-op clip should preserve previous pixels",
    ) else {
        return;
    };
    assert_eq!(frame.rgba_pixels, vec![0; 16]);
}

#[test]
fn composite_error_messages_preserve_typed_failure_context() {
    let cases = [
        (
            ArtifactCompositeError::ZeroCanvas,
            "artifact canvas must have non-zero dimensions",
        ),
        (
            ArtifactCompositeError::Overflow {
                context: "indexing",
            },
            "artifact arithmetic overflow while indexing",
        ),
        (
            ArtifactCompositeError::ZeroTexture {
                identity: "texture-a".to_owned(),
            },
            "artifact texture `texture-a` has zero dimensions",
        ),
        (
            ArtifactCompositeError::TextureByteLength {
                identity: "texture-b".to_owned(),
                expected: 8,
                actual: 4,
            },
            "artifact texture `texture-b` has 4 RGBA bytes; expected 8",
        ),
        (
            ArtifactCompositeError::TexturePixelRange {
                identity: "texture-c".to_owned(),
                start: 4,
                end: 8,
                actual: 4,
            },
            "artifact texture `texture-c` cannot provide RGBA range 4..8 from 4 bytes",
        ),
        (
            ArtifactCompositeError::Serialization("unsupported map key".to_owned()),
            "artifact plan serialization failed: unsupported map key",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn artifact_plan_serialization_fails_closed_for_unsupported_json_map_keys() {
    let unsupported_json_key = std::collections::BTreeMap::from([((1_u8, 2_u8), 3_u8)]);
    let Some(error) = require_err(
        super::artifact_compositor_hash::serialize_value(&unsupported_json_key),
        "JSON object keys must not silently accept tuple semantics",
    ) else {
        return;
    };
    assert!(
        matches!(error, ArtifactCompositeError::Serialization(message) if message.contains("key must be a string"))
    );
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
    let plans = [
        ArtifactPaintPlanRef::TextSurface(&text),
        ArtifactPaintPlanRef::SourceAddress(&source),
        ArtifactPaintPlanRef::StatusBar(&status),
        ArtifactPaintPlanRef::DiagnosticsList(&diagnostics),
        ArtifactPaintPlanRef::CommandChrome(&chrome),
        ArtifactPaintPlanRef::ContextMenu(&context),
    ];
    let Some(frame) = require_ok(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(
                CANVAS_X,
                CANVAS_Y,
                SURFACE_WIDTH,
                SURFACE_HEIGHT,
            )),
            plans: &plans,
        }),
        "disjoint clips must be ignored for every clipped plan",
    ) else {
        return;
    };
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
    let mut tab_strip = tab_strip_plan(TabStripPaintOperationKind::Fill {
        bounds: overflow,
        color_rgba: [0, 0, 0, 0],
    });
    tab_strip.surface_bounds = overflow;
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
    for plan in [
        ArtifactPaintPlanRef::TextSurface(&text),
        ArtifactPaintPlanRef::SourceAddress(&source),
        ArtifactPaintPlanRef::StatusBar(&status),
        ArtifactPaintPlanRef::DiagnosticsList(&diagnostics),
        ArtifactPaintPlanRef::TabStrip(&tab_strip),
        ArtifactPaintPlanRef::CommandChrome(&chrome),
        ArtifactPaintPlanRef::ContextMenu(&context),
    ] {
        assert!(matches!(
            ArtifactCompositor::compose(ArtifactCompositeRequest {
                canvas: ArtifactCanvasBounds::new(UiRect::new(0, 0, 4, 4)),
                plans: &[plan]
            }),
            Err(ArtifactCompositeError::Overflow { .. })
        ));
    }
}

#[test]
fn tab_strip_texture_and_rounded_chrome_propagate_blend_failures() {
    let tab_texture = tab_strip_plan(TabStripPaintOperationKind::Texture {
        bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
        texture: TabStripPaintTexture {
            identity: "zero-tab-texture".to_owned(),
            width: 0,
            height: 1,
            rgba_pixels: Vec::new(),
        },
    });
    assert!(matches!(
        ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(UiRect::new(
                CANVAS_X,
                CANVAS_Y,
                SURFACE_WIDTH,
                SURFACE_HEIGHT,
            )),
            plans: &[ArtifactPaintPlanRef::TabStrip(&tab_texture)]
        }),
        Err(ArtifactCompositeError::ZeroTexture { .. })
    ));
    let overflow = UiRect::new(i32::MAX, 0, 1, 1);
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
            plans: &[ArtifactPaintPlanRef::CommandChrome(&rounded)]
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
            plans: &[]
        }),
        Err(ArtifactCompositeError::Overflow {
            context: "sizing canvas RGBA bytes"
        })
    ));
}
