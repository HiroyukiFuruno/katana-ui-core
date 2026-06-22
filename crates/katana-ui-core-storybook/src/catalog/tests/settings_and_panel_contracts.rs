use super::*;

#[test]
fn panel_story_exposes_real_axis_scroll_models_and_inspector_contract() -> Result<(), &'static str>
{
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "panel")
        .ok_or("panel page missing")?;
    let root = story.tree.root();
    let navigation = panel_child(root, "Navigation panel").ok_or("navigation panel missing")?;
    let preview = panel_child(root, "Preview panel").ok_or("preview panel missing")?;
    let details = StoryDetailContent::from_example(story);

    assert!(
        navigation.props().panel.vertical_scrollbar_visible,
        "panel story navigation must expose a real vertical scrollbar"
    );
    assert!(
        preview.props().panel.vertical_scrollbar_visible,
        "panel story preview must expose a real vertical scrollbar"
    );
    assert!(
        preview.props().panel.horizontal_scrollbar_visible,
        "panel story preview must expose a real horizontal scrollbar"
    );
    for setting in [
        "panel.vertical_scroll",
        "panel.horizontal_scroll",
        "panel.scrollbar_visibility",
        "panel.nested_state",
    ] {
        assert!(
            details.settings.contains(setting),
            "panel settings inspector lacks {setting}"
        );
    }
    for action in [
        "panel_wheel_y",
        "panel_wheel_x",
        "panel_scrollbar_visibility",
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.action == action && it.after.contains("event=panel_scroll")),
            "panel callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn theme_tokens_story_exposes_theme_switch_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "theme-tokens")
        .ok_or("theme-tokens page missing")?;

    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "theme_switch" && it.after.contains("theme=light"))
    );
    Ok(())
}

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
            "reset",
            "label",
            "section label",
            "section description",
            "section icon",
            "field count",
            "section footer",
            "section collapse",
            "default collapsed",
            "field label",
            "field description",
            "control options",
            "custom control",
            "set value"
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
        "settings_list.label",
        "settings_list.section_label",
        "settings_list.section_description",
        "settings_list.section_icon",
        "settings_list.field_count",
        "settings_list.section_footer",
        "settings_list.section_collapsible",
        "settings_list.default_collapsed",
        "settings_list.field_label",
        "settings_list.field_description",
        "control_kind",
        "settings_list.control_options",
        "settings_list.custom_control",
        "settings_list.set_value",
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

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
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
