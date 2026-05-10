use crate::primitive::icon::IconSource;

/// Semantic color tone for Badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeTone {
    #[default]
    Neutral,
    Accent,
    Danger,
    Warning,
    Success,
    Info,
}

/// Visual style variant for Badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Solid,
    Subtle,
    Outline,
}

/// Size of Badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeSize {
    Sm,
    #[default]
    Md,
}

/// Properties for `Badge`.
#[derive(Debug, Clone)]
pub struct BadgeProps {
    pub label: String,
    pub tone: BadgeTone,
    pub variant: BadgeVariant,
    pub size: BadgeSize,
    pub leading_icon: Option<IconSource>,
}
