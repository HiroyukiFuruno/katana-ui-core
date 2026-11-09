use super::TextArea;

impl TextArea {
    /// Synchronizes host-controlled placeholder and validation state without replacing input state.
    pub(crate) fn synchronize_field_presentation(
        &mut self,
        placeholder: &str,
        invalid: bool,
    ) -> bool {
        let changed = self.options.placeholder != placeholder
            || self.options.invalid != invalid
            || self.state.invalid != invalid;
        if !changed {
            return false;
        }
        self.options.placeholder = placeholder.to_owned();
        self.options.invalid = invalid;
        self.state.invalid = invalid;
        true
    }
}
