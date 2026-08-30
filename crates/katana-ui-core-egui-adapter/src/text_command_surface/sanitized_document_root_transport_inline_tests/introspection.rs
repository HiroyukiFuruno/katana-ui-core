#[test]
fn public_transport_is_opaque_and_has_no_payload_accessor() {
    let declaration = before(
        required(
            include_str!("../sanitized_document_root_transport/types.rs"),
            "pub struct SanitizedDocumentRootEventTransport",
        ),
        "/// Failure while the host consumes the opaque root port.",
    );
    let implementation = before(
        required(
            include_str!("../sanitized_document_root_transport.rs"),
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
    let source = include_str!("../sanitized_document_root_transport.rs");
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
    let source = include_str!("../sanitized_document_root_transport.rs");
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
