#![cfg(feature = "egui")]

use std::sync::{Arc, Mutex};

use egui::accesskit::Role;
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TabStripCorrelation, TabStripGroupCapabilities,
    TabStripGroupDescriptor, TabStripGroupPopupPresentation, TabStripGroupTarget,
    TabStripMenuEntry, TabStripMenuOperation, TabStripProjection, TabStripProjectionLease,
    TabStripProposal, TabStripProposalOperation, TabStripProposalOrigin, TabStripProposalPort,
    TabStripProposalPortError, TabStripSurfaceCapabilities, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

type OriginRecord = (
    TabStripCorrelation,
    TabStripProposalOperation,
    Option<TabStripProposalOrigin>,
);
type OriginRecords = Arc<Mutex<Vec<OriginRecord>>>;

#[derive(Clone, Default)]
struct RecordingPort(OriginRecords);

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let origin = proposal.origin();
        let correlation = proposal.correlation();
        self.0
            .lock()
            .unwrap()
            .push((correlation, proposal.into_operation(), origin));
        Ok(())
    }
}

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("proposal-origin-full-root").value("本文"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"proposal-origin-full-root-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("proposal-origin-full-root-text")),
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

fn frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    context.enable_accesskit();
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

fn bounds(output: &egui::FullOutput, label: &str) -> egui::Rect {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.label() == Some(label))
                    .then(|| node.bounds())
                    .flatten()
                    .map(|rect| {
                        egui::Rect::from_min_max(
                            egui::pos2(rect.x0 as f32, rect.y0 as f32),
                            egui::pos2(rect.x1 as f32, rect.y1 as f32),
                        )
                    })
            })
        })
        .expect(label)
}

fn click(output: &egui::FullOutput, label: &str) -> egui::Event {
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AX update");
    let (id, node) = update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == Role::Button && node.label() == Some(label))
        .expect(label);
    let _ = node;
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        target_tree: egui::accesskit::TreeId::ROOT,
        action: egui::accesskit::Action::Click,
        target_node: *id,
        data: None,
    })
}

fn projection(revision: u64) -> TabStripProjection {
    let tab = |target: &'static [u8], label: &'static str, selectable: bool, draggable: bool| {
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(target),
            TabStripText::new(label),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .selectable(selectable)
                .draggable(draggable),
        )
    };
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(
            format!("origin-correlation-{revision}").into_bytes(),
        ),
    )
    .tab(tab(b"body", "Body", true, true))
    .tab(tab(b"target", "Target", false, false))
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group"),
            TabStripText::new("Group"),
        )
        .capabilities(
            TabStripGroupCapabilities::new()
                .collapsible(true)
                .closeable(true)
                .menu_available(true),
        )
        .popup(
            TabStripGroupPopupPresentation::new().entry(TabStripMenuEntry::action(
                TabStripText::new("Close group"),
                TabStripText::new("Close group"),
                TabStripMenuOperation::CloseGroup,
            )),
        ),
    )
    .capabilities(TabStripSurfaceCapabilities::new().tab_drop_at_end_available(true))
}

fn mounted(
    revision: u64,
    recorded: OriginRecords,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot {
    EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(revision),
                |_| Ok(None),
            )
            .with_tab_strip(
                TabStripProjectionLease::new(projection(revision))
                    .with_proposal_port(RecordingPort(recorded)),
            ),
        )
        .unwrap()
}

#[test]
fn public_full_root_reports_tab_body_and_group_header_origins_with_target_and_correlation() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let mut root = mounted(1, Arc::clone(&recorded));
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, vec![click(&initial, "Body")]);
    let group = bounds(&frame(&context, &mut root, Vec::new()), "Group");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(group.center()),
            egui::Event::PointerButton {
                pos: group.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: group.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let records = recorded.lock().unwrap();
    assert!(records.iter().any(|(correlation, operation, origin)| {
        correlation == &TabStripCorrelation::from_opaque_bytes(b"origin-correlation-1")
            && matches!(operation, TabStripProposalOperation::SelectTab(target) if target == &TabStripTabTarget::from_opaque_bytes(b"body"))
            && *origin == Some(TabStripProposalOrigin::TabBody)
    }));
    assert!(records.iter().any(|(correlation, operation, origin)| {
        correlation == &TabStripCorrelation::from_opaque_bytes(b"origin-correlation-1")
            && matches!(operation, TabStripProposalOperation::SetGroupCollapsed { group, collapsed: true } if group == &TabStripGroupTarget::from_opaque_bytes(b"group"))
            && *origin == Some(TabStripProposalOrigin::GroupHeader)
    }));
}

#[test]
fn public_full_root_reports_group_popup_origin_for_host_projected_operation() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let mut root = mounted(1, Arc::clone(&recorded));
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let group = bounds(&initial, "Group");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(group.center()),
            egui::Event::PointerButton {
                pos: group.center(),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: group.center(),
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let menu = frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, vec![click(&menu, "Close group")]);
    let records = recorded.lock().unwrap();
    assert!(records.iter().any(|(correlation, operation, origin)| {
        correlation == &TabStripCorrelation::from_opaque_bytes(b"origin-correlation-1")
            && matches!(operation, TabStripProposalOperation::CloseGroup(group) if group == &TabStripGroupTarget::from_opaque_bytes(b"group"))
            && *origin == Some(TabStripProposalOrigin::GroupPopup)
    }));
}

#[test]
fn public_full_root_keeps_drag_terminal_on_start_lease_and_forwards_it_once() {
    let old = Arc::new(Mutex::new(Vec::new()));
    let new = Arc::new(Mutex::new(Vec::new()));
    let mut root = mounted(1, Arc::clone(&old));
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let source = bounds(&initial, "Body");
    let target = bounds(&initial, "Target");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source.center()),
            egui::Event::PointerButton {
                pos: source.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(target.center())],
    );
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection(2))
                .with_proposal_port(RecordingPort(new.clone())),
        ),
    )
    .unwrap();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(target.center()),
            egui::Event::PointerButton {
                pos: target.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let old_records = old.lock().unwrap();
    assert!(old_records.iter().any(|(correlation, operation, origin)| {
        correlation == &TabStripCorrelation::from_opaque_bytes(b"origin-correlation-1")
            && matches!(
                operation,
                TabStripProposalOperation::FinishDrag {
                    committed: true,
                    ..
                }
            )
            && *origin == Some(TabStripProposalOrigin::DragTerminal)
    }));
    assert_eq!(
        old_records
            .iter()
            .filter(|(_, operation, _)| matches!(
                operation,
                TabStripProposalOperation::FinishDrag { .. }
            ))
            .count(),
        1
    );
    assert!(new.lock().unwrap().is_empty());
}

#[test]
fn public_full_root_rejects_disabled_target_without_forwarding() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let mut root = mounted(1, Arc::clone(&recorded));
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let target = bounds(&initial, "Target");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(target.center()),
            egui::Event::PointerButton {
                pos: target.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: target.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert!(recorded.lock().unwrap().is_empty());
}
