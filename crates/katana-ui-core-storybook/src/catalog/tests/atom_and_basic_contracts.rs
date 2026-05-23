use super::*;

#[test]
fn atom_examples_use_typed_props_without_type_classes() {
    let examples = StoryCatalog.examples();
    let atoms = examples
        .iter()
        .filter(|it| is_atom_kind(it.tree.root().kind()));

    for example in atoms {
        let props = example.tree.root().props();
        assert!(props.style_classes.is_empty(), "{}", example.page);
    }
    let key_cap = examples.iter().find(|it| it.page == "key-cap");
    assert!(key_cap.is_some(), "key-cap story is required");
    let key_cap_props = key_cap.map(|it| it.tree.root().props());
    assert_eq!(
        Some(UiVisualRole::Shortcut),
        key_cap_props.map(|it| it.visual_role)
    );
    assert_eq!(Some("code"), key_cap_props.map(|it| it.font_role.as_str()));
}

#[test]
fn interactive_atom_examples_expose_callback_logs() {
    let examples = StoryCatalog.examples();
    let log_pages: Vec<&str> = examples
        .iter()
        .filter(|it| !it.callback_logs.is_empty())
        .map(|it| it.page)
        .collect();

    assert!(log_pages.contains(&"button"));
    assert!(log_pages.contains(&"text-input"));
    assert!(log_pages.contains(&"checkbox"));
    assert!(log_pages.contains(&"toggle"));
}

#[test]
fn badge_story_remains_passive_and_points_to_chip_for_dismiss() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let badge = examples
        .iter()
        .find(|it| it.page == "badge")
        .ok_or("badge page missing")?;
    let details = StoryDetailContent::from_example(badge);

    assert!(badge.callback_logs.is_empty());
    assert!(details.settings.contains("passive"));
    assert!(details.settings.contains("Chip"));
    Ok(())
}

#[test]
fn story_page_contract_is_derived_from_materialized_evidence() {
    let incomplete =
        StoryPageContract::from_tree("button", &UiTree::new(atom::Button::new("Button")), 99, &[]);
    let passive = StoryPageContract::from_tree(
        "divider",
        &UiTree::new(atom::Divider::new("Divider")),
        1,
        &[],
    );

    assert!(!incomplete.preview);
    assert!(!incomplete.action_history);
    assert!(!incomplete.event_history);
    assert!(!incomplete.is_complete());
    assert!(passive.action_history);
    assert!(passive.event_history);
}

#[test]
fn color_picker_and_code_diff_stories_materialize_required_controls() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let color_picker =
        page_children(&examples, "color-picker-rgba").ok_or("color picker page missing")?;
    let code_diff = page_children(&examples, "code-diff").ok_or("code diff page missing")?;

    assert!(color_picker.iter().any(|it| it.contains("trigger")));
    assert!(color_picker.iter().any(|it| it.contains("floating")));
    assert!(color_picker.iter().any(|it| it.contains("R=64")));
    assert!(code_diff.iter().any(|it| it.contains("split / inline")));
    assert!(code_diff.iter().any(|it| it.contains("collapse")));
    assert!(code_diff.iter().any(|it| it.contains("日本語")));
    Ok(())
}

#[test]
fn color_picker_and_code_diff_presets_are_dod_specific() {
    assert_eq!(
        &[
            "rgba panel",
            "color trigger",
            "size presets",
            "borderless",
            "floating panel"
        ],
        StoryPresetLabels::for_page("color-picker-rgba")
    );
    assert_eq!(
        &[
            "split left-right",
            "split top-bottom",
            "inline",
            "collapsed",
            "japanese whitespace"
        ],
        StoryPresetLabels::for_page("code-diff")
    );
}

#[test]
fn checkbox_story_uses_public_props_and_typed_callback_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let checkbox = examples
        .iter()
        .find(|it| it.page == "checkbox")
        .ok_or("checkbox page missing")?;
    let harness = checkbox.tree.root();
    let checkbox_node = harness
        .children()
        .first()
        .ok_or("checkbox node missing from harness")?;
    let control_row = harness
        .children()
        .get(1)
        .ok_or("checkbox control row missing")?;
    let callback = checkbox
        .callback_logs
        .first()
        .ok_or("checkbox callback log missing")?;

    assert_eq!("Markdown Linter", checkbox_node.props().label);
    assert_eq!("Checkbox", checkbox_node.props().accessibility_label);
    assert!(!checkbox_node.props().checked);
    assert!(!checkbox_node.props().disabled);
    let control_labels: Vec<&str> = control_row
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect();
    assert_eq!(&["state read", "toggle", "reset"], &control_labels[..]);
    assert_eq!(
        callback.target.as_str(),
        checkbox_node.props().state_id.as_str()
    );
    assert_eq!("checkbox_state_read", callback.action);
    assert_eq!("checked=false", callback.before);
    assert_eq!("checked=false", callback.after);
    assert!(
        checkbox
            .callback_logs
            .iter()
            .any(|it| it.action == "checkbox_toggle")
    );
    assert!(
        checkbox
            .callback_logs
            .iter()
            .any(|it| it.action == "checkbox_reset")
    );
    assert!(
        checkbox
            .callback_logs
            .iter()
            .any(|it| it.action == "checkbox_state_read" && it.before == it.after)
    );
    Ok(())
}

#[test]
fn radio_story_uses_public_props_and_typed_callback_log() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let radio = examples
        .iter()
        .find(|it| it.page == "radio")
        .ok_or("radio page missing")?;
    let harness = radio.tree.root();
    let radio_node = harness
        .children()
        .first()
        .ok_or("radio node missing from harness")?;
    let control_row = harness
        .children()
        .get(1)
        .ok_or("radio control row missing")?;
    let callback = radio
        .callback_logs
        .first()
        .ok_or("radio callback log missing")?;

    assert_eq!("Radio", radio_node.props().label);
    assert_eq!("Radio", radio_node.props().accessibility_label);
    assert!(!radio_node.props().checked);
    let control_labels: Vec<&str> = control_row
        .children()
        .iter()
        .map(|it| it.props().label.as_str())
        .collect();
    assert_eq!(&["state read", "select", "reset"], &control_labels[..]);
    assert_eq!(callback.target.as_str(), radio_node.props().state_id.as_str());
    assert_eq!("radio_state_read", callback.action);
    assert_eq!("selected=false", callback.before);
    assert_eq!("selected=false", callback.after);
    assert!(radio.callback_logs.iter().any(|it| it.action == "radio_select"));
    assert!(radio.callback_logs.iter().any(|it| it.action == "radio_reset"));
    assert!(
        radio
            .callback_logs
            .iter()
            .any(|it| it.action == "radio_state_read" && it.before == it.after)
    );
    Ok(())
}
