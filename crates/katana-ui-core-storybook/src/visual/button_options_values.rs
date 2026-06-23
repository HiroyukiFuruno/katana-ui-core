#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonWidthMode {
    Auto,
    Px,
    Percent,
    Fill,
}

impl StorybookButtonWidthMode {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Auto => Self::Px,
            Self::Px => Self::Percent,
            Self::Percent => Self::Fill,
            Self::Fill => Self::Auto,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Px => "160px",
            Self::Percent => "72%",
            Self::Fill => "fill",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonHeightMode {
    Auto,
    Compact,
    Tall,
}

impl StorybookButtonHeightMode {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Auto => Self::Compact,
            Self::Compact => Self::Tall,
            Self::Tall => Self::Auto,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Compact => "28px",
            Self::Tall => "48px",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonTabIndex {
    Zero,
    One,
    Disabled,
}

impl StorybookButtonTabIndex {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Disabled,
            Self::Disabled => Self::Zero,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::Disabled => "-1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonZIndex {
    Auto,
    Raised,
    Overlay,
}

impl StorybookButtonZIndex {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Auto => Self::Raised,
            Self::Raised => Self::Overlay,
            Self::Overlay => Self::Auto,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Raised => "10",
            Self::Overlay => "100",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonCommandMode {
    Save,
    Open,
}

impl StorybookButtonCommandMode {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Save => Self::Open,
            Self::Open => Self::Save,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Save => "save",
            Self::Open => "open",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonIconPosition {
    Leading,
    Trailing,
}

impl StorybookButtonIconPosition {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Leading => Self::Trailing,
            Self::Trailing => Self::Leading,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Leading => "leading",
            Self::Trailing => "trailing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonLayoutPreset {
    Page,
    Dense,
}

impl StorybookButtonLayoutPreset {
    pub(in crate::visual) const fn next(self) -> Self {
        match self {
            Self::Page => Self::Dense,
            Self::Dense => Self::Page,
        }
    }

    pub(in crate::visual) const fn label(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::Dense => "dense",
        }
    }

    pub(in crate::visual) const fn preset_index(self, fallback: usize) -> usize {
        const DENSE_PRESET_INDEX: usize = 3;
        match self {
            Self::Page => fallback,
            Self::Dense => DENSE_PRESET_INDEX,
        }
    }
}
