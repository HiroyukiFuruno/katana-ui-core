use super::*;

#[test]
fn settings_list_story_exposes_presets_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "settings-list")
        .ok_or("settings-list page missing")?;
    let labels = page_children(&examples, "settings-list").ok_or("settings-list page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "app settings",
            "chat settings",
            "lint settings",
            "dirty 表示",
            "query filter",
            "reset"
        ],
        StoryPresetLabels::for_page("settings-list")
    );
    for preset in StoryPresetLabels::for_page("settings-list") {
        assert!(
            details.preset.contains(preset),
            "settings-list detail preset lacks {preset}"
        );
    }
    for label in [
        "App settings",
        "Chat settings",
        "Lint settings",
        "Format on save",
        "Model",
        "Tags",
        "Custom action",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(label)),
            "settings-list preview lacks {label}"
        );
    }
    for setting in [
        "density",
        "dirty_visualization",
        "query",
        "sections",
        "control_kind",
        "reset",
    ] {
        assert!(
            details.settings.contains(setting),
            "settings-list settings inspector lacks {setting}"
        );
    }
    for action in [
        "settings_query_filter",
        "settings_update_field",
        "settings_toggle_section",
        "settings_route_child_event",
        "settings_reset_field",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "settings-list callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn collapsible_panel_story_exposes_sidebar_presets_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "collapsible-panel")
        .ok_or("collapsible-panel page missing")?;
    let labels =
        page_children(&examples, "collapsible-panel").ok_or("collapsible-panel page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "Explorer panel",
            "Chat history panel",
            "TOC panel",
            "Floating overlay",
            "IconOnly"
        ],
        StoryPresetLabels::for_page("collapsible-panel")
    );
    for preset in StoryPresetLabels::for_page("collapsible-panel") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "collapsible-panel preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "collapsible-panel detail preset lacks {preset}"
        );
    }
    for setting in [
        "mode",
        "width",
        "pinned",
        "expand_on_hover",
        "resize_handle",
    ] {
        assert!(
            details.settings.contains(setting),
            "collapsible-panel settings inspector lacks {setting}"
        );
    }
    for action in [
        "collapsible_panel_resize",
        "collapsible_panel_overlay",
        "collapsible_panel_icon_only",
        "collapsible_panel_hover",
        "collapsible_panel_pin",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "collapsible-panel callback log lacks action {action}"
        );
    }
    for event in [
        "WidthChanged",
        "FloatingShown",
        "FloatingHidden",
        "HoverTemporaryExpanded",
        "PinChanged",
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "collapsible-panel callback log lacks event {event}"
        );
    }
    assert!(
        examples.iter().all(|it| it.page != "app-shell"),
        "storybook must not add a Structured > AppShell page"
    );
    Ok(())
}
