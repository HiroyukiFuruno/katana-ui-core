use super::*;

const RGBA_PIXEL_BYTES: usize = 4;

fn assert_pixel(plan: ArtifactPaintPlanRef<'_>, expected: [u8; RGBA_PIXEL_BYTES], context: &str) {
    let request = ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            CANVAS_X,
            CANVAS_Y,
            SURFACE_WIDTH,
            SURFACE_HEIGHT,
        )),
        plans: &[plan],
    };
    let Some(frame) = require_ok(ArtifactCompositor::compose(request), context) else {
        return;
    };
    assert_eq!(&frame.rgba_pixels[0..RGBA_PIXEL_BYTES], &expected);
}

#[test]
fn public_api_composes_source_address_plan_input_fill() {
    assert_pixel(
        ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
            SourceAddressPaintOperationKind::Input(TextSurfacePaintOperationKind::Fill {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                color_rgba: [10, 20, 30, 40],
            }),
        )),
        [10, 20, 30, 40],
        "input fill composes",
    );
}

#[test]
fn public_api_composes_source_address_plan_input_texture() {
    assert_pixel(
        ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
            SourceAddressPaintOperationKind::Input(TextSurfacePaintOperationKind::Texture {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                texture: TextSurfacePaintTexture {
                    identity: "source-address-input-texture".to_owned(),
                    width: 1,
                    height: 1,
                    rgba_pixels: vec![0, 10, 20, 255],
                },
            }),
        )),
        [0, 10, 20, 255],
        "input texture composes",
    );
}

#[test]
fn public_api_composes_source_address_plan_fill() {
    assert_pixel(
        ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
            SourceAddressPaintOperationKind::Fill {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                color_rgba: [30, 40, 50, 60],
            },
        )),
        [30, 40, 50, 60],
        "fill composes",
    );
}

#[test]
fn public_api_composes_source_address_plan_texture() {
    assert_pixel(
        ArtifactPaintPlanRef::SourceAddress(&source_address_plan(
            SourceAddressPaintOperationKind::Texture {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                texture: SourceAddressPaintTexture {
                    identity: "source-address-texture".to_owned(),
                    width: 1,
                    height: 1,
                    rgba_pixels: vec![30, 40, 50, 255],
                },
            },
        )),
        [30, 40, 50, 255],
        "texture composes",
    );
}

#[test]
fn public_api_composes_status_bar_plan_fill() {
    assert_pixel(
        ArtifactPaintPlanRef::StatusBar(&status_bar_plan(StatusBarPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [70, 80, 90, 100],
        })),
        [70, 80, 90, 100],
        "status-bar fill composes",
    );
}

#[test]
fn public_api_composes_status_bar_plan_texture() {
    assert_pixel(
        ArtifactPaintPlanRef::StatusBar(&status_bar_plan(StatusBarPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: StatusBarPaintTexture {
                identity: "status-bar-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![70, 80, 90, 255],
            },
        })),
        [70, 80, 90, 255],
        "status-bar texture composes",
    );
}

#[test]
fn public_api_composes_diagnostics_list_plan_fill() {
    assert_pixel(
        ArtifactPaintPlanRef::DiagnosticsList(&diagnostics_list_plan(
            DiagnosticsListPaintOperationKind::Fill {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                color_rgba: [90, 100, 110, 120],
            },
        )),
        [90, 100, 110, 120],
        "diagnostics fill composes",
    );
}

#[test]
fn public_api_composes_diagnostics_list_plan_texture() {
    assert_pixel(
        ArtifactPaintPlanRef::DiagnosticsList(&diagnostics_list_plan(
            DiagnosticsListPaintOperationKind::Texture {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                texture: DiagnosticsListPaintTexture {
                    identity: "diagnostics-texture".to_owned(),
                    width: 1,
                    height: 1,
                    rgba_pixels: vec![90, 100, 110, 255],
                },
            },
        )),
        [90, 100, 110, 255],
        "diagnostics texture composes",
    );
}

#[test]
fn public_api_composes_context_menu_plan_fill() {
    assert_pixel(
        ArtifactPaintPlanRef::ContextMenu(&context_menu_plan(
            ContextMenuPaintOperationKind::Fill {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                color_rgba: [110, 120, 130, 140],
            },
        )),
        [110, 120, 130, 140],
        "context menu fill composes",
    );
}

#[test]
fn public_api_composes_context_menu_plan_texture() {
    assert_pixel(
        ArtifactPaintPlanRef::ContextMenu(&context_menu_plan(
            ContextMenuPaintOperationKind::Texture {
                bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
                texture: ContextMenuPaintTexture {
                    identity: "context-menu-texture".to_owned(),
                    width: 1,
                    height: 1,
                    rgba_pixels: vec![110, 120, 130, 255],
                },
            },
        )),
        [110, 120, 130, 255],
        "context menu texture composes",
    );
}

#[test]
fn public_api_composes_tab_strip_plan_fill() {
    assert_pixel(
        ArtifactPaintPlanRef::TabStrip(&tab_strip_plan(TabStripPaintOperationKind::Fill {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            color_rgba: [130, 140, 150, 160],
        })),
        [130, 140, 150, 160],
        "tab strip fill composes",
    );
}

#[test]
fn public_api_composes_tab_strip_plan_texture() {
    assert_pixel(
        ArtifactPaintPlanRef::TabStrip(&tab_strip_plan(TabStripPaintOperationKind::Texture {
            bounds: UiRect::new(CANVAS_X, CANVAS_Y, ONE_PIXEL, ONE_PIXEL),
            texture: TabStripPaintTexture {
                identity: "tab-strip-texture".to_owned(),
                width: 1,
                height: 1,
                rgba_pixels: vec![130, 140, 150, 255],
            },
        })),
        [130, 140, 150, 255],
        "tab strip texture composes",
    );
}
