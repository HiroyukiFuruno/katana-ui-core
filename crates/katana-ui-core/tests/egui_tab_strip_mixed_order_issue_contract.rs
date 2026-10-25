#![cfg(feature = "egui")]

use egui::accesskit::Role;
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupTarget, TabStripProjection,
    TabStripProjectionLease, TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget,
    TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn token() -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("mixed-order").value("host"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("mixed-order-text")),
        text: TextSurfacePresentation::from_props(surface.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    };
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"mixed-order-host",
        presentation,
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap()
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 720.0),
        )),
        ..Default::default()
    }
}

#[test]
fn public_mixed_root_projection_keeps_host_occurrence_order() {
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"order"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"p"),
                TabStripText::new("P"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"g"),
                TabStripText::new("G"),
            )
            .capabilities(TabStripGroupCapabilities::new().collapsible(true))
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"m"),
                    TabStripText::new("Member"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            ),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tail"),
                TabStripText::new("Tail"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        );
    assert_order(projection, &["P", "G", "Member", "Tail"]);
}

#[test]
fn public_tab_only_and_group_only_controls_are_horizontal() {
    assert_order(
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"tabs"))
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"a"),
                    TabStripText::new("A"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            )
            .tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"b"),
                    TabStripText::new("B"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            ),
        &["A", "B"],
    );
    assert_order(
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"groups"))
            .group(
                TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"a"),
                    TabStripText::new("A"),
                )
                .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
            )
            .group(
                TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"b"),
                    TabStripText::new("B"),
                )
                .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
            ),
        &["A", "B"],
    );
}

#[test]
fn public_group_tab_group_and_collapsed_empty_group_keep_order() {
    assert_order(
        TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"group-tab-group"),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-a"),
                TabStripText::new("Group A"),
            )
            .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"middle"),
                TabStripText::new("U"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group-b"),
                TabStripText::new("Group B"),
            )
            .capabilities(
                TabStripGroupCapabilities::new()
                    .collapsible(true)
                    .collapsed(true),
            ),
        ),
        &["Group A", "U", "Group B"],
    );
}

fn assert_order(projection: TabStripProjection, labels: &[&str]) {
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(token(), |_context| Ok(None))
        .with_tab_strip(TabStripProjectionLease::new(projection));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease)
        .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut output = None;
    let mut full = context.run_ui(raw_input(), |ui| output = Some(root.show(ui).unwrap()));
    full.textures_delta.clear();
    let _ = output.expect("root output");
    let update = full
        .platform_output
        .accesskit_update
        .expect("accesskit update");
    let bounds = labels
        .iter()
        .map(|label| {
            update
                .nodes
                .iter()
                .find_map(|(_, node)| {
                    (matches!(node.role(), Role::Button | Role::Label)
                        && node.label() == Some(*label))
                    .then(|| node.bounds())
                    .flatten()
                })
                .expect(label)
        })
        .collect::<Vec<_>>();
    for rect in &bounds {
        assert!(
            [rect.x0, rect.y0, rect.x1, rect.y1]
                .iter()
                .all(|value| value.is_finite())
        );
        assert!(rect.x0 >= 0.0 && rect.x1 <= 1280.0 && rect.y0 >= 0.0 && rect.y1 <= 720.0);
        assert!(rect.x0 < rect.x1 && rect.y0 < rect.y1);
        assert_eq!(rect.y0, bounds[0].y0);
    }
    assert!(
        bounds.windows(2).all(|pair| pair[0].x1 <= pair[1].x0),
        "actual rectangles must preserve nonoverlapping host order: {bounds:?}"
    );
}

#[test]
fn passive_empty_groups_and_tabs_remain_accessible_in_root_order() {
    assert_order(
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"passive"))
            .group(TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"passive-group"),
                TabStripText::new("Static Group"),
            ))
            .tab(TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"passive-tab"),
                TabStripText::new("Static Tab"),
            )),
        &["Static Group", "Static Tab"],
    );
}
