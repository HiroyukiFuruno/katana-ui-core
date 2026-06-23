use super::*;

#[test]
fn banner_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "banner")
        .ok_or("banner page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "保存失敗",
            "adapter 未接続",
            "添付サイズ超過",
            "成功通知",
            "details 展開",
            "title 表示",
            "leading icon",
            "sticky placement"
        ],
        StoryPresetLabels::for_page("banner")
    );
    for preset in StoryPresetLabels::for_page("banner") {
        assert!(
            details.preset.contains(preset),
            "banner detail preset lacks {preset}"
        );
    }
    for setting in [
        "severity",
        "density",
        "actions",
        "details",
        "dismissible",
        "title",
        "leading_icon",
        "placement",
    ] {
        assert!(
            details.settings.contains(setting),
            "banner settings inspector lacks {setting}"
        );
    }
    for action in [
        "banner_toggle_details",
        "banner_primary_action",
        "banner_dismiss",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "banner callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn toast_stack_manager_story_exposes_settings_presets_and_links() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "toast-stack-manager")
        .ok_or("toast-stack-manager page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "位置 6 種類",
            "dedup ById",
            "pause_on_hover",
            "queue 上限超過",
            "action 付き toast"
        ],
        StoryPresetLabels::for_page("toast-stack-manager")
    );
    for preset in StoryPresetLabels::for_page("toast-stack-manager") {
        assert!(
            details.preset.contains(preset),
            "toast-stack-manager detail preset lacks {preset}"
        );
    }
    for setting in [
        "position",
        "max_visible",
        "dedup",
        "duration",
        "pause_on_hover",
        "stack_gap",
    ] {
        assert!(
            details.settings.contains(setting),
            "toast-stack-manager settings inspector lacks {setting}"
        );
    }
    for action in [
        "toast_enqueue_visible",
        "toast_queue_and_overflow",
        "toast_pause_hover",
        "toast_action_dismiss",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "toast-stack-manager callback log lacks action {action}"
        );
    }

    let notification = examples
        .iter()
        .find(|it| it.page == "notification-toast")
        .ok_or("notification-toast page missing")?;
    let notification_details = StoryDetailContent::from_example(notification);
    assert!(notification_details.settings.contains("ToastStackManager"));
    Ok(())
}

#[test]
fn status_bar_story_exposes_multi_segment_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "status-bar")
        .ok_or("status-bar page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "editor status bar",
            "chat usage bar",
            "linter summary",
            "progress segment",
            "popover segment",
            "single message",
            "severity tone",
            "dismiss action",
            "segment a11y"
        ],
        StoryPresetLabels::for_page("status-bar")
    );
    for preset in StoryPresetLabels::for_page("status-bar") {
        assert!(
            details.preset.contains(preset),
            "status-bar detail preset lacks {preset}"
        );
    }
    for setting in [
        "mode",
        "segments",
        "density",
        "message",
        "severity",
        "dismiss",
        "segment_a11y",
    ] {
        assert!(
            details.settings.contains(setting),
            "status-bar settings inspector lacks {setting}"
        );
    }
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "status_bar_segment_popover")
    );
    Ok(())
}

#[test]
fn shortcut_stories_expose_settings_presets_and_keycap_boundary() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let combo = examples
        .iter()
        .find(|it| it.page == "shortcut-combo")
        .ok_or("shortcut-combo page missing")?;
    let combo_details = StoryDetailContent::from_example(combo);
    let cheatsheet = examples
        .iter()
        .find(|it| it.page == "shortcut-cheatsheet")
        .ok_or("shortcut-cheatsheet page missing")?;
    let cheatsheet_details = StoryDetailContent::from_example(cheatsheet);
    let key_cap = examples
        .iter()
        .find(|it| it.page == "key-cap")
        .ok_or("key-cap page missing")?;
    let key_cap_details = StoryDetailContent::from_example(key_cap);

    assert_eq!(
        &[
            "macOS",
            "Windows",
            "Linux",
            "custom separator",
            "a11y label"
        ],
        StoryPresetLabels::for_page("shortcut-combo")
    );
    assert_eq!(
        &[
            "cheatsheet sample",
            "カテゴリ filter",
            "two column",
            "one column",
            "select combo",
            "label",
            "groups",
            "group title",
            "items",
            "item combo"
        ],
        StoryPresetLabels::for_page("shortcut-cheatsheet")
    );
    for setting in [
        "platform_display",
        "separator",
        "size",
        "tone",
        "a11y_label",
    ] {
        assert!(
            combo_details.settings.contains(setting),
            "shortcut-combo settings inspector lacks {setting}"
        );
    }
    for setting in [
        "label",
        "groups",
        "group_title",
        "items",
        "item_combo",
        "group_layout",
        "query",
        "selected",
        "result_count",
    ] {
        assert!(
            cheatsheet_details.settings.contains(setting),
            "shortcut-cheatsheet settings inspector lacks {setting}"
        );
    }
    assert!(key_cap_details.settings.contains("ShortcutCombo"));
    Ok(())
}
