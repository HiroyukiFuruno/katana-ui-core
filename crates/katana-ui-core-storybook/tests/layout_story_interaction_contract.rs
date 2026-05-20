use katana_ui_core::render_model::UiNode;
use katana_ui_core_storybook::StoryCatalog;

#[test]
fn split_pane_story_exposes_preset_specific_action_history() {
    let examples = StoryCatalog.examples();
    let split = examples.iter().find(|it| it.page == "split-pane");
    assert!(split.is_some(), "split-pane story is missing");
    let Some(split) = split else {
        return;
    };

    let labels = descendant_labels(split.tree.root());
    for preset in [
        "horizontal",
        "vertical",
        "min clamp",
        "reset",
        "keyboard resize",
        "nested",
    ] {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "split-pane story lacks {preset} preset"
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
            split.callback_logs.iter().any(|it| it.action == action),
            "split-pane story lacks {action} action"
        );
    }
    assert!(
        split
            .callback_logs
            .iter()
            .any(|it| it.action == "split_pane_clamped" && it.after.contains("clamped=true")),
        "split-pane story lacks clamp result evidence"
    );
}

fn descendant_labels(root: &UiNode) -> Vec<String> {
    let mut labels = Vec::new();
    collect_labels(root, &mut labels);
    labels
}

fn collect_labels(node: &UiNode, labels: &mut Vec<String>) {
    labels.push(node.props().label.clone());
    for child in node.children() {
        collect_labels(child, labels);
    }
}
