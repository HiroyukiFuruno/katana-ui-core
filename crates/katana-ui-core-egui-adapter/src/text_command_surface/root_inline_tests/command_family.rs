#[test]
fn duplicate_command_family_is_rejected_before_render() -> Result<(), Box<dyn std::error::Error>> {
    let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
        .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("p", "P")))
        .with_floating_toolbar(
            CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
            FloatingCommandToolbarVisibility::Visible,
        );
    let mut root = EguiTextCommandSurfaceRoot::with_identity("duplicate-family", surface)?;
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard()?;
    let mut result = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(ROOT_FRAME_WIDTH, ROOT_FRAME_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui, &style)),
    );
    platform_output.textures_delta.clear();
    let error = result
        .ok_or_else(|| "root invocation missing".to_owned())?
        .err()
        .ok_or_else(|| "duplicate family unexpectedly rendered".to_owned())?;
    assert!(matches!(
        error,
        EguiTextCommandSurfaceRootError::Surface(
            EguiTextCommandSurfaceError::DuplicateCommandFamilyMount { .. }
        )
    ));

    Ok(())
}

#[test]
fn source_address_lease_mounts_before_legacy_children_and_keeps_state_local()
-> Result<(), Box<dyn std::error::Error>> {
    let surface = EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface())
        .with_toolbar(CommandChromeToolbar::new().action(CommandChromeAction::new("p", "P")));
    let mut root = EguiTextCommandSurfaceRoot::with_identity("source-address-root", surface)?;
    root.attach_source_address(SourceAddressProjectionLease::new(SourceAddressStrip::new(
        SourceAddressPresentation::new("ソース", "ソースの説明", "ソース入力"),
    )));

    let output = render(&context_for_test(), &mut root)?;

    assert_eq!(
        output
            .events
            .current_context()
            .source_address_submission_count(),
        0
    );
    assert!(
        output
            .evidence_composite
            .rgba_pixels
            .iter()
            .any(|pixel| *pixel != 0)
    );

    Ok(())
}
