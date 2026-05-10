/// Toggle switch size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `Toggle`.
#[derive(Debug, Clone)]
pub struct ToggleProps {
    pub value: bool,
    pub size: ToggleSize,
    pub disabled: bool,
    pub a11y_label: String,
}
