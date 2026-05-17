use crate::composite::button::text::{Size, Tone, Variant};
use crate::primitive::icon::{IconSize, IconSource};

/// Position of the icon relative to the label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconPosition {
    #[default]
    Leading,
    Trailing,
}

/// Properties for `IconTextButton`.
#[derive(Debug, Clone)]
pub struct IconTextButtonProps {
    pub icon: IconSource,
    pub label: String,
    pub icon_position: IconPosition,
    pub icon_size: IconSize,
    pub variant: Variant,
    pub tone: Tone,
    pub size: Size,
    pub disabled: bool,
    pub loading: bool,
}
