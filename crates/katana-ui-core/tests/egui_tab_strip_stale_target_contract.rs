#![cfg(feature = "egui")]

use std::sync::{Arc, Mutex};

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface as kuc;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

struct SelectPort(Arc<Mutex<Vec<kuc::TabStripTabTarget>>>);

impl kuc::TabStripProposalPort for SelectPort {
    fn forward_proposal(
        &mut self,
        proposal: kuc::TabStripProposal,
    ) -> Result<(), kuc::TabStripProposalPortError> {
        if let kuc::TabStripProposalOperation::SelectTab(target) = proposal.into_operation() {
            self.0.lock().unwrap().push(target);
        }
        Ok(())
    }
}

fn lease(
    revision: u64,
    tabs: &[(&[u8], &str)],
    selected: &Arc<Mutex<Vec<kuc::TabStripTabTarget>>>,
) -> kuc::EguiTextCommandSurfaceHostProjectionLease {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("stale-target-body").value("本文"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    let token = kuc::EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"stale-target-host",
        kuc::EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("stale-target-document")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        kuc::TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap();
    let projection = tabs.iter().fold(
        kuc::TabStripProjection::new(
            revision,
            kuc::TabStripCorrelation::from_opaque_bytes(b"stale-target-correlation"),
        ),
        |projection, (target, label)| {
            projection.tab(
                kuc::TabStripTabDescriptor::new(
                    kuc::TabStripTabTarget::from_opaque_bytes(*target),
                    kuc::TabStripText::new(*label),
                )
                .capabilities(kuc::TabStripTabCapabilities::new().selectable(true)),
            )
        },
    );
    kuc::EguiTextCommandSurfaceHostProjectionLease::new(token, |_context| Ok(None)).with_tab_strip(
        kuc::TabStripProjectionLease::new(projection)
            .with_proposal_port(SelectPort(Arc::clone(selected))),
    )
}

fn frame(
    context: &egui::Context,
    root: &mut kuc::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            root.show(ui).unwrap();
        },
    );
    output.textures_delta.clear();
    output
}

fn node(output: &egui::FullOutput, label: &str) -> egui::accesskit::NodeId {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find_map(|(id, node)| {
            (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                .then_some(*id)
        })
        .expect("tab button")
}

#[test]
fn removed_tab_accesskit_action_cannot_be_retargeted_to_a_shifted_occurrence() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let selected = Arc::new(Mutex::new(Vec::new()));
    let mut root = kuc::EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, &[(b"a", "A"), (b"b", "B")], &selected))
        .unwrap();
    let initial = frame(&context, &mut root, Vec::new());
    let removed_id = node(&initial, "A");
    let original_remaining_id = node(&initial, "B");
    root.synchronize_with_lease(lease(2, &[(b"b", "B")], &selected))
        .unwrap();
    let shifted = frame(&context, &mut root, Vec::new());
    let remaining_id = node(&shifted, "B");
    frame(
        &context,
        &mut root,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                target_tree: egui::accesskit::TreeId::ROOT,
                action: egui::accesskit::Action::Click,
                target_node: removed_id,
                data: None,
            },
        )],
    );
    assert!(
        selected.lock().unwrap().is_empty(),
        "an expired tab action must never select the replacement occurrence"
    );
    assert_ne!(removed_id, remaining_id);
    assert_eq!(original_remaining_id, remaining_id);
    let click = |target_node| {
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                target_tree: egui::accesskit::TreeId::ROOT,
                action: egui::accesskit::Action::Click,
                target_node,
                data: None,
            },
        )]
    };
    frame(&context, &mut root, click(remaining_id));
    assert_eq!(
        *selected.lock().unwrap(),
        vec![kuc::TabStripTabTarget::from_opaque_bytes(b"b")]
    );
    root.synchronize_with_lease(lease(3, &[(b"c", "B")], &selected))
        .unwrap();
    let replacement = frame(&context, &mut root, Vec::new());
    assert_ne!(node(&replacement, "B"), remaining_id);
    frame(&context, &mut root, click(remaining_id));
    assert_eq!(selected.lock().unwrap().len(), 1);
}
