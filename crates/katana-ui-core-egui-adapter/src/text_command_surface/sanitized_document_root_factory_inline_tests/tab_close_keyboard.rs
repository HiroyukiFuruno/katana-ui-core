use super::root_frame_support::*;
use super::support::*;
use super::*;

#[test]
fn physical_focused_close_button_enter_and_space_each_emit_one_opaque_intent() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let factory = SanitizedDocumentRootFactory::new();
        let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
        let context = egui::Context::default();

        let first = run_root_frame(
            &context,
            &mut root,
            egui::Event::PointerMoved(egui::Pos2::ZERO),
        );
        assert!(
            first
                .tab_close_rects()
                .iter()
                .any(|(_, rect)| rect.width() > 0.0)
        );

        for _ in 0..4 {
            let focused = run_root_frame(&context, &mut root, key_press(egui::Key::Tab));
            assert_eq!(focused.tab_closed_event_count(), 0);
            assert_eq!(focused.tab_activation_event_count(), 0);
        }

        let activated = run_root_frame(&context, &mut root, key_press(key));
        assert_eq!(activated.tab_activation_event_count(), 0);
        assert_eq!(activated.tab_close_request_event_count(), 1);
        assert_eq!(activated.tab_closed_event_count(), 1);
        assert!(
            activated
                .tab_close_rects()
                .iter()
                .any(|(_, rect)| rect.width() > 0.0),
            "close affordance must remain until the host projects the next state"
        );

        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
            reject_forwarding: false,
        };
        let receipt = activated
            .forward_events_once(&mut forwarder)
            .expect("physical key close forwarding succeeds");
        assert_eq!(receipt.event_cardinality(), 1);
        assert_eq!(forwarder.calls, 1);
        assert_eq!(
            activated.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);

        let transport_debug = forwarder
            .transport_debug
            .as_deref()
            .expect("transport debug exists");
        for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
            assert!(!transport_debug.contains(forbidden));
        }
        assert!(transport_debug.contains("<opaque>"));

        let retained = run_root_frame(
            &context,
            &mut root,
            egui::Event::PointerMoved(egui::Pos2::ZERO),
        );
        assert!(
            retained
                .tab_close_rects()
                .iter()
                .any(|(_, rect)| rect.width() > 0.0)
        );

        root.synchronize(input_with_one_tab(2))
            .expect("host next projection synchronizes");
        let projected = run_root_frame(
            &context,
            &mut root,
            egui::Event::PointerMoved(egui::Pos2::ZERO),
        );
        assert!(projected.tab_close_rects().is_empty());
    }
}

fn input_with_one_tab(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_tab_projection(SanitizedTabProjection::new([
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([1]),
            0,
            "最初",
        )),
    ]))
}
