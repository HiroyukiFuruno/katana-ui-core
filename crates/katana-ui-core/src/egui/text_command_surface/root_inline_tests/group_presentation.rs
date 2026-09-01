#[test]
fn group_popup_swatch_uses_its_host_projected_color_and_forwards_recolor_once()
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
        TabStripCorrelation::from_opaque_bytes(b"group-swatch-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group-swatch-target"),
            TabStripText::new("色 ⭐️"),
        )
        .swatch(
            TabStripSwatchDescriptor::new(
                TabStripSwatchTarget::from_opaque_bytes(b"group-swatch-target"),
                RgbaColor::new(17, 177, 127, 255),
            )
            .accessibility_label(TabStripText::new("緑 ⭐️")),
        )
        .popup(
            TabStripGroupPopupPresentation::new()
                .rename_placeholder(TabStripText::new("色のグループ ⭐️")),
        ),
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-group-swatch-root",
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
    let (opened_platform, _) = render_with_platform_output(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![secondary_pointer_button(header, false)],
            ..egui::RawInput::default()
        },
    )?;
    let (_, swatch, _) = accesskit_button(&opened_platform, "緑 ⭐️")?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(swatch.center()),
                pointer_button(swatch.center(), true),
            ],
            ..egui::RawInput::default()
        },
    )?;
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(swatch.center()),
                pointer_button(swatch.center(), false),
            ],
            ..egui::RawInput::default()
        },
    )?;
    assert!(swatch.width() > 0.0 && swatch.height() > 0.0);
    assert_eq!(
        &[TabStripProposalOperationClass::RecolorGroup],
        forwarded.borrow().as_slice(),
        "same-frame recolor must take precedence over a pending rename proposal"
    );

    Ok(())
}

#[test]
fn tab_strip_group_header_forwards_collapse_without_locally_changing_its_presentation()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::egui::text_command_surface::{
        TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor,
        TabStripGroupTarget, TabStripProjection, TabStripProjectionLease, TabStripText,
    };

    let forwarded = Rc::new(RefCell::new(Vec::new()));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"group-collapse-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group"),
            TabStripText::new("グループ ⭐️"),
        )
        .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
    );
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-group-collapse-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(
        TabStripProjectionLease::new(projection)
            .with_proposal_port(GroupCollapseTabStripPort(Rc::clone(&forwarded))),
    )?;

    let context = context_for_test();
    let initial = render(&context, &mut root)?;
    let group_header = egui::pos2(TAB_STRIP_POINTER_X, TAB_STRIP_POINTER_Y);
    let _ = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(group_header, true)],
            ..egui::RawInput::default()
        },
    )?;
    let released = render_with_input(
        &context,
        &mut root,
        egui::RawInput {
            events: vec![pointer_button(group_header, false)],
            ..egui::RawInput::default()
        },
    )?;

    assert_eq!(vec![true], *forwarded.borrow());
    assert_eq!(
        initial.evidence_composite.pixel_hash, released.evidence_composite.pixel_hash,
        "a group-collapse proposal cannot mutate retained presentation before host republish"
    );

    Ok(())
}

#[test]
fn tab_strip_active_reveal_changes_only_retained_scroll_artifact()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::egui::text_command_surface::{
        TabStripCorrelation, TabStripProjection, TabStripProjectionLease,
        TabStripScrollPresentation, TabStripTabCapabilities, TabStripTabDescriptor,
        TabStripTabTarget, TabStripText,
    };

    let projection = |reveal_active| {
        let mut projection = TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"active-reveal-correlation"),
        )
        .scroll_presentation(
            TabStripScrollPresentation::new().request_active_reveal(reveal_active),
        );
        for index in 0..6 {
            projection = projection.tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(format!("tab-{index}").into_bytes()),
                    TabStripText::new(format!("長いタブ {index} ⭐️")),
                )
                .capabilities(TabStripTabCapabilities::new().active(index == 5)),
            );
        }
        projection
    };
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-active-reveal-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(TabStripProjectionLease::new(projection(false)))?;
    let context = context_for_test();
    let before = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(1.0),
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;

    root.attach_tab_strip(TabStripProjectionLease::new(projection(true)))?;
    let revealed = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(2.0),
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;
    let settled = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(3.0),
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;
    let visible = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(4.0),
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;

    assert_ne!(
        before.evidence_composite.pixel_hash, visible.evidence_composite.pixel_hash,
        "active reveal must move the clipped retained tab artifact under constrained width"
    );
    assert!(
        revealed.evidence_composite.non_transparent_pixel_count > 0
            && settled.evidence_composite.non_transparent_pixel_count > 0
            && visible.evidence_composite.non_transparent_pixel_count > 0
    );

    Ok(())
}
