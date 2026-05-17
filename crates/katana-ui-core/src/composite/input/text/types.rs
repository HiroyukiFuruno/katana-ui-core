use crate::primitive::icon::IconSource;
use std::rc::Rc;

/// Content for the trailing slot.
#[derive(Debug, Clone, Default)]
pub enum TrailingSlot {
    #[default]
    None,
    Reserved,
    ClearButton,
    Custom(IconSource),
    Spinner,
}

/// Visibility behavior for an input icon slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconSlotMode {
    #[default]
    Hidden,
    Visible,
    Reserved,
}

/// Input size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `TextInput`.
#[derive(Clone)]
pub struct TextInputProps {
    pub value: String,
    pub placeholder: Option<String>,
    pub leading_icon: Option<IconSource>,
    pub leading_icon_mode: IconSlotMode,
    pub trailing: TrailingSlot,
    pub size: InputSize,
    pub disabled: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(String)>,
}
