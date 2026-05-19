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
