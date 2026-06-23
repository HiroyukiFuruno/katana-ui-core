use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiNode, UiStateId};
use katana_ui_core::{atom, molecule};

pub(super) fn story() -> StoryExample {
    let control = UiStateId::new("field:repository-name");
    let field = molecule::FormField::new("Repository name")
        .required(true)
        .control_state_id(control.clone())
        .helper_text("Visible helper text")
        .child(atom::Text::new("Repository name"))
        .child(atom::Input::new("katana").stable_state_id(control))
        .child(atom::Button::new("state read"))
        .child(atom::Button::new("field validate"))
        .child(atom::Button::new("helper text"))
        .child(atom::Text::new(
            "state: invalid=false helper=Visible helper text",
        ))
        .child(atom::Text::new(
            "event: form_field_state_read validation_changed helper_text_changed",
        ))
        .child(atom::Text::new(
            "action: form_field_state_read field_validate form_field_helper_text",
        ));
    let root = UiNode::from(field.clone());
    let target = root.props().state_id.clone();
    let invalid_field = field
        .clone()
        .invalid(true)
        .helper_text("Repository name is required");
    let helper_field = field
        .clone()
        .helper_text("Used for release notes and package metadata");
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "form_field_state_read",
            form_field_state_label(&field),
            form_field_state_label(&field),
        ),
        UiCallbackLog::new(
            target.clone(),
            "field_validate",
            form_field_state_label(&field),
            form_field_state_label(&invalid_field),
        ),
        UiCallbackLog::new(
            target,
            "form_field_helper_text",
            form_field_state_label(&field),
            form_field_state_label(&helper_field),
        ),
    ];
    StoryCatalog::interactive_story("form-field", root, logs)
}

fn form_field_state_label(field: &molecule::FormField) -> String {
    format!(
        "invalid={} helper={}",
        field.invalid_model(),
        field.helper_text_model()
    )
}
