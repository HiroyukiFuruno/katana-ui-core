use super::{
    Button, Checkbox, ColorSwatch, IconTextButton, Radio, SlideControl, SvgButton, Text,
    TextButton, Toggle,
};
use crate::component::ComponentAction;
use crate::interaction::UiAction;
use crate::render_model::{
    UI_TASK_TOGGLE_ACTION_ID, UiHostActionPayload, UiHostActionPlan, UiHostActionSpec, UiNode,
    UiNodeKind, UiTree,
};

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
fn button_accepts_command_action_with_explicit_host_label() {
    let node = UiNode::from(Button::new("x").command_action("slideshow.close", "Close slideshow"));

    let plan = UiHostActionPlan::collect_from_root(&node)
        .into_iter()
        .find(|plan| plan.action_id == "slideshow.close");

    assert!(plan.is_some(), "missing command action");
    let Some(plan) = plan else {
        return;
    };
    assert_eq!("Close slideshow", plan.label);
    assert_eq!(UiHostActionPayload::None, plan.typed_payload);
}

#[test]
fn button_accepts_typed_surface_control_action_without_custom_spec_at_call_site() {
    let node = UiNode::from(Button::new("Zoom").surface_control_action(
        "surface.zoom",
        "Zoom",
        "diagram-1",
    ));

    let plan = UiHostActionPlan::collect_from_root(&node)
        .into_iter()
        .find(|plan| plan.action_id == "surface.zoom");

    assert!(plan.is_some(), "missing typed surface control action");
    let Some(plan) = plan else {
        return;
    };
    assert_eq!("Zoom", plan.label);
    assert!(
        matches!(plan.typed_payload, UiHostActionPayload::SurfaceControl(_)),
        "expected typed surface control payload"
    );
    let UiHostActionPayload::SurfaceControl(payload) = plan.typed_payload else {
        return;
    };
    assert_eq!("diagram-1", payload.node_id);
}

#[test]
fn checkbox_accepts_typed_task_control_action_without_custom_spec_at_call_site() {
    let node = UiNode::from(Checkbox::new("").task_control_action("Toggle task", "list", 3));

    let plan = UiHostActionPlan::collect_from_root(&node)
        .into_iter()
        .find(|plan| plan.action_id == UI_TASK_TOGGLE_ACTION_ID);

    assert!(plan.is_some(), "missing typed task control action");
    let Some(plan) = plan else {
        return;
    };
    assert_eq!("Toggle task", plan.label);
    assert!(
        matches!(plan.typed_payload, UiHostActionPayload::TaskControl(_)),
        "expected typed task control payload"
    );
    let UiHostActionPayload::TaskControl(payload) = plan.typed_payload else {
        return;
    };
    assert_eq!("list", payload.node_id);
    assert_eq!(3, payload.row_index);
    assert_eq!("ui-task-state:list:3", payload.state_id);
}

#[test]
fn toggle_exposes_default_hover_target_for_host_consumers() {
    let mut toggle = Toggle::new("Markdown Linter").stable_state_id("toggle.markdown");
    let hover = toggle.hover_target(true);

    assert_eq!(toggle.state_id(), &hover.target);
    assert_eq!(
        UiAction::hover(toggle.state_id().clone(), true),
        hover.action()
    );

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
