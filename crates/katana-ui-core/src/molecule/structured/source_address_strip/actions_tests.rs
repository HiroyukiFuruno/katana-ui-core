use super::apply;
use crate::molecule::structured::{
    SourceAddressAction, SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};

fn strip() -> SourceAddressStrip {
    SourceAddressStrip::new(SourceAddressPresentation::new(
        "Source address",
        "Open source address",
        "Source address input",
    ))
}

#[test]
fn close_history_when_not_open_returns_none() {
    let mut strip = strip();
    assert!(apply(&mut strip, SourceAddressAction::CloseHistory).is_none());
}

#[test]
fn close_candidates_when_not_open_returns_none() {
    let mut strip = strip();
    assert!(apply(&mut strip, SourceAddressAction::CloseCandidates).is_none());
}

#[test]
fn focused_action_is_ignored_when_disabled() {
    let mut strip = strip();
    assert!(matches!(
        apply(&mut strip, SourceAddressAction::SetEnabled(false)),
        Some(SourceAddressEvent::EnabledChanged)
    ));
    assert!(apply(&mut strip, SourceAddressAction::SetFocused(true)).is_none());
}
