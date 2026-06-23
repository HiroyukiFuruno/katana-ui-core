use super::*;

#[test]
fn select_box_story_root_props_match_initial_callback_log() -> Result<(), &'static str> {
    let story = story_for("select-box")?;
    let harness = story.tree.root();
    let select = harness
        .children()
        .first()
        .ok_or("select-box node missing from harness")?;
    let controls = harness
        .children()
        .get(1)
        .ok_or("select-box control row missing")?;
    let callback = first_callback(&story)?;

    assert_eq!(
        &["state read", "open", "close", "select dark", "reset"],
        labels(controls).as_slice()
    );
    assert_eq!(callback.target.as_str(), select.props().state_id.as_str());
    assert_eq!("select_state_read", callback.action);
    assert_eq!("open=false selected=none", callback.before);
    assert_eq!("open=false selected=none", callback.after);
    assert_initial_choice_state(select);
    Ok(())
}

#[test]
fn combo_box_story_root_props_match_initial_callback_log() -> Result<(), &'static str> {
    let story = story_for("combo-box")?;
    let harness = story.tree.root();
    let combo = harness
        .children()
        .first()
        .ok_or("combo-box node missing from harness")?;
    let controls = harness
        .children()
        .get(1)
        .ok_or("combo-box control row missing")?;
    let callback = first_callback(&story)?;

    assert_eq!(
        &["state read", "filter", "select two", "reset"],
        labels(controls).as_slice()
    );
    assert_eq!(callback.target.as_str(), combo.props().state_id.as_str());
    assert_eq!("combo_state_read", callback.action);
    assert_eq!("open=false query=empty selected=none", callback.before);
    assert_eq!("open=false query=empty selected=none", callback.after);
    assert_initial_choice_state(combo);
    assert!(has_action(&story, "select_box_selected"));
    Ok(())
}

#[test]
fn selection_list_story_root_props_match_initial_callback_log() -> Result<(), &'static str> {
    let story = story_for("selection-list")?;
    let list = story.tree.root();
    let callback = story
        .callback_logs
        .iter()
        .find(|it| it.action == "selection_list_state_read")
        .ok_or("selection-list state read callback log missing")?;

    assert_eq!(callback.target.as_str(), list.props().state_id.as_str());
    assert_eq!("single=none multi=none focus=none", callback.before);
    assert_eq!("single=none multi=none focus=none", callback.after);
    assert_initial_choice_state(list);
    assert!(has_action(&story, "select_box_selected"));
    assert!(has_action(&story, "set_selected_index"));
    assert!(story.contract.action_history);
    assert!(story.contract.event_history);
    Ok(())
}

fn story_for(page: &str) -> Result<StoryExample, &'static str> {
    StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == page)
        .ok_or("choice page missing")
}

fn first_callback(
    story: &StoryExample,
) -> Result<&katana_ui_core::interaction::UiCallbackLog, &'static str> {
    story.callback_logs.first().ok_or("callback log missing")
}

fn labels(node: &UiNode) -> Vec<&str> {
    node.children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect()
}

fn assert_initial_choice_state(node: &UiNode) {
    assert!(!node.props().interaction.open);
    assert!(!node.props().interaction.has_selection);
    assert_eq!(0, node.props().interaction.selected_index);
    assert!(node.props().interaction.value.is_empty());
}

fn has_action(story: &StoryExample, action: &str) -> bool {
    story.callback_logs.iter().any(|it| it.action == action)
}
