use super::{
    ProgressMeterShape, ProgressMeterSpec, STATUS_PROGRESS_PERCENT, StatusBarAction,
    StatusBarDensity, StatusBarMode, StatusBarPopoverSpec, StatusBarSegment,
    StatusBarSegmentAlignment, StoryCatalog, StoryExample, UiCallbackLog, UiStateId, UiTone, atom,
    molecule,
};

pub(super) fn status_bar_story() -> StoryExample {
    let mut status = molecule::StatusBar::new("Status bar")
        .mode(StatusBarMode::MultiSegment)
        .density(StatusBarDensity::Compact)
        .segment(
            StatusBarSegment::new("branch", "main")
                .icon("git-branch")
                .tooltip("Current branch")
                .alignment(StatusBarSegmentAlignment::Leading)
                .popover(StatusBarPopoverSpec::new(
                    "Git branch",
                    "main is ahead by 2",
                )),
        )
        .segment(
            StatusBarSegment::new("diagnostics", "2 warnings")
                .alignment(StatusBarSegmentAlignment::Center)
                .interactive(true)
                .tooltip("Linter summary")
                .accessibility_label("Diagnostics summary"),
        )
        .segment(
            StatusBarSegment::new("index", "Indexing")
                .alignment(StatusBarSegmentAlignment::Trailing)
                .tooltip("Index progress")
                .progress(
                    ProgressMeterSpec::new(ProgressMeterShape::Linear, STATUS_PROGRESS_PERCENT)
                        .label("Indexing")
                        .tooltip("Indexing 72%")
                        .tone(UiTone::Accent),
                ),
        )
        .child(atom::Badge::new("Sync"))
        .child(atom::Text::new("Ln 12, Col 4"));
    let pressed = status.apply_action(&StatusBarAction::PressSegment {
        id: "branch".to_string(),
    });
    let closed = status.apply_action(&StatusBarAction::ClosePopover {
        id: "branch".to_string(),
    });
    let logs = vec![UiCallbackLog::new(
        UiStateId::new("state:StatusBar:storybook"),
        "status_bar_segment_popover",
        "open_popover=None",
        format!("pressed={pressed:?} closed={closed:?}"),
    )];
    StoryCatalog::interactive_story("status-bar", status, logs)
}
