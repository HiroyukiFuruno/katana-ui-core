use super::*;

#[test]
fn checkbox_story_uses_public_props_and_typed_callback_log() -> Result<(), &'static str> {
    let story = story_for("checkbox")?;
    let checkbox = story.tree.root();
    let callback = story
        .callback_logs
        .first()
        .ok_or("checkbox callback log missing")?;

    assert_eq!(UiNodeKind::Checkbox, checkbox.kind());
    assert_eq!("Markdown Linter", checkbox.props().label);
    assert_eq!("Checkbox", checkbox.props().accessibility_label);
    assert!(!checkbox.props().checked);
    assert!(!checkbox.props().disabled);
    assert_eq!(callback.target.as_str(), checkbox.props().state_id.as_str());
    assert_eq!("checkbox_state_read", callback.action);
    assert_eq!("checked=false", callback.before);
    assert_eq!("checked=false", callback.after);
    assert!(has_action(&story, "checkbox_toggle"));
    assert!(has_action(&story, "checkbox_reset"));
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| { it.action == "checkbox_state_read" && it.before == it.after })
    );
    Ok(())
}

#[test]
fn radio_story_uses_public_props_and_typed_callback_log() -> Result<(), &'static str> {
    let story = story_for("radio")?;
    let radio = story.tree.root();
    let callback = story
        .callback_logs
        .first()
        .ok_or("radio callback log missing")?;

    assert_eq!(UiNodeKind::Radio, radio.kind());
    assert_eq!("Radio", radio.props().label);
    assert_eq!("Radio", radio.props().accessibility_label);
    assert!(!radio.props().checked);
    assert_eq!(callback.target.as_str(), radio.props().state_id.as_str());
    assert_eq!("radio_state_read", callback.action);
    assert_eq!("selected=false", callback.before);
    assert_eq!("selected=false", callback.after);
    assert!(has_action(&story, "radio_select"));
    assert!(has_action(&story, "radio_reset"));
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| { it.action == "radio_state_read" && it.before == it.after })
    );
    Ok(())
}

fn story_for(page: &str) -> Result<StoryExample, &'static str> {
    StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == page)
        .ok_or("binary atom page missing")
}

fn has_action(story: &StoryExample, action: &str) -> bool {
    story.callback_logs.iter().any(|it| it.action == action)
}
