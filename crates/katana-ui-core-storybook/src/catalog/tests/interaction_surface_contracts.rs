use super::*;

#[test]
fn drag_and_drop_story_exposes_preset_specific_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let drag_and_drop = examples
        .iter()
        .find(|it| it.page == "drag-and-drop")
        .ok_or("drag-and-drop page missing")?;
    let labels = page_children(&examples, "drag-and-drop").ok_or("drag-and-drop page missing")?;
    let details = StoryDetailContent::from_example(drag_and_drop);

    assert_eq!(5, labels.len());
    for preset in StoryPresetLabels::for_page("drag-and-drop") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "drag-and-drop preview lacks preset {preset}"
        );
    }
    for setting in ["accept=", "autoscroll=", "keyboard_draggable="] {
        assert!(
            labels.iter().all(|it| it.contains(setting)),
            "drag-and-drop preview lacks setting {setting}"
        );
        assert!(
            drag_and_drop
                .callback_logs
                .iter()
                .all(|it| it.before.contains(setting) || it.after.contains(setting)),
            "drag-and-drop logs lack setting {setting}"
        );
    }
    assert!(details.settings.contains("accept="));
    assert!(details.settings.contains("autoscroll="));
    assert!(details.settings.contains("keyboard_draggable="));
    for event in [
        "DragStart",
        "DragMove",
        "DragEnter",
        "Drop",
        "DragCancel",
        "DragEnd",
    ] {
        assert!(
            drag_and_drop
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "drag-and-drop logs lack event {event}"
        );
    }
    assert!(
        drag_and_drop
            .callback_logs
            .iter()
            .any(|it| it.action == "file_drop_accept" && it.after.contains("os/file-list")),
        "file drop log must expose the OS file-list payload"
    );
    Ok(())
}

#[test]
fn context_menu_story_exposes_detail_settings_and_callback_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let context_menu = examples
        .iter()
        .find(|it| it.page == "context-menu")
        .ok_or("context-menu page missing")?;
    let labels = page_children(&examples, "context-menu").ok_or("context-menu page missing")?;
    let details = StoryDetailContent::from_example(context_menu);

    for preset in StoryPresetLabels::for_page("context-menu") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "context-menu preview lacks preset {preset}"
        );
    }
    for setting in [
        "context_menu.anchor=",
        "context_menu.placement_priority=",
        "context_menu.placement_used=",
        "context_menu.min_width=",
        "context_menu.max_height=",
        "item_kind=",
        "callback_log=",
    ] {
        assert!(
            details.settings.contains(setting),
            "context-menu settings inspector lacks {setting}"
        );
    }
    for action in [
        "context_menu_open",
        "context_menu_highlight",
        "context_menu_submenu",
        "context_menu_select",
    ] {
        assert!(
            context_menu
                .callback_logs
                .iter()
                .any(|it| it.action == action),
            "context-menu callback log lacks {action}"
        );
    }
    Ok(())
}

#[test]
fn closeable_tab_strip_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "closeable-tab-strip")
        .ok_or("closeable-tab-strip page missing")?;
    let labels = page_children(&examples, "closeable-tab-strip")
        .ok_or("closeable-tab-strip page missing")?;
    let details = StoryDetailContent::from_example(story);

    for preset in [
        "default", "overflow", "pinned", "groups", "dirty", "dragging",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "closeable-tab-strip preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "closeable-tab-strip details lack preset {preset}"
        );
    }
    for action in [
        "add_tab",
        "delete_tab",
        "pin_tab",
        "dirty_toggle",
        "group_toggle",
        "drag_tab",
        "overflow_open",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "closeable-tab-strip callback log lacks action {action}"
        );
        assert!(
            details.settings.contains(action),
            "closeable-tab-strip settings inspector lacks action {action}"
        );
    }
    for event in [
        "closeable_tab_added",
        "closeable_tab_closed",
        "closeable_tab_reordered",
        "closeable_tab_overflow_opened",
        "tab_dirty_changed",
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "closeable-tab-strip callback log lacks event {event}"
        );
    }
    Ok(())
}

#[test]
fn toolbar_story_exposes_overflow_split_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "toolbar")
        .ok_or("toolbar page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "overflow menu",
            "split action",
            "display mode",
            "density",
            "accelerator",
            "context anchor",
            "action priority",
            "action accelerator",
            "action split",
            "action group",
            "action tooltip",
            "action a11y",
            "action disabled",
            "group label",
            "group divider",
            "split disabled",
            "split tooltip",
            "split a11y"
        ],
        StoryPresetLabels::for_page("toolbar")
    );
    for setting in [
        "toolbar.display_mode",
        "toolbar.density",
        "toolbar.overflow_strategy",
        "toolbar.context_menu_anchor",
        "toolbar.action_priority",
        "toolbar.action_accelerator",
        "toolbar.action_split",
        "toolbar.action_group",
        "toolbar.action_tooltip",
        "toolbar.action_a11y",
        "toolbar.action_disabled",
        "toolbar.group_label",
        "toolbar.group_divider",
        "toolbar.split_disabled",
        "toolbar.split_tooltip",
        "toolbar.split_a11y",
    ] {
        assert!(
            details.settings.contains(setting),
            "toolbar settings inspector lacks {setting}"
        );
    }
    for action in ["toolbar_overflow_plan", "toolbar_split_open"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "toolbar callback log lacks {action}"
        );
    }
    Ok(())
}

#[test]
fn split_pane_story_exposes_presets_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "split-pane")
        .ok_or("split-pane page missing")?;
    let labels =
        page_descendant_labels(&examples, "split-pane").ok_or("split-pane page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "axis vertical",
            "wide gap",
            "center alignment",
            "overflow scroll",
            "ratio percent",
            "min percent clamp",
            "max percent clamp",
            "reset percent",
            "wide handle",
            "keyboard resize mode"
        ],
        StoryPresetLabels::for_page("split-pane")
    );
    for preset in StoryPresetLabels::for_page("split-pane") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "split-pane preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "split-pane detail preset lacks {preset}"
        );
    }
    for setting in [
        "axis=",
        "gap=",
        "alignment=",
        "overflow=",
        "ratio_percent=",
        "min_percent=",
        "max_percent=",
        "reset_percent=",
        "handle_width_px=",
        "resize_mode=",
        "children=",
        "nested=",
    ] {
        assert!(
            details.settings.contains(setting),
            "split-pane settings inspector lacks {setting}"
        );
    }
    for action in [
        "split_pane_resized",
        "split_pane_keyboard_resize",
        "split_pane_reset",
        "split_pane_drag_start",
        "split_pane_drag_end",
        "split_pane_clamped",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "split-pane callback log lacks action {action}"
        );
        assert!(
            details.settings.contains(action),
            "split-pane settings inspector lacks action {action}"
        );
    }
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "split_pane_clamped" && it.after.contains("clamped=true")),
        "split-pane callback log lacks clamp evidence"
    );
    Ok(())
}
