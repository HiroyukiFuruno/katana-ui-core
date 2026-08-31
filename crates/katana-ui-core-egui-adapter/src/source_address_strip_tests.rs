use super::*;
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};

fn output() -> EguiSourceAddressStripOutput {
    EguiSourceAddressStripOutput {
        event_classes: Vec::new(),
        submissions: Vec::new(),
    }
}

fn strip_with_draft(draft: &str) -> SourceAddressStrip {
    let mut strip =
        SourceAddressStrip::new(SourceAddressPresentation::new("source", "source", "source"));
    let _ = strip.apply_action(SourceAddressAction::SetDraft(draft.to_owned()));
    strip
}

#[test]
fn output_classifies_every_generic_event_without_debugging_submission_payload() {
    let mut output = output();
    output.record_all(vec![
        SourceAddressEvent::DraftChanged,
        SourceAddressEvent::EnabledChanged,
        SourceAddressEvent::Focused,
        SourceAddressEvent::Blurred,
        SourceAddressEvent::HistoryOpened,
        SourceAddressEvent::HistoryClosed,
        SourceAddressEvent::CandidatesOpened,
        SourceAddressEvent::CandidatesClosed,
        SourceAddressEvent::HistorySelected,
        SourceAddressEvent::CandidateSelected,
    ]);
    let mut strip = strip_with_draft("opaque-draft-value");
    let submitted = strip
        .apply_action(SourceAddressAction::Submit)
        .expect("enabled source-address accepts submission");
    output.record(submitted);

    assert_eq!(
        output.event_classes(),
        &[
            SourceAddressFrameEventClass::DraftChanged,
            SourceAddressFrameEventClass::EnabledChanged,
            SourceAddressFrameEventClass::Focused,
            SourceAddressFrameEventClass::Blurred,
            SourceAddressFrameEventClass::HistoryOpened,
            SourceAddressFrameEventClass::HistoryClosed,
            SourceAddressFrameEventClass::CandidatesOpened,
            SourceAddressFrameEventClass::CandidatesClosed,
            SourceAddressFrameEventClass::HistorySelected,
            SourceAddressFrameEventClass::CandidateSelected,
            SourceAddressFrameEventClass::Submitted,
        ]
    );
    let debug = format!("{output:?}");
    assert!(debug.contains("event_class_count: 11"));
    assert!(debug.contains("submission_count: 1"));
    assert!(!debug.contains("opaque-draft-value"));

    let submissions = output.take_submissions();
    assert_eq!(submissions.len(), 1);
    assert_eq!(
        submissions.into_iter().next().map(|it| it.into_draft()),
        Some("opaque-draft-value".to_owned())
    );
}

struct RejectingForwarder;

impl SourceAddressSubmissionForwarder for RejectingForwarder {
    type Error = &'static str;

    fn forward_submission(
        &mut self,
        _submission: katana_ui_core::molecule::structured::source_address_strip::SourceAddressSubmission,
    ) -> Result<(), Self::Error> {
        Err("host rejected source submission")
    }
}

#[test]
fn submission_forwarding_propagates_the_host_error_once() {
    let mut output = output();
    let mut strip = strip_with_draft("opaque-draft-value");
    output.record(
        strip
            .apply_action(SourceAddressAction::Submit)
            .expect("enabled source-address accepts submission"),
    );

    assert_eq!(
        output.forward_submissions_once(&mut RejectingForwarder),
        Err("host rejected source submission")
    );
}
