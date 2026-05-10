use crate::composite::input::text::InputSize;

/// Properties for `SearchBox`.
#[derive(Debug, Clone)]
pub struct SearchBoxProps {
    pub value: String,
    pub placeholder: Option<String>,
    pub size: InputSize,
    pub disabled: bool,
    pub a11y_label: String,
}
