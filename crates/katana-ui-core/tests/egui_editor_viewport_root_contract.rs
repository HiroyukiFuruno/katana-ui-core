#![cfg(feature = "egui")]
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EditorViewportProjectionLease, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory,
    EguiTextCommandSurfaceRootFactoryError, KucRootEventBatchContext, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiImageSurfaceProps, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn presentation() -> EguiTextCommandSurfacePresentation {
    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("editor-viewport-document").value("generic document ⭐️"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 800, 480),
    ));
    EguiTextCommandSurfacePresentation {
        text_state_id: None,
        text: TextSurfacePresentation::from_props(text.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    }
}

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"generic-editor-viewport-target".to_vec(),
        presentation(),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("opaque token")
}

fn lease(revision: u64) -> EguiTextCommandSurfaceHostProjectionLease {
    let preview = UiImageSurfaceProps::new(
        "private-preview-fingerprint",
        2,
        1,
        vec![9, 18, 27, 255, 36, 45, 54, 255],
    )
    .expect("preview")
    .accessibility_label("Generic preview");
    EguiTextCommandSurfaceHostProjectionLease::new(
        token(revision),
        |_context: KucRootEventBatchContext| Ok(None),
    )
    .with_editor_viewport(
        EditorViewportProjectionLease::new(preview)
            .with_split_ratio_percent(60)
            .expect("split ratio"),
    )
}

fn paint_hash(context: &egui::Context, root: &mut EguiTextCommandSurfaceHostRoot) -> String {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| frame = Some(root.show(ui).expect("root show")),
    );
    output.textures_delta.clear();
    frame
        .expect("closed root frame")
        .record()
        .paint_plan_hash()
        .to_owned()
}

#[test]
fn additive_preview_renders_and_plain_token_restores_legacy_root() {
    let context = egui::Context::default();
    let factory = EguiTextCommandSurfaceRootFactory::new();
    let mut projected = factory.retain_with_lease(lease(1)).expect("projected root");
    let projected_hash = paint_hash(&context, &mut projected);

    assert!(projected.synchronize(token(2)).expect("plain synchronize"));
    let restored_hash = paint_hash(&context, &mut projected);
    let mut legacy = factory.retain(token(2)).expect("legacy root");
    let legacy_hash = paint_hash(&context, &mut legacy);

    assert_ne!(projected_hash, restored_hash);
    assert_eq!(restored_hash, legacy_hash);
}

#[test]
fn viewport_lease_replay_and_older_revision_fail_closed() {
    let factory = EguiTextCommandSurfaceRootFactory::new();
    let mut root = factory.retain_with_lease(lease(4)).expect("projected root");
    assert!(matches!(
        root.synchronize_with_lease(lease(4)),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 4 })
    ));
    assert!(matches!(
        root.synchronize_with_lease(lease(3)),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 3 })
    ));
}
