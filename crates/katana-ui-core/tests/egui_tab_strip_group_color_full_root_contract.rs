#![cfg(feature = "egui")]

use egui::epaint::Shape;
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TabStripCorrelation, TabStripGroupCapabilities,
    TabStripGroupColorPresentation, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("group-colors").value("本文"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"group-colors-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("group-colors-text")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap()
}

fn output(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
) -> egui::FullOutput {
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..Default::default()
        },
        |ui| {
            root.show(ui).unwrap();
        },
    );
    output.textures_delta.clear();
    output
}

fn contains_fill(output: &egui::FullOutput, color: RgbaColor) -> bool {
    let expected =
        egui::Color32::from_rgba_unmultiplied(color.red, color.green, color.blue, color.alpha);
    output
        .shapes
        .iter()
        .any(|clipped| matches!(&clipped.shape, Shape::Rect(rect) if rect.fill == expected))
}

fn colored_rects(output: &egui::FullOutput, color: RgbaColor) -> Vec<egui::Rect> {
    let expected =
        egui::Color32::from_rgba_unmultiplied(color.red, color.green, color.blue, color.alpha);
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            Shape::Rect(rect) if rect.fill == expected => Some(rect.rect),
            _ => None,
        })
        .collect()
}

fn body_region(
    output: &egui::FullOutput,
    group_colors: &[RgbaColor],
) -> (egui::Rect, egui::Color32) {
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            Shape::Rect(rect)
                if !group_colors.iter().any(|color| {
                    rect.fill
                        == egui::Color32::from_rgba_unmultiplied(
                            color.red,
                            color.green,
                            color.blue,
                            color.alpha,
                        )
                }) =>
            {
                Some((rect.rect, rect.fill))
            }
            _ => None,
        })
        .max_by(|(left, _), (right, _)| {
            (left.width() * left.height())
                .partial_cmp(&(right.width() * right.height()))
                .expect("finite body paint bounds")
        })
        .expect("body paint region")
}

fn colored_group(
    target: &'static [u8],
    label: &str,
    colors: (RgbaColor, RgbaColor, RgbaColor, RgbaColor),
    collapsed: bool,
    with_member: bool,
) -> TabStripGroupDescriptor {
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(target),
        TabStripText::new(label),
    )
    .capabilities(
        TabStripGroupCapabilities::new()
            .collapsible(true)
            .collapsed(collapsed),
    )
    .color_presentation(TabStripGroupColorPresentation::new(
        colors.0, colors.1, colors.2, colors.3,
    ));
    if with_member {
        group.tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(target),
                TabStripText::new("member"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        )
    } else {
        group
    }
}

fn colored_lease(
    revision: u64,
    groups: Vec<TabStripGroupDescriptor>,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease {
    let projection = groups.into_iter().fold(
        TabStripProjection::new(
            revision,
            TabStripCorrelation::from_opaque_bytes(b"same-label-colors"),
        ),
        |projection, group| projection.group(group),
    );
    katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
        token(revision),
        |_| Ok(None),
    )
    .with_tab_strip(TabStripProjectionLease::new(projection))
}

#[test]
fn public_full_root_paints_independent_expanded_collapsed_and_member_colors() {
    let expanded_marker = RgbaColor::new(221, 31, 47, 255);
    let expanded_background = RgbaColor::new(71, 23, 31, 255);
    let expanded_member = RgbaColor::new(99, 29, 37, 255);
    let collapsed_marker = RgbaColor::new(31, 121, 221, 255);
    let collapsed_background = RgbaColor::new(23, 43, 71, 255);

    let expanded = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"expanded"),
        TabStripText::new("Expanded"),
    )
    .capabilities(TabStripGroupCapabilities::new().collapsible(true))
    .color_presentation(TabStripGroupColorPresentation::new(
        expanded_marker,
        expanded_background,
        expanded_background,
        expanded_member,
    ))
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"expanded-member"),
            TabStripText::new("Expanded member"),
        )
        .capabilities(TabStripTabCapabilities::new().selectable(true)),
    );
    let collapsed = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"collapsed"),
        TabStripText::new("Collapsed"),
    )
    .capabilities(
        TabStripGroupCapabilities::new()
            .collapsible(true)
            .collapsed(true),
    )
    .color_presentation(TabStripGroupColorPresentation::new(
        collapsed_marker,
        collapsed_background,
        collapsed_background,
        RgbaColor::new(29, 57, 99, 255),
    ))
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"collapsed-member"),
            TabStripText::new("Collapsed member"),
        )
        .capabilities(TabStripTabCapabilities::new().selectable(true)),
    );

    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"group-colors-correlation"),
    )
    .group(expanded)
    .group(collapsed);
    let lease = TabStripProjectionLease::new(projection);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let output = output(&egui::Context::default(), &mut root);

    for color in [
        expanded_marker,
        expanded_background,
        expanded_member,
        collapsed_marker,
        collapsed_background,
    ] {
        assert!(
            contains_fill(&output, color),
            "missing rendered RGBA fill: {color:?}"
        );
    }
}

#[test]
fn public_full_root_tracks_same_label_targets_color_updates_membership_and_regeneration() {
    let a_marker = RgbaColor::new(220, 40, 50, 180);
    let a_background = RgbaColor::new(80, 20, 30, 180);
    let a_collapsed = RgbaColor::new(60, 15, 25, 180);
    let a_member = RgbaColor::new(110, 25, 35, 180);
    let b_marker = RgbaColor::new(30, 140, 70, 200);
    let b_background = RgbaColor::new(20, 70, 40, 200);
    let b_collapsed = RgbaColor::new(15, 50, 30, 200);
    let b_member = RgbaColor::new(25, 95, 50, 200);
    let updated_a_marker = RgbaColor::new(230, 100, 20, 180);
    let regenerated_marker = RgbaColor::new(40, 80, 220, 210);
    let regenerated_background = RgbaColor::new(20, 35, 100, 210);
    let regenerated_collapsed = RgbaColor::new(15, 25, 70, 210);
    let regenerated_member = RgbaColor::new(25, 45, 150, 210);

    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(colored_lease(
            1,
            vec![
                colored_group(
                    b"target-a",
                    "Same",
                    (a_marker, a_background, a_collapsed, a_member),
                    false,
                    true,
                ),
                colored_group(
                    b"target-b",
                    "Same",
                    (b_marker, b_background, b_collapsed, b_member),
                    false,
                    true,
                ),
            ],
        ))
        .unwrap();
    let context = egui::Context::default();
    let initial = output(&context, &mut root);
    let body_colors = [
        a_marker,
        a_background,
        a_collapsed,
        a_member,
        b_marker,
        b_background,
        b_collapsed,
        b_member,
    ];
    let body_before = body_region(&initial, &body_colors);
    let a_rects = colored_rects(&initial, a_background);
    let b_rects = colored_rects(&initial, b_background);
    assert_eq!(a_rects.len(), 1);
    assert_eq!(b_rects.len(), 1);
    assert_ne!(
        a_rects[0], b_rects[0],
        "same labels retain distinct target regions"
    );
    assert_eq!(a_rects[0].width(), b_rects[0].width());
    assert!(contains_fill(&initial, a_member));
    assert!(contains_fill(&initial, b_member));
    assert!(contains_fill(&initial, a_marker));
    assert!(contains_fill(&initial, b_marker));

    root.synchronize_with_lease(colored_lease(
        2,
        vec![
            colored_group(
                b"target-a",
                "Same",
                (updated_a_marker, a_background, a_collapsed, a_member),
                false,
                true,
            ),
            colored_group(
                b"target-b",
                "Same",
                (b_marker, b_background, b_collapsed, b_member),
                true,
                true,
            ),
        ],
    ))
    .unwrap();
    let collapsed = output(&context, &mut root);
    assert_eq!(body_before, body_region(&collapsed, &body_colors));
    assert!(contains_fill(&collapsed, updated_a_marker));
    assert!(!contains_fill(&collapsed, a_marker));
    assert!(contains_fill(&collapsed, b_collapsed));
    assert!(!contains_fill(&collapsed, b_member));
    assert_eq!(colored_rects(&collapsed, b_collapsed).len(), 1);
    assert_eq!(colored_rects(&collapsed, a_background), a_rects);
    assert_eq!(colored_rects(&collapsed, b_background).len(), 0);

    root.synchronize_with_lease(colored_lease(
        3,
        vec![
            colored_group(
                b"target-a",
                "Same",
                (updated_a_marker, a_background, a_collapsed, a_member),
                false,
                true,
            ),
            colored_group(
                b"target-b",
                "Same",
                (b_marker, b_background, b_collapsed, b_member),
                false,
                false,
            ),
        ],
    ))
    .unwrap();
    let reopened = output(&context, &mut root);
    assert!(contains_fill(&reopened, b_background));
    assert!(!contains_fill(&reopened, b_member));
    assert!(contains_fill(&reopened, a_member));

    root.synchronize_with_lease(colored_lease(
        4,
        vec![
            colored_group(
                b"target-a",
                "Same",
                (updated_a_marker, a_background, a_collapsed, a_member),
                false,
                true,
            ),
            colored_group(
                b"target-c-regenerated",
                "Same",
                (
                    regenerated_marker,
                    regenerated_background,
                    regenerated_collapsed,
                    regenerated_member,
                ),
                false,
                true,
            ),
        ],
    ))
    .unwrap();
    let regenerated = output(&context, &mut root);
    assert!(!contains_fill(&regenerated, b_marker));
    assert!(contains_fill(&regenerated, regenerated_marker));
    assert!(contains_fill(&regenerated, regenerated_background));
    assert!(contains_fill(&regenerated, regenerated_member));
    assert!(contains_fill(&regenerated, a_member));
}

#[test]
fn rejected_member_operation_does_not_leak_group_color_into_next_root_frame() {
    use katana_ui_core::egui::text_command_surface::{
        EguiTextCommandSurfaceHostProjectionLease, TabStripProposal, TabStripProposalOperation,
        TabStripProposalPort, TabStripProposalPortError,
    };
    use std::cell::Cell;
    use std::rc::Rc;

    struct RejectSelection(Rc<Cell<bool>>);
    impl TabStripProposalPort for RejectSelection {
        fn forward_proposal(
            &mut self,
            proposal: TabStripProposal,
        ) -> Result<(), TabStripProposalPortError> {
            if matches!(
                proposal.into_operation(),
                TabStripProposalOperation::SelectTab(_)
            ) {
                self.0.set(true);
                Err(TabStripProposalPortError::Rejected)
            } else {
                Ok(())
            }
        }
    }
    let rejected = Rc::new(Cell::new(false));
    let member = RgbaColor::new(207, 18, 49, 255);
    let background = RgbaColor::new(34, 45, 56, 255);
    let projection =
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"rejected-color"))
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"root"),
                    TabStripText::new("Root"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            )
            .group(
                TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"group"),
                    TabStripText::new("Group"),
                )
                .color_presentation(TabStripGroupColorPresentation::new(
                    RgbaColor::new(20, 100, 200, 255),
                    background,
                    background,
                    member,
                ))
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"member"),
                        TabStripText::new("Member"),
                    )
                    .capabilities(TabStripTabCapabilities::new().selectable(true)),
                ),
            );
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token(1), |_| Ok(None)).with_tab_strip(
                TabStripProjectionLease::new(projection)
                    .with_proposal_port(RejectSelection(Rc::clone(&rejected))),
            ),
        )
        .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let before = output(&context, &mut root);
    let nodes = &before
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes;
    let (member_id, _) = nodes
        .iter()
        .find(|(_, node)| node.label() == Some("Member"))
        .unwrap();
    let mut failed = false;
    let mut failed_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            events: vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    target_tree: egui::accesskit::TreeId::ROOT,
                    action: egui::accesskit::Action::Click,
                    target_node: *member_id,
                    data: None,
                },
            )],
            ..Default::default()
        },
        |ui| {
            failed = root.show(ui).is_err();
        },
    );
    failed_output.textures_delta.clear();
    assert!(failed);
    assert!(rejected.get());
    let after = output(&context, &mut root);
    assert_eq!(colored_rects(&before, member).len(), 1);
    assert_eq!(
        colored_rects(&before, member),
        colored_rects(&after, member)
    );
}
