use super::*;
use crate::atom::TextArea;
use crate::egui::text_command_surface::{EguiTextCommandSurface, KucGlobalActionsProjection};
use crate::text_raster::PlatformFontCatalog;
use crate::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};
use std::sync::Arc;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;

struct RetainGlobal(Option<super::super::EguiTextCommandSurfaceRootEventTransport>);
impl super::super::KucRootEventBatchForwarder for RetainGlobal {
    type Error = ();
    fn forward_root_event_batch(
        &mut self,
        transport: super::super::EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), ()> {
        self.0 = Some(transport);
        Ok(())
    }
}

#[test]
fn unauthorized_real_global_batch_is_retained_and_selection_restore_invalidates_transport() {
    use super::super::{KucGlobalAction, KucGlobalActionTarget, KucGlobalShortcut};
    let mut root = root();
    root.global_actions = Some(KucGlobalActionsProjection::new("Actions").action(
        KucGlobalAction::new(
            KucGlobalActionTarget::from_opaque_bytes(b"action"),
            "Action",
            KucGlobalShortcut::new(
                egui::Key::F,
                egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            ),
        ),
    ));
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(WIDTH as f32, HEIGHT as f32),
            )),
            events: vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            }],
            ..Default::default()
        },
        |ui| result = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style"))),
    );
    output.textures_delta.clear();
    let frame = result.expect("frame").expect("show");
    let mut forwarder = RetainGlobal(None);
    assert!(matches!(frame.events().forward_once(&mut forwarder), Err(super::super::EguiTextCommandSurfaceRootEventBatchForwardError::GlobalAuthorizationRequired)));
    assert!(forwarder.0.is_none());
    let generation = root.global_generation.get();
    assert!(frame.events().authorize_global(
        std::rc::Rc::downgrade(&root.global_generation),
        generation,
        std::rc::Rc::new(|| true)
    ));
    let snapshot = root.selection_surface().ui_selection();
    root.restore_selection_surface(&snapshot);
    assert_eq!(root.global_generation.get(), generation + 1);
    assert!(matches!(frame.events().forward_once(&mut forwarder), Err(super::super::EguiTextCommandSurfaceRootEventBatchForwardError::GlobalAuthorizationFailed)));
    assert!(forwarder.0.is_none());
}

fn root() -> EguiTextCommandSurfaceRoot {
    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("global-error").value("body"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, WIDTH, HEIGHT),
    ));
    EguiTextCommandSurfaceRoot::with_identity("global-error", EguiTextCommandSurface::new(text))
        .expect("root")
}

#[test]
fn catalog_mismatch_propagates_from_global_renderer_initialization() {
    let mut root = root();
    root.global_actions = Some(KucGlobalActionsProjection::new("Actions"));
    let mut policy = root.adapter.text_raster_config.catalog_policy();
    policy
        .proportional_candidates
        .push("different-catalog-policy".into());
    root.adapter.catalog = Arc::new(PlatformFontCatalog::new(policy));
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style")));
    });
    output.textures_delta.clear();
    assert!(matches!(
        result,
        Some(Err(EguiTextCommandSurfaceRootError::GlobalActionRaster(_)))
    ));
    assert!(root.global_renderer.is_none());
}

#[test]
fn failed_global_raster_keeps_initialized_renderer_for_retry() {
    let mut root = root();
    root.global_actions = Some(KucGlobalActionsProjection::new(""));
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style")));
    });
    output.textures_delta.clear();
    assert!(matches!(
        result,
        Some(Err(EguiTextCommandSurfaceRootError::GlobalActionRaster(_)))
    ));
    assert!(root.global_renderer.is_some());
    root.global_actions = Some(KucGlobalActionsProjection::new("Actions"));
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style")));
    });
    output.textures_delta.clear();
    assert!(matches!(result, Some(Ok(_))));
    assert!(root.global_renderer.is_some());
    root.global_actions = Some(KucGlobalActionsProjection::new(""));
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style")));
    });
    output.textures_delta.clear();
    assert!(matches!(
        result,
        Some(Err(EguiTextCommandSurfaceRootError::GlobalActionRaster(_)))
    ));
    assert!(
        root.global_renderer.is_some(),
        "existing renderer survives the failed render"
    );
}
