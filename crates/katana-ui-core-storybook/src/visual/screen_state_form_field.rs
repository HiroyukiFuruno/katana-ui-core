use super::screen_state::StorybookScreenState;
use katana_ui_core::atom::Input;
use katana_ui_core::molecule::FormField;
use katana_ui_core::render_model::{UiStateId, UiTree};

const CONTROL_STATE_ID: &str = "field:repository-name";

impl StorybookScreenState {
    pub(in crate::visual) fn register_form_field_focus_link(&mut self) {
        let target = form_field_focus_target();
        debug_assert_eq!(
            target.as_ref().map(UiStateId::as_str),
            Some(CONTROL_STATE_ID)
        );
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "form_field_focus_link";
        self.last_event = "form_field_control_focused";
        self.last_setting = "form_field.control_state_id";
        self.last_setting_value = CONTROL_STATE_ID;
        self.state_label = "focus=control";
    }
}

fn form_field_focus_target() -> Option<UiStateId> {
    let control = UiStateId::new(CONTROL_STATE_ID);
    let tree = UiTree::new(
        FormField::new("Repository name")
            .required(true)
            .helper_text("Visible helper text")
            .control_state_id(control.clone())
            .child(Input::new("katana").stable_state_id(control)),
    );
    tree.root().props().form_field.control_state_id.clone()
}
