use super::*;

fn base_segment() -> StatusBarSegment {
    StatusBarSegment::new("segment-id", "segment-label")
        .icon("icon-id")
        .tooltip("segment tooltip")
        .alignment(StatusBarSegmentAlignment::Center)
}

#[test]
fn status_bar_getters_expose_configured_values() {
    let bar = StatusBar::new("Build status")
        .mode(StatusBarMode::MultiSegment)
        .density(StatusBarDensity::Compact)
        .message("override me")
        .segment(base_segment());

    assert_eq!(bar.label(), "Build status");
    assert_eq!(bar.mode_value(), StatusBarMode::MultiSegment);
    assert_eq!(bar.density_value(), StatusBarDensity::Compact);
    assert_eq!(bar.single_message(), Some("override me"));
    assert_eq!(bar.segments().len(), 1);
}

#[test]
fn press_segment_ignores_non_interactive_segment() {
    let mut bar = StatusBar::new("Build status").segment(base_segment());
    let events = bar.apply_action(&StatusBarAction::PressSegment {
        id: "segment-id".to_owned(),
    });

    assert!(events.is_empty());
}

#[test]
fn press_segment_with_popover_opens_and_closes() {
    let popover_segment = StatusBarSegment::new("segment-popover", "Segment with popover").popover(
        StatusBarPopoverSpec::new("Status detail", "Current state details"),
    );
    let mut bar = StatusBar::new("Build status").segment(popover_segment);

    let events = bar.apply_action(&StatusBarAction::PressSegment {
        id: "segment-popover".to_owned(),
    });
    assert_eq!(
        events,
        vec![
            StatusBarEvent::SegmentPressed {
                id: "segment-popover".to_owned()
            },
            StatusBarEvent::SegmentPopoverOpened {
                id: "segment-popover".to_owned()
            }
        ]
    );
    assert_eq!(
        bar.state().open_popover(),
        Some(&"segment-popover".to_owned())
    );

    let close_events = bar.apply_action(&StatusBarAction::ClosePopover {
        id: "segment-popover".to_owned(),
    });
    assert_eq!(
        close_events,
        vec![StatusBarEvent::SegmentPopoverClosed {
            id: "segment-popover".to_owned()
        }]
    );
    assert!(bar.state().open_popover().is_none());
}

#[test]
fn segments_for_returns_only_requested_alignment() {
    let bar = StatusBar::new("Build status")
        .segment(
            StatusBarSegment::new("leading", "Leading")
                .alignment(StatusBarSegmentAlignment::Leading),
        )
        .segment(
            StatusBarSegment::new("trailing", "Trailing")
                .alignment(StatusBarSegmentAlignment::Trailing),
        );
    let bar = bar.segment(
        StatusBarSegment::new("center", "Center").alignment(StatusBarSegmentAlignment::Center),
    );

    assert_eq!(bar.segments_for(StatusBarSegmentAlignment::Center).len(), 1);
    assert_eq!(
        bar.segments_for(StatusBarSegmentAlignment::Center)[0].label(),
        "Center"
    );
}
