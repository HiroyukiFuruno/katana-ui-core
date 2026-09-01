const GROUP_SWATCH_RED: u8 = 253;
const GROUP_SWATCH_GREEN: u8 = 211;
const GROUP_SWATCH_BLUE: u8 = 98;
const GROUP_SWATCH_ALPHA: u8 = 255;

#[test]
fn tab_strip_context_menu_uses_secondary_input_accesskit_and_one_opaque_route()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
    use crate::egui::text_command_surface::{
        TabStripContextMenuPresentation, TabStripCorrelation, TabStripMenuEntry,
        TabStripMenuOperation, TabStripProjection, TabStripProjectionLease, TabStripTabDescriptor,
        TabStripTabTarget, TabStripText,
    };

    let forwarded = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"tab-context-menu-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab-context-menu-target"),
            TabStripText::new("日本語 ⭐️"),
        )
        .context_menu(TabStripContextMenuPresentation::new().entry(
            TabStripMenuEntry::action(
                TabStripText::new("閉じる ⭐️"),
                TabStripText::new("閉じる ⭐️"),
                TabStripMenuOperation::RequestClose,
            ),
        )),
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-context-menu-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
    )?;
    let context = context_for_test();
    context.enable_accesskit();
    let initial = render(&context, &mut root)?;
    let tab = egui::pos2(TAB_STRIP_POINTER_X, TAB_STRIP_POINTER_Y);
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![secondary_pointer_button(tab, true)],
            ..egui::RawInput::default()
        },
    )?;
    let (opened_platform, opened) = render_with_platform_output(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![secondary_pointer_button(tab, false)],
            ..egui::RawInput::default()
        },
    )?;
    let (_, bounds, _disabled) = accesskit_button(&opened_platform, "閉じる ⭐️")?;
    assert_ne!(
        initial.evidence_composite.pixel_hash, opened.evidence_composite.pixel_hash,
        "the root artifact must include the actual foreground menu"
    );
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(bounds.center()),
                pointer_button(bounds.center(), true),
            ],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(bounds.center()),
                pointer_button(bounds.center(), false),
            ],
            ..egui::RawInput::default()
        },
    )?;
    assert!(bounds.width() > 0.0 && bounds.height() > 0.0);
    assert_eq!(
        &[TabStripProposalOperationClass::RequestClose],
        forwarded.borrow().as_slice(),
        "the menu must forward only the route table's opaque close proposal"
    );

    Ok(())
}

#[test]
fn group_popup_rasters_rename_and_projects_host_swatch_without_local_confirmation()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
    use crate::egui::text_command_surface::{
        TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
        TabStripGroupTarget, TabStripProjection, TabStripProjectionLease, TabStripSwatchDescriptor,
        TabStripSwatchTarget, TabStripText,
    };
    use crate::molecule::RgbaColor;

    let forwarded = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"group-popup-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group-popup-target"),
            TabStripText::new("グループ ⭐️"),
        )
        .swatch(
            TabStripSwatchDescriptor::new(
                TabStripSwatchTarget::from_opaque_bytes(b"group-popup-swatch"),
                RgbaColor::new(
                    GROUP_SWATCH_RED,
                    GROUP_SWATCH_GREEN,
                    GROUP_SWATCH_BLUE,
                    GROUP_SWATCH_ALPHA,
                ),
            )
            .accessibility_label(TabStripText::new("黄色 ⭐️")),
        )
        .popup(
            TabStripGroupPopupPresentation::new()
                .rename_placeholder(TabStripText::new("グループ名 ⭐️")),
        ),
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-group-popup-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
    )?;
    let context = context_for_test();
    context.enable_accesskit();
    let _ = render(&context, &mut root)?;
    let header = egui::pos2(TAB_STRIP_POINTER_X, TAB_STRIP_POINTER_Y);
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![secondary_pointer_button(header, true)],
            ..egui::RawInput::default()
        },
    )?;
    let (opened_platform, opened) = render_with_platform_output(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![secondary_pointer_button(header, false)],
            ..egui::RawInput::default()
        },
    )?;
    let input = accesskit_text_input(&opened_platform, "グループ名 ⭐️")?;
    assert!(input.width() > 0.0 && input.height() > 0.0);
    assert!(opened.evidence_composite.non_transparent_pixel_count > 0);
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(input.center()),
                pointer_button(input.center(), true),
            ],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(input.center()),
                pointer_button(input.center(), false),
            ],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![key_press(egui::Key::Enter)],
            ..egui::RawInput::default()
        },
    )?;
    assert!(
        forwarded.borrow().is_empty(),
        "an unchanged group name must not create a rename proposal"
    );
    let preedit = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                text: "変更 ⭐️".to_owned(),
                active_range_chars: None,
            })],
            ..egui::RawInput::default()
        },
    )?;
    assert_ne!(
        opened.evidence_composite.pixel_hash, preedit.evidence_composite.pixel_hash,
        "IME preedit must be platform-rastered in the foreground popup"
    );
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::Text("変更 ⭐️".to_owned())],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![key_press(egui::Key::Enter)],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(
        &[TabStripProposalOperationClass::RenameGroup],
        forwarded.borrow().as_slice(),
        "rename must leave KUC only as its one-shot opaque group proposal"
    );

    Ok(())
}
