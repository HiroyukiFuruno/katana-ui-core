use katana_ui_core_storybook::{StoryCatalog, StoryDetailContent, StorybookPanel};

#[test]
fn command_launcher_story_exposes_settings_state_events_actions_and_quality() -> Result<(), String>
{
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "command-palette")
        .ok_or_else(|| "command-palette story is missing".to_string())?;
    let details = StoryDetailContent::from_example(story);
    let panel_report = StorybookPanel::interaction_report(&examples);

    for setting in [
        "query=open->theme",
        "highlight=0->2",
        "row_count=5->50",
        "provider_group=workspace/editor/app",
        "shortcut_display=visible/hidden",
        "disabled_reason=readonly",
        "visible_range",
    ] {
        assert!(
            details.settings.contains(setting),
            "command-palette settings lacks {setting}"
        );
    }
    for state in [
        "query=theme",
        "highlighted_row=theme",
        "virtual_range=",
        "disabled_reason=readonly",
    ] {
        assert!(
            details.state.contains(state),
            "command-palette state lacks {state}"
        );
    }
    for action in [
        "command_query_changed",
        "command_highlight_moved",
        "command_execute",
        "command_close",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "command-palette callback log lacks {action}"
        );
    }
    for event in [
        "QueryChanged",
        "ResultHighlighted",
        "ResultExecuted",
        "Closed",
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "command-palette callback log lacks {event}"
        );
    }
    for quality in [
        "keyboard_contract=true",
        "virtualized_highlight=true",
        "disabled_execution_guard=true",
    ] {
        assert!(
            details.quality.contains(quality),
            "command-palette quality lacks {quality}"
        );
    }
    for option in [
        "command_palette.query",
        "command_palette.highlight",
        "command_palette.row_count",
        "command_palette.provider_group",
        "command_palette.shortcut_display",
    ] {
        assert!(
            panel_report.settings_mutations.iter().any(|it| {
                it.page == "command-palette"
                    && it.option.name == option
                    && it.event == "command_palette_settings_changed"
            }),
            "command-palette settings mutation lacks {option}"
        );
    }
    Ok(())
}
