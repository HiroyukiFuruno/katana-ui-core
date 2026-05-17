use std::rc::Rc;

/// Toggle switch size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `Toggle`.
#[derive(Clone)]
pub struct ToggleProps {
    pub value: bool,
    pub size: ToggleSize,
    pub disabled: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(bool)>,
}
