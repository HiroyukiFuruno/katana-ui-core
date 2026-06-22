use katana_ui_core::atom::Input;
use katana_ui_core::molecule::FormField;
use katana_ui_core::render_model::{UiStateId, UiTree};

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
