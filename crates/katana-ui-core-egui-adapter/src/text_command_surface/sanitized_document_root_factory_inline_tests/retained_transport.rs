use super::root_frame_support::*;
use super::support::*;
use super::*;

#[test]
fn retained_transport_propagates_a_real_child_dispatch_rejection() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"child-rejection", "本文"))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let frame = run_root_frame_events(&context, &mut root, Vec::new()).1;
    let mut forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };

    frame
        .forward_events_once(&mut forwarder)
        .expect("root forwarding succeeds");
    assert_eq!(
        forwarder.dispatch_root_once_rejecting_text(),
        Err(SanitizedDocumentRootEventDispatchError::Child(()))
    );
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
}
