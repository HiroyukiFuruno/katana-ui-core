use crate::primitive::icon::{IconSize, IconSource};

/// Visual style of the button container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// No background, no border.
    #[default]
    Plain,
    /// Subtle background on hover only.
    Subtle,
    /// Always filled background.
    Filled,
}

/// Semantic intent / color tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Accent,
    Danger,
}

/// Properties for `SvgButton`.
#[derive(Debug, Clone)]
pub struct SvgButtonProps {
    pub icon: IconSource,
    pub size: IconSize,
    pub variant: Variant,
    pub tone: Tone,
    pub disabled: bool,
    pub loading: bool,
    /// Accessibility label — required for icon-only buttons.
    pub a11y_label: String,
}
