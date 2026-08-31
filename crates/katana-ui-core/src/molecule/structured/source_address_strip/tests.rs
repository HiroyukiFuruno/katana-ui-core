use super::{
    SourceAddressAction, SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};

fn strip() -> SourceAddressStrip {
    SourceAddressStrip::new(SourceAddressPresentation::new(
        "Source address",
        "Open source address",
        "Source address input",
    ))
}

fn entry(visible: &str, target: &[u8]) -> super::super::SourceAddressEntry {
    super::super::SourceAddressEntry::new(
        SourceAddressPresentation::new(visible, "item tooltip", "item accessibility"),
        target.to_vec(),
    )
}

#[test]
fn retained_state_transitions_and_toggle_selection_are_generic() {
    let mut control = strip();
    control.set_history(vec![entry("history-value", b"history-secret")]);
    control.set_candidates(vec![entry("candidate-value", b"candidate-secret")]);

    assert!(matches!(
        control.apply_action(SourceAddressAction::OpenHistory),
        Some(SourceAddressEvent::HistoryOpened)
    ));
    assert!(control.history_open());
    assert!(!control.candidates_open());
    assert!(matches!(
        control.apply_action(SourceAddressAction::SelectHistory(0)),
        Some(SourceAddressEvent::HistorySelected)
    ));
    assert_eq!(control.draft(), "history-value");
    assert_eq!(control.selected_history(), Some(0));
    assert!(!control.history_open());

    assert!(matches!(
        control.apply_action(SourceAddressAction::OpenCandidates),
        Some(SourceAddressEvent::CandidatesOpened)
    ));
    assert!(matches!(
        control.apply_action(SourceAddressAction::SelectCandidate(0)),
        Some(SourceAddressEvent::CandidateSelected)
    ));
    assert_eq!(control.draft(), "candidate-value");
    assert_eq!(control.selected_candidate(), Some(0));
    assert!(!control.candidates_open());
}

#[test]
fn disabled_submit_and_mutations_emit_no_event() -> Result<(), String> {
    let mut control = strip();
    control
        .apply_action(SourceAddressAction::SetDraft("draft".to_owned()))
        .ok_or_else(|| "enabled draft must change".to_string())?;
    control.apply_action(SourceAddressAction::SetEnabled(false));

    assert!(control.apply_action(SourceAddressAction::Submit).is_none());
    assert!(
        control
            .apply_action(SourceAddressAction::SetDraft("ignored".to_owned()))
            .is_none()
    );
    assert_eq!(control.draft(), "draft");
    assert!(
        control
            .apply_action(SourceAddressAction::OpenHistory)
            .is_none()
    );
    Ok(())
}

#[test]
fn submission_is_one_shot_and_consumed_without_debug_or_serde_leakage() -> Result<(), String> {
    let mut control = strip();
    control
        .apply_action(SourceAddressAction::SetDraft("日本語⭐️".to_owned()))
        .ok_or_else(|| "draft must change".to_string())?;
    let event = control
        .apply_action(SourceAddressAction::Submit)
        .ok_or_else(|| "submission event is required".to_string())?;
    let submission = match event {
        SourceAddressEvent::Submitted(value) => value,
        _ => return Err("expected submission event".to_string()),
    };
    assert_eq!(submission.into_draft(), "日本語⭐️");
    Ok(())
}

#[test]
fn opaque_targets_and_address_values_never_appear_in_debug_or_serialized_entry()
-> Result<(), String> {
    let value = entry("visible-value", b"opaque-target-secret");
    let debug = format!("{value:?}");
    let mut control = strip();
    control
        .apply_action(SourceAddressAction::SetDraft("draft-secret".to_owned()))
        .ok_or_else(|| "draft must change".to_string())?;
    control.set_history(vec![value]);
    let strip_debug = format!("{control:?}");

    assert!(!debug.contains("opaque-target-secret"));
    assert!(!debug.contains("visible-value"));
    assert!(!strip_debug.contains("draft-secret"));
    assert!(!strip_debug.contains("visible-value"));
    assert!(!strip_debug.contains("opaque-target-secret"));
    assert!(
        !include_str!("model.rs").contains("impl Serialize for SourceAddressEntry"),
        "opaque address entries must not expose a serializable transport"
    );
    Ok(())
}
