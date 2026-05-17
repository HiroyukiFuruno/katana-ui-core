/// Visual style variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Link,
}

/// Semantic color tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Accent,
    Danger,
    Success,
}

/// Button size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Properties for `TextButton`.
#[derive(Debug, Clone)]
pub struct TextButtonProps {
    pub label: String,
    pub variant: Variant,
    pub tone: Tone,
    pub size: Size,
    pub disabled: bool,
    pub loading: bool,
}
