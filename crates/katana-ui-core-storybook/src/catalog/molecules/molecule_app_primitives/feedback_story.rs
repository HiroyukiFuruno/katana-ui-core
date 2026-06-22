use super::{
    BannerAction, BannerActionKind, BannerCommand, BannerDensity, BannerPlacementHint,
    BannerSeverity, STORY_TOAST_STACK_GAP, StoryCatalog, StoryExample, TOAST_DURATION_MS,
    TOAST_TICK_MS, ToastAction, ToastActionKind, ToastDedupStrategy, ToastPayload, ToastPosition,
    ToastStackAction, ToastStackDirection, ToastStackOptions, UiCallbackLog, UiStateId, UiTone,
    molecule,
};

pub(super) fn banner_story() -> StoryExample {
    let mut banner = molecule::Banner::new("Formatter changed 3 files.")
        .severity(BannerSeverity::Warning)
        .title("Format result")
        .leading_icon("alert-triangle")
        .dismissible(true)
        .expanded_details("src/lib.rs, src/panel.rs, tests/storybook.rs")
        .density(BannerDensity::Compact)
        .placement_hint(BannerPlacementHint::Sticky)
        .action(BannerAction::new(
            "open-diff",
            "Open diff",
            BannerActionKind::Primary,
        ))
        .action(BannerAction::new(
            "dismiss",
            "Dismiss",
            BannerActionKind::Secondary,
        ));
    let target = banner.state_id().clone();
    let opened = banner.apply_action(BannerCommand::ToggleDetails);
    let actioned = banner.apply_action(BannerCommand::PressAction("open-diff".to_string()));
    let dismissed = banner.apply_action(BannerCommand::Dismiss);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "banner_toggle_details",
            "details_open=false",
            format!("events={opened:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "banner_primary_action",
            "action=none",
            format!("events={actioned:?}"),
        ),
        UiCallbackLog::new(
            target,
            "banner_dismiss",
            "visible=true",
            format!("events={dismissed:?}"),
        ),
    ];
    StoryCatalog::interactive_story("banner", banner, logs)
}

pub(super) fn toast_stack_manager_story() -> StoryExample {
    let mut stack = molecule::ToastStackManager::new().options(ToastStackOptions {
        position: ToastPosition::BottomEnd,
        max_visible: 2,
        dedup_strategy: ToastDedupStrategy::ById,
        default_duration_ms: TOAST_DURATION_MS,
        pause_on_hover: true,
        stack_gap: STORY_TOAST_STACK_GAP,
        enter_direction: ToastStackDirection::Up,
        exit_direction: ToastStackDirection::Down,
        replace_resets_duration: false,
        max_queued: 1,
    });
    let shown = stack.apply_action(ToastStackAction::Enqueue(toast_payload("save", "Saved")));
    let second = stack.apply_action(ToastStackAction::Enqueue(toast_payload(
        "lint",
        "Lint warning",
    )));
    let queued = stack.apply_action(ToastStackAction::Enqueue(toast_payload(
        "build",
        "Build failed",
    )));
    let overflow = stack.apply_action(ToastStackAction::Enqueue(toast_payload(
        "docs",
        "Docs ready",
    )));
    let story_stack = stack.clone();
    let paused = stack.apply_action(ToastStackAction::PauseHover(true));
    let tick = stack.apply_action(ToastStackAction::Tick(TOAST_TICK_MS));
    let dismissed = stack.apply_action(ToastStackAction::ActivateToastAction {
        toast_id: "save".to_string(),
        action_id: "undo".to_string(),
    });
    let resumed = stack.apply_action(ToastStackAction::Resume);
    let timed_out = stack.apply_action(ToastStackAction::Tick(TOAST_DURATION_MS));
    let target = UiStateId::new("state:ToastStackManager:storybook");
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "toast_enqueue_visible",
            "visible=0 queued=0",
            format!("events={shown:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "toast_queue_and_overflow",
            "visible=2 queued=0",
            format!("second={second:?} queued={queued:?} overflow={overflow:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "toast_pause_hover",
            "paused=false",
            format!("events={paused:?} tick_while_paused={tick:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "toast_action_dismiss",
            "visible=2 queued=1",
            format!("events={dismissed:?}"),
        ),
        UiCallbackLog::new(
            target,
            "toast_timeout",
            "paused=true visible=2",
            format!("resume={resumed:?} timed_out={timed_out:?}"),
        ),
    ];
    StoryCatalog::interactive_story("toast-stack-manager", story_stack, logs)
}

fn toast_payload(id: &str, message: &str) -> ToastPayload {
    ToastPayload::new(id, message)
        .severity(UiTone::Warning)
        .duration_ms(TOAST_DURATION_MS)
        .action(ToastAction::new("undo", "Undo", ToastActionKind::Primary))
}
