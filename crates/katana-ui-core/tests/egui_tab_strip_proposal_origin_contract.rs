#![cfg(feature = "egui")]

use std::sync::{Arc, Mutex};

use egui::accesskit::Role;
use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TabStripContextMenuPresentation,
    TabStripControlPresentation, TabStripCorrelation, TabStripMenuEntry, TabStripMenuOperation,
    TabStripNavigationPresentation, TabStripProjection, TabStripProjectionLease, TabStripProposal,
    TabStripProposalOperation, TabStripProposalOrigin, TabStripProposalPort,
    TabStripProposalPortError, TabStripSurfaceCapabilities, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

type OriginRecord = (TabStripProposalOperation, Option<TabStripProposalOrigin>);
type OriginRecords = Arc<Mutex<Vec<OriginRecord>>>;

#[derive(Clone, Default)]
struct RecordingPort(OriginRecords);

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let origin = proposal.origin();
        self.0
            .lock()
            .unwrap()
            .push((proposal.into_operation(), origin));
        Ok(())
    }
}

fn token() -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("proposal-origin").value("本文"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"proposal-origin-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("proposal-origin-text")),
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

fn projection() -> TabStripProjection {
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab"),
        TabStripText::new("Tab"),
    )
    .capabilities(
        TabStripTabCapabilities::new()
            .selectable(true)
            .closeable(true),
    )
    .trailing_control(TabStripControlPresentation::new(
        TabStripText::new("Close tab"),
        TabStripText::new("Close tab"),
    ))
    .context_menu(
        TabStripContextMenuPresentation::new().entry(TabStripMenuEntry::action(
            TabStripText::new("Context close"),
            TabStripText::new("Context close"),
            TabStripMenuOperation::RequestClose,
        )),
    );
    TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"correlation"))
        .tab(tab)
        .capabilities(
            TabStripSurfaceCapabilities::new()
                .previous_available(true)
                .next_available(true),
        )
        .navigation(TabStripNavigationPresentation::new(
            TabStripControlPresentation::new(
                TabStripText::new("Previous"),
                TabStripText::new("Previous"),
            ),
            TabStripControlPresentation::new(TabStripText::new("Next"), TabStripText::new("Next")),
        ))
}

#[test]
fn legacy_proposal_constructor_keeps_origin_absent() {
    let proposal = TabStripProposal::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"legacy"),
        TabStripProposalOperation::SelectPrevious,
    );
    assert_eq!(proposal.origin(), None);
}

#[test]
fn public_root_tags_close_context_menu_and_navigation_origins() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let lease = TabStripProjectionLease::new(projection())
        .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
    let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(), |_| Ok(None))
            .with_tab_strip(lease),
    ).unwrap();
    let context = egui::Context::default();

    let initial = frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, vec![click(&initial, "Close tab")]);

    let tab_bounds = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == Role::Button && node.label() == Some("Tab"))
                .then(|| node.bounds())
                .flatten()
        })
        .expect("tab bounds");
    let menu = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(egui::pos2(
                tab_bounds.x0 as f32 + 8.0,
                tab_bounds.y0 as f32 + 8.0,
            )),
            egui::Event::PointerButton {
                pos: egui::pos2(tab_bounds.x0 as f32 + 8.0, tab_bounds.y0 as f32 + 8.0),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: egui::pos2(tab_bounds.x0 as f32 + 8.0, tab_bounds.y0 as f32 + 8.0),
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let menu_bounds = menu
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.label() == Some("Context close"))
                .then(|| node.bounds())
                .flatten()
        })
        .unwrap();
    let point = egui::pos2(
        (menu_bounds.x0 + menu_bounds.x1) as f32 / 2.0,
        (menu_bounds.y0 + menu_bounds.y1) as f32 / 2.0,
    );
    frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );

    let navigation = frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, vec![click(&navigation, "Previous")]);

    let records = recorded.lock().unwrap();
    assert!(records.iter().any(|(operation, origin)| matches!(
        operation,
        TabStripProposalOperation::RequestClose(_)
    ) && *origin
        == Some(TabStripProposalOrigin::TrailingControl)));
    assert!(records.iter().any(|(operation, origin)| matches!(
        operation,
        TabStripProposalOperation::RequestClose(_)
    ) && *origin
        == Some(TabStripProposalOrigin::ContextMenu)));
    assert!(records.iter().any(|(operation, origin)| matches!(
        operation,
        TabStripProposalOperation::SelectPrevious
    ) && *origin
        == Some(TabStripProposalOrigin::Navigation)));
}

#[test]
fn secondary_click_outside_dismisses_an_open_tab_menu() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let lease = TabStripProjectionLease::new(projection())
        .with_proposal_port(RecordingPort(Arc::clone(&recorded)));
    let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(token(), |_| Ok(None))
            .with_tab_strip(lease),
    ).unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let tab_bounds = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == Role::Button && node.label() == Some("Tab"))
                .then(|| node.bounds())
                .flatten()
        })
        .unwrap();
    let tab = egui::pos2(
        (tab_bounds.x0 + tab_bounds.x1) as f32 / 2.0,
        (tab_bounds.y0 + tab_bounds.y1) as f32 / 2.0,
    );
    let opened = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(tab),
            egui::Event::PointerButton {
                pos: tab,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: tab,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let menu_click = click(&opened, "Context close");
    let outside = egui::pos2(500.0, 500.0);
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(outside),
            egui::Event::PointerButton {
                pos: outside,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: outside,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(&context, &mut root, vec![menu_click]);
    assert!(
        recorded.lock().unwrap().is_empty(),
        "dismissed menu must not accept its old action"
    );
}
