use super::{Button, Text};
use crate::render_model::{UiNodeKind, UiTree};

#[test]
fn atom_snapshot_uses_neutral_node_kind() {
    let tree = UiTree::new(Button::new("Save"));
    assert_eq!(UiNodeKind::Button, tree.root().kind());
}

#[test]
fn text_atom_can_be_tree_root() {
    let tree = UiTree::new(Text::new("Title"));
    assert_eq!(UiNodeKind::Text, tree.root().kind());
}
