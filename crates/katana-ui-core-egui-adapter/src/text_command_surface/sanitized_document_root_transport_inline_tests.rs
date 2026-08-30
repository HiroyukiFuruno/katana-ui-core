use super::{
    SanitizedCommandActivationTransport, SanitizedContextMenuActivationTransport,
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwarder,
    SanitizedDocumentRootEventTransport, SanitizedSearchEventTransport,
    SanitizedTabProjectionClosedEvent, forward_root_events_once,
};
use crate::text_command_surface::{
    EguiTextCommandSurfaceRootFactory, FullTextCommandSurfaceScenarioFactory,
    FullTextCommandSurfaceScenarioId, KucRootEventBatchDispatcher, SanitizedDocumentRootFactory,
    SanitizedDocumentRootIdentity, SanitizedDocumentRootInput, SanitizedDocumentRootStyleKey,
};
use std::cell::RefCell;

fn required<'a>(source: &'a str, marker: &str) -> &'a str {
    match source.split_once(marker) {
        Some((_, remainder)) => remainder,
        None => panic!("source marker is missing: {marker}"),
    }
}

fn before<'a>(source: &'a str, marker: &str) -> &'a str {
    match source.split_once(marker) {
        Some((prefix, _)) => prefix,
        None => panic!("source marker is missing: {marker}"),
    }
}

#[test]
fn public_transport_is_opaque_and_has_no_payload_accessor() {
    let declaration = before(
        required(
            include_str!("sanitized_document_root_transport/types.rs"),
            "pub struct SanitizedDocumentRootEventTransport",
        ),
        "/// Failure while the host consumes the opaque root port.",
    );
    let implementation = before(
        required(
            include_str!("sanitized_document_root_transport.rs"),
            "impl SanitizedDocumentRootEventTransport",
        ),
        "impl std::fmt::Debug",
    );
    let public = format!("{declaration}\n{implementation}");
    for forbidden in [
        "pub fn payload",
        "pub fn events",
        "pub fn target",
        "pub fn correlation",
        "pub fn handler",
        "pub fn into_inner",
        "TextSurfaceEvent",
        "CommandChrome",
        "rgba_pixels",
    ] {
        assert!(
            !public.contains(forbidden),
            "transport leaked `{forbidden}`"
        );
    }
}

#[test]
fn bridge_invokes_callbacks_only_from_the_opaque_effect_closure() {
    let source = include_str!("sanitized_document_root_transport.rs");
    let implementation = before(source, "#[cfg(test)]");
    let implementation = required(
        implementation,
        "impl<Forwarder> KucRootEventBatchForwarder for RootEventForwarderBridge",
    );
    let closure = required(
        implementation,
        "let effect_batch = KucOpaqueHostEffectBatch::from_handler(move || {",
    );
    let closure_end = match closure.split_once("        });") {
        Some((body, _)) => body,
        None => panic!("opaque effect closure is not closed"),
    };

    assert_eq!(closure_end.matches("event.invoke_once").count(), 3);
    for event_kind in ["command_events", "context_menu_events", "search_events"] {
        assert!(
            closure_end.contains(&format!("for mut event in {event_kind}")),
            "callback batch is not attached for {event_kind}"
        );
    }
    assert_eq!(implementation.matches("event.invoke_once").count(), 3);
}

#[test]
fn opaque_effect_is_attached_before_the_outer_forwarder_call() {
    let source = include_str!("sanitized_document_root_transport.rs");
    let implementation = before(source, "#[cfg(test)]");
    let implementation = required(
        implementation,
        "impl<Forwarder> KucRootEventBatchForwarder for RootEventForwarderBridge",
    );
    let effect = match implementation.find("let effect_batch =") {
        Some(position) => position,
        None => panic!("opaque effect batch is not constructed"),
    };
    let attach = match implementation.find("with_opaque_host_effect_batch(effect_batch)") {
        Some(position) => position,
        None => panic!("opaque effect batch is not attached"),
    };
    let forward = match implementation.find("forward_sanitized_document_root_event(transport)") {
        Some(position) => position,
        None => panic!("outer sanitized forwarder call is missing"),
    };
    assert!(
        effect < attach,
        "effect batch must be attached after construction"
    );
    assert!(
        attach < forward,
        "effect batch must be attached before forwarding"
    );

    let before_effect = &implementation[..effect];
    assert_eq!(before_effect.matches("event.invoke_once").count(), 0);
}

struct RetainingForwarder {
    transport: Option<SanitizedDocumentRootEventTransport>,
}

impl SanitizedDocumentRootEventForwarder for RetainingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.transport = Some(transport);
        Ok(())
    }
}

struct TestDispatcher {
    fail_text: bool,
}

impl KucRootEventBatchDispatcher for TestDispatcher {
    type Error = &'static str;

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        if self.fail_text {
            Err("text dispatch rejected")
        } else {
            Ok(())
        }
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

fn real_sanitized_transport() -> SanitizedDocumentRootEventTransport {
    let input = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes(b"transport-root"),
        "本文 ⭐️",
        SanitizedDocumentRootStyleKey::Default,
    );
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input)
        .expect("sanitized root retains");
    let context = egui::Context::default();
    let mut frame = None;
    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui).expect("sanitized root renders"));
            });
        },
    );
    let frame = frame.expect("sanitized frame exists");
    let mut forwarder = RetainingForwarder { transport: None };
    frame
        .forward_events_once(&mut forwarder)
        .expect("real frame forwards its opaque transport");
    forwarder
        .transport
        .expect("forwarder retained the real transport")
}

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
        crate::run_ui_discard(&context, input, |ui| {
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
