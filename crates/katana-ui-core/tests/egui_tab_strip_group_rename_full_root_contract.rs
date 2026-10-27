#![cfg(feature = "egui")]

use std::sync::{Arc, Mutex};

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootFactory, TabStripContextMenuPresentation, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripProjectionLease, TabStripProposal, TabStripProposalOperation, TabStripProposalPort,
    TabStripProposalPortError, TabStripRenamePolicy, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

type RenameDelivery = (TabStripGroupTarget, TabStripCorrelation, String);
type RenameDeliveries = Arc<Mutex<Vec<RenameDelivery>>>;

#[derive(Default)]
struct RecordingPort {
    starts: Arc<Mutex<Vec<(TabStripGroupTarget, TabStripCorrelation)>>>,
    names: Arc<Mutex<Vec<String>>>,
}

impl TabStripProposalPort for RecordingPort {
    fn inline_rename_started(
        &mut self,
        receipt: katana_ui_core::egui::text_command_surface::TabStripInlineRenameStarted,
    ) -> Result<(), TabStripProposalPortError> {
        self.starts
            .lock()
            .unwrap()
            .push((receipt.group(), receipt.correlation()));
        Ok(())
    }

    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        if let TabStripProposalOperation::RenameGroup { name, .. } = proposal.into_operation() {
            self.names.lock().unwrap().push(name.as_str().to_owned());
        }
        Ok(())
    }
}

fn token(
    revision: u64,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("rename-root").value("本文"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"rename-root-host",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("rename-root-text")),
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

fn projection(revision: u64, request_inline_rename: bool) -> TabStripProjection {
    projection_variant(
        revision,
        request_inline_rename,
        true,
        true,
        b"rename-group",
        b"rename-correlation",
        "Group name",
    )
}

fn projection_variant(
    revision: u64,
    request_inline_rename: bool,
    renamable: bool,
    with_popup: bool,
    target: &'static [u8],
    correlation: &'static [u8],
    placeholder: &'static str,
) -> TabStripProjection {
    let mut group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(target),
        TabStripText::new("Group"),
    )
    .capabilities(TabStripGroupCapabilities::new().renamable(renamable));
    if with_popup {
        group = group.popup(
            TabStripGroupPopupPresentation::new()
                .rename_placeholder(TabStripText::new(placeholder))
                .request_inline_rename(request_inline_rename)
                .rename_policy(TabStripRenamePolicy::OnChange),
        );
    }
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(correlation),
    )
    .group(group)
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

fn frame_discard(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
) {
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..Default::default()
        },
        |ui| {
            root.show(ui).unwrap();
            ui.ctx().request_discard("rename replacement");
        },
    );
}

fn frame_discard_with_events(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) {
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
            ui.ctx().request_discard("rename input discard");
        },
    );
    output.textures_delta.clear();
}

fn access_bounds(output: &egui::FullOutput, label: &str) -> egui::Rect {
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

fn tab_menu_projection(revision: u64, request_inline_rename: bool) -> TabStripProjection {
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"rename-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"menu-tab"),
            TabStripText::new("Menu tab"),
        )
        .capabilities(TabStripTabCapabilities::new().selectable(true))
        .context_menu(
            TabStripContextMenuPresentation::new().entry(TabStripMenuEntry::action(
                TabStripText::new("Close"),
                TabStripText::new("Close"),
                TabStripMenuOperation::RequestClose,
            )),
        ),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"rename-group"),
            TabStripText::new("Group"),
        )
        .capabilities(TabStripGroupCapabilities::new().renamable(true))
        .popup(
            TabStripGroupPopupPresentation::new()
                .rename_placeholder(TabStripText::new("Group name"))
                .request_inline_rename(request_inline_rename)
                .rename_policy(TabStripRenamePolicy::OnChange),
        ),
    )
}

fn group_then_tab_menu_projection(
    revision: u64,
    request_inline_rename: bool,
) -> TabStripProjection {
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"rename-group"),
        TabStripText::new("Group"),
    )
    .capabilities(TabStripGroupCapabilities::new().renamable(true))
    .popup(
        TabStripGroupPopupPresentation::new()
            .rename_placeholder(TabStripText::new("Group name"))
            .request_inline_rename(request_inline_rename)
            .rename_policy(TabStripRenamePolicy::OnChange),
    );
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"menu-tab"),
        TabStripText::new("Menu tab"),
    )
    .capabilities(TabStripTabCapabilities::new().selectable(true))
    .context_menu(
        TabStripContextMenuPresentation::new().entry(TabStripMenuEntry::action(
            TabStripText::new("Close"),
            TabStripText::new("Close"),
            TabStripMenuOperation::RequestClose,
        )),
    );
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"rename-correlation"),
    )
    .group(group)
    .tab(tab)
}

fn replacement_projection() -> TabStripProjection {
    TabStripProjection::new(
        2,
        TabStripCorrelation::from_opaque_bytes(b"replacement-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"replacement-group"),
            TabStripText::new("Replacement Group"),
        )
        .capabilities(TabStripGroupCapabilities::new().renamable(true))
        .popup(
            TabStripGroupPopupPresentation::new()
                .rename_placeholder(TabStripText::new("Replacement name"))
                .request_inline_rename(true)
                .rename_policy(TabStripRenamePolicy::OnChange),
        ),
    )
}

fn labeled_projection(revision: u64, label: &str) -> TabStripProjection {
    TabStripProjection::new(
        revision,
        TabStripCorrelation::from_opaque_bytes(b"rename-correlation"),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"rename-group"),
            TabStripText::new(label),
        )
        .capabilities(TabStripGroupCapabilities::new().renamable(true))
        .popup(
            TabStripGroupPopupPresentation::new()
                .rename_placeholder(TabStripText::new("Group name"))
                .request_inline_rename(true)
                .rename_policy(TabStripRenamePolicy::OnChange),
        ),
    )
}

#[test]
fn public_full_root_rename_on_change_reports_start_ime_names_empty_and_enter() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let lease = TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(
        starts.lock().unwrap().len(),
        1,
        "inline start is delivered once"
    );
    let started = starts.lock().unwrap();
    assert_eq!(
        started[0].0,
        TabStripGroupTarget::from_opaque_bytes(b"rename-group")
    );
    assert_eq!(
        started[0].1,
        TabStripCorrelation::from_opaque_bytes(b"rename-correlation")
    );
    drop(started);
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "にほんご".to_owned(),
            active_range_chars: None,
        })],
    );
    assert!(
        names.lock().unwrap().is_empty(),
        "preedit alone must not submit a name"
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️".to_owned(),
        ))],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Backspace,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Backspace,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(&context, &mut root, Vec::new());
    let recorded_names = names.lock().unwrap().clone();
    assert!(
        recorded_names.windows(2).all(|pair| pair[0] != pair[1]),
        "each input receipt represents a distinct text change"
    );
    assert_eq!(
        recorded_names
            .iter()
            .filter(|name| name.as_str() == "日本語 ⭐️")
            .count(),
        1,
        "IME commit is delivered exactly once"
    );
    assert_eq!(
        recorded_names.iter().filter(|name| name.is_empty()).count(),
        1,
        "Enter submits the empty replacement exactly once"
    );
}

#[test]
fn public_full_root_escape_closes_rename_without_new_name_and_next_receipt_is_observable() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let lease = TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let receipt = frame(&context, &mut root, Vec::new());
    assert!(
        !receipt.shapes.is_empty(),
        "subsequent frame remains renderable"
    );
    assert_eq!(starts.lock().unwrap().len(), 1);
    assert!(
        names.lock().unwrap().is_empty(),
        "Escape must not submit a name"
    );
}

#[test]
fn same_input_frame_ime_commit_and_escape_deliver_one_name_then_remain_closed() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let lease = TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(lease),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(starts.lock().unwrap().len(), 1);
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::Ime(egui::ImeEvent::Commit("同時入力".to_owned())),
            egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    let recorded_names = names.lock().unwrap().clone();
    assert_eq!(recorded_names.len(), 1);
    assert!(recorded_names[0].contains("同時入力"));
    assert_eq!(starts.lock().unwrap().len(), 1);
}

#[test]
fn same_target_reprojection_can_enable_inline_rename_after_disabled_request() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let make_port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(TabStripProjectionLease::new(projection(1, false)).with_proposal_port(make_port())),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection(2, true)).with_proposal_port(make_port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(starts.lock().unwrap().len(), 1);
}

#[test]
fn same_group_reprojection_reopens_once_and_keeps_input_focus_without_duplicate_terminal_receipts()
{
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(
                TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port()),
            ),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(starts.lock().unwrap().len(), 1);

    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection(2, false)).with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(3),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection(3, true)).with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(starts.lock().unwrap().len(), 2, "reprojection reopens once");

    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "再投影入力".to_owned(),
        ))],
    );
    frame(&context, &mut root, Vec::new());
    assert_eq!(
        names.lock().unwrap().len(),
        1,
        "focus survives re-projection"
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(1100.0, 600.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(&context, &mut root, Vec::new());
    assert_eq!(
        starts.lock().unwrap().len(),
        2,
        "terminal input does not reopen"
    );
    assert_eq!(
        names.lock().unwrap().len(),
        1,
        "Enter/Escape/outside do not duplicate"
    );
}

#[test]
fn same_open_group_reissues_start_after_false_then_true_without_losing_draft_focus() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(
                TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port()),
            ),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(starts.lock().unwrap().len(), 1);

    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection(2, false)).with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(3),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection(3, true)).with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    let started = starts.lock().unwrap().clone();
    assert_eq!(started.len(), 2, "true re-projection reissues one start");
    assert!(started.iter().all(|(target, correlation)| {
        target == &TabStripGroupTarget::from_opaque_bytes(b"rename-group")
            && correlation == &TabStripCorrelation::from_opaque_bytes(b"rename-correlation")
    }));

    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "継続入力".to_owned(),
        ))],
    );
    frame(&context, &mut root, Vec::new());
    let recorded_names = names.lock().unwrap().clone();
    assert_eq!(recorded_names.len(), 1, "draft input remains focused");
    assert!(recorded_names[0].contains("継続入力"));
    assert_eq!(
        starts.lock().unwrap().len(),
        2,
        "input does not duplicate start"
    );
}

#[test]
fn blocked_popup_or_non_editable_capability_does_not_start_until_released() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(TabStripProjectionLease::new(projection_variant(
                1,
                true,
                false,
                false,
                b"rename-group",
                b"rename-correlation",
                "Group name",
            ))
            .with_proposal_port(port())),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert!(
        starts.lock().unwrap().is_empty(),
        "blocked projection is fail-closed"
    );
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(projection_variant(
                2,
                true,
                true,
                true,
                b"rename-group",
                b"rename-correlation",
                "Group name",
            ))
            .with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert_eq!(
        starts.lock().unwrap().len(),
        1,
        "released projection starts once"
    );
    assert!(names.lock().unwrap().is_empty());
}

#[test]
fn open_tab_context_menu_blocks_group_inline_rename_until_outside_dismissal() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(
                TabStripProjectionLease::new(tab_menu_projection(1, false))
                    .with_proposal_port(port()),
            ),
        )
        .unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let tab = access_bounds(&initial, "Menu tab");
    let tab_center = tab.center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(tab_center),
            egui::Event::PointerButton {
                pos: tab_center,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: tab_center,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(tab_menu_projection(2, true)).with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    assert!(
        starts.lock().unwrap().is_empty(),
        "an unrelated open tab menu blocks inline rename"
    );

    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(1100.0, 600.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(1100.0, 600.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    let started = starts.lock().unwrap();
    assert_eq!(started.len(), 1, "dismissal permits one committed receipt");
    assert_eq!(
        started[0].0,
        TabStripGroupTarget::from_opaque_bytes(b"rename-group")
    );
    assert_eq!(
        started[0].1,
        TabStripCorrelation::from_opaque_bytes(b"rename-correlation")
    );
    assert!(names.lock().unwrap().is_empty());
}

#[test]
fn same_frame_inline_request_and_tab_menu_prefers_menu_then_reveals_persisted_request() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(
                TabStripProjectionLease::new(group_then_tab_menu_projection(1, false))
                    .with_proposal_port(port()),
            ),
        )
        .unwrap();
    let context = egui::Context::default();
    let initial = frame(&context, &mut root, Vec::new());
    let tab_center = access_bounds(&initial, "Menu tab").center();
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(group_then_tab_menu_projection(2, true))
                .with_proposal_port(port()),
        ),
    )
    .unwrap();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(tab_center),
            egui::Event::PointerButton {
                pos: tab_center,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: tab_center,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(&context, &mut root, Vec::new());
    assert!(
        starts.lock().unwrap().is_empty(),
        "tab menu replaces the tentative group editor"
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(1100.0, 600.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(1100.0, 600.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    let started = starts.lock().unwrap();
    assert_eq!(started.len(), 1);
    assert_eq!(
        started[0].0,
        TabStripGroupTarget::from_opaque_bytes(b"rename-group")
    );
    assert_eq!(
        started[0].1,
        TabStripCorrelation::from_opaque_bytes(b"rename-correlation")
    );
    assert!(names.lock().unwrap().is_empty());
}

#[test]
fn committed_name_projects_into_same_group_label_and_keeps_ime_focus() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = || RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(
                TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port()),
            ),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "変更名".to_owned(),
        ))],
    );
    frame(&context, &mut root, Vec::new());
    assert_eq!(names.lock().unwrap().as_slice(), ["変更名"]);

    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(labeled_projection(2, "変更名"))
                .with_proposal_port(port()),
        ),
    )
    .unwrap();
    let projected = frame(&context, &mut root, Vec::new());
    assert!(access_bounds(&projected, "変更名").width() > 0.0);
    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "継続入力".to_owned(),
        ))],
    );
    frame(&context, &mut root, Vec::new());
    let recorded_names = names.lock().unwrap().clone();
    assert_eq!(recorded_names.len(), 2);
    assert!(recorded_names[1].contains("継続入力"));
    assert_eq!(starts.lock().unwrap().len(), 1);
}

#[test]
fn discarded_input_does_not_cross_same_path_target_or_correlation_lease() {
    struct CorrelatedPort {
        inner: RecordingPort,
        deliveries: RenameDeliveries,
    }
    impl TabStripProposalPort for CorrelatedPort {
        fn inline_rename_started(
            &mut self,
            receipt: katana_ui_core::egui::text_command_surface::TabStripInlineRenameStarted,
        ) -> Result<(), TabStripProposalPortError> {
            self.inner.inline_rename_started(receipt)
        }
        fn forward_proposal(
            &mut self,
            proposal: TabStripProposal,
        ) -> Result<(), TabStripProposalPortError> {
            let correlation = proposal.correlation().clone();
            if let TabStripProposalOperation::RenameGroup { group, name } =
                proposal.into_operation()
            {
                let value = name.as_str().to_owned();
                self.inner.names.lock().unwrap().push(value.clone());
                self.deliveries
                    .lock()
                    .unwrap()
                    .push((group, correlation, value));
            }
            Ok(())
        }
    }
    let deliveries = Arc::new(Mutex::new(Vec::new()));
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = CorrelatedPort {
        inner: RecordingPort {
            starts: Arc::clone(&starts),
            names: Arc::clone(&names),
        },
        deliveries: Arc::clone(&deliveries),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port)),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    {
        let started = starts.lock().unwrap();
        assert_eq!(started.len(), 1, "old editor was actually started");
        assert_eq!(
            started[0].0,
            TabStripGroupTarget::from_opaque_bytes(b"rename-group")
        );
        assert_eq!(
            started[0].1,
            TabStripCorrelation::from_opaque_bytes(b"rename-correlation")
        );
    }
    frame_discard_with_events(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "破棄される名前".to_owned(),
        ))],
    );
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(
                TabStripProjection::new(
                    2,
                    TabStripCorrelation::from_opaque_bytes(b"replacement-correlation"),
                )
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"replacement-group"),
                        TabStripText::new("Replacement"),
                    )
                    .capabilities(TabStripGroupCapabilities::new().renamable(true))
                    .popup(
                        TabStripGroupPopupPresentation::new()
                            .rename_placeholder(TabStripText::new("Replacement name"))
                            .request_inline_rename(true)
                            .rename_policy(TabStripRenamePolicy::OnChange),
                    ),
                ),
            )
            .with_proposal_port(CorrelatedPort {
                inner: RecordingPort {
                    starts: Arc::clone(&starts),
                    names: Arc::clone(&names),
                },
                deliveries: Arc::clone(&deliveries),
            }),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    let replacement = frame(&context, &mut root, Vec::new());
    assert!(
        replacement
            .platform_output
            .accesskit_update
            .as_ref()
            .unwrap()
            .nodes
            .iter()
            .any(|(_, node)| node.label() == Some("Replacement name")
                && node.value() == Some("Replacement"))
    );
    assert!(deliveries.lock().unwrap().is_empty());
    assert!(
        names.lock().unwrap().is_empty(),
        "discarded name is not forwarded"
    );
    let started = starts.lock().unwrap();
    assert_eq!(started.len(), 2);
    assert_eq!(
        started[1].0,
        TabStripGroupTarget::from_opaque_bytes(b"replacement-group")
    );
    assert_eq!(
        started[1].1,
        TabStripCorrelation::from_opaque_bytes(b"replacement-correlation")
    );
    drop(started);
    frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "新しい名前".to_owned(),
        ))],
    );
    frame(&context, &mut root, Vec::new());
    assert_eq!(names.lock().unwrap().as_slice(), ["Replacement新しい名前"]);
    assert_eq!(
        deliveries.lock().unwrap().as_slice(),
        &[(
            TabStripGroupTarget::from_opaque_bytes(b"replacement-group"),
            TabStripCorrelation::from_opaque_bytes(b"replacement-correlation"),
            "Replacement新しい名前".to_owned(),
        )]
    );
}

#[test]
fn discarded_pass_does_not_forward_old_rename_to_replacement_target() {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let names = Arc::new(Mutex::new(Vec::new()));
    let port = RecordingPort {
        starts: Arc::clone(&starts),
        names: Arc::clone(&names),
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
                token(1),
                |_| Ok(None),
            )
            .with_tab_strip(TabStripProjectionLease::new(projection(1, true)).with_proposal_port(port)),
        )
        .unwrap();
    let context = egui::Context::default();
    frame(&context, &mut root, Vec::new());
    frame_discard(&context, &mut root);
    root.synchronize_with_lease(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
            token(2),
            |_| Ok(None),
        )
        .with_tab_strip(
            TabStripProjectionLease::new(replacement_projection()).with_proposal_port(
                RecordingPort {
                    starts: Arc::clone(&starts),
                    names: Arc::clone(&names),
                },
            ),
        ),
    )
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    frame(&context, &mut root, Vec::new());
    let started = starts.lock().unwrap();
    assert_eq!(started.len(), 1);
    assert_eq!(
        started[0].0,
        TabStripGroupTarget::from_opaque_bytes(b"replacement-group")
    );
    assert_eq!(
        started[0].1,
        TabStripCorrelation::from_opaque_bytes(b"replacement-correlation")
    );
    assert!(names.lock().unwrap().is_empty());
}
