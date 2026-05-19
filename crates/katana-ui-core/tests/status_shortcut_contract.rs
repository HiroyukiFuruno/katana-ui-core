#[path = "status_shortcut_contract/shortcut_cases.rs"]
mod shortcut_cases;

use katana_ui_core::molecule::{
    ProgressMeterShape, ProgressMeterSpec, StatusBar, StatusBarAction, StatusBarContractViolation,
    StatusBarEvent, StatusBarMode, StatusBarPopoverSpec, StatusBarSegment,
    StatusBarSegmentAlignment,
};
use katana_ui_core::render_model::{UiNodeKind, UiTone, UiTree};

#[test]
fn status_bar_keeps_single_message_compatible_and_rejects_mode_conflict() {
    let single = StatusBar::new("Ready").severity(UiTone::Success);
    let invalid = StatusBar::new("Status")
        .mode(StatusBarMode::MultiSegment)
        .message("Ready")
        .segment(StatusBarSegment::new("file", "README.md"));

    assert!(single.validate().is_empty());
    assert_eq!(
        vec![StatusBarContractViolation::MultiSegmentHasSingleMessage],
        invalid.validate()
    );

    let tree = UiTree::new(single);
    assert_eq!(UiNodeKind::StatusBar, tree.root().kind());
    assert_eq!(UiTone::Success, tree.root().props().status.severity);
}

#[test]
fn status_bar_groups_segments_by_alignment_and_reading_order() {
    let bar = StatusBar::new("Editor status")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("encoding", "UTF-8")
                .alignment(StatusBarSegmentAlignment::Trailing),
        )
        .segment(
            StatusBarSegment::new("file", "main.rs")
                .alignment(StatusBarSegmentAlignment::Leading)
                .accessibility_label("File main.rs"),
        )
        .segment(
            StatusBarSegment::new("lint", "0 warnings")
                .alignment(StatusBarSegmentAlignment::Center),
        )
        .segment(
            StatusBarSegment::new("cursor", "12:4").alignment(StatusBarSegmentAlignment::Trailing),
        );

    assert_eq!(
        vec!["file"],
        segment_ids(&bar, StatusBarSegmentAlignment::Leading)
    );
    assert_eq!(
        vec!["lint"],
        segment_ids(&bar, StatusBarSegmentAlignment::Center)
    );
    assert_eq!(
        vec!["encoding", "cursor"],
        segment_ids(&bar, StatusBarSegmentAlignment::Trailing)
    );
    assert_eq!(
        vec!["File main.rs", "0 warnings", "UTF-8", "12:4"],
        bar.live_region_labels()
    );
}

#[test]
fn status_bar_interactive_segment_emits_press_and_popover_event() {
    let mut bar = StatusBar::new("Usage")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("tokens", "75%")
                .interactive(true)
                .popover(StatusBarPopoverSpec::new("Usage", "Token budget")),
        );

    let events = bar.apply_action(&StatusBarAction::PressSegment {
        id: "tokens".to_string(),
    });

    assert_eq!(
        vec![
            StatusBarEvent::SegmentPressed {
                id: "tokens".to_string()
            },
            StatusBarEvent::SegmentPopoverOpened {
                id: "tokens".to_string()
            },
        ],
        events
    );
    assert_eq!(Some(&"tokens".to_string()), bar.state().open_popover());
}

#[test]
fn status_bar_progress_meter_clamps_and_renders_progress_child() {
    let bar =
        StatusBar::new("Progress")
            .mode(StatusBarMode::MultiSegment)
            .segment(StatusBarSegment::new("usage", "Usage").progress(
                ProgressMeterSpec::new(ProgressMeterShape::Ring, 150).tone(UiTone::Accent),
            ));
    let progress = bar
        .segments_for(StatusBarSegmentAlignment::Leading)
        .into_iter()
        .find_map(StatusBarSegment::progress_spec);

    assert_eq!(Some(100), progress.map(ProgressMeterSpec::percent));

    let tree = UiTree::new(bar);
    let segment_nodes = tree.root().children();
    assert_eq!(1, segment_nodes.len());
    assert_eq!(
        UiNodeKind::ProgressBar,
        segment_nodes[0].children()[0].kind()
    );
    assert_eq!(100, segment_nodes[0].children()[0].props().progress_percent);
}

fn segment_ids(bar: &StatusBar, alignment: StatusBarSegmentAlignment) -> Vec<&str> {
    bar.segments_for(alignment)
        .into_iter()
        .map(StatusBarSegment::id)
        .collect()
}
