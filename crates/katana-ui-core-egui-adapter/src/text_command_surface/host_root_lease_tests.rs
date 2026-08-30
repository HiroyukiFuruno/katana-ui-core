use super::*;
use crate::text_command_surface::{
    KucOpaqueHostEffectBatch, KucOpaqueHostEffectError, KucRootEventBatchContext,
};
use katana_ui_core::molecule::{DiagnosticsList, StatusBar, StatusBarSegment};
use katana_ui_core::render_model::UiImageSurfaceProps;

const TEST_SCREEN_WIDTH: f32 = 800.0;
const TEST_SCREEN_HEIGHT: f32 = 600.0;

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT),
        )),
        ..Default::default()
    }
}

#[test]
fn boxed_router_lease_keeps_router_and_debug_opaque() {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"boxed-router-target",
        presentation(),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("token");
    let lease = EguiTextCommandSurfaceHostProjectionLease::from_router(
        token,
        Box::new(|_context: KucRootEventBatchContext| Ok(None)),
    );

    assert_eq!(
        format!("{lease:?}"),
        "EguiTextCommandSurfaceHostProjectionLease(..)"
    );
    let (_token, _router, source, tab, status, viewport) = lease.into_parts();
    assert!(source.is_none());
    assert!(tab.is_none());
    assert!(status.is_none());
    assert!(viewport.is_none());
}

#[test]
fn show_with_router_failure_returns_opaque_host_effect_error() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"router-fail-target",
        presentation(),
        style,
    )
    .expect("token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(EguiTextCommandSurfaceHostProjectionLease::new(
            token,
            |_context: KucRootEventBatchContext| Err(KucOpaqueHostEffectError),
        ))
        .expect("lease retain");

    let context = egui::Context::default();
    let mut error = None;
    let mut platform_output = context.run_ui(raw_input(), |ui| {
        error = Some(root.show(ui));
    });
    platform_output.textures_delta.clear();

    assert!(matches!(
        error.expect("show should be called"),
        Err(EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffect)
    ));
}

#[test]
fn retain_with_lease_attaches_auxiliary_projection_lease_slots() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"aux-lease-target",
        presentation(),
        style,
    )
    .expect("token");
    let status_bar = StatusBar::new("status for tests")
        .segment(StatusBarSegment::new("seg-a", "single message"));
    let diagnostics = DiagnosticsList::new("diagnostics for tests");

    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token, |_context| Ok(None))
                .with_status_diagnostics(
                    StatusDiagnosticsProjectionLease::new()
                        .with_status_bar(status_bar)
                        .with_diagnostics_list(diagnostics),
                )
                .with_editor_viewport(
                    EditorViewportProjectionLease::new(
                        UiImageSurfaceProps::new("preview", 1, 1, vec![1, 2, 3, 255])
                            .expect("preview"),
                    )
                    .with_split_ratio_percent(50)
                    .expect("split ratio"),
                ),
        )
        .expect("retain with aux leases");

    let context = egui::Context::default();
    let mut output = None;
    let mut frame = None;
    let mut platform_output = context.run_ui(raw_input(), |ui| {
        output = Some(root.show_output_for_test(ui));
    });
    platform_output.textures_delta.clear();
    let mut platform_output = context.run_ui(raw_input(), |ui| {
        frame = Some(root.show(ui));
    });
    platform_output.textures_delta.clear();
    let output = output
        .expect("show output_for_test should succeed")
        .expect("show output_for_test should succeed");
    let frame = frame
        .expect("show frame should succeed")
        .expect("show frame should succeed");

    assert!(
        output
            .artifact_order()
            .contains(&crate::text_command_surface::EguiTextCommandSurfaceChild::StatusBar)
    );
    assert!(
        output
            .artifact_order()
            .contains(&crate::text_command_surface::EguiTextCommandSurfaceChild::DiagnosticsList)
    );
    assert!(
        output
            .artifact_order()
            .contains(&crate::text_command_surface::EguiTextCommandSurfaceChild::Preview)
    );
    assert!(!frame.record().identity().is_empty());

    output
        .events()
        .attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(|| Ok(())))
        .expect("fresh root batch accepts one effect");
    let attach_error = output
        .events()
        .attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(|| Ok(())))
        .expect_err("second effect is rejected");
    assert!(matches!(
        EguiTextCommandSurfaceRootFactoryError::from(attach_error),
        EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffectRejected
    ));
}
