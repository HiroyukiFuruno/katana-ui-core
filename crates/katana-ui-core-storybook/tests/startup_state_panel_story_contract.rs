use katana_ui_core_storybook::{StoryCatalog, StoryDetailContent, StorybookPanel};

#[test]
fn startup_state_panel_story_exposes_settings_presets_and_action_history() -> Result<(), String> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "startup-state-panel")
        .ok_or_else(|| "startup-state-panel story is missing".to_string())?;
    let details = StoryDetailContent::from_example(story);
    let panel_report = StorybookPanel::interaction_report(&examples);

    for preset in ["app boot", "session init", "update install", "error retry"] {
        assert!(
            details.preset.contains(preset),
            "startup-state-panel detail lacks preset {preset}"
        );
    }
    for setting in [
        "state=Idle/Loading/Error",
        "progress=None/64/100",
        "label=Loading workspace",
        "retry=true/false",
        "cancel=true/false",
    ] {
        assert!(
            details.settings.contains(setting),
            "startup-state-panel settings lacks {setting}"
        );
    }
    for action in ["startup_state_error", "startup_retry", "startup_cancel"] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "startup-state-panel callback log lacks {action}"
        );
    }
    for option in [
        "startup_state.state",
        "startup_state.progress",
        "startup_state.label",
        "startup_state.retry",
        "startup_state.cancel",
    ] {
        assert!(
            panel_report.settings_mutations.iter().any(|it| {
                it.page == "startup-state-panel"
                    && it.option.name == option
                    && it.event == "startup_state_settings_changed"
            }),
            "startup-state-panel settings mutation lacks {option}"
        );
    }
    Ok(())
}
