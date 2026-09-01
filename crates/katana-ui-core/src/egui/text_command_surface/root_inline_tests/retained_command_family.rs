#[test]
fn distinct_command_families_render_once_in_their_slots() -> Result<(), Box<dyn std::error::Error>>
{
    let surface = EguiTextCommandSurface::new(selected_surface())
        .with_toolbar(
            CommandChromeToolbar::new()
                .command_family(CommandChromeFamilyId::new("primary"))
                .action(CommandChromeAction::new("p", "P")),
        )
        .with_floating_toolbar(
            CommandChromeToolbar::new()
                .command_family(CommandChromeFamilyId::new("floating"))
                .action(CommandChromeAction::new("f", "F")),
            FloatingCommandToolbarVisibility::Visible,
        );
    let mut root = EguiTextCommandSurfaceRoot::with_identity("distinct-families", surface)?;
    let output = render(&context_for_test(), &mut root)?;
    assert_eq!(
        output.toolbar_record.as_ref().map(|record| {
            record
                .actions
                .iter()
                .filter(|action| action.action_id == "p")
                .count()
        }),
        Some(1)
    );
    assert_eq!(
        output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .map(|record| {
                record
                    .toolbar
                    .actions
                    .iter()
                    .filter(|action| action.action_id == "f")
                    .count()
            }),
        Some(1)
    );

    Ok(())
}

#[test]
fn retained_family_update_preserves_text_selection_scroll_focus_and_composition()
-> Result<(), Box<dyn std::error::Error>> {
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "retained-family-update",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    let _ = root.surface.text.apply_action(TextSurfaceAction::ScrollBy {
        delta_x: 0,
        delta_y: 24,
    });
    let _ = root
        .surface
        .text
        .apply_action(TextSurfaceAction::SetFocus(true));
    let _ = root
        .surface
        .text
        .apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
            TextAreaSelection { start: 1, end: 3 },
        )));
    let _ =
        root.surface
            .text
            .apply_action(TextSurfaceAction::TextArea(TextAreaAction::composition(
                TextAreaCompositionPhase::Update,
                "入力中⭐️",
                3,
            )));
    let before = root.surface.text.state().clone();

    assert!(root.synchronize_command_families(
        Some(CommandChromeFamilyId::new("primary-next")),
        Some(CommandChromeFamilyId::new("floating-next")),
    ));
    assert_eq!(root.surface.text.state(), &before);

    Ok(())
}

#[test]
fn floating_only_surface_remains_supported() -> Result<(), Box<dyn std::error::Error>> {
    let surface = EguiTextCommandSurface::new(selected_surface()).with_floating_toolbar(
        CommandChromeToolbar::new().action(CommandChromeAction::new("f", "F")),
        FloatingCommandToolbarVisibility::Visible,
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity("floating-only", surface)?;
    let output = render(&context_for_test(), &mut root)?;
    assert!(output.toolbar_record.is_none());
    assert!(output.floating.and_then(|value| value.record).is_some());

    Ok(())
}
