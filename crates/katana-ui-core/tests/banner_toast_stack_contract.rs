use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTone, UiVariant};
use katana_ui_core::widget::molecules::{
    Banner, BannerAccessibilityRole, BannerAction, BannerActionKind, BannerCommand, BannerDensity,
    BannerEvent, BannerLiveRegion, BannerPlacementHint, BannerSeverity, ToastAction,
    ToastActionKind, ToastDedupStrategy, ToastDismissReason, ToastPayload, ToastPosition,
    ToastReplaceKind, ToastStackAction, ToastStackDirection, ToastStackEvent, ToastStackManager,
    ToastStackOptions,
};

#[test]
fn banner_severity_drives_icon_tone_and_role() {
    let warning = Banner::new("保存に失敗しました").severity(BannerSeverity::Warning);
    let contract = warning.visual_contract();

    assert_eq!(Some("alert-triangle".to_string()), contract.icon);
    assert_eq!(UiTone::Warning, contract.tone);
    assert_eq!(BannerAccessibilityRole::Alert, contract.role);
    assert_eq!(BannerLiveRegion::Assertive, contract.live_region);
    let node = UiNode::from(warning);
    assert_eq!(UiNodeKind::Banner, node.kind());
    assert_eq!("alert", node.props().interaction.value);
    let custom = Banner::new("接続先を確認してください")
        .severity(BannerSeverity::Danger)
        .leading_icon("plug-disconnected")
        .visual_contract();
    assert_eq!(Some("plug-disconnected".to_string()), custom.icon);
    assert_eq!(UiTone::Danger, custom.tone);
}

#[test]
fn banner_severity_value_contract_covers_all_tones_icons_roles_and_live_regions() {
    for (severity, tone, icon, role, live_region) in [
        (
            BannerSeverity::Info,
            UiTone::Accent,
            Some("info".to_string()),
            BannerAccessibilityRole::Status,
            BannerLiveRegion::Polite,
        ),
        (
            BannerSeverity::Success,
            UiTone::Success,
            Some("check".to_string()),
            BannerAccessibilityRole::Status,
            BannerLiveRegion::Polite,
        ),
        (
            BannerSeverity::Warning,
            UiTone::Warning,
            Some("alert-triangle".to_string()),
            BannerAccessibilityRole::Alert,
            BannerLiveRegion::Assertive,
        ),
        (
            BannerSeverity::Danger,
            UiTone::Danger,
            Some("alert-octagon".to_string()),
            BannerAccessibilityRole::Alert,
            BannerLiveRegion::Assertive,
        ),
        (
            BannerSeverity::Neutral,
            UiTone::Neutral,
            None,
            BannerAccessibilityRole::Status,
            BannerLiveRegion::Polite,
        ),
    ] {
        assert_eq!(tone, severity.tone());
        assert_eq!(icon, severity.default_icon());
        assert_eq!(role, severity.role());
        assert_eq!(live_region, severity.live_region());
    }
    assert_eq!("status", BannerAccessibilityRole::Status.as_str());
}

#[test]
fn banner_dismiss_and_details_emit_typed_events() {
    let mut banner = Banner::new("添付サイズが上限を超えています")
        .dismissible(true)
        .expanded_details("上限は 20MB です");
    let second = Banner::new("別の告知").dismissible(true);

    let toggle_events = banner.apply_action(BannerCommand::ToggleDetails);
    assert!(matches!(
        toggle_events.as_slice(),
        [BannerEvent::BannerDetailsToggled { open: true, .. }]
    ));
    assert!(banner.state().details_open);
    let dismiss_events = banner.apply_action(BannerCommand::Dismiss);
    assert!(matches!(
        dismiss_events.as_slice(),
        [BannerEvent::BannerDismissed { .. }]
    ));
    assert!(!banner.state().visible);
    assert!(second.state().visible);
    assert_ne!(banner.state_id(), second.state_id());
}

#[test]
fn banner_details_open_renders_detail_child_and_visual_contract_counts() {
    let mut banner = Banner::new("保存に失敗しました")
        .severity(BannerSeverity::Danger)
        .title("保存できません")
        .dismissible(true)
        .density(BannerDensity::Compact)
        .placement_hint(BannerPlacementHint::Sticky)
        .expanded_details("権限と保存先を確認してください")
        .action(BannerAction::new(
            "retry",
            "再試行",
            BannerActionKind::Primary,
        ))
        .action(BannerAction::new(
            "details",
            "詳細",
            BannerActionKind::Secondary,
        ));
    let _ = banner.apply_action(BannerCommand::ToggleDetails);
    let contract = banner.visual_contract();
    let node = UiNode::from(banner);

    assert_eq!(2, contract.action_count);
    assert!(contract.dismissible);
    assert!(contract.details_available);
    assert_eq!(BannerDensity::Compact, contract.density);
    assert_eq!(BannerPlacementHint::Sticky, contract.placement_hint);
    assert!(
        node.children()
            .iter()
            .any(|it| it.props().label == "保存できません")
    );
    assert!(
        node.children()
            .iter()
            .any(|it| it.props().label == "権限と保存先を確認してください")
    );
    assert!(node.children().iter().any(|it| {
        it.kind() == UiNodeKind::Button
            && it.props().label == "再試行"
            && it.props().variant == UiVariant::Filled
    }));
    assert!(node.children().iter().any(|it| {
        it.kind() == UiNodeKind::Button
            && it.props().label == "詳細"
            && it.props().variant == UiVariant::Text
    }));
}

#[test]
fn banner_action_disabled_state_suppresses_event() {
    let mut banner = Banner::new("adapter が未接続です")
        .action(BannerAction::new(
            "connect",
            "接続",
            BannerActionKind::Primary,
        ))
        .action(BannerAction::new("later", "後で", BannerActionKind::Secondary).disabled(true));

    let primary = banner.apply_action(BannerCommand::PressAction("connect".to_string()));
    assert!(matches!(
        primary.as_slice(),
        [BannerEvent::BannerActioned {
            action_id,
            kind: BannerActionKind::Primary,
            ..
        }] if action_id == "connect"
    ));
    let disabled = banner.apply_action(BannerCommand::PressAction("later".to_string()));
    assert!(disabled.is_empty());
}

#[test]
fn banner_noop_commands_and_callback_log_are_explicit() {
    let mut banner = Banner::new("Passive");

    assert!(banner.apply_action(BannerCommand::Dismiss).is_empty());
    assert!(banner.apply_action(BannerCommand::ToggleDetails).is_empty());
    assert!(
        banner
            .apply_action(BannerCommand::PressAction("missing".to_string()))
            .is_empty()
    );
    assert!(!banner.state().details_open);
    assert!(banner.callback_log().is_empty());
}

#[test]
fn toast_enqueue_queues_beyond_max_visible_and_promotes_on_dismiss() {
    let options = ToastStackOptions {
        position: ToastPosition::BottomEnd,
        max_visible: 1,
        stack_gap: 12,
        ..ToastStackOptions::default()
    };
    let mut manager = ToastStackManager::new().options(options);
    let contract = manager.visual_contract();

    let first = manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new("one", "one")));
    let second = manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new("two", "two")));
    assert_eq!(ToastStackDirection::Up, contract.stack_direction);
    assert_eq!(12, contract.stack_gap);
    assert!(matches!(
        first.as_slice(),
        [ToastStackEvent::ToastShown { id }] if id == "one"
    ));
    assert!(matches!(
        second.as_slice(),
        [ToastStackEvent::ToastQueued { id }] if id == "two"
    ));
    assert_eq!(1, manager.state().visible.len());
    assert_eq!(1, manager.state().queued.len());

    let dismissed = manager.apply_action(ToastStackAction::Dismiss("one".to_string()));
    assert!(dismissed.iter().any(|it| matches!(
        it,
        ToastStackEvent::ToastShown { id } if id == "two"
    )));
}

#[test]
fn toast_stack_renders_actions_and_exposes_option_contract() {
    let options = ToastStackOptions {
        position: ToastPosition::TopCenter,
        max_visible: 2,
        dedup_strategy: ToastDedupStrategy::ById,
        default_duration_ms: 7_000,
        pause_on_hover: false,
        stack_gap: 14,
        ..ToastStackOptions::default()
    };
    let mut manager = ToastStackManager::new().options(options);
    let _ = manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("sync", "同期しました")
            .action(ToastAction::new("open", "開く", ToastActionKind::Primary))
            .action(ToastAction::new(
                "ignore",
                "閉じる",
                ToastActionKind::Secondary,
            )),
    ));
    let contract = manager.visual_contract();
    let node = UiNode::from(manager);
    let toast = node.children().first();
    assert!(toast.is_some(), "visible toast is rendered");
    let Some(toast) = toast else {
        return;
    };

    assert_eq!(ToastPosition::TopCenter, contract.position);
    assert_eq!(2, contract.max_visible);
    assert_eq!(ToastDedupStrategy::ById, contract.dedup_strategy);
    assert_eq!(7_000, contract.default_duration_ms);
    assert!(!contract.pause_on_hover);
    assert_eq!(14, contract.stack_gap);
    assert!(toast.children().iter().any(|it| {
        it.kind() == UiNodeKind::Button
            && it.props().label == "開く"
            && it.props().variant == UiVariant::Filled
    }));
    assert!(toast.children().iter().any(|it| {
        it.kind() == UiNodeKind::Button
            && it.props().label == "閉じる"
            && it.props().variant == UiVariant::Text
    }));
}

#[test]
fn toast_dedup_by_id_replaces_visible_and_preserves_remaining_duration() {
    let options = ToastStackOptions {
        dedup_strategy: ToastDedupStrategy::ById,
        replace_resets_duration: false,
        ..ToastStackOptions::default()
    };
    let mut manager = ToastStackManager::new().options(options);

    let _ = manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("same", "before").duration_ms(100),
    ));
    let _ = manager.apply_action(ToastStackAction::Tick(40));
    let replaced = manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("same", "after").duration_ms(100),
    ));

    assert!(matches!(
        replaced.as_slice(),
        [ToastStackEvent::ToastReplaced {
            id,
            kind: ToastReplaceKind::Visible
        }] if id == "same"
    ));
    assert!(
        manager
            .state()
            .visible
            .iter()
            .any(|it| it.remaining_duration_ms == Some(60))
    );
}

#[test]
fn toast_pause_hover_and_focus_stop_duration_tick() {
    let mut manager = ToastStackManager::new();
    let _ = manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("slow", "wait").duration_ms(100),
    ));

    let pause = manager.apply_action(ToastStackAction::PauseHover(true));
    let _ = manager.apply_action(ToastStackAction::Tick(100));

    assert_eq!(vec![ToastStackEvent::ToastPaused], pause);
    assert_eq!(1, manager.state().visible.len());

    let resume = manager.apply_action(ToastStackAction::PauseHover(false));
    assert_eq!(vec![ToastStackEvent::ToastResumed], resume);
}

#[test]
fn toast_action_button_dismisses_with_typed_reason() {
    let mut manager = ToastStackManager::new();
    let payload = ToastPayload::new("saved", "保存しました").action(ToastAction::new(
        "undo",
        "元に戻す",
        ToastActionKind::Primary,
    ));
    let _ = manager.apply_action(ToastStackAction::Enqueue(payload));

    let events = manager.apply_action(ToastStackAction::ActivateToastAction {
        toast_id: "saved".to_string(),
        action_id: "undo".to_string(),
    });

    assert!(events.iter().any(|it| matches!(
        it,
        ToastStackEvent::ToastDismissed {
            id,
            reason: ToastDismissReason::Action(action_id)
        } if id == "saved" && action_id == "undo"
    )));
}

#[test]
fn toast_queue_overflow_drops_oldest() {
    let options = ToastStackOptions {
        max_visible: 0,
        max_queued: 1,
        ..ToastStackOptions::default()
    };
    let mut manager = ToastStackManager::new().options(options);
    let _ = manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new("old", "old")));

    let events = manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new("new", "new")));

    assert!(events.iter().any(|it| matches!(
        it,
        ToastStackEvent::ToastQueueOverflow { dropped_id } if dropped_id == "old"
    )));
    assert!(manager.state().queued.iter().any(|it| it.id == "new"));
}

#[test]
fn toast_zero_queue_capacity_reports_incoming_overflow_and_records_callbacks() {
    let options = ToastStackOptions {
        max_visible: 0,
        max_queued: 0,
        ..ToastStackOptions::default()
    };
    let mut manager = ToastStackManager::default().options(options);

    let events = manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new(
        "blocked", "blocked",
    )));

    assert!(matches!(
        events.as_slice(),
        [
            ToastStackEvent::ToastQueueOverflow { dropped_id },
            ToastStackEvent::ToastQueued { id }
        ] if dropped_id == "blocked" && id == "blocked"
    ));
    assert_eq!(events, manager.callback_log());
}

#[test]
fn toast_actions_cover_empty_dismiss_persistent_tick_and_dismiss_all() {
    let options = ToastStackOptions {
        max_visible: 1,
        ..ToastStackOptions::default()
    };
    let mut manager = ToastStackManager::new().options(options);

    assert!(
        manager
            .apply_action(ToastStackAction::Dismiss("missing".to_string()))
            .is_empty()
    );
    let _ = manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("persistent", "persistent").duration_ms(0),
    ));
    let _ = manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new(
        "queued", "queued",
    )));
    assert!(
        manager
            .apply_action(ToastStackAction::Tick(10_000))
            .is_empty()
    );
    assert_eq!(None, manager.state().visible[0].remaining_duration_ms);

    let dismissed = manager.apply_action(ToastStackAction::DismissAll);
    assert!(matches!(
        dismissed.as_slice(),
        [ToastStackEvent::ToastDismissed {
            id,
            reason: ToastDismissReason::DismissAll
        }] if id == "persistent"
    ));
    assert!(manager.state().visible.is_empty());
    assert!(manager.state().queued.is_empty());
}

#[test]
fn toast_focus_pause_resume_and_timeout_emit_complete_event_sequence() {
    let mut manager = ToastStackManager::new();
    let _ = manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("short", "short").duration_ms(10),
    ));

    assert_eq!(
        vec![ToastStackEvent::ToastPaused],
        manager.apply_action(ToastStackAction::FocusInside(true))
    );
    assert!(
        manager
            .apply_action(ToastStackAction::FocusInside(true))
            .is_empty()
    );
    assert!(manager.apply_action(ToastStackAction::Tick(10)).is_empty());
    assert_eq!(
        vec![ToastStackEvent::ToastResumed],
        manager.apply_action(ToastStackAction::Resume)
    );

    let timed_out = manager.apply_action(ToastStackAction::Tick(10));
    assert!(matches!(
        timed_out.as_slice(),
        [
            ToastStackEvent::ToastTimedOut { id: timed_out_id },
            ToastStackEvent::ToastDismissed {
                id: dismissed_id,
                reason: ToastDismissReason::Timeout
            }
        ] if timed_out_id == "short" && dismissed_id == "short"
    ));
}

#[test]
fn toast_dedup_replaces_queued_and_honors_severity_and_duration_policy() {
    let queued_options = ToastStackOptions {
        max_visible: 0,
        dedup_strategy: ToastDedupStrategy::ById,
        ..ToastStackOptions::default()
    };
    let mut queued_manager = ToastStackManager::new().options(queued_options);
    let _ = queued_manager.apply_action(ToastStackAction::Enqueue(ToastPayload::new(
        "queued", "before",
    )));
    let queued_replacement = queued_manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("queued", "after"),
    ));
    assert!(matches!(
        queued_replacement.as_slice(),
        [ToastStackEvent::ToastReplaced {
            id,
            kind: ToastReplaceKind::Queued
        }] if id == "queued"
    ));
    assert_eq!("after", queued_manager.state().queued[0].message);

    let severity_options = ToastStackOptions {
        dedup_strategy: ToastDedupStrategy::ByIdAndSeverity,
        replace_resets_duration: true,
        ..ToastStackOptions::default()
    };
    let mut severity_manager = ToastStackManager::new().options(severity_options);
    let _ = severity_manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("same", "neutral").duration_ms(100),
    ));
    let _ = severity_manager.apply_action(ToastStackAction::Tick(40));
    let different = severity_manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("same", "warning")
            .severity(UiTone::Warning)
            .duration_ms(200),
    ));
    assert!(matches!(
        different.as_slice(),
        [ToastStackEvent::ToastShown { id }] if id == "same"
    ));

    let reset = severity_manager.apply_action(ToastStackAction::Enqueue(
        ToastPayload::new("same", "warning replaced")
            .severity(UiTone::Warning)
            .duration_ms(300),
    ));
    assert!(matches!(
        reset.as_slice(),
        [ToastStackEvent::ToastReplaced {
            id,
            kind: ToastReplaceKind::Visible
        }] if id == "same"
    ));
    assert_eq!(
        Some(300),
        severity_manager.state().visible[1].remaining_duration_ms
    );
}
