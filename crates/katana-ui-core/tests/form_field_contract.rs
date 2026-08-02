use katana_ui_core::atom::Input;
use katana_ui_core::molecule::{FormField, MoleculeEventRouting};
use katana_ui_core::render_model::{UiNodeId, UiStateId, UiTree};

#[test]
fn form_field_exposes_required_helper_and_control_focus_target() {
    let control = UiStateId::new("field:repository-name");
    let tree = UiTree::new(
        FormField::new("Repository name")
            .required(true)
            .invalid(true)
            .helper_text("Repository name is required")
            .control_state_id(control.clone())
            .child(Input::new("katana").stable_state_id(control.clone())),
    );
    let props = tree.root().props();

    assert!(props.invalid);
    assert!(props.form_field.required);
    assert_eq!("Repository name is required", props.form_field.helper_text);
    assert_eq!(Some(control), props.form_field.control_state_id);
}

#[test]
fn form_field_model_accessors_and_nested_route_are_stable() {
    let control = UiStateId::new("field:control");
    let field = FormField::new("Field")
        .required(true)
        .invalid(true)
        .helper_text("Required")
        .control_state_id(control.clone());

    assert!(field.invalid_model());
    assert!(field.required_model());
    assert_eq!(Some(&control), field.control_state_id_model());
    assert_eq!("Required", field.helper_text_model());

    let route = MoleculeEventRouting::bubble_nested(
        UiNodeId::new("control"),
        UiNodeId::new("field"),
        UiNodeId::new("root"),
        false,
    );
    assert_eq!(3, route.order().len());
}
