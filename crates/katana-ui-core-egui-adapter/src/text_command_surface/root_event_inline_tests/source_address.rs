#[test]
fn source_submission_requires_a_dedicated_port_and_forwards_once() {
    let mut payload = full_payload();
    payload.source_address_submissions.push(source_submission());
    let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, String::new());
    let transport = batch
        .transport
        .borrow_mut()
        .take()
        .expect("transport exists");
    let received = Rc::new(RefCell::new(Vec::new()));
    let mut dispatcher = OrderRecorder {
        calls: Vec::new(),
        context_menu_dispatch_complete: Rc::new(Cell::new(false)),
    };
    let receipt = transport
        .with_source_address_submission_port(Some(SourceAddressSubmissionPortHandle::new(
            RecordingSourcePort {
                received: Rc::clone(&received),
                fail: false,
            },
        )))
        .dispatch_once(&mut dispatcher)
        .expect("source port dispatch succeeds");
    assert_eq!(receipt.class_dispatches().len(), 7);
    assert_eq!(received.borrow().as_slice(), &["opaque-source-draft"]);
}

#[test]
fn source_submission_port_handle_survives_distinct_one_shot_transports() {
    let received = Rc::new(RefCell::new(Vec::new()));
    let port = SourceAddressSubmissionPortHandle::new(RecordingSourcePort {
        received: Rc::clone(&received),
        fail: false,
    });

    for _ in 0..2 {
        let mut payload = full_payload();
        payload.source_address_submissions.push(source_submission());
        let transport = EguiTextCommandSurfaceRootEventTransport {
            payload,
            opaque_host_effect_batch: None,
            source_address_submission_port: Some(port.clone()),
        };
        let mut dispatcher = OrderRecorder {
            calls: Vec::new(),
            context_menu_dispatch_complete: Rc::new(Cell::new(false)),
        };
        transport
            .dispatch_once(&mut dispatcher)
            .expect("each one-shot transport forwards once");
    }

    assert_eq!(
        received.borrow().as_slice(),
        &["opaque-source-draft", "opaque-source-draft"]
    );
}

#[test]
fn source_submission_missing_or_rejected_port_fails_closed() {
    let mut payload = full_payload();
    payload.source_address_submissions.push(source_submission());
    let transport = EguiTextCommandSurfaceRootEventTransport {
        payload,
        opaque_host_effect_batch: None,
        source_address_submission_port: None,
    };
    let mut dispatcher = OrderRecorder {
        calls: Vec::new(),
        context_menu_dispatch_complete: Rc::new(Cell::new(false)),
    };
    assert!(matches!(
        transport.dispatch_once(&mut dispatcher),
        Err(
            EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort(
                SourceAddressSubmissionPortError::Rejected
            )
        )
    ));

    let mut payload = full_payload();
    payload.source_address_submissions.push(source_submission());
    let transport = EguiTextCommandSurfaceRootEventTransport {
        payload,
        opaque_host_effect_batch: None,
        source_address_submission_port: Some(SourceAddressSubmissionPortHandle::new(
            RecordingSourcePort {
                received: Rc::new(RefCell::new(Vec::new())),
                fail: true,
            },
        )),
    };
    let mut dispatcher = OrderRecorder {
        calls: Vec::new(),
        context_menu_dispatch_complete: Rc::new(Cell::new(false)),
    };
    assert!(matches!(
        transport.dispatch_once(&mut dispatcher),
        Err(
            EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort(
                SourceAddressSubmissionPortError::Rejected
            )
        )
    ));
}
