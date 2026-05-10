use crate::composite::input::text::InputSize;
use crate::primitive::icon::IconSource;
use std::rc::Rc;

/// Visibility behavior for a SearchBox icon slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchBoxIconMode {
    #[default]
    Hidden,
    Visible,
    Reserved,
}

/// Icon slot inside the SearchBox input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchBoxIconSlot {
    Leading,
    Clear,
    Submit,
}

/// Built-in SearchBox icon presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchBoxIconPreset {
    Search,
    Clear,
    Submit,
}

/// Icon configuration for a SearchBox slot.
#[derive(Debug, Clone)]
pub struct SearchBoxIconConfig {
    pub mode: SearchBoxIconMode,
    pub source: IconSource,
}

/// Properties for `SearchBox`.
#[derive(Clone)]
pub struct SearchBoxProps {
    pub value: String,
    pub placeholder: Option<String>,
    pub size: InputSize,
    pub disabled: bool,
    pub leading_icon: SearchBoxIconConfig,
    pub clear_icon: SearchBoxIconConfig,
    pub submit_icon: SearchBoxIconConfig,
    pub a11y_label: String,
    pub on_submit: Rc<dyn Fn(String)>,
}

/// Resolved visual properties for `SearchBox`.
pub type ResolvedSearchBox = crate::composite::input::text::ResolvedTextInput;

/// Builder for the SearchBox composite widget.
#[derive(Clone)]
pub struct SearchBox {
    pub(super) props: SearchBoxProps,
}
