use super::*;
use crate::egui::text_command_surface::EguiTextCommandSurfaceChild;

#[test]
fn retain_with_lease_rejects_a_versioned_duplicate_family() {
    let family = crate::molecule::command_chrome::CommandChromeFamilyId::new("duplicate-family");
    let toolbar = CommandChromeToolbarPresentation {
        actions: vec![CommandChromeAction::new("primary", "Primary")],
        groups: Vec::new(),
        display_mode: Default::default(),
        density: Default::default(),
        overflow_strategy: Default::default(),
    };
    let mut duplicate_family_presentation = presentation();
    duplicate_family_presentation.toolbar = Some(toolbar.clone());
    duplicate_family_presentation.floating = Some(
        crate::egui::text_command_surface::EguiTextCommandSurfaceFloatingPresentation {
            toolbar,
            visibility: FloatingCommandToolbarVisibility::Visible,
        },
    );
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        1,
        b"duplicate-family-lease-target",
        duplicate_family_presentation,
        TextCommandSurfaceStyle::standard().expect("standard style"),
        EguiTextCommandSurfaceCommandFamilyProjection::new(Some(family.clone()), Some(family)),
    )
    .expect("versioned token");

    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(EguiTextCommandSurfaceHostProjectionLease::new(
            token,
            |_context| Ok(None),
        ))
        .expect("retain with versioned duplicate family token");
    let mut result = None;
    let context = egui::Context::default();
    let mut output = context.run_ui(
        RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..RawInput::default()
        },
        |ui| result = Some(root.show(ui)),
    );
    output.textures_delta.clear();
    match result.expect("render result") {
        Err(EguiTextCommandSurfaceRootFactoryError::Root(message)) => {
            assert!(
                message.contains("command family is mounted"),
                "unexpected root error: {message}"
            );
        }
        Err(error) => panic!("unexpected render error: {error}"),
        Ok(_) => panic!("duplicate family must fail closed during rendering"),
    }
}

#[test]
fn synchronize_with_lease_attaches_source_address_tab_strip_and_aux_lease_slots() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"router-slot-target",
        presentation(),
        style.clone(),
    )
    .expect("router lease token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");

    let status_bar = StatusBar::new("status");
    let diagnostics = DiagnosticsList::new("diagnostics");
    let source_address = SourceAddressProjectionLease::new(SourceAddressStrip::new(
        SourceAddressPresentation::new("source-address", "show source address", "アクセシビリティ"),
    ));
    let tab_strip = TabStripProjectionLease::new(
        TabStripProjection::new(
            2,
            TabStripCorrelation::from_opaque_bytes("tab-strip-correlation"),
        )
        .navigation(
            TabStripNavigationPresentation::new(
                TabStripControlPresentation::new(
                    TabStripText::new("prev"),
                    TabStripText::new("prev-a11y"),
                ),
                TabStripControlPresentation::new(
                    TabStripText::new("next"),
                    TabStripText::new("next-a11y"),
                ),
            )
            .overflow(TabStripControlPresentation::new(
                TabStripText::new("more"),
                TabStripText::new("more-a11y"),
            )),
        )
        .scroll_presentation(TabStripScrollPresentation::default().request_active_reveal(true)),
    );
    let status = StatusDiagnosticsProjectionLease::new()
        .with_status_bar(status_bar.segment(StatusBarSegment::new("status-segment", "status")))
        .with_diagnostics_list(diagnostics);
    let editor_viewport = EditorViewportProjectionLease::new(
        crate::render_model::UiImageSurfaceProps::new("preview", 1, 1, vec![255, 0, 0, 255])
            .expect("editor preview"),
    )
    .with_split_ratio_percent(50)
    .expect("split ratio");
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"router-slot-target",
            presentation(),
            TextCommandSurfaceStyle::standard().expect("standard style"),
        )
        .expect("updated lease token"),
        |_context| Ok(None),
    )
    .with_source_address(source_address)
    .with_tab_strip(tab_strip)
    .with_status_diagnostics(status)
    .with_editor_viewport(editor_viewport);

    assert!(root.synchronize_with_lease(lease).is_ok());
}

#[test]
fn synchronize_with_lease_removes_an_omitted_source_address_projection() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let target = b"source-address-removal-target";
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token(
                1,
                target,
                presentation(),
                style.clone(),
            )
            .expect("initial token"),
        )
        .expect("retain initial root");
    let source_address = SourceAddressProjectionLease::new(SourceAddressStrip::new(
        SourceAddressPresentation::new("source-address", "show source address", "accessibility"),
    ));
    let with_source = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            target,
            presentation(),
            style.clone(),
        )
        .expect("source-address lease token"),
        |_context| Ok(None),
    )
    .with_source_address(source_address);
    root.synchronize_with_lease(with_source)
        .expect("source-address lease synchronizes");

    let context = egui::Context::default();
    let render_children = |root: &mut EguiTextCommandSurfaceHostRoot| {
        let mut output = None;
        let mut platform_output = context.run_ui(RawInput::default(), |ui| {
            output = Some(root.show_output_for_test(ui));
        });
        platform_output.textures_delta.clear();
        output
            .expect("root renders")
            .expect("root output")
            .artifact_order()
            .to_vec()
    };
    assert!(render_children(&mut root).contains(&EguiTextCommandSurfaceChild::SourceAddress));

    let without_source = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(3, target, presentation(), style)
            .expect("source-address removal lease token"),
        |_context| Ok(None),
    );
    root.synchronize_with_lease(without_source)
        .expect("lease without source-address synchronizes");
    assert!(
        !render_children(&mut root).contains(&EguiTextCommandSurfaceChild::SourceAddress),
        "a lease omitting source-address must detach the previous strip"
    );
}
