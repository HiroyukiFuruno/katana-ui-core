use super::*;

#[test]
fn window_control_button_group_story_exposes_settings_presets_and_logs() -> Result<(), &'static str>
{
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "window-control-button-group")
        .ok_or("window-control-button-group page missing")?;
    let labels = page_descendant_labels(&examples, "window-control-button-group")
        .ok_or("window-control-button-group page missing")?;
    let details = StoryDetailContent::from_example(story);
    let window_control_groups = story
        .tree
        .root()
        .children()
        .iter()
        .filter(|it| it.kind() == UiNodeKind::WindowControlButtonGroup)
        .count();

    assert_eq!(5, window_control_groups);
    assert_eq!(
        &[
            "macOS",
            "Windows",
            "Linux",
            "fullscreen hover",
            "close only"
        ],
        StoryPresetLabels::for_page("window-control-button-group")
    );
    for preset in StoryPresetLabels::for_page("window-control-button-group") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "window-control-button-group preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "window-control-button-group detail preset lacks {preset}"
        );
    }
    for setting in [
        "position=Leading/Trailing/Auto",
        "size=Compact/Default/Tall",
        "controls=Close+Minimize+Maximize+Restore",
        "visibility=Always/Hover/FullscreenHover",
        "state=visible",
        "event=ControlPressed+VisibilityChanged+FullscreenChanged",
        "action=window_control_press",
    ] {
        assert!(
            details.settings.contains(setting),
            "window-control-button-group settings inspector lacks {setting}"
        );
    }
    for action in [
        "window_control_press",
        "window_controls_hover",
        "window_controls_fullscreen",
        "window_controls_close_only",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "window-control-button-group callback log lacks action {action}"
        );
    }
    for event in ["ControlPressed", "VisibilityChanged", "FullscreenChanged"] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "window-control-button-group callback log lacks event {event}"
        );
    }
    Ok(())
}

#[test]
fn hover_card_story_exposes_rich_slots_and_callback_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "hover-card")
        .ok_or("hover-card page missing")?;
    let labels = page_children(&examples, "hover-card").ok_or("hover-card page missing")?;
    let details = StoryDetailContent::from_example(story);

    for expected in [
        "Anchor",
        "Heading: Capability",
        "Body: Shows rich hover and focus content",
        "Footer: Keeps open while the card is focused",
        "Configure",
    ] {
        assert!(
            labels.iter().any(|it| it == expected),
            "hover-card preview lacks {expected}"
        );
    }
    assert_eq!(
        &[
            "delayed open",
            "pointer follow",
            "focus trigger",
            "rich content",
            "actions"
        ],
        StoryPresetLabels::for_page("hover-card")
    );
    for action in ["hover_card_open", "hover_card_keep_open"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "hover-card callback log lacks {action}"
        );
    }
    for setting in ["delay", "placement", "arrow", "focus", "slot"] {
        assert!(
            details.settings.contains(setting),
            "hover-card settings inspector lacks {setting}"
        );
    }
    Ok(())
}
