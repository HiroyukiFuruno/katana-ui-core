#[test]
fn tab_strip_click_forwards_one_proposal_without_locally_accepting_presentation()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::text_command_surface::{
        TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities,
        TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    };

    let count = Rc::new(Cell::new(0));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"root-test-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"root-test-tab"),
            TabStripText::new("日本語 ⭐️"),
        )
        .capabilities(TabStripTabCapabilities::new().selectable(true)),
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(CountingTabStripPort(Rc::clone(&count))),
    )?;

    let context = context_for_test();
    let initial = render(&context, &mut root)?;
    let tab_center = egui::pos2(TAB_STRIP_POINTER_X, TAB_STRIP_POINTER_Y);
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(tab_center, true)],
            ..egui::RawInput::default()
        },
    )?;
    let released = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(tab_center, false)],
            ..egui::RawInput::default()
        },
    )?;

    assert_eq!(1, count.get());
    assert_eq!(
        initial.evidence_composite.pixel_hash, released.evidence_composite.pixel_hash,
        "a proposal cannot mutate retained tab presentation before a newer host lease"
    );
    assert!(released.evidence_composite.non_transparent_pixel_count > 0);

    Ok(())
}

#[test]
fn tab_strip_drag_uses_physical_pointer_and_forwards_one_start_then_one_destination()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
    use crate::text_command_surface::{
        TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities,
        TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    };

    let forwarded = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"tab-drag-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab-drag-source"),
            TabStripText::new("source ⭐️"),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(true),
        ),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"tab-drag-destination"),
            TabStripText::new("destination ⭐️"),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .accepts_tab_drop(true),
        ),
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-drag-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
    )?;
    let context = context_for_test();
    context.enable_accesskit();
    let (initial_platform, initial) =
        render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
    let (_, source, _) = accesskit_button(&initial_platform, "source ⭐️")?;
    let (_, destination, _) = accesskit_button(&initial_platform, "destination ⭐️")?;
    let source_pointer = source.center();
    let destination_pointer = egui::pos2(destination.max.x - 2.0, destination.center().y);

    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(source_pointer, true)],
            ..egui::RawInput::default()
        },
    )?;
    let dragging = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![egui::Event::PointerMoved(destination_pointer)],
            ..egui::RawInput::default()
        },
    )?;
    assert_ne!(
        initial.evidence_composite.pixel_hash, dragging.evidence_composite.pixel_hash,
        "an active drag must be visible in the root-owned artifact"
    );
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(destination_pointer),
                pointer_button(destination_pointer, false),
            ],
            ..egui::RawInput::default()
        },
    )?;
    assert_eq!(
        [
            TabStripProposalOperationClass::StartDrag,
            TabStripProposalOperationClass::FinishDragAfter,
        ],
        forwarded.borrow().as_slice(),
        "a drag must be a single start proposal followed by one opaque destination proposal"
    );

    Ok(())
}
