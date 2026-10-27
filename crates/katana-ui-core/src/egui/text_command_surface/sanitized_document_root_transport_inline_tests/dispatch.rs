#[test]
fn real_transport_debug_remains_opaque() {
    let debug = format!("{:?}", real_sanitized_transport());

    assert_eq!(
        debug,
        "SanitizedDocumentRootEventTransport { payload: \"<opaque>\" }"
    );
    assert!(!debug.contains("本文"));
    assert!(!debug.contains("transport-root"));
}

#[test]
fn real_global_batch_requires_authorization_before_sanitized_bridge() {
    use crate::atom::TextArea;
    use crate::egui::text_command_surface::{
        EguiTextCommandSurface, EguiTextCommandSurfaceRoot, KucGlobalAction, KucGlobalActionTarget,
        KucGlobalActionsProjection, KucGlobalShortcut, SanitizedDocumentRootEventForwardError,
        TextCommandSurfaceStyle,
    };
    use crate::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};
    let text = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("global-sanitized"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 640, 480),
    ));
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "global-sanitized",
        EguiTextCommandSurface::new(text),
    )
    .expect("root");
    root.global_actions = Some(KucGlobalActionsProjection::new("Actions").action(
        KucGlobalAction::new(
            KucGlobalActionTarget::from_opaque_bytes(b"action"),
            "Action",
            KucGlobalShortcut::new(
                egui::Key::F,
                egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            ),
        ),
    ));
    let context = egui::Context::default();
    let mut result = None;
    let mut frame = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, ROOT_VIEWPORT_SIZE)),
            events: vec![egui::Event::Key {
                key: egui::Key::F,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            }],
            ..Default::default()
        },
        |ui| result = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style"))),
    );
    frame.textures_delta.clear();
    let output = result.expect("frame").expect("show");
    let tabs = RefCell::new(Some(Vec::<SanitizedTabProjectionClosedEvent>::new()));
    let search = RefCell::new(Some(Vec::<SanitizedSearchEventTransport>::new()));
    let commands = RefCell::new(Some(Vec::<SanitizedCommandActivationTransport>::new()));
    let menus = RefCell::new(Some(Vec::<SanitizedContextMenuActivationTransport>::new()));
    let mut forwarder = RetainingForwarder { transport: None };
    assert!(matches!(
        forward_root_events_once(&output, &tabs, &search, &commands, &menus, &mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    ));
    assert!(forwarder.transport.is_none());
    assert!(
        tabs.borrow().is_some()
            && search.borrow().is_some()
            && commands.borrow().is_some()
            && menus.borrow().is_some()
    );
}

#[test]
fn real_transport_maps_child_rejection_and_second_dispatch_consumption() {
    let mut transport = real_sanitized_transport();
    let mut dispatcher = TestDispatcher { fail_text: true };

    assert_eq!(
        transport.dispatch_root_once(&mut dispatcher),
        Err(SanitizedDocumentRootEventDispatchError::Child(
            "text dispatch rejected"
        ))
    );
    dispatcher.fail_text = false;
    assert_eq!(
        transport.dispatch_root_once(&mut dispatcher),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
}

#[test]
fn real_transport_dispatches_successfully_with_the_same_dispatcher_type() {
    let mut transport = real_sanitized_transport();
    let mut dispatcher = TestDispatcher { fail_text: false };

    let receipt = transport
        .dispatch_root_once(&mut dispatcher)
        .expect("the actual opaque transport dispatches successfully");

    assert!(
        receipt
            .class_dispatches()
            .iter()
            .all(|dispatch| dispatch.event_count == 0)
    );
}

struct RemovingSourcePortForwarder {
    transport: Option<SanitizedDocumentRootEventTransport>,
}

impl SanitizedDocumentRootEventForwarder for RemovingSourcePortForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        mut transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        let root_transport = transport
            .root_transport
            .take()
            .expect("the bridge supplied a root transport")
            .with_source_address_submission_port(None);
        transport.root_transport = Some(root_transport);
        self.transport = Some(transport);
        Ok(())
    }
}

#[test]
fn real_source_submission_without_its_port_maps_to_closed_host_effect_error() {
    let scenario = FullTextCommandSurfaceScenarioFactory::new()
        .issue(FullTextCommandSurfaceScenarioId::NavigationInput)
        .expect("navigation scenario issues");
    let stages = scenario.stages().to_vec();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            scenario
                .into_lease()
                .expect("scenario lease remains available"),
        )
        .expect("navigation root retains");
    let context = egui::Context::default();
    let mut submitted = None;
    for stage in &stages {
        let mut input = egui::RawInput::default();
        stage.apply_to(&mut input);
        let mut output = None;
        crate::egui::run_ui_discard(&context, input, |ui| {
            output = Some(
                root.show_output_for_test(ui)
                    .expect("navigation stage renders"),
            );
        });
        submitted = output;
    }
    let submitted = submitted.expect("navigation submit frame exists");
    assert_eq!(
        submitted
            .events()
            .current_context()
            .source_address_submission_count(),
        1
    );

    let tab_events = RefCell::new(Some(Vec::<SanitizedTabProjectionClosedEvent>::new()));
    let search_events = RefCell::new(Some(Vec::<SanitizedSearchEventTransport>::new()));
    let command_events = RefCell::new(Some(Vec::<SanitizedCommandActivationTransport>::new()));
    let context_menu_events =
        RefCell::new(Some(Vec::<SanitizedContextMenuActivationTransport>::new()));
    let mut forwarder = RemovingSourcePortForwarder { transport: None };
    let receipt = forward_root_events_once(
        &submitted,
        &tab_events,
        &search_events,
        &command_events,
        &context_menu_events,
        &mut forwarder,
    )
    .expect("the real source event reaches the sanitized transport bridge");
    assert_eq!(receipt.event_cardinality(), 1);

    let mut transport = forwarder
        .transport
        .expect("the sanitized source transport was retained");
    let mut dispatcher = TestDispatcher { fail_text: false };
    assert_eq!(
        transport.dispatch_root_once(&mut dispatcher),
        Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
    );
}
