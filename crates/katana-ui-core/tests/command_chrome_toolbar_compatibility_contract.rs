use katana_ui_core::molecule::toolbar::{
    ToolbarAction, ToolbarEvent, ToolbarOptions, ToolbarPlacementRequest,
};

#[test]
fn legacy_toolbar_action_and_options_keep_their_serialized_public_shape() {
    let action = ToolbarAction::new("save", "Save")
        .tooltip("Save document")
        .accessibility_label("Save document");
    let options = ToolbarOptions::new().action(action);

    let action_json = serde_json::to_value(&options);

    assert!(action_json.is_ok());
    let Ok(action_json) = action_json else {
        return;
    };
    assert_eq!(
        Some("IconLeading"),
        action_json.get("display_mode").and_then(|it| it.as_str())
    );
    assert_eq!(Some("save"), action_json["actions"][0]["id"].as_str());
    assert_eq!(Some("Save"), action_json["actions"][0]["label"].as_str());
    assert_eq!(
        Some("Save document"),
        action_json["actions"][0]["accessibility_label"].as_str()
    );
}

#[test]
fn legacy_toolbar_event_consumer_remains_exhaustive() {
    let events = vec![
        ToolbarEvent::Command {
            action_id: "save".into(),
        },
        ToolbarEvent::OverflowOpened,
        ToolbarEvent::SplitDropdownOpened {
            action_id: "save-as".into(),
            placement: ToolbarPlacementRequest::Menu,
        },
        ToolbarEvent::AcceleratorTriggered {
            action_id: "save".into(),
            combo: katana_ui_core::molecule::toolbar::KeyCombo::command_or_control("s"),
        },
        ToolbarEvent::GroupCollapseToggled {
            group_id: "document".into(),
        },
    ];

    let names = events
        .into_iter()
        .map(legacy_event_name)
        .collect::<Vec<_>>();

    assert_eq!(
        vec!["command", "overflow", "split", "accelerator", "group"],
        names
    );
}

fn legacy_event_name(event: ToolbarEvent) -> &'static str {
    match event {
        ToolbarEvent::Command { .. } => "command",
        ToolbarEvent::OverflowOpened => "overflow",
        ToolbarEvent::SplitDropdownOpened { .. } => "split",
        ToolbarEvent::AcceleratorTriggered { .. } => "accelerator",
        ToolbarEvent::GroupCollapseToggled { .. } => "group",
    }
}
