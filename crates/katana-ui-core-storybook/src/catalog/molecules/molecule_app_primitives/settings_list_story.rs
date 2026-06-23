use super::{
    SettingsListAction, SettingsValue, StoryCatalog, StoryExample, UPDATED_FONT_SIZE,
    UiCallbackLog, settings_story,
};

pub(super) fn settings_list_story() -> StoryExample {
    let mut settings = settings_story::settings_list();
    let target = settings.state_id().clone();
    let query =
        settings.apply_settings_action(SettingsListAction::SetQuery(Some("format".to_string())));
    let updated = settings.apply_settings_action(SettingsListAction::UpdateField {
        field_id: "app.font-size".to_string(),
        value: SettingsValue::Number(UPDATED_FONT_SIZE),
    });
    let collapsed = settings.apply_settings_action(SettingsListAction::ToggleSection {
        section_id: "chat".to_string(),
    });
    let child = settings.apply_settings_action(SettingsListAction::RouteChildEvent {
        field_id: "lint.custom-action".to_string(),
        event: "custom_button_pressed".to_string(),
    });
    let reset = settings.apply_settings_action(SettingsListAction::ResetField {
        field_id: "app.font-size".to_string(),
    });
    let restored = settings.apply_settings_action(SettingsListAction::SetQuery(None));
    let reopened = settings.apply_settings_action(SettingsListAction::ToggleSection {
        section_id: "chat".to_string(),
    });
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "settings_query_filter",
            "query=None visible_sections=3",
            format!("query={query:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "settings_update_field",
            "field=app.font-size value=14 dirty=false",
            format!("updated={updated:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "settings_toggle_section",
            "collapsed_sections=0",
            format!("collapsed={collapsed:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "settings_route_child_event",
            "field=lint.custom-action event=None",
            format!("child={child:?}"),
        ),
        UiCallbackLog::new(
            target,
            "settings_reset_field",
            "field=app.font-size value=16 dirty=true",
            format!("reset={reset:?} restored={restored:?} reopened={reopened:?}"),
        ),
    ];
    StoryCatalog::interactive_story("settings-list", settings, logs)
}
