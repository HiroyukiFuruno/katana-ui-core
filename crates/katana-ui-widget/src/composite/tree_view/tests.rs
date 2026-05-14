use super::TreeViewItem;
use super::ops::TreeViewOps;

fn sample_tree() -> Vec<TreeViewItem> {
    vec![
        TreeViewItem::new("src").expanded(true).children(vec![
            TreeViewItem::new("main.rs"),
            TreeViewItem::new("lib.rs").active(true),
        ]),
        TreeViewItem::new("target")
            .disabled(true)
            .children(vec![TreeViewItem::new("debug")]),
    ]
}

#[test]
fn flatten_visible_items_includes_expanded_children() {
    let flattened = TreeViewOps::flatten_visible_items(&sample_tree(), false);

    assert_eq!(flattened.len(), 4);
    assert_eq!(flattened[0].label, "src");
    assert_eq!(flattened[1].label, "main.rs");
    assert_eq!(flattened[2].label, "lib.rs");
}

#[test]
fn toggle_expand_updates_parent_state() {
    let mut items = sample_tree();

    assert_eq!(TreeViewOps::toggle_expand(&mut items, &[0]), Some(false));
    assert!(!items[0].expanded);
}

#[test]
fn set_active_moves_active_marker() {
    let mut items = sample_tree();

    assert!(TreeViewOps::set_active(&mut items, &[0, 0]));
    assert!(items[0].children[0].active);
    assert!(!items[0].children[1].active);
}

#[test]
fn nested_node_input_is_flattened_recursively_with_icons() {
    let nodes = vec![
        super::TreeViewNode::new("root", "root")
            .icon(crate::primitive::icon::IconSource::SvgString(
                "<svg />".to_string(),
            ))
            .expanded(true)
            .children(vec![
                super::TreeViewNode::new("child", "child")
                    .expanded(true)
                    .children(vec![super::TreeViewNode::new("leaf", "leaf").icon(
                        crate::primitive::icon::IconSource::SvgString("<svg />".to_string()),
                    )]),
            ]),
    ];
    let items = nodes
        .into_iter()
        .map(super::TreeViewItem::from)
        .collect::<Vec<_>>();
    let flattened = TreeViewOps::flatten_visible_items(&items, true);

    assert_eq!(flattened.len(), 3);
    assert_eq!(flattened[0].label, "root");
    assert_eq!(flattened[1].label, "child");
    assert_eq!(flattened[2].label, "leaf");
    assert!(flattened[0].icon.is_some());
    assert!(flattened[2].icon.is_some());
}

#[test]
fn expand_all_and_collapse_all_skip_disabled_parents() {
    let mut items = sample_tree();

    TreeViewOps::expand_all(&mut items);
    assert!(items[0].expanded);
    assert!(!items[1].expanded);

    TreeViewOps::collapse_all(&mut items);
    assert!(!items[0].expanded);
}
