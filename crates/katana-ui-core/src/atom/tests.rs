use super::{
    Button, Checkbox, ColorSwatch, IconTextButton, Radio, SlideControl, SvgButton, Text,
    TextButton, Toggle,
};
use crate::component::ComponentAction;
use crate::interaction::UiAction;
use crate::render_model::{UiHostActionPlan, UiHostActionSpec, UiNode, UiNodeKind, UiTree};

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

#[test]
fn button_defaults_to_no_host_action() {
    let node = UiNode::from(Button::new("Copy"));

    assert!(UiHostActionPlan::collect_from_root(&node).is_empty());
}

#[test]
fn button_accepts_explicit_host_action() {
    let node = UiNode::from(
        Button::new("Copy").host_action(UiHostActionSpec::command("copy-code", "Copy code")),
    );

    let plan = UiHostActionPlan::collect_from_root(&node)
        .into_iter()
        .find(|plan| plan.action_id == "copy-code");

    assert!(plan.is_some());
}

#[test]
fn checkbox_checked_public_prop_reaches_render_node_props() {
    let checked = UiNode::from(Checkbox::new("Markdown Linter").checked(true));
    let unchecked = UiNode::from(Checkbox::new("Markdown Linter").checked(false));

    assert_eq!(UiNodeKind::Checkbox, checked.kind());
    assert_eq!("Markdown Linter", checked.props().label);
    assert!(checked.props().checked);
    assert!(checked.props().interaction.has_selection);
    assert_eq!(1, checked.props().interaction.selected_index);
    assert!(!unchecked.props().checked);
    assert!(!unchecked.props().interaction.has_selection);
    assert_eq!(0, unchecked.props().interaction.selected_index);
}

#[test]
fn interactive_atoms_default_to_no_host_action() {
    for node in interactive_atom_nodes_without_action() {
        assert!(UiHostActionPlan::collect_from_root(&node).is_empty());
    }
}

#[test]
fn interactive_atoms_accept_explicit_host_action() {
    for node in interactive_atom_nodes_with_action() {
        let plan = UiHostActionPlan::collect_from_root(&node)
            .into_iter()
            .find(|plan| plan.action_id == "custom-action");

        assert!(plan.is_some());
    }
}

#[test]
fn toggle_exposes_default_hover_target_for_host_consumers() {
    let mut toggle = Toggle::new("Markdown Linter").stable_state_id("toggle.markdown");
    let hover = toggle.hover_target(true);

    assert_eq!(toggle.state_id(), &hover.target);
    assert_eq!(UiAction::hover(toggle.state_id().clone(), true), hover.action());

    let result = toggle.apply_action(&hover.action());

    assert!(result.handled);
    assert!(result.after.hovered);
}

fn interactive_atom_nodes_without_action() -> Vec<UiNode> {
    vec![
        UiNode::from(Button::new("Button")),
        UiNode::from(TextButton::new("Text button")),
        UiNode::from(SvgButton::new("Svg button")),
        UiNode::from(IconTextButton::new("Icon text")),
        UiNode::from(Checkbox::new("Checkbox")),
        UiNode::from(Radio::new("Radio")),
        UiNode::from(Toggle::new("Toggle")),
        UiNode::from(ColorSwatch::new("Color")),
        UiNode::from(SlideControl::new("Slide")),
    ]
}

fn interactive_atom_nodes_with_action() -> Vec<UiNode> {
    vec![
        UiNode::from(Button::new("Button").host_action(custom_action())),
        UiNode::from(TextButton::new("Text button").host_action(custom_action())),
        UiNode::from(SvgButton::new("Svg button").host_action(custom_action())),
        UiNode::from(IconTextButton::new("Icon text").host_action(custom_action())),
        UiNode::from(Checkbox::new("Checkbox").host_action(custom_action())),
        UiNode::from(Radio::new("Radio").host_action(custom_action())),
        UiNode::from(Toggle::new("Toggle").host_action(custom_action())),
        UiNode::from(ColorSwatch::new("Color").host_action(custom_action())),
        UiNode::from(SlideControl::new("Slide").host_action(custom_action())),
    ]
}

fn custom_action() -> UiHostActionSpec {
    UiHostActionSpec::command("custom-action", "Custom action")
}
