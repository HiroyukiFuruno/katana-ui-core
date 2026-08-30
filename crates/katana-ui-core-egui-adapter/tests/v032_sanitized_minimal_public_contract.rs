use katana_ui_core::render_model::UiIconProps;
use katana_ui_core_egui_adapter::text_command_surface::{
    SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
    SanitizedCommandTarget, SanitizedDocumentRootFactory, SanitizedDocumentRootIdentity,
    SanitizedDocumentRootInput, SanitizedDocumentRootStyleKey, SanitizedTab,
    SanitizedTabCapabilities, SanitizedTabGroup, SanitizedTabGroupCapabilities,
    SanitizedTabGroupTarget, SanitizedTabProjection, SanitizedTabTarget,
};

fn run_frame(
    root: &mut katana_ui_core_egui_adapter::text_command_surface::SanitizedDocumentRoot,
) -> Result<
    katana_ui_core_egui_adapter::text_command_surface::SanitizedDocumentRootFrame,
    katana_ui_core_egui_adapter::text_command_surface::SanitizedDocumentRootFactoryError,
> {
    let context = egui::Context::default();
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| frame = Some(root.show(ui)),
    );
    output.textures_delta.clear();
    frame.expect("real egui frame invokes the sanitized root")
}

#[test]
fn default_factory_renders_without_optional_projections() {
    let input = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes([0x32]),
        "本文 ⭐️",
        SanitizedDocumentRootStyleKey::Default,
    );
    let mut root = SanitizedDocumentRootFactory::default()
        .retain(input)
        .expect("minimal sanitized root is retained");
    let frame = run_frame(&mut root).expect("minimal sanitized root renders");
    assert_eq!(frame.record().revision(), 1);
}

#[test]
fn public_nonempty_tab_projection_reaches_the_retained_root() {
    let projection = SanitizedTabProjection::new(vec![
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0x41]),
            0,
            "作業",
        )
        .with_capabilities(
            SanitizedTabGroupCapabilities::new()
                .collapse_state(true)
                .menu_state(true),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x42]), 0, "本文")
                .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
        ),
    ]);
    let input = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes([0x43]),
        "本文 ⭐️",
        SanitizedDocumentRootStyleKey::Default,
    )
    .with_tab_projection(projection);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input)
        .expect("public nonempty tab projection is retained");

    let frame = run_frame(&mut root).expect("public nonempty tab projection renders");
    assert_eq!(frame.record().revision(), 1);
    assert!(!frame.record().record_hash().is_empty());
}

#[test]
fn invalid_public_command_icon_fails_closed_at_the_root_boundary() {
    let projection = SanitizedCommandProjection::new(vec![
        SanitizedCommandGroup::new(0, "操作").item(
            SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes([0x51]),
                0,
                "壊れたアイコン",
            )
            .with_icon(UiIconProps::new("<not-svg")),
        ),
    ]);
    let input = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes([0x52]),
        "本文",
        SanitizedDocumentRootStyleKey::Default,
    )
    .with_command_projection(projection);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input)
        .expect("invalid icon is rejected while rendering, not while retaining");

    let error = run_frame(&mut root).expect_err("invalid icon must fail closed");
    assert!(matches!(
        error,
        katana_ui_core_egui_adapter::text_command_surface::SanitizedDocumentRootFactoryError::Render(_)
    ));
}
