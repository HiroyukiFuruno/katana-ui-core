#![cfg(feature = "egui")]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::TextCommandSurfaceStyle;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootFactory, KucDialogControls, KucDialogTarget, KucGlobalAction,
    KucGlobalActionEvent, KucGlobalActionFocusPolicy, KucGlobalActionTarget,
    KucGlobalActionsProjection, KucGlobalShortcut, KucRootEventBatchContext,
    KucRootEventBatchDispatcher, KucRootEventBatchForwarder, SearchResultMode,
    SearchResultProjection, SelectionDialogProjection, SelectionDialogProjectionLease,
};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn presentation_for(read_only: bool, document: bool) -> EguiTextCommandSurfacePresentation {
    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new(if read_only { "read-only" } else { "editable" }).readonly(read_only),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 800, 480),
    ));
    EguiTextCommandSurfacePresentation {
        text_state_id: document
            .then(|| katana_ui_core::render_model::UiStateId::new("global-action")),
        text: TextSurfacePresentation::from_props(text.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    }
}

fn presentation(read_only: bool) -> EguiTextCommandSurfacePresentation {
    presentation_for(read_only, true)
}

fn lease(
    revision: u64,
    read_only: bool,
    enabled: bool,
    _seen: Rc<RefCell<Vec<KucGlobalActionTarget>>>,
) -> EguiTextCommandSurfaceHostProjectionLease {
    lease_with_focus_policy(
        revision,
        read_only,
        enabled,
        _seen,
        KucGlobalActionFocusPolicy::Any,
    )
}

fn lease_with_focus_policy(
    revision: u64,
    read_only: bool,
    enabled: bool,
    _seen: Rc<RefCell<Vec<KucGlobalActionTarget>>>,
    focus_policy: KucGlobalActionFocusPolicy,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let target = KucGlobalActionTarget::from_opaque_bytes(b"same-global-target");
    let action = KucGlobalAction::new(
        target,
        "Global action",
        KucGlobalShortcut::new(
            egui::Key::F,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
    )
    .enabled(enabled);
    let action = action.focus_policy(focus_policy);
    let projection = KucGlobalActionsProjection::new("Global actions").action(action);
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"global-action-root",
        presentation(read_only),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("global-action token");
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_context: KucRootEventBatchContext| {
        Ok::<_, katana_ui_core::egui::text_command_surface::KucOpaqueHostEffectError>(None)
    })
    .with_global_actions(projection)
}

fn lease_case(
    revision: u64,
    read_only: bool,
    document: bool,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"global-action-root",
        presentation_for(read_only, document),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("global-action token");
    let action = KucGlobalAction::new(
        KucGlobalActionTarget::from_opaque_bytes(b"same-global-target"),
        "Global action",
        KucGlobalShortcut::new(
            egui::Key::F,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
    );
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_context: KucRootEventBatchContext| {
        Ok(None)
    })
    .with_global_actions(KucGlobalActionsProjection::new("Global actions").action(action))
}

fn lease_with_search_action(revision: u64) -> EguiTextCommandSurfaceHostProjectionLease {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"global-action-root",
        presentation_for(false, true),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("global-action token");
    let global = KucGlobalAction::new(
        KucGlobalActionTarget::from_opaque_bytes(b"global-target"),
        "Global action",
        KucGlobalShortcut::new(
            egui::Key::F,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
    );
    let search = KucGlobalAction::new(
        KucGlobalActionTarget::from_opaque_bytes(b"document-search-target"),
        "Document search",
        KucGlobalShortcut::new(egui::Key::F, egui::Modifiers::COMMAND),
    );
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_context: KucRootEventBatchContext| {
        Ok(None)
    })
    .with_global_actions(
        KucGlobalActionsProjection::new("Global actions")
            .action(global)
            .action(search),
    )
}

fn lease_without_global_actions(
    revision: u64,
    read_only: bool,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"global-action-root",
        presentation(read_only),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("global-action token");
    EguiTextCommandSurfaceHostProjectionLease::new(token, |_context: KucRootEventBatchContext| {
        Ok::<_, katana_ui_core::egui::text_command_surface::KucOpaqueHostEffectError>(None)
    })
}

fn open_query_dialog(revision: u64) -> SelectionDialogProjectionLease {
    let target = KucDialogTarget::from_opaque_bytes(b"all");
    SelectionDialogProjectionLease::new(
        revision,
        "Query",
        KucDialogControls {
            cancel: "Cancel".into(),
            select: "Select".into(),
            search: "Search".into(),
            include: "Include".into(),
            exclude: "Exclude".into(),
            clear_history: "Clear".into(),
            delete_history: "Delete".into(),
            empty: "Empty".into(),
        },
        SelectionDialogProjection::Search(SearchResultProjection {
            active_mode: target.clone(),
            modes: vec![SearchResultMode {
                target,
                label: "All".into(),
                query: String::new(),
                include: None,
                exclude: None,
                options: Vec::new(),
                history: Vec::new(),
                results: Vec::new(),
                searched: false,
            }],
        }),
    )
    .expect("open query dialog")
}

fn frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> (
    katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    egui::FullOutput,
) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 480.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("global action root frame"));
            });
        },
    );
    output.textures_delta.clear();
    (frame.expect("global action frame"), output)
}

fn button(output: &egui::FullOutput, label: &str) -> Option<(egui::accesskit::NodeId, egui::Rect)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find_map(|(id, node)| {
            (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                .then(|| {
                    node.bounds().map(|rect| {
                        (
                            *id,
                            egui::Rect::from_min_max(
                                egui::pos2(rect.x0 as f32, rect.y0 as f32),
                                egui::pos2(rect.x1 as f32, rect.y1 as f32),
                            ),
                        )
                    })
                })
                .flatten()
        })
}

fn text_input_bounds(output: &egui::FullOutput) -> Option<egui::Rect> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (matches!(
                node.role(),
                egui::accesskit::Role::TextInput | egui::accesskit::Role::MultilineTextInput
            ))
            .then(|| node.bounds())
            .flatten()
            .map(|rect| {
                egui::Rect::from_min_max(
                    egui::pos2(rect.x0 as f32, rect.y0 as f32),
                    egui::pos2(rect.x1 as f32, rect.y1 as f32),
                )
            })
        })
}

fn frame_with_external_input(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> (
    katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    egui::FullOutput,
    egui::Rect,
    egui::Id,
) {
    let mut frame = None;
    let mut response = None;
    let mut value = String::new();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 480.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                response = Some(egui::TextEdit::singleline(&mut value).show(ui).response);
                frame = Some(root.show(ui).expect("root frame"));
            });
        },
    );
    output.textures_delta.clear();
    let response = response.expect("external input response");
    (
        frame.expect("root frame"),
        output,
        response.rect,
        response.id,
    )
}

struct Forwarder(Option<EguiTextCommandSurfaceRootEventTransport>);
impl KucRootEventBatchForwarder for Forwarder {
    type Error = ();
    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.0 = Some(transport);
        Ok(())
    }
}

struct Dispatcher {
    seen: Vec<KucGlobalActionTarget>,
}
impl KucRootEventBatchDispatcher for Dispatcher {
    type Error = ();
    fn dispatch_global_action_events(
        &mut self,
        events: Vec<KucGlobalActionEvent>,
    ) -> Result<(), Self::Error> {
        self.seen
            .extend(events.into_iter().map(|event| event.target()));
        Ok(())
    }
    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn dispatch_floating_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn dispatch_search_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn global_action_menu_click_and_primary_shift_shortcut_share_target_and_are_one_shot() {
    assert_eq!(
        format!(
            "{:?}",
            KucGlobalActionTarget::from_opaque_bytes(b"private-target")
        ),
        "KucGlobalActionTarget(..)",
        "Debug must keep the host target opaque"
    );
    for scale in [1.0, 2.0] {
        assert_menu_and_shortcut_are_one_shot_at_scale(scale);
    }
}

fn assert_menu_and_shortcut_are_one_shot_at_scale(scale: f32) {
    let context = egui::Context::default();
    context.set_pixels_per_point(scale);
    context.enable_accesskit();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_focus_policy(
            1,
            false,
            true,
            Rc::clone(&seen),
            KucGlobalActionFocusPolicy::DocumentOnly,
        ))
        .expect("retain global action root");

    let (initial, initial_output) = frame(&context, &mut root, Vec::new());
    assert_eq!(context.pixels_per_point(), scale);
    assert!(
        initial
            .interaction_locator()
            .request(KucGlobalActionsProjection::new("Global actions").menu_selector())
            .is_ok(),
        "global menu selector must expose measured bounds"
    );
    let (_, menu_bounds) = button(&initial_output, "Global actions")
        .unwrap_or_else(|| panic!("global action menu AccessKit button was not published"));
    let menu_click = vec![
        egui::Event::PointerMoved(menu_bounds.center()),
        egui::Event::PointerButton {
            pos: menu_bounds.center(),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: menu_bounds.center(),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    let (opened, opened_output) = frame(&context, &mut root, menu_click);
    assert!(
        opened
            .interaction_locator()
            .request(
                KucGlobalActionTarget::from_opaque_bytes(b"same-global-target")
                    .interaction_selector()
            )
            .is_ok(),
        "global action selector must expose measured entry bounds"
    );
    let (_, bounds) = button(&opened_output, "Global action")
        .unwrap_or_else(|| panic!("global action AccessKit menu item was not published"));
    let action_click = vec![
        egui::Event::PointerMoved(bounds.center()),
        egui::Event::PointerButton {
            pos: bounds.center(),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: bounds.center(),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    let (clicked, _) = frame(&context, &mut root, action_click);
    let mut menu_forwarder = Forwarder(None);
    root.forward_global_events_once(&clicked, &mut menu_forwarder)
        .expect("authorize menu event");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    menu_forwarder
        .0
        .take()
        .expect("captured menu transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch menu event");
    assert_eq!(
        dispatcher.seen,
        vec![KucGlobalActionTarget::from_opaque_bytes(
            b"same-global-target"
        )]
    );
    assert!(
        root.forward_global_events_once(&clicked, &mut Forwarder(None))
            .is_err(),
        "root forwarding is one-shot"
    );

    let (shortcut, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    assert!(
        shortcut.forward_events_once(&mut Forwarder(None)).is_err(),
        "global frame requires authenticated forwarding"
    );
    let mut shortcut_forwarder = Forwarder(None);
    root.forward_global_events_once(&shortcut, &mut shortcut_forwarder)
        .expect("authorize shortcut event");
    shortcut_forwarder
        .0
        .take()
        .expect("captured shortcut transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch shortcut event");
    assert_eq!(dispatcher.seen.len(), 2);
}

#[test]
fn global_action_default_any_policy_allows_modal_focus() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain global action root");
    let _ = frame(&context, &mut root, Vec::new());
    context.memory_mut(|memory| memory.request_focus(egui::Id::new("modal-query")));
    let (modal, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let mut forwarder = Forwarder(None);
    root.forward_global_events_once(&modal, &mut forwarder)
        .expect("authorize modal shortcut");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    forwarder
        .0
        .take()
        .expect("captured modal transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch modal shortcut");
    assert_eq!(
        dispatcher.seen,
        vec![KucGlobalActionTarget::from_opaque_bytes(
            b"same-global-target"
        )]
    );
}

#[test]
fn global_action_any_policy_remains_toggleable_while_selection_dialog_query_is_open() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain global action root");
    root.synchronize_selection_dialog(open_query_dialog(1))
        .expect("attach query dialog");
    let _ = frame(&context, &mut root, Vec::new());

    for ime in [
        egui::ImeEvent::Preedit {
            text: "modal-preedit".into(),
            active_range_chars: None,
        },
        egui::ImeEvent::Commit("modal-commit".into()),
    ] {
        let (ime_frame, _) = frame(
            &context,
            &mut root,
            vec![
                egui::Event::Ime(ime),
                egui::Event::Key {
                    key: egui::Key::F,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
                },
            ],
        );
        let mut ime_forwarder = Forwarder(None);
        root.forward_global_events_once(&ime_frame, &mut ime_forwarder)
            .expect("authorize modal IME frame");
        let mut ime_dispatcher = Dispatcher { seen: Vec::new() };
        ime_forwarder
            .0
            .take()
            .expect("captured modal IME transport")
            .dispatch_once(&mut ime_dispatcher)
            .expect("dispatch modal IME frame");
        assert!(
            ime_dispatcher.seen.is_empty(),
            "modal IME input suppresses the shortcut"
        );
    }

    let (modal, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let mut forwarder = Forwarder(None);
    root.forward_global_events_once(&modal, &mut forwarder)
        .expect("authorize modal toggle");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    forwarder
        .0
        .take()
        .expect("captured modal toggle transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch modal toggle");
    assert_eq!(
        dispatcher.seen,
        vec![KucGlobalActionTarget::from_opaque_bytes(
            b"same-global-target"
        )]
    );

    // The host consumes the authenticated event and projects the dialog closed.
    root.synchronize_selection_dialog(open_query_dialog(2).open(false))
        .expect("project query dialog closed");
    let (closed, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let mut closed_forwarder = Forwarder(None);
    root.forward_global_events_once(&closed, &mut closed_forwarder)
        .expect("authorize post-close shortcut");
    let mut closed_dispatcher = Dispatcher { seen: Vec::new() };
    closed_forwarder
        .0
        .take()
        .expect("captured post-close transport")
        .dispatch_once(&mut closed_dispatcher)
        .expect("dispatch post-close shortcut");
    assert_eq!(
        closed_dispatcher.seen,
        vec![KucGlobalActionTarget::from_opaque_bytes(
            b"same-global-target"
        )]
    );
}

#[test]
fn global_action_rejects_primary_f_disabled_and_read_only_or_ime_focus_inputs() {
    let context = egui::Context::default();
    context.enable_accesskit();
    for (read_only, enabled) in [(false, false), (true, true)] {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease(1, read_only, enabled, Rc::clone(&seen)))
            .expect("retain global action root");
        let input = vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        }];
        let (output, _) = frame(&context, &mut root, input);
        let mut forwarder = Forwarder(None);
        output
            .forward_events_once(&mut forwarder)
            .expect("forward negative input");
        assert!(
            forwarder.0.take().is_some(),
            "negative input still forwards an empty root batch"
        );
        assert!(
            seen.borrow().is_empty(),
            "primary+F must not activate global action"
        );
    }
}

#[test]
fn global_action_ime_focus_and_stale_or_other_root_frames_are_rejected() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_focus_policy(
            1,
            false,
            true,
            Rc::clone(&seen),
            KucGlobalActionFocusPolicy::DocumentOnly,
        ))
        .expect("retain global action root");

    for ime in [
        egui::ImeEvent::Preedit {
            text: "preedit".to_owned(),
            active_range_chars: None,
        },
        egui::ImeEvent::Commit("commit".to_owned()),
    ] {
        let (frame, _) = frame(
            &context,
            &mut root,
            vec![
                egui::Event::Ime(ime),
                egui::Event::Key {
                    key: egui::Key::F,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
                },
            ],
        );
        let mut forwarder = Forwarder(None);
        root.forward_global_events_once(&frame, &mut forwarder)
            .expect("IME frame remains an authenticated empty batch");
        let mut dispatcher = Dispatcher { seen: Vec::new() };
        forwarder
            .0
            .take()
            .expect("captured IME transport")
            .dispatch_once(&mut dispatcher)
            .expect("dispatch IME frame");
        assert!(
            dispatcher.seen.is_empty(),
            "IME input must suppress the shortcut"
        );
    }

    context.memory_mut(|memory| memory.request_focus(egui::Id::new("other-input")));
    let (other_focus, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let mut other_focus_forwarder = Forwarder(None);
    root.forward_global_events_once(&other_focus, &mut other_focus_forwarder)
        .expect("other-focus frame remains authenticated");
    let mut other_focus_dispatcher = Dispatcher { seen: Vec::new() };
    other_focus_forwarder
        .0
        .take()
        .expect("captured other-focus transport")
        .dispatch_once(&mut other_focus_dispatcher)
        .expect("dispatch other-focus frame");
    assert!(
        other_focus_dispatcher.seen.is_empty(),
        "other input focus must suppress the shortcut"
    );

    context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("other-input")));
    let (stale, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let _ = frame(&context, &mut root, Vec::new());
    assert!(
        root.forward_global_events_once(&stale, &mut Forwarder(None))
            .is_err()
    );

    let mut other = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain other root");
    let (other_frame, _) = frame(
        &context,
        &mut other,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    assert!(
        root.forward_global_events_once(&other_frame, &mut Forwarder(None))
            .is_err()
    );
}

#[test]
fn global_menu_escape_and_outside_pointer_dismiss_the_open_menu() {
    for dismiss in [
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        vec![
            egui::Event::PointerMoved(egui::pos2(760.0, 440.0)),
            egui::Event::PointerButton {
                pos: egui::pos2(760.0, 440.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    ] {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
            .expect("retain global action root");
        let (_, output) = frame(&context, &mut root, Vec::new());
        let (_, menu_bounds) = button(&output, "Global actions").expect("menu button");
        let _ = frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerButton {
                    pos: menu_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: menu_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let (dismissed, dismissed_output) = frame(&context, &mut root, dismiss);
        assert!(button(&dismissed_output, "Global action").is_none());
        assert!(
            dismissed
                .interaction_locator()
                .request(
                    KucGlobalActionTarget::from_opaque_bytes(b"same-global-target")
                        .interaction_selector()
                )
                .is_err()
        );
    }
}

#[test]
fn removing_or_disabling_the_projection_invalidates_old_global_action_hits() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain global action root");
    let (_, initial_output) = frame(&context, &mut root, Vec::new());
    let (_, menu_bounds) = button(&initial_output, "Global actions").expect("menu button");
    let (initial, _) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(menu_bounds.center()),
            egui::Event::PointerButton {
                pos: menu_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: menu_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let selector =
        KucGlobalActionTarget::from_opaque_bytes(b"same-global-target").interaction_selector();
    let mut old_hit = initial
        .interaction_locator()
        .request(selector.clone())
        .expect("old global hit");

    root.synchronize_with_lease(lease_without_global_actions(2, false))
        .expect("remove global projection");
    let (removed, _) = frame(&context, &mut root, Vec::new());
    assert!(
        removed
            .interaction_locator()
            .request(selector.clone())
            .is_err()
    );
    let mut stale_input = egui::RawInput::default();
    old_hit
        .apply_to_raw_input_once(&mut stale_input)
        .expect("queue old global hit");
    let (stale_frame, _) = frame(&context, &mut root, stale_input.events);
    let mut stale_forwarder = Forwarder(None);
    root.forward_global_events_once(&stale_frame, &mut stale_forwarder)
        .expect("authorize stale-hit frame");
    let mut stale_dispatcher = Dispatcher { seen: Vec::new() };
    stale_forwarder
        .0
        .take()
        .expect("captured stale-hit transport")
        .dispatch_once(&mut stale_dispatcher)
        .expect("dispatch stale-hit frame");
    assert!(
        stale_dispatcher.seen.is_empty(),
        "old locator hit must not activate after projection removal"
    );

    root.synchronize_with_lease(lease(3, false, false, Rc::new(RefCell::new(Vec::new()))))
        .expect("disable global projection");
    let (_, disabled_output) = frame(&context, &mut root, Vec::new());
    let (_, disabled_menu) = button(&disabled_output, "Global actions").expect("disabled menu");
    let (disabled, _) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(disabled_menu.center()),
            egui::Event::PointerButton {
                pos: disabled_menu.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: disabled_menu.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert!(matches!(
        disabled.interaction_locator().request(selector),
        Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Disabled)
    ));
}

#[test]
fn captured_global_transport_is_rejected_after_lease_update_and_root_drop_before_dispatch() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain global action root");
    let (frame, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let mut forwarder = Forwarder(None);
    root.forward_global_events_once(&frame, &mut forwarder)
        .expect("capture authenticated global transport");
    let transport = forwarder.0.take().expect("captured transport");
    root.synchronize_with_lease(lease_without_global_actions(2, false))
        .expect("advance lease before dispatch");
    drop(root);

    let mut dispatcher = Dispatcher { seen: Vec::new() };
    assert!(transport.dispatch_once(&mut dispatcher).is_err());
    assert!(
        dispatcher.seen.is_empty(),
        "stale transport must not dispatch global events or sibling classes"
    );
}

#[test]
fn global_menu_and_primary_shift_shortcut_share_target_without_document_normal_and_readonly() {
    let context = egui::Context::default();
    context.enable_accesskit();
    for (read_only, document) in [(false, false), (false, true), (true, true)] {
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease_case(1, read_only, document))
            .expect("retain case root");
        let (_, initial_output) = frame(&context, &mut root, Vec::new());
        let (_, menu_bounds) = button(&initial_output, "Global actions").expect("menu button");
        let (_, opened_output) = frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerMoved(menu_bounds.center()),
                egui::Event::PointerButton {
                    pos: menu_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: menu_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let (_, action_bounds) = button(&opened_output, "Global action").expect("action row");
        let (menu_frame, _) = frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerButton {
                    pos: action_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: action_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let mut menu_forwarder = Forwarder(None);
        root.forward_global_events_once(&menu_frame, &mut menu_forwarder)
            .expect("authorize menu");
        let mut menu_dispatcher = Dispatcher { seen: Vec::new() };
        menu_forwarder
            .0
            .take()
            .expect("menu transport")
            .dispatch_once(&mut menu_dispatcher)
            .expect("dispatch menu");
        assert_eq!(
            menu_dispatcher.seen,
            vec![KucGlobalActionTarget::from_opaque_bytes(
                b"same-global-target"
            )]
        );

        let (shortcut_frame, _) = frame(
            &context,
            &mut root,
            vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            }],
        );
        let mut shortcut_forwarder = Forwarder(None);
        root.forward_global_events_once(&shortcut_frame, &mut shortcut_forwarder)
            .expect("authorize shortcut");
        let mut shortcut_dispatcher = Dispatcher { seen: Vec::new() };
        shortcut_forwarder
            .0
            .take()
            .expect("shortcut transport")
            .dispatch_once(&mut shortcut_dispatcher)
            .expect("dispatch shortcut");
        assert_eq!(
            shortcut_dispatcher.seen, menu_dispatcher.seen,
            "menu and shortcut target must match"
        );
    }
}

#[test]
fn same_frame_focus_competition_is_document_only_conservative_but_any_still_dispatches() {
    let context = egui::Context::default();
    context.enable_accesskit();
    for policy in [
        KucGlobalActionFocusPolicy::DocumentOnly,
        KucGlobalActionFocusPolicy::Any,
    ] {
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease_with_focus_policy(
                1,
                false,
                true,
                Rc::new(RefCell::new(Vec::new())),
                policy,
            ))
            .expect("retain focus-policy root");
        let (_, initial_output) = frame(&context, &mut root, Vec::new());
        let document_bounds =
            text_input_bounds(&initial_output).expect("document TextInput bounds");
        context.memory_mut(|memory| memory.request_focus(egui::Id::new("other-input")));
        let (transition, _) = frame(
            &context,
            &mut root,
            vec![
                egui::Event::PointerButton {
                    pos: document_bounds.center(),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::Key {
                    key: egui::Key::F,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
                },
            ],
        );
        let mut forwarder = Forwarder(None);
        root.forward_global_events_once(&transition, &mut forwarder)
            .expect("authorize focus transition");
        let mut dispatcher = Dispatcher { seen: Vec::new() };
        forwarder
            .0
            .take()
            .expect("focus transition transport")
            .dispatch_once(&mut dispatcher)
            .expect("dispatch focus transition");
        if policy == KucGlobalActionFocusPolicy::Any {
            assert_eq!(dispatcher.seen.len(), 1);
        } else {
            assert!(dispatcher.seen.is_empty());
        }
    }
}

#[test]
fn real_external_text_edit_focus_moves_both_directions_in_one_frame() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_focus_policy(
            1,
            false,
            true,
            Rc::new(RefCell::new(Vec::new())),
            KucGlobalActionFocusPolicy::DocumentOnly,
        ))
        .expect("retain root");
    let (_, initial, external_bounds, external_id) =
        frame_with_external_input(&context, &mut root, Vec::new());
    let document_bounds = text_input_bounds(&initial).expect("root text input bounds");
    context.memory_mut(|memory| memory.request_focus(external_id));
    assert_eq!(context.memory(|memory| memory.focused()), Some(external_id));
    let (other_to_document, _, _, _) = frame_with_external_input(
        &context,
        &mut root,
        vec![
            egui::Event::PointerButton {
                pos: document_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            },
        ],
    );
    let mut forwarder = Forwarder(None);
    root.forward_global_events_once(&other_to_document, &mut forwarder)
        .expect("authorize other to document");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    forwarder
        .0
        .take()
        .expect("transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch");
    assert!(dispatcher.seen.is_empty());
    let (document_to_other, _, _, _) = frame_with_external_input(
        &context,
        &mut root,
        vec![
            egui::Event::PointerButton {
                pos: external_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            },
        ],
    );
    let mut second = Forwarder(None);
    root.forward_global_events_once(&document_to_other, &mut second)
        .expect("authorize document to other");
    let mut second_dispatcher = Dispatcher { seen: Vec::new() };
    second
        .0
        .take()
        .expect("transport")
        .dispatch_once(&mut second_dispatcher)
        .expect("dispatch");
    assert!(second_dispatcher.seen.is_empty());
}

#[test]
fn registered_document_search_and_global_shortcuts_deliver_distinct_targets_once() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_search_action(1))
        .expect("retain registry root");
    let mut targets = Vec::new();
    for modifiers in [
        egui::Modifiers::COMMAND,
        egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
    ] {
        let (frame, _) = frame(
            &context,
            &mut root,
            vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            }],
        );
        let mut forwarder = Forwarder(None);
        root.forward_global_events_once(&frame, &mut forwarder)
            .expect("authorize shortcut");
        let mut dispatcher = Dispatcher { seen: Vec::new() };
        forwarder
            .0
            .take()
            .expect("transport")
            .dispatch_once(&mut dispatcher)
            .expect("dispatch shortcut");
        assert_eq!(dispatcher.seen.len(), 1);
        targets.push(dispatcher.seen.remove(0));
    }
    assert_ne!(targets[0], targets[1]);
    let mut unregistered = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain unregistered root");
    let (frame, _) = frame(
        &context,
        &mut unregistered,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        }],
    );
    let mut forwarder = Forwarder(None);
    unregistered
        .forward_global_events_once(&frame, &mut forwarder)
        .expect("authorize unregistered frame");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    forwarder
        .0
        .take()
        .expect("transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch unregistered frame");
    assert!(dispatcher.seen.is_empty());
}

#[test]
fn global_transport_becomes_stale_when_selection_dialog_state_changes_before_dispatch() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain root");
    let (frame, _) = frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
    );
    let mut forwarder = Forwarder(None);
    root.forward_global_events_once(&frame, &mut forwarder)
        .expect("capture global transport");
    let transport = forwarder.0.take().expect("transport");
    root.synchronize_selection_dialog(open_query_dialog(2))
        .expect("change root state before dispatch");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    assert!(transport.dispatch_once(&mut dispatcher).is_err());
    assert!(dispatcher.seen.is_empty());
}

#[test]
fn shortcut_dispatch_can_open_then_close_selection_dialog_with_same_target() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("retain root");
    let mut delivered = Vec::new();
    for (revision, dialog) in [(1, true), (2, false)] {
        let (toggle_frame, _) = frame(
            &context,
            &mut root,
            vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            }],
        );
        let mut forwarder = Forwarder(None);
        root.forward_global_events_once(&toggle_frame, &mut forwarder)
            .expect("authorize toggle");
        let mut dispatcher = Dispatcher { seen: Vec::new() };
        forwarder
            .0
            .take()
            .expect("transport")
            .dispatch_once(&mut dispatcher)
            .expect("dispatch toggle");
        delivered.extend(dispatcher.seen);
        root.synchronize_selection_dialog(open_query_dialog(revision).open(dialog))
            .expect("project dialog state");
        let _ = frame(&context, &mut root, Vec::new());
    }
    assert_eq!(
        delivered,
        vec![
            KucGlobalActionTarget::from_opaque_bytes(b"same-global-target"),
            KucGlobalActionTarget::from_opaque_bytes(b"same-global-target"),
        ]
    );
}

#[test]
fn global_menu_and_tab_context_menu_do_not_share_pointer_input() {
    use katana_ui_core::egui::text_command_surface::{
        TabStripContextMenuPresentation, TabStripCorrelation, TabStripMenuEntry,
        TabStripMenuOperation, TabStripProjection, TabStripProjectionLease,
        TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    };
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"other-popup-tab"),
        TabStripText::new("Other popup tab"),
    )
    .capabilities(
        TabStripTabCapabilities::new()
            .selectable(true)
            .closeable(true),
    )
    .context_menu(
        TabStripContextMenuPresentation::new().entry(TabStripMenuEntry::action(
            TabStripText::new("Other popup close"),
            TabStripText::new("Other popup close"),
            TabStripMenuOperation::RequestClose,
        )),
    );
    let tabs = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"other-popup-correlation"),
    )
    .tab(tab);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            lease(1, false, true, Rc::new(RefCell::new(Vec::new())))
                .with_tab_strip(TabStripProjectionLease::new(tabs)),
        )
        .expect("root");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (_, initial) = frame(&context, &mut root, Vec::new());
    let (_, tab_bounds) = button(&initial, "Other popup tab").expect("tab bounds");
    let (_, menu_bounds) = button(&initial, "Global actions").expect("global bounds");
    let tab_point = tab_bounds.center();
    let (_, tab_open) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(tab_point),
            egui::Event::PointerButton {
                pos: tab_point,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: tab_point,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert!(
        button(&tab_open, "Other popup close").is_some(),
        "real tab menu must open first"
    );
    let global_point = menu_bounds.center();
    let (_, global_open) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(global_point),
            egui::Event::PointerButton {
                pos: global_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: global_point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert!(
        button(&global_open, "Global action").is_some(),
        "global menu must open"
    );
    let (_, after_global) = frame(&context, &mut root, Vec::new());
    assert!(
        button(&after_global, "Other popup close").is_none(),
        "global trigger dismisses the old popup"
    );
    let (_, after_tab_click) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(tab_point),
            egui::Event::PointerButton {
                pos: tab_point,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: tab_point,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert!(
        button(&after_tab_click, "Other popup close").is_none(),
        "open global menu owns this outside click"
    );
    let (_, settled) = frame(&context, &mut root, Vec::new());
    assert!(
        button(&settled, "Other popup close").is_none(),
        "the same click cannot reopen the tab menu"
    );
}

#[test]
fn stale_global_transport_rejects_all_sibling_events_and_opaque_effect_before_dispatch() {
    use katana_ui_core::egui::text_command_surface::{
        KucOpaqueHostEffectBatch, KucOpaqueHostEffectError,
    };
    use std::cell::Cell;
    struct Audit(Rc<Cell<usize>>);
    impl KucRootEventBatchDispatcher for Audit {
        type Error = ();
        fn dispatch_global_action_events(
            &mut self,
            _: Vec<KucGlobalActionEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_text_events(
            &mut self,
            _: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_toolbar_events(
            &mut self,
            _: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_floating_events(
            &mut self,
            _: Vec<katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_search_events(
            &mut self,
            _: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_context_menu_events(
            &mut self,
            _: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_selection_dialog_events(
            &mut self,
            _: Vec<katana_ui_core::egui::text_command_surface::SelectionDialogEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_status_bar_events(
            &mut self,
            _: Vec<katana_ui_core::molecule::StatusBarEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
        fn dispatch_diagnostics_list_events(
            &mut self,
            _: Vec<katana_ui_core::molecule::DiagnosticsListEvent>,
        ) -> Result<(), Self::Error> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
    }
    let effect_count = Rc::new(Cell::new(0));
    let effect_for_router = Rc::clone(&effect_count);
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"atomic-global-root",
        presentation(false),
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    let action = KucGlobalAction::new(
        KucGlobalActionTarget::from_opaque_bytes(b"atomic-global"),
        "Atomic global",
        KucGlobalShortcut::new(
            egui::Key::F,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
    );
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |_context: KucRootEventBatchContext| {
            let count = Rc::clone(&effect_for_router);
            Ok::<_, KucOpaqueHostEffectError>(Some(KucOpaqueHostEffectBatch::from_handler(
                move || {
                    count.set(count.get() + 1);
                    Ok(())
                },
            )))
        },
    )
    .with_global_actions(KucGlobalActionsProjection::new("Atomic menu").action(action));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease)
        .expect("root");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (_, initial) = frame(&context, &mut root, Vec::new());
    let point = text_input_bounds(&initial)
        .expect("document input bounds")
        .center();
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
    let (produced, _) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::Text("x".to_owned()),
            egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            },
        ],
    );
    let mut forwarder = Forwarder(None);
    let receipt = root
        .forward_global_events_once(&produced, &mut forwarder)
        .expect("authorize current frame");
    assert!(
        receipt.event_cardinality() >= 2,
        "the batch must include global and document text events"
    );
    let transport = forwarder.0.take().expect("transport");
    frame(&context, &mut root, Vec::new());
    let callbacks = Rc::new(Cell::new(0));
    assert!(
        transport
            .dispatch_once(&mut Audit(Rc::clone(&callbacks)))
            .is_err()
    );
    assert_eq!(callbacks.get(), 0, "no sibling or global callback may run");
    assert_eq!(effect_count.get(), 0, "opaque host effect must not run");
}

fn guarded_frame(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    disabled: bool,
    clip: bool,
    cover: bool,
) -> (
    katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    egui::FullOutput,
) {
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 480.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                ui.add_enabled_ui(!disabled, |ui| {
                    if clip {
                        ui.set_clip_rect(egui::Rect::from_min_size(
                            egui::pos2(400.0, 400.0),
                            egui::vec2(10.0, 10.0),
                        ));
                    }
                    result = Some(root.show(ui).expect("root"));
                });
            });
            if cover {
                egui::Area::new(egui::Id::new("cover"))
                    .fixed_pos(egui::pos2(0.0, 0.0))
                    .order(egui::Order::Tooltip)
                    .show(ui.ctx(), |ui| {
                        ui.allocate_exact_size(egui::vec2(200.0, 80.0), egui::Sense::click());
                    });
            }
        },
    );
    output.textures_delta.clear();
    (result.expect("root frame"), output)
}

fn pointer(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

#[test]
fn native_global_hit_respects_cover_disabled_clip_and_outside_release() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("root");
    let (_, initial) = guarded_frame(&context, &mut root, Vec::new(), false, false, false);
    let pos = button(&initial, "Global actions").expect("menu").1.center();
    assert!(
        button(&initial, "Global action").is_none(),
        "closed menu is absent"
    );
    let (clipped_frame, clipped) =
        guarded_frame(&context, &mut root, Vec::new(), false, true, false);
    assert!(
        clipped_frame
            .interaction_locator()
            .request(KucGlobalActionsProjection::new("Global actions").menu_selector())
            .is_err()
    );
    assert!(
        button(&clipped, "Global actions").is_none(),
        "clipped menu has no accessible hit"
    );
    let (disabled, _) = guarded_frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        }],
        true,
        false,
        false,
    );
    let mut f = Forwarder(None);
    root.forward_global_events_once(&disabled, &mut f)
        .expect("forward disabled");
    let mut d = Dispatcher { seen: Vec::new() };
    f.0.take()
        .expect("transport")
        .dispatch_once(&mut d)
        .expect("dispatch");
    assert!(d.seen.is_empty(), "disabled root suppresses shortcuts");
    let _ = guarded_frame(&context, &mut root, Vec::new(), false, false, true);
    let _ = guarded_frame(&context, &mut root, Vec::new(), false, false, true);
    let (_, covered) = guarded_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(pos),
            pointer(pos, true),
            pointer(pos, false),
        ],
        false,
        false,
        true,
    );
    assert!(
        button(&covered, "Global action").is_none(),
        "cover owns pointer"
    );
    let _ = guarded_frame(&context, &mut root, Vec::new(), false, false, false);
    let _ = guarded_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(pos), pointer(pos, true)],
        false,
        false,
        false,
    );
    let outside = egui::pos2(600.0, 400.0);
    let _ = guarded_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(outside), pointer(outside, false)],
        false,
        false,
        false,
    );
    let _ = guarded_frame(
        &context,
        &mut root,
        vec![pointer(outside, true)],
        false,
        false,
        false,
    );
    let (_, after) = guarded_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(pos), pointer(pos, false)],
        false,
        false,
        false,
    );
    assert!(
        button(&after, "Global action").is_none(),
        "outside release clears capture"
    );
}

const IDLE_FRAME_LIMIT: usize = 32;

fn assert_idle_repaint_delay(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    disabled: bool,
    case: &str,
) {
    let mut settled = false;
    for _ in 0..IDLE_FRAME_LIMIT {
        let (_, output) = guarded_frame(context, root, Vec::new(), disabled, false, false);
        if output.viewport_output[&egui::ViewportId::ROOT].repaint_delay > Duration::ZERO {
            settled = true;
            break;
        }
    }
    assert!(settled, "{case} should stop requesting immediate repaints");
}

#[test]
fn disabled_root_with_closed_global_menu_settles_without_repaint_requests() {
    let baseline_context = egui::Context::default();
    let mut baseline = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_without_global_actions(1, false))
        .expect("baseline root");
    assert_idle_repaint_delay(&baseline_context, &mut baseline, true, "baseline root");

    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_case(1, false, false))
        .expect("global action root without document");
    assert_idle_repaint_delay(&context, &mut root, true, "global root");
}

#[test]
fn disabling_parent_closes_open_global_menu_without_dispatch_or_implicit_reopen() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_case(1, false, false))
        .expect("root");
    let (_, initial) = guarded_frame(&context, &mut root, vec![], false, false, false);
    let menu = button(&initial, "Global actions").expect("menu").1.center();
    let (_, opened) = guarded_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(menu),
            pointer(menu, true),
            pointer(menu, false),
        ],
        false,
        false,
        false,
    );
    let action = button(&opened, "Global action")
        .expect("open action")
        .1
        .center();
    let (disabled, hidden) = guarded_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(action),
            pointer(action, true),
            pointer(action, false),
        ],
        true,
        false,
        false,
    );
    assert!(button(&hidden, "Global action").is_none());
    let mut forwarder = Forwarder(None);
    root.forward_global_events_once(&disabled, &mut forwarder)
        .expect("authorize disabled frame");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    forwarder
        .0
        .take()
        .expect("transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch disabled frame");
    assert!(dispatcher.seen.is_empty());
    assert_idle_repaint_delay(&context, &mut root, true, "disabled formerly open menu");
    let (_, enabled) = guarded_frame(&context, &mut root, vec![], false, false, false);
    assert!(
        button(&enabled, "Global action").is_none(),
        "reenabling must not reopen the menu"
    );
}

#[test]
fn covered_action_never_dispatches() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("root");
    let (_, initial) = frame(&context, &mut root, Vec::new());
    let trigger = button(&initial, "Global actions")
        .expect("trigger")
        .1
        .center();
    let (_, opened) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(trigger),
            pointer(trigger, true),
            pointer(trigger, false),
        ],
    );
    let action = button(&opened, "Global action").expect("action").1.center();
    let _ = guarded_frame(&context, &mut root, Vec::new(), false, false, true);
    let _ = guarded_frame(&context, &mut root, Vec::new(), false, false, true);
    let (covered, _) = guarded_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(action),
            pointer(action, true),
            pointer(action, false),
        ],
        false,
        false,
        true,
    );
    let mut f = Forwarder(None);
    root.forward_global_events_once(&covered, &mut f)
        .expect("forward");
    let mut d = Dispatcher { seen: Vec::new() };
    f.0.take()
        .expect("transport")
        .dispatch_once(&mut d)
        .expect("dispatch");
    assert!(d.seen.is_empty(), "covered action cannot dispatch");
}

#[test]
fn global_frame_rejects_pending_pass_then_accepts_once_after_commit() {
    let context = egui::Context::default();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, false, true, Rc::new(RefCell::new(Vec::new()))))
        .expect("root");
    let mut pending_frame = None;
    let mut forwarder = Forwarder(None);
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 480.0),
            )),
            events: vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            }],
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let current = root.show(ui).expect("root frame");
                assert!(
                    root.forward_global_events_once(&current, &mut forwarder)
                        .is_err(),
                    "pending pass cannot forward"
                );
                assert!(forwarder.0.is_none());
                pending_frame = Some(current);
            });
        },
    );
    output.textures_delta.clear();
    let accepted = pending_frame.expect("accepted frame");
    root.forward_global_events_once(&accepted, &mut forwarder)
        .expect("accepted pass");
    let mut dispatcher = Dispatcher { seen: Vec::new() };
    forwarder
        .0
        .take()
        .expect("transport")
        .dispatch_once(&mut dispatcher)
        .expect("dispatch");
    assert_eq!(
        dispatcher.seen,
        vec![KucGlobalActionTarget::from_opaque_bytes(
            b"same-global-target"
        )]
    );
    assert!(
        root.forward_global_events_once(&accepted, &mut forwarder)
            .is_err(),
        "one shot"
    );
}
