#![cfg(feature = "egui")]

use std::cell::RefCell;
use std::rc::Rc;

use katana_ui_core::egui::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    EguiTextCommandSurfaceRootFactoryError, KucContextMenuEnabledResolverError,
    KucInteractionActionClass, KucInteractionSelector, KucOpaqueHostEffectBatch,
    KucRootEventBatchContext, KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
    TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::selection::{ContextMenuEvent, ContextMenuItemKind};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

fn presentation() -> EguiTextCommandSurfacePresentation {
    let text = TextSurface::new(
        TextSurfaceProps::new(
            katana_ui_core::atom::TextArea::new("context-enabled-text").value("本文"),
            Vec::<UiTextSpan>::new(),
            TextSurfaceViewport::new(0, 0, 640, 360),
        )
        .accessibility_label("Context enabled text")
        .context_target_label("Context target"),
    );
    EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("context-enabled-text")),
        text: TextSurfacePresentation::from_props(text.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: Some(ContextMenuPresentation {
            visible: true,
            items: vec![
                ContextMenuPresentationItem::action("context.toggle", "Toggle"),
                ContextMenuPresentationItem {
                    id: "context.divider".into(),
                    label: "Divider".into(),
                    accessibility_label: String::new(),
                    icon: None,
                    enabled: false,
                    checked: false,
                    kind: ContextMenuItemKind::Divider,
                    children: Vec::new(),
                },
                ContextMenuPresentationItem::action("context.parent", "Parent").child(
                    ContextMenuPresentationItem::action("context.child", "Child"),
                ),
            ],
        }),
    }
}

#[derive(Default)]
struct Forwarder {
    context_events: usize,
    activations: Vec<String>,
}

impl KucRootEventBatchDispatcher for Forwarder {
    type Error = String;

    fn dispatch_text_events(&mut self, _: Vec<TextSurfaceEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_toolbar_events(&mut self, _: Vec<CommandChromeToolbarEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_floating_events(
        &mut self,
        _: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_search_events(&mut self, _: Vec<CommandChromeSearchEvent>) -> Result<(), String> {
        Ok(())
    }
    fn dispatch_context_menu_events(
        &mut self,
        events: Vec<ContextMenuEvent>,
    ) -> Result<(), String> {
        self.context_events += events.len();
        self.activations
            .extend(events.into_iter().filter_map(|event| {
                if let ContextMenuEvent::ItemSelected { command, .. } = event {
                    Some(command)
                } else {
                    None
                }
            }));
        Ok(())
    }
}

impl KucRootEventBatchForwarder for Forwarder {
    type Error = String;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), String> {
        transport
            .dispatch_once(self)
            .map(|_| ())
            .map_err(|error| format!("{error:?}"))
    }
}

fn frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> Result<
    (
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
        egui::FullOutput,
    ),
    EguiTextCommandSurfaceRootFactoryError,
> {
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                result = Some(root.show(ui));
            });
        },
    );
    output.textures_delta.clear();
    result
        .expect("host root frame")
        .map(|value| (value, output))
}

fn frame_with_texture_limit(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    max_texture_side: usize,
) -> Result<
    (
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
        egui::FullOutput,
    ),
    EguiTextCommandSurfaceRootFactoryError,
> {
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            max_texture_side: Some(max_texture_side),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                result = Some(root.show(ui));
            });
        },
    );
    match result.expect("host root frame") {
        Ok(value) => Ok((value, output)),
        Err(error) => {
            output.textures_delta.clear();
            Err(error)
        }
    }
}

fn frame_with_parent_enabled(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
    enabled: bool,
) -> Result<
    (
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
        egui::FullOutput,
    ),
    EguiTextCommandSurfaceRootFactoryError,
> {
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 360.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    result = Some(root.show(ui));
                });
            });
        },
    );
    output.textures_delta.clear();
    result
        .expect("host root frame")
        .map(|value| (value, output))
}

fn lease<F>(
    revision: u64,
    resolver: F,
    router_events: Rc<RefCell<usize>>,
) -> EguiTextCommandSurfaceHostProjectionLease
where
    F: FnMut(&str, bool) -> Result<bool, KucContextMenuEnabledResolverError> + 'static,
{
    lease_for_target_with_presentation(
        revision,
        b"context-enabled-root",
        presentation(),
        resolver,
        router_events,
    )
}

fn lease_for_target<F>(
    revision: u64,
    target: &[u8],
    resolver: F,
    router_events: Rc<RefCell<usize>>,
) -> EguiTextCommandSurfaceHostProjectionLease
where
    F: FnMut(&str, bool) -> Result<bool, KucContextMenuEnabledResolverError> + 'static,
{
    lease_for_target_with_presentation(revision, target, presentation(), resolver, router_events)
}

fn lease_for_target_with_presentation<F>(
    revision: u64,
    target: &[u8],
    presentation: EguiTextCommandSurfacePresentation,
    resolver: F,
    router_events: Rc<RefCell<usize>>,
) -> EguiTextCommandSurfaceHostProjectionLease
where
    F: FnMut(&str, bool) -> Result<bool, KucContextMenuEnabledResolverError> + 'static,
{
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        target.to_vec(),
        presentation,
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            if !context.context_menu_events().is_empty() {
                *router_events.borrow_mut() += context.context_menu_events().len();
                Ok(Some(KucOpaqueHostEffectBatch::from_handler(|| Ok(()))))
            } else {
                Ok(None)
            }
        },
    )
    .with_context_menu_enabled_resolver(resolver)
}

fn lease_without_resolver(
    revision: u64,
    router_events: Rc<RefCell<usize>>,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"context-enabled-root".to_vec(),
        presentation(),
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            if !context.context_menu_events().is_empty() {
                *router_events.borrow_mut() += context.context_menu_events().len();
                Ok(Some(KucOpaqueHostEffectBatch::from_handler(|| Ok(()))))
            } else {
                Ok(None)
            }
        },
    )
}

fn lease_with_actions<F>(
    revision: u64,
    resolver: F,
    host_actions: Rc<RefCell<Vec<String>>>,
) -> EguiTextCommandSurfaceHostProjectionLease
where
    F: FnMut(&str, bool) -> Result<bool, KucContextMenuEnabledResolverError> + 'static,
{
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"context-enabled-root".to_vec(),
        presentation(),
        TextCommandSurfaceStyle::standard().expect("style"),
    )
    .expect("token");
    EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        move |context: KucRootEventBatchContext| {
            let commands = context
                .context_menu_events()
                .iter()
                .filter_map(|event| {
                    if let ContextMenuEvent::ItemSelected { command, .. } = event {
                        Some(command.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if commands.is_empty() {
                Ok(None)
            } else {
                let actions_for_effect = Rc::clone(&host_actions);
                Ok(Some(KucOpaqueHostEffectBatch::from_handler(move || {
                    actions_for_effect.borrow_mut().extend(commands);
                    Ok(())
                })))
            }
        },
    )
    .with_context_menu_enabled_resolver(resolver)
}

#[test]
fn resolver_projects_enabled_state_into_ax_and_locator_without_host_geometry() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let router_events = Rc::new(RefCell::new(0));
    let enabled = Rc::new(RefCell::new(false));
    let calls = Rc::new(RefCell::new(Vec::<(String, bool)>::new()));
    let enabled_for_resolver = Rc::clone(&enabled);
    let calls_for_resolver = Rc::clone(&calls);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            1,
            move |id, enabled| {
                calls_for_resolver
                    .borrow_mut()
                    .push((id.to_owned(), enabled));
                Ok(*enabled_for_resolver.borrow())
            },
            Rc::clone(&router_events),
        ))
        .expect("root");

    let (initial, _) = frame(&context, &mut root, Vec::new()).expect("closed frame");
    assert!(
        calls.borrow().is_empty(),
        "closed frame must not query enabled state"
    );
    let mut open = initial
        .interaction_locator()
        .request_context_open()
        .expect("context target");
    let mut input = egui::RawInput::default();
    open.apply_to_raw_input_once(&mut input)
        .expect("open input");

    let calls_for_open = Rc::clone(&calls);
    root.synchronize_with_lease(lease(
        2,
        move |id, enabled| {
            calls_for_open.borrow_mut().push((id.to_owned(), enabled));
            Ok(false)
        },
        Rc::clone(&router_events),
    ))
    .expect("fresh lease");
    let (opened, output) = frame(&context, &mut root, input.events).expect("opened frame");
    let queried_ids = calls
        .borrow()
        .iter()
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();
    assert!(queried_ids.iter().any(|id| id == "context.toggle"));
    assert!(queried_ids.iter().any(|id| id == "context.parent"));
    assert!(queried_ids.iter().any(|id| id == "context.child"));
    assert!(!queried_ids.iter().any(|id| id == "context.divider"));
    for id in ["context.toggle", "context.parent", "context.child"] {
        assert_eq!(
            queried_ids
                .iter()
                .filter(|candidate| candidate == &id)
                .count(),
            1
        );
    }
    assert!(
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some("Toggle") && node.is_disabled())
            })
    );
    assert!(
        opened
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "context.toggle",
                KucInteractionActionClass::ContextMenuItem,
            ))
            .is_err()
    );

    *enabled.borrow_mut() = true;
    let enabled_for_resolver = Rc::clone(&enabled);
    root.synchronize_with_lease(lease(
        3,
        move |_, _| Ok(*enabled_for_resolver.borrow()),
        Rc::clone(&router_events),
    ))
    .expect("enable lease");
    let (enabled_frame, enabled_output) =
        frame(&context, &mut root, Vec::new()).expect("enabled frame");
    assert_ne!(
        opened.record().accessibility_snapshot_hash(),
        enabled_frame.record().accessibility_snapshot_hash()
    );
    assert!(
        enabled_output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some("Toggle") && !node.is_disabled())
            })
    );
    let mut click = enabled_frame
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "context.toggle",
            KucInteractionActionClass::ContextMenuItem,
        ))
        .expect("enabled locator request");
    let mut click_input = egui::RawInput::default();
    click
        .apply_to_raw_input_once(&mut click_input)
        .expect("click input");

    *enabled.borrow_mut() = false;
    let enabled_for_resolver = Rc::clone(&enabled);
    root.synchronize_with_lease(lease(
        4,
        move |_, _| Ok(*enabled_for_resolver.borrow()),
        Rc::clone(&router_events),
    ))
    .expect("disable lease");
    let (disabled_frame, disabled_output) =
        frame(&context, &mut root, click_input.events).expect("disabled frame");
    assert_ne!(
        enabled_frame.record().accessibility_snapshot_hash(),
        disabled_frame.record().accessibility_snapshot_hash()
    );
    assert!(
        disabled_output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some("Toggle") && node.is_disabled())
            })
    );
    let mut blocked_forwarder = Forwarder::default();
    disabled_frame
        .forward_events_once(&mut blocked_forwarder)
        .expect("disabled frame forward");
    assert!(blocked_forwarder.activations.is_empty());
}

#[test]
fn disabled_parent_area_rejects_split_pointer_and_reenabled_accepts() {
    let context = egui::Context::default();
    let router_events = Rc::new(RefCell::new(0));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            1,
            |_, enabled| Ok(enabled),
            Rc::clone(&router_events),
        ))
        .expect("root");
    let pointer = egui::pos2(24.0, 24.0);
    let (opened, _) = frame_with_parent_enabled(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(pointer),
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        true,
    )
    .expect("open context menu");
    let mut click = opened
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "context.toggle",
            KucInteractionActionClass::ContextMenuItem,
        ))
        .expect("context item");
    let mut click_input = egui::RawInput::default();
    click
        .apply_to_raw_input_once(&mut click_input)
        .expect("click input");
    let mut down_events = Vec::new();
    let mut up_events = Vec::new();
    let mut after_down = false;
    for event in click_input.events {
        let pressed = matches!(&event, egui::Event::PointerButton { pressed: true, .. });
        if after_down {
            up_events.push(event);
        } else {
            down_events.push(event);
            after_down |= pressed;
        }
    }
    assert!(after_down, "locator must include a pointer-down event");
    frame_with_parent_enabled(&context, &mut root, down_events, true).expect("pointer down");
    let baseline_router_events = *router_events.borrow();
    let (blocked, _) =
        frame_with_parent_enabled(&context, &mut root, up_events, false).expect("disabled up");
    let mut blocked_forwarder = Forwarder::default();
    blocked
        .forward_events_once(&mut blocked_forwarder)
        .expect("blocked forward");
    assert!(blocked_forwarder.activations.is_empty());
    assert_eq!(*router_events.borrow(), baseline_router_events);

    let (reenabled, _) =
        frame_with_parent_enabled(&context, &mut root, Vec::new(), true).expect("reenable");
    let mut retry = reenabled
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "context.toggle",
            KucInteractionActionClass::ContextMenuItem,
        ))
        .expect("retry context item");
    let mut retry_input = egui::RawInput::default();
    retry
        .apply_to_raw_input_once(&mut retry_input)
        .expect("retry input");
    let (accepted, _) = frame_with_parent_enabled(&context, &mut root, retry_input.events, true)
        .expect("reenabled click");
    let mut accepted_forwarder = Forwarder::default();
    accepted
        .forward_events_once(&mut accepted_forwarder)
        .expect("accepted forward");
    assert_eq!(accepted_forwarder.activations, vec!["context.toggle"]);
}

#[test]
fn cold_open_secondary_pointer_resolves_each_item_once() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let calls = Rc::new(RefCell::new(Vec::<String>::new()));
    let calls_for_resolver = Rc::clone(&calls);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            1,
            move |id, _| {
                calls_for_resolver.borrow_mut().push(id.to_owned());
                Ok(true)
            },
            Rc::new(RefCell::new(0)),
        ))
        .expect("root");
    let pointer = egui::pos2(24.0, 24.0);
    let (opened, output) = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(pointer),
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    )
    .expect("cold open frame");
    assert!(output.platform_output.accesskit_update.is_some());
    let queried = calls.borrow().clone();
    for id in ["context.toggle", "context.parent", "context.child"] {
        assert_eq!(
            queried
                .iter()
                .filter(|candidate| candidate.as_str() == id)
                .count(),
            1
        );
    }
    assert!(!queried.iter().any(|id| id == "context.divider"));
    assert!(
        opened
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "context.toggle",
                KucInteractionActionClass::ContextMenuItem,
            ))
            .is_ok()
    );
}

#[test]
fn resolver_failure_returns_typed_error_without_reusing_previous_success() {
    let context = egui::Context::default();
    let router_events = Rc::new(RefCell::new(0));
    let marker = KucContextMenuEnabledResolverError;
    assert_eq!(marker.to_string(), "context-menu enabled resolver failed");
    assert_eq!(format!("{marker:?}"), "KucContextMenuEnabledResolverError");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, |_, _| Ok(true), router_events))
        .expect("root");
    let (initial, _) = frame(&context, &mut root, Vec::new()).expect("initial frame");
    let mut open = initial
        .interaction_locator()
        .request_context_open()
        .expect("context target");
    let mut open_input = egui::RawInput::default();
    open.apply_to_raw_input_once(&mut open_input)
        .expect("open input");
    assert_eq!(
        open_input
            .events
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Secondary,
                        ..
                    }
                )
            })
            .count(),
        2
    );
    root.synchronize_with_lease(lease(2, |_, _| Ok(true), Rc::new(RefCell::new(0))))
        .expect("open lease");
    frame(&context, &mut root, open_input.events).expect("opened frame");
    let error = root
        .synchronize_with_lease(lease(3, move |_, _| Err(marker), Rc::new(RefCell::new(0))))
        .and_then(|_| frame(&context, &mut root, Vec::new()).map(|_| ()))
        .expect_err("resolver failure");
    assert!(matches!(
        error,
        EguiTextCommandSurfaceRootFactoryError::ContextMenuEnabledResolver(_)
    ));
    root.synchronize_with_lease(lease(4, |_, _| Ok(false), Rc::new(RefCell::new(0))))
        .expect("recovery lease");
    frame(&context, &mut root, Vec::new()).expect("recovery frame");
}

#[test]
fn child_item_resolution_failure_rejects_the_entire_visible_frame() {
    let context = egui::Context::default();
    let router_events = Rc::new(RefCell::new(0));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            1,
            |id, enabled| {
                if id == "context.child" {
                    Err(KucContextMenuEnabledResolverError)
                } else {
                    Ok(enabled)
                }
            },
            Rc::clone(&router_events),
        ))
        .expect("root");
    let (closed, _) = frame(&context, &mut root, Vec::new()).expect("closed frame");
    let mut request = closed
        .interaction_locator()
        .request_context_open()
        .expect("open request");
    let mut input = egui::RawInput::default();
    request
        .apply_to_raw_input_once(&mut input)
        .expect("open input");
    let error = frame(&context, &mut root, input.events)
        .err()
        .expect("child resolver must reject frame");
    assert!(matches!(
        error,
        EguiTextCommandSurfaceRootFactoryError::ContextMenuEnabledResolver(_)
    ));
    assert_eq!(*router_events.borrow(), 0);
}

#[test]
fn resolver_enabled_item_dispatches_one_real_host_event_and_recovers_after_failure() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let host_actions = Rc::new(RefCell::new(Vec::<String>::new()));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_actions(
            1,
            |_, enabled| Ok(enabled),
            Rc::clone(&host_actions),
        ))
        .expect("root");
    let (initial, _) = frame(&context, &mut root, Vec::new()).expect("initial frame");
    let mut open = initial
        .interaction_locator()
        .request_context_open()
        .expect("context target");
    let mut open_input = egui::RawInput::default();
    open.apply_to_raw_input_once(&mut open_input)
        .expect("open input");
    root.synchronize_with_lease(lease_with_actions(
        2,
        |_, enabled| Ok(enabled),
        Rc::clone(&host_actions),
    ))
    .expect("open lease");
    let _ = frame(&context, &mut root, open_input.events).expect("opened frame");
    let (stable, _) = frame(&context, &mut root, Vec::new()).expect("stable open frame");
    let mut click = stable
        .interaction_locator()
        .request(KucInteractionSelector::new(
            "context.toggle",
            KucInteractionActionClass::ContextMenuItem,
        ))
        .expect("enabled context item");
    let mut click_input = egui::RawInput::default();
    click
        .apply_to_raw_input_once(&mut click_input)
        .expect("click input");
    let (clicked, _) = frame(&context, &mut root, click_input.events).expect("clicked frame");
    assert!(host_actions.borrow().is_empty());
    let mut forwarder = Forwarder::default();
    clicked
        .forward_events_once(&mut forwarder)
        .expect("forward once");
    assert_eq!(forwarder.activations, vec!["context.toggle"]);
    assert!(forwarder.context_events >= 1);
    assert_eq!(*host_actions.borrow(), vec!["context.toggle"]);
    let actions_after_first_forward = host_actions.borrow().clone();
    assert!(clicked.forward_events_once(&mut forwarder).is_err());
    assert_eq!(*host_actions.borrow(), actions_after_first_forward);
}

#[test]
fn resolver_lease_keeps_duplicate_stale_and_identity_guards() {
    let router_events = Rc::new(RefCell::new(0));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            2,
            |_, enabled| Ok(enabled),
            Rc::clone(&router_events),
        ))
        .expect("root");
    assert!(matches!(
        root.synchronize_with_lease(lease(
            2,
            |_, enabled| Ok(enabled),
            Rc::clone(&router_events),
        )),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 2 })
    ));
    assert!(matches!(
        root.synchronize_with_lease(lease(
            1,
            |_, enabled| Ok(enabled),
            Rc::clone(&router_events),
        )),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 1 })
    ));
    assert!(matches!(
        root.synchronize_with_lease(lease_for_target(
            3,
            b"different-context-root",
            |_, enabled| Ok(enabled),
            router_events,
        )),
        Err(EguiTextCommandSurfaceRootFactoryError::IdentityChanged)
    ));
}

#[test]
fn resolver_can_be_removed_by_a_new_plain_lease() {
    let context = egui::Context::default();
    let calls = Rc::new(RefCell::new(0usize));
    let calls_for_resolver = Rc::clone(&calls);
    let router_events = Rc::new(RefCell::new(0));
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            1,
            move |_, enabled| {
                *calls_for_resolver.borrow_mut() += 1;
                Ok(enabled)
            },
            Rc::clone(&router_events),
        ))
        .expect("root");
    let (initial, _) = frame(&context, &mut root, Vec::new()).expect("initial frame");
    let mut open = initial
        .interaction_locator()
        .request_context_open()
        .expect("context target");
    let mut input = egui::RawInput::default();
    open.apply_to_raw_input_once(&mut input)
        .expect("open input");
    let calls_for_open = Rc::clone(&calls);
    root.synchronize_with_lease(lease(
        2,
        move |_, enabled| {
            *calls_for_open.borrow_mut() += 1;
            Ok(enabled)
        },
        Rc::clone(&router_events),
    ))
    .expect("resolver lease");
    frame(&context, &mut root, input.events).expect("resolver frame");
    let queried = *calls.borrow();
    assert!(queried > 0);
    root.synchronize_with_lease(lease_without_resolver(3, router_events))
        .expect("plain lease");
    frame(&context, &mut root, Vec::new()).expect("plain lease frame");
    assert_eq!(*calls.borrow(), queried);
}

fn key_event(key: egui::Key, pressed: bool) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

#[test]
fn failed_resolver_cannot_block_escape_or_outside_dismissal() {
    for outside in [false, true] {
        let context = egui::Context::default();
        let failed = Rc::new(std::cell::Cell::new(false));
        let calls = Rc::new(std::cell::Cell::new(0usize));
        let resolver_failed = Rc::clone(&failed);
        let resolver_calls = Rc::clone(&calls);
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(lease(
                1,
                move |_, enabled| {
                    resolver_calls.set(resolver_calls.get() + 1);
                    if resolver_failed.get() {
                        Err(KucContextMenuEnabledResolverError)
                    } else {
                        Ok(enabled)
                    }
                },
                Rc::new(RefCell::new(0)),
            ))
            .expect("root");
        let (initial, _) = frame(&context, &mut root, Vec::new()).expect("closed frame");
        let mut request = initial
            .interaction_locator()
            .request_context_open()
            .expect("open");
        let mut input = egui::RawInput::default();
        request.apply_to_raw_input_once(&mut input).expect("input");
        frame(&context, &mut root, input.events).expect("opened frame");
        frame(&context, &mut root, Vec::new()).expect("stable open frame");
        failed.set(true);
        assert!(matches!(
            frame(&context, &mut root, Vec::new()),
            Err(EguiTextCommandSurfaceRootFactoryError::ContextMenuEnabledResolver(_))
        ));
        let before_close = calls.get();
        let close_events = if outside {
            let position = egui::pos2(620.0, 340.0);
            vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ]
        } else {
            vec![key_event(egui::Key::Escape, true)]
        };
        let (closed, _) = frame(&context, &mut root, close_events)
            .expect("resolver failure must not trap an open menu");
        assert_eq!(calls.get(), before_close);
        assert!(
            closed
                .interaction_locator()
                .request(KucInteractionSelector::new(
                    "context.toggle",
                    KucInteractionActionClass::ContextMenuItem
                ))
                .is_err()
        );
        frame(&context, &mut root, Vec::new())
            .expect("closed frame must not query failed resolver");
        assert_eq!(calls.get(), before_close);
    }
}

#[test]
fn long_context_menu_labels_upload_only_backend_sized_tiles() {
    let mut long_presentation = presentation();
    long_presentation
        .context_menu
        .as_mut()
        .expect("context menu presentation")
        .items[0]
        .label = "長いメニュー ".repeat(40);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_for_target_with_presentation(
            1,
            b"long-context-menu",
            long_presentation,
            |_, enabled| Ok(enabled),
            Rc::new(RefCell::new(0)),
        ))
        .expect("root");
    let context = egui::Context::default();
    let (initial, mut initial_output) =
        frame_with_texture_limit(&context, &mut root, Vec::new(), 1024).expect("initial frame");
    initial_output.textures_delta.clear();
    let mut request = initial
        .interaction_locator()
        .request_context_open()
        .expect("context target");
    let mut input = egui::RawInput::default();
    request
        .apply_to_raw_input_once(&mut input)
        .expect("open input");
    let (_, mut output) =
        frame_with_texture_limit(&context, &mut root, input.events, 1024).expect("opened frame");
    let uploaded = output
        .textures_delta
        .set
        .values()
        .flat_map(|deltas| deltas.iter().map(|delta| delta.image.size()))
        .collect::<Vec<_>>();
    output.textures_delta.clear();
    assert!(!uploaded.is_empty(), "context menu emitted no textures");
    assert!(
        uploaded
            .iter()
            .all(|size| size.iter().all(|side| *side <= 1024)),
        "context-menu texture upload exceeded backend limit"
    );
}

#[test]
fn escape_press_release_closes_menu_and_stops_enabled_queries() {
    let context = egui::Context::default();
    let calls = Rc::new(RefCell::new(0usize));
    let calls_for_resolver = Rc::clone(&calls);
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(
            1,
            move |_, enabled| {
                *calls_for_resolver.borrow_mut() += 1;
                Ok(enabled)
            },
            Rc::new(RefCell::new(0)),
        ))
        .expect("root");
    let (initial, _) = frame(&context, &mut root, Vec::new()).expect("initial");
    let mut open = initial
        .interaction_locator()
        .request_context_open()
        .expect("context target");
    let mut open_input = egui::RawInput::default();
    open.apply_to_raw_input_once(&mut open_input)
        .expect("open input");
    assert_eq!(
        open_input
            .events
            .iter()
            .filter(|event| matches!(
                event,
                egui::Event::PointerButton {
                    button: egui::PointerButton::Secondary,
                    ..
                }
            ))
            .count(),
        2
    );
    let calls_for_open = Rc::clone(&calls);
    root.synchronize_with_lease(lease(
        2,
        move |_, enabled| {
            *calls_for_open.borrow_mut() += 1;
            Ok(enabled)
        },
        Rc::new(RefCell::new(0)),
    ))
    .expect("open lease");
    let (opened, _) = frame(&context, &mut root, open_input.events).expect("opened");
    assert!(
        opened
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "context.toggle",
                KucInteractionActionClass::ContextMenuItem,
            ))
            .is_ok()
    );
    let queried = *calls.borrow();
    assert!(queried > 0);

    frame(
        &context,
        &mut root,
        vec![key_event(egui::Key::Escape, true)],
    )
    .expect("escape press");
    let after_press = *calls.borrow();
    frame(
        &context,
        &mut root,
        vec![key_event(egui::Key::Escape, false)],
    )
    .expect("escape release");
    let after_release = *calls.borrow();
    let (idle, idle_output) = frame(&context, &mut root, Vec::new()).expect("idle after escape");
    let after_idle = *calls.borrow();
    assert_eq!(after_press, queried);
    assert_eq!(after_release, after_press);
    assert_eq!(after_idle, after_release);
    assert!(
        !idle_output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some("Toggle")))
    );
    assert!(
        idle.interaction_locator()
            .request(KucInteractionSelector::new(
                "context.toggle",
                KucInteractionActionClass::ContextMenuItem,
            ))
            .is_err()
    );
}
