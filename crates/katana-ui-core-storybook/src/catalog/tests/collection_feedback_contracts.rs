use super::*;

#[test]
fn chip_attachment_stories_expose_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let chip = examples
        .iter()
        .find(|it| it.page == "chip")
        .ok_or("chip page missing")?;
    let attachment = examples
        .iter()
        .find(|it| it.page == "attachment-chip")
        .ok_or("attachment-chip page missing")?;
    let group = examples
        .iter()
        .find(|it| it.page == "chip-group")
        .ok_or("chip-group page missing")?;
    let chip_details = StoryDetailContent::from_example(chip);
    let attachment_details = StoryDetailContent::from_example(attachment);
    let group_details = StoryDetailContent::from_example(group);

    assert_eq!(
        &[
            "filter label",
            "leading filter icon",
            "trailing dismiss icon",
            "filled chip variant",
            "danger chip tone",
            "large chip size",
            "interactive chip",
            "selected chip",
            "disabled chip",
            "dismissible chip",
            "accessible chip label",
            "chip focus ring",
        ],
        StoryPresetLabels::for_page("chip")
    );
    assert_eq!(
        &[
            "file attachment",
            "image attachment",
            "url attachment",
            "uploading",
            "error retry",
            "name",
            "meta",
            "thumbnail"
        ],
        StoryPresetLabels::for_page("attachment-chip")
    );
    assert_eq!(
        &[
            "wrap",
            "overflow menu",
            "horizontal scroll",
            "reorder",
            "label",
            "chip count",
            "gap",
            "available width",
            "trigger width"
        ],
        StoryPresetLabels::for_page("chip-group")
    );
    for setting in [
        "chip.label",
        "chip.leading_icon",
        "chip.trailing_icon",
        "chip.variant",
        "chip.tone",
        "chip.size",
        "chip.interactive",
        "chip.selected",
        "chip.disabled",
        "chip.dismissible",
        "chip.a11y_label",
        "chip.focused",
    ] {
        assert!(
            chip_details.settings.contains(setting),
            "chip settings inspector lacks {setting}"
        );
    }
    for setting in [
        "attachment.kind",
        "attachment.name",
        "attachment.meta",
        "attachment.thumbnail",
        "attachment.status",
        "attachment.progress",
        "attachment.retry",
    ] {
        assert!(
            attachment_details.settings.contains(setting),
            "attachment-chip settings inspector lacks {setting}"
        );
    }
    for setting in [
        "chip_group.label",
        "chip_group.chip_count",
        "chip_group.wrap",
        "chip_group.overflow",
        "chip_group.reorder",
        "chip_group.gap",
        "chip_group.available_width",
        "chip_group.overflow_trigger_width",
        "chip_group.hidden_count",
    ] {
        assert!(
            group_details.settings.contains(setting),
            "chip-group settings inspector lacks {setting}"
        );
    }
    assert!(
        chip.callback_logs
            .iter()
            .any(|it| it.action == "chip_dismiss")
    );
    assert!(
        attachment
            .callback_logs
            .iter()
            .any(|it| it.action == "attachment_status")
    );
    assert!(
        group
            .callback_logs
            .iter()
            .any(|it| it.action == "chip_group_overflow")
    );
    Ok(())
}

#[test]
fn diagnostics_list_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "diagnostics-list")
        .ok_or("diagnostics-list page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "lint result",
            "editor inline",
            "tool result",
            "empty",
            "loading",
            "bulk fix",
            "Virtualization"
        ],
        StoryPresetLabels::for_page("diagnostics-list")
    );
    for preset in StoryPresetLabels::for_page("diagnostics-list") {
        assert!(
            details.preset.contains(preset),
            "diagnostics-list detail preset lacks {preset}"
        );
    }
    for setting in [
        "group_by",
        "sort_by",
        "severity_filter",
        "bulk_action",
        "fix_preview",
    ] {
        assert!(
            details.settings.contains(setting),
            "diagnostics-list settings inspector lacks {setting}"
        );
    }
    for action in [
        "diagnostic_fix_preview",
        "diagnostic_bulk_preview",
        "diagnostic_select_error",
        "diagnostic_apply_fix",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "diagnostics-list callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn dynamic_array_editor_story_exposes_real_actions_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "dynamic-array-editor")
        .ok_or("dynamic-array-editor page missing")?;
    let root = story.tree.root();

    assert_eq!(3, root.props().interaction.item_count);
    assert!(
        root.children()
            .iter()
            .any(|it| it.props().label == "Add row")
    );
    assert!(
        root.children()
            .iter()
            .any(|it| it.props().label == "Remove row")
    );
    assert!(
        root.children()
            .iter()
            .any(|it| it.props().label == "Move row")
    );
    for action in ["array_add", "array_remove", "array_reorder"] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.action == action && it.after.contains("event=array_changed")),
            "dynamic-array-editor callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn virtualized_collection_pages_expose_range_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    for page in [
        "list",
        "selection-list",
        "tree-view",
        "command-palette",
        "diagnostics-list",
    ] {
        let story = examples
            .iter()
            .find(|it| it.page == page)
            .ok_or("virtualized collection page missing")?;
        let details = StoryDetailContent::from_example(story);
        assert!(
            StoryPresetLabels::for_page(page).contains(&"Virtualization"),
            "{page} preset lacks Virtualization"
        );
        for setting in [
            "enabled=true->false",
            "overscan",
            "row_height_provider",
            "visible_range",
        ] {
            assert!(
                details.settings.contains(setting),
                "{page} settings inspector lacks {setting}"
            );
        }
        assert!(
            story.callback_logs.iter().any(|it| {
                it.action.contains("virtualization_range")
                    && it.before.contains("visible_range=")
                    && it.before.contains("total_count=")
                    && it.after.contains("enabled=false")
            }),
            "{page} callback log lacks virtualization range and total count"
        );
    }
    Ok(())
}

#[test]
fn tree_view_story_exposes_marker_depth_and_trigger_contract() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "tree-view")
        .ok_or("tree-view page missing")?;
    let tree = &story.tree.root().props().tree;
    let depths: Vec<usize> = tree.nodes.iter().map(|it| it.depth).collect();

    assert_eq!(&[0, 1, 2], &depths[..]);
    assert!(tree.icons_visible, "tree-view markers should be visible");
    assert_eq!("<svg data-icon=\"branch\"/>", tree.directory_icon);
    assert_eq!("<svg data-icon=\"leaf\"/>", tree.file_icon);
    assert!(tree.line_display);
    assert_eq!(1, tree.line_width);
    assert_eq!(
        katana_ui_core::render_model::UiTreeLineStyle::Solid,
        tree.line_style
    );
    assert_eq!(
        katana_ui_core::render_model::UiTreeToggleTriggerArea::IconAndText,
        tree.toggle_trigger_area
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action.starts_with("tree_")
                && it.action != "tree_view_virtualization_range"),
        "tree-view callback log lacks typed tree interaction action"
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "tree_view_virtualization_range"),
        "tree-view callback log lacks virtualization range action"
    );
    Ok(())
}

#[test]
fn empty_state_story_exposes_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "empty-state")
        .ok_or("empty-state page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "explorer empty",
            "search no result",
            "diagnostics clean",
            "history empty",
            "error fallback",
            "heading",
            "body",
            "icon",
            "illustration"
        ],
        StoryPresetLabels::for_page("empty-state")
    );
    for preset in StoryPresetLabels::for_page("empty-state") {
        assert!(
            details.preset.contains(preset),
            "empty-state detail preset lacks {preset}"
        );
    }
    for setting in [
        "heading",
        "body",
        "icon",
        "illustration",
        "tone",
        "size",
        "alignment",
        "actions",
    ] {
        assert!(
            details.settings.contains(setting),
            "empty-state settings inspector lacks {setting}"
        );
    }
    for action in ["empty_state_primary", "empty_state_secondary"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "empty-state callback log lacks action {action}"
        );
    }
    Ok(())
}
