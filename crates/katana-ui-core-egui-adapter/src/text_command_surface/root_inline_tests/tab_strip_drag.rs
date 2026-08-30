#[test]
fn tab_strip_drag_cancels_for_escape_or_a_host_rejected_destination()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
    use crate::text_command_surface::{
        TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities,
        TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    };

    let projection = || {
        TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"tab-drag-cancel-correlation"),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-drag-cancel-source"),
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
                TabStripTabTarget::from_opaque_bytes(b"tab-drag-rejected-destination"),
                TabStripText::new("rejected ⭐️"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        )
    };

    for escape in [false, true] {
        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            format!("tab-strip-drag-cancel-root-{escape}"),
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection())
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;
        let context = context_for_test();
        context.enable_accesskit();
        let (platform, _) =
            render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
        let (_, source, _) = accesskit_button(&platform, "source ⭐️")?;
        let (_, rejected, _) = accesskit_button(&platform, "rejected ⭐️")?;
        let source_pointer = source.center();
        let rejected_pointer = rejected.center();
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(source_pointer, true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::PointerMoved(rejected_pointer)],
                ..egui::RawInput::default()
            },
        )?;
        let end_event = if escape {
            key_press(egui::Key::Escape)
        } else {
            pointer_button(rejected_pointer, false)
        };
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::PointerMoved(rejected_pointer), end_event],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(
            [
                TabStripProposalOperationClass::StartDrag,
                TabStripProposalOperationClass::CancelDrag,
            ],
            forwarded.borrow().as_slice(),
            "escape and a non-accepting destination must both cancel without a fallback reorder"
        );
    }

    Ok(())
}

#[test]
fn tab_strip_drag_uses_only_host_projected_group_or_end_destinations()
-> Result<(), Box<dyn std::error::Error>> {
    use super::super::tab_strip_proposal_port::TabStripProposalOperationClass;
    use crate::text_command_surface::{
        TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor,
        TabStripGroupTarget, TabStripProjection, TabStripProjectionLease,
        TabStripSurfaceCapabilities, TabStripTabCapabilities, TabStripTabDescriptor,
        TabStripTabTarget, TabStripText,
    };

    let cases = [
        (
            "group",
            TabStripProposalOperationClass::FinishDragInGroup,
            true,
        ),
        (
            "end",
            TabStripProposalOperationClass::FinishDragAtEnd,
            false,
        ),
    ];
    for (case, expected, group_destination) in cases {
        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let mut projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(format!("tab-drag-{case}-correlation")),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(format!("tab-drag-{case}-source")),
                TabStripText::new("source ⭐️"),
            )
            .capabilities(
                TabStripTabCapabilities::new()
                    .selectable(true)
                    .draggable(true),
            ),
        );
        if group_destination {
            projection = projection.group(
                TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"tab-drag-group-destination"),
                    TabStripText::new("group ⭐️"),
                )
                .capabilities(
                    TabStripGroupCapabilities::new()
                        .collapsible(true)
                        .accepts_tab_drop(true),
                ),
            );
        } else {
            projection = projection
                .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true));
        }
        let mut root = EguiTextCommandSurfaceRoot::with_identity(
            format!("tab-strip-drag-{case}-root"),
            EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
        )?;
        root.attach_tab_strip(
            TabStripProjectionLease::new(projection)
                .with_proposal_port(TrailingTabStripPort(Rc::clone(&forwarded))),
        )?;
        let context = context_for_test();
        context.enable_accesskit();
        let (platform, _) =
            render_with_platform_output(&context, &mut root, egui::RawInput::default())?;
        let (_, source, _) = accesskit_button(&platform, "source ⭐️")?;
        let destination = if group_destination {
            let (_, group, _) = accesskit_button(&platform, "group ⭐️")?;
            group.center()
        } else {
            egui::pos2(TAB_DRAG_END_X, source.center().y)
        };
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![pointer_button(source.center(), true)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![egui::Event::PointerMoved(destination)],
                ..egui::RawInput::default()
            },
        )?;
        let _ = render_with_input(
            &context,
            &mut root,
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(destination),
                    pointer_button(destination, false),
                ],
                ..egui::RawInput::default()
            },
        )?;
        assert_eq!(
            [TabStripProposalOperationClass::StartDrag, expected],
            forwarded.borrow().as_slice(),
            "the renderer must only use the destination capability projected by the host"
        );
    }

    Ok(())
}
