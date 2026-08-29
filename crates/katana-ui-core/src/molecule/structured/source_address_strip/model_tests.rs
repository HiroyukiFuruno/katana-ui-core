use crate::molecule::structured::{
    SourceAddressEntry, SourceAddressEvent, SourceAddressPresentation, SourceAddressStrip,
};

fn strip() -> SourceAddressStrip {
    SourceAddressStrip::new(SourceAddressPresentation::new(
        "Source address",
        "Open source address",
        "Source address input",
    ))
}

#[test]
fn set_draft_avoids_event_if_unchanged() {
    let mut strip = strip();
    assert!(matches!(
        strip.set_draft("same".to_owned()),
        Some(SourceAddressEvent::DraftChanged)
    ));
    assert!(matches!(
        strip.set_draft("changed".to_owned()),
        Some(SourceAddressEvent::DraftChanged)
    ));
    assert!(strip.set_draft("changed".to_owned()).is_none());
    assert_eq!(strip.draft(), "changed");
}

#[test]
fn set_enabled_updates_focus_and_opens() {
    let mut strip = strip();
    assert!(matches!(
        strip.set_focused(true),
        Some(SourceAddressEvent::Focused)
    ));
    assert_eq!(
        strip.set_candidates(vec![SourceAddressEntry::new(
            SourceAddressPresentation::new("candidate", "c-tip", "c-access"),
            Vec::new()
        )]),
        ()
    );
    assert!(matches!(
        strip.open_candidates(),
        Some(SourceAddressEvent::CandidatesOpened)
    ));
    assert!(matches!(
        strip.set_enabled(false),
        Some(SourceAddressEvent::EnabledChanged)
    ));
    assert!(!strip.focused());
    assert!(!strip.candidates_open());
}

#[test]
fn set_enabled_no_change_returns_none() {
    let mut strip = strip();
    assert!(matches!(
        strip.set_enabled(false),
        Some(SourceAddressEvent::EnabledChanged)
    ));
    assert!(strip.set_enabled(false).is_none());
}

#[test]
fn open_and_close_history() {
    let mut strip = strip();
    assert!(matches!(
        strip.open_history(),
        Some(SourceAddressEvent::HistoryOpened)
    ));
    assert!(strip.open_history().is_none());
    assert!(matches!(
        strip.close_history(),
        Some(SourceAddressEvent::HistoryClosed)
    ));
    assert!(strip.close_history().is_none());
    assert!(!strip.history_open());
}

#[test]
fn open_and_close_candidates() {
    let mut strip = strip();
    assert!(matches!(
        strip.open_candidates(),
        Some(SourceAddressEvent::CandidatesOpened)
    ));
    assert!(strip.open_candidates().is_none());
    assert!(matches!(
        strip.close_candidates(),
        Some(SourceAddressEvent::CandidatesClosed)
    ));
    assert!(strip.close_candidates().is_none());
    assert!(!strip.candidates_open());
}

#[test]
fn focus_change_reports_blur_after_focus() {
    let mut strip = strip();
    assert!(matches!(
        strip.set_focused(true),
        Some(SourceAddressEvent::Focused)
    ));
    assert!(matches!(
        strip.set_focused(false),
        Some(SourceAddressEvent::Blurred)
    ));
}

#[test]
fn selecting_entries_updates_draft_and_selection() {
    let mut strip = strip();
    strip.set_history(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new(
            "history value 1",
            "history tooltip",
            "history accessibility",
        ),
        b"history target 1".to_vec(),
    )]);
    strip.set_candidates(vec![SourceAddressEntry::new(
        SourceAddressPresentation::new(
            "candidate value 1",
            "candidate tooltip",
            "candidate accessibility",
        ),
        b"candidate target 1".to_vec(),
    )]);
    assert!(matches!(
        strip.select_history(0),
        Some(SourceAddressEvent::HistorySelected)
    ));
    assert_eq!(strip.draft(), "history value 1");
    assert_eq!(strip.selected_history(), Some(0));
    assert!(matches!(
        strip.select_candidate(0),
        Some(SourceAddressEvent::CandidateSelected)
    ));
    assert_eq!(strip.draft(), "candidate value 1");
    assert_eq!(strip.selected_candidate(), Some(0));
}

#[test]
fn selecting_unknown_indices_is_noop() {
    let mut strip = strip();
    assert!(strip.select_history(0).is_none());
    assert!(strip.select_candidate(0).is_none());
}
