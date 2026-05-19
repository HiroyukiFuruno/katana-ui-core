use katana_ui_core_storybook::StoryCatalog;

#[test]
fn split_pane_story_exposes_resize_action_history() {
    let examples = StoryCatalog.examples();
    let split = examples.iter().find(|it| it.page == "split-pane");
    assert!(split.is_some(), "split-pane story is missing");
    let Some(split) = split else {
        return;
    };

    assert!(
        split
            .callback_logs
            .iter()
            .any(|it| it.action == "split_pane_resized"),
        "split-pane story lacks resize action"
    );
}
