#[test]
fn tab_strip_navigation_forwards_enabled_previous_and_rejects_disabled_next()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::text_command_surface::{
        TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
        TabStripProjection, TabStripProjectionLease, TabStripSurfaceCapabilities, TabStripText,
    };

    let directions = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"navigation-correlation"),
    )
    .capabilities(TabStripSurfaceCapabilities::new().previous_available(true))
    .navigation(TabStripNavigationPresentation::new(
        TabStripControlPresentation::new(
            TabStripText::new("Previous"),
            TabStripText::new("Previous tab"),
        ),
        TabStripControlPresentation::new(TabStripText::new("Next"), TabStripText::new("Next tab")),
    ));
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-navigation-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(NavigationTabStripPort(Rc::clone(&directions))),
    )?;

    let context = context_for_test();
    let _ = render(&context, &mut root)?;
    let previous = egui::pos2(NAVIGATION_PREVIOUS_X, TAB_STRIP_POINTER_Y);
    let next = egui::pos2(NAVIGATION_NEXT_X, TAB_STRIP_POINTER_Y);
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(previous, true)],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(previous, false)],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(&[true], directions.borrow().as_slice());

    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(next, true)],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(next, false)],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(&[true], directions.borrow().as_slice());

    Ok(())
}

#[test]
fn tab_strip_accesskit_keyboard_and_same_frame_activation_forward_only_current_routes()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::text_command_surface::{
        TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
        TabStripProjection, TabStripProjectionLease, TabStripSurfaceCapabilities, TabStripText,
    };

    let directions = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"accesskit-navigation-correlation"),
    )
    .capabilities(TabStripSurfaceCapabilities::new().previous_available(true))
    .navigation(TabStripNavigationPresentation::new(
        TabStripControlPresentation::new(
            TabStripText::new("Previous"),
            TabStripText::new("Previous tab"),
        ),
        TabStripControlPresentation::new(TabStripText::new("Next"), TabStripText::new("Next tab")),
    ));
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-accesskit-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(NavigationTabStripPort(Rc::clone(&directions))),
    )?;

    let context = context_for_test();
    context.enable_accesskit();
    let (initial_platform, initial) =
        render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
    let (previous_node, previous_bounds, previous_disabled) =
        accesskit_button(&initial_platform, "Previous tab")?;
    let (next_node, _, next_disabled) = accesskit_button(&initial_platform, "Next tab")?;
    assert!(!previous_disabled);
    assert!(next_disabled);
    assert!(previous_bounds.width() > 0.0 && previous_bounds.height() > 0.0);
    assert_eq!(64, initial.frame().accessibility().snapshot_hash().len());
    let public_frame = format!("{:?}", initial.frame());
    for forbidden in ["Previous tab", "accesskit-navigation-correlation"] {
        assert!(!public_frame.contains(forbidden));
    }

    let (accesskit_platform, _) = render_with_platform_output(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![accesskit_click(previous_node)],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(&[true], directions.borrow().as_slice());
    assert!(
        accesskit_platform
            .platform_output
            .accesskit_update
            .is_some()
    );

    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(previous_bounds.center(), true)],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(previous_bounds.center(), false)],
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
    assert_eq!(&[true, true, true], directions.borrow().as_slice());

    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(previous_bounds.center(), true)],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                pointer_button(previous_bounds.center(), false),
                key_press(egui::Key::Enter),
            ],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(
        &[true, true, true, true],
        directions.borrow().as_slice(),
        "pointer and keyboard activation in one frame must forward one proposal"
    );

    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![accesskit_click(next_node)],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(
        &[true, true, true, true],
        directions.borrow().as_slice(),
        "a disabled current-frame route must reject an AccessKit click"
    );

    Ok(())
}

#[test]
fn tab_strip_trailing_control_maps_close_and_pinned_to_distinct_proposals()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
    use crate::text_command_surface::{
        TabStripControlPresentation, TabStripCorrelation, TabStripProjection,
        TabStripProjectionLease, TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget,
        TabStripText,
    };

    let normal = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"closable"),
        TabStripText::new("A"),
    )
    .capabilities(TabStripTabCapabilities::new().closeable(true))
    .trailing_control(TabStripControlPresentation::new(
        TabStripText::new("Close"),
        TabStripText::new("Close tab"),
    ));
    let pinned = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"pinned"),
        TabStripText::new("B"),
    )
    .capabilities(TabStripTabCapabilities::new().pinned(true))
    .trailing_control(TabStripControlPresentation::new(
        TabStripText::new("Unpin"),
        TabStripText::new("Unpin tab"),
    ));
    let forwarded = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"trailing-correlation"),
    )
    .tab(normal)
    .tab(pinned);
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-trailing-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
    )?;

    let context = context_for_test();
    let _ = render(&context, &mut root)?;
    let close = egui::pos2(TRAILING_CLOSE_X, TAB_STRIP_POINTER_Y);
    let unpin = egui::pos2(TRAILING_UNPIN_X, TAB_STRIP_POINTER_Y);
    for position in [close, unpin] {
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(position, true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(position, false)],
                ..egui::RawInput::default()
            },
        )?;
    }
    assert_eq!(
        [
            TabStripProposalOperationClass::RequestClose,
            TabStripProposalOperationClass::Unpin,
        ],
        forwarded.borrow().as_slice(),
    );

    Ok(())
}
