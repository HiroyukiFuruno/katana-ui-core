use super::{EguiSourceAddressStripAdapter, EguiSourceAddressStripOutput};
use crate::source_address_strip::SourceAddressFrameEventClass;
use katana_ui_core::atom::{TextAreaEvent, TextAreaKeyChord, TextAreaValidationError};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressPresentation, SourceAddressStrip,
};
use katana_ui_core::text_surface::TextSurfaceEvent;

fn strip() -> SourceAddressStrip {
    SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソース",
        "ソースを入力",
        "ソースを入力",
    ))
}

#[test]
fn source_address_adapter_text_surface_events_are_dispatched_to_output() {
    let adapter = EguiSourceAddressStripAdapter::new("source-address-dispatch-test")
        .expect("adapter should initialize");
    let mut strip = strip();
    let mut output = EguiSourceAddressStripOutput {
        event_classes: Vec::new(),
        submissions: Vec::new(),
    };

    adapter.apply_text_surface_events(
        &mut output,
        &mut strip,
        &[
            TextSurfaceEvent::FocusChanged(true),
            TextSurfaceEvent::TextArea(TextAreaEvent::Change("next".to_owned())),
            TextSurfaceEvent::TextArea(TextAreaEvent::Submit("value".to_owned())),
        ],
    );

    assert!(
        output
            .event_classes
            .contains(&SourceAddressFrameEventClass::Focused)
    );
    assert!(
        output
            .event_classes
            .contains(&SourceAddressFrameEventClass::Submitted)
    );
    assert!(
        output
            .event_classes
            .iter()
            .any(|event| matches!(event, SourceAddressFrameEventClass::DraftChanged))
    );
    let submissions = output.take_submissions();
    assert_eq!(submissions.len(), 1);
    assert_eq!(
        submissions.into_iter().next().map(|item| item.into_draft()),
        Some("next".to_owned())
    );
}

#[test]
fn source_address_adapter_drops_non_text_surface_events_without_output() {
    let adapter = EguiSourceAddressStripAdapter::new("source-address-dispatch-ignore-test")
        .expect("adapter should initialize");
    let mut strip = strip();
    let mut output = EguiSourceAddressStripOutput {
        event_classes: Vec::new(),
        submissions: Vec::new(),
    };

    adapter.apply_text_surface_events(
        &mut output,
        &mut strip,
        &[TextSurfaceEvent::KeyValidationFailed {
            key: TextAreaKeyChord::enter(),
            error: TextAreaValidationError::ConflictingKeyBindings,
        }],
    );
    assert!(output.event_classes.is_empty());
}
