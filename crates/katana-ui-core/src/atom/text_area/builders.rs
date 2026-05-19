use super::{
    TextArea, TextAreaNewlineKey, TextAreaSubmitKey, TextAreaTabBehavior, TextAreaValidationError,
    TextAreaWrapPolicy,
};
use crate::render_model::{UiCommonProps, UiSlotPlacement, UiSlotSpec, UiVisualRole};

impl TextArea {
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.set_value(value.into());
        self
    }

    #[must_use]
    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.options.placeholder = value.into();
        self
    }

    #[must_use]
    pub fn font_role(mut self, value: impl Into<String>) -> Self {
        self.options.font_role = value.into();
        self
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.options.disabled = value;
        self.state.disabled = value;
        self.common.disabled = value;
        self
    }

    #[must_use]
    pub fn readonly(mut self, value: bool) -> Self {
        self.options.readonly = value;
        self.state.readonly = value;
        self
    }

    #[must_use]
    pub fn invalid(mut self, value: bool) -> Self {
        self.options.invalid = value;
        self.state.invalid = value;
        self
    }

    #[must_use]
    pub fn min_rows(mut self, value: u16) -> Self {
        self.options.min_rows = value;
        self.remeasure();
        self
    }

    #[must_use]
    pub fn max_rows(mut self, value: u16) -> Self {
        self.options.max_rows = value;
        self.remeasure();
        self
    }

    #[must_use]
    pub fn auto_grow(mut self, value: bool) -> Self {
        self.options.auto_grow = value;
        self.remeasure();
        self
    }

    #[must_use]
    pub fn wrap_policy(mut self, value: TextAreaWrapPolicy) -> Self {
        self.options.wrap_policy = value;
        self
    }

    #[must_use]
    pub fn submit_key(mut self, value: TextAreaSubmitKey) -> Self {
        self.options.submit_key = value;
        self
    }

    #[must_use]
    pub fn newline_key(mut self, value: TextAreaNewlineKey) -> Self {
        self.options.newline_key = value;
        self
    }

    #[must_use]
    pub fn tab_behavior(mut self, value: TextAreaTabBehavior) -> Self {
        self.options.tab_behavior = value;
        self
    }

    #[must_use]
    pub fn ime_enabled(mut self, value: bool) -> Self {
        self.options.ime_enabled = value;
        self
    }

    #[must_use]
    pub fn leading_slot(mut self, label: impl Into<String>) -> Self {
        self.options.leading_slot = Some(UiSlotSpec::new(UiSlotPlacement::Leading, label));
        self
    }

    #[must_use]
    pub fn trailing_slot(mut self, label: impl Into<String>) -> Self {
        self.options.trailing_slot = Some(UiSlotSpec::new(UiSlotPlacement::Trailing, label));
        self
    }

    #[must_use]
    pub fn visual_role(mut self, value: UiVisualRole) -> Self {
        self.visual_role = value;
        self
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.common = value;
        self.options.disabled = self.common.disabled;
        self.state.disabled = self.common.disabled;
        self
    }

    pub fn validate(&self) -> Result<(), TextAreaValidationError> {
        self.options.validate()
    }
}
