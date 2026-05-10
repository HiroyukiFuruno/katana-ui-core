pub mod color;
pub mod spacing;
pub mod typography;

pub use color::ColorTokens;
pub use spacing::SpacingTokens;
pub use typography::TypographyTokens;

use floem_reactive::{provide_context, use_context};

/// All design tokens bundled together.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub color: ColorTokens,
    pub spacing: SpacingTokens,
    pub typography: TypographyTokens,
}

impl Theme {
    #[must_use]
    pub fn default_light() -> Self {
        Self {
            color: ColorTokens::light(),
            spacing: SpacingTokens::default(),
            typography: TypographyTokens::default(),
        }
    }

    #[must_use]
    pub fn default_dark() -> Self {
        Self {
            color: ColorTokens::dark(),
            spacing: SpacingTokens::default(),
            typography: TypographyTokens::default(),
        }
    }

    /// Injects this theme into the Floem reactive context.
    pub fn provide(self) {
        provide_context(self);
    }

    /// Retrieves the current `Theme` from the Floem reactive context.
    /// Falls back to `Theme::default_light()` if not injected.
    #[must_use]
    pub fn current() -> Self {
        use_context::<Self>().unwrap_or_else(Self::default_light)
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;

    #[test]
    fn light_and_dark_color_tokens_differ() {
        let light = Theme::default_light();
        let dark = Theme::default_dark();
        assert_ne!(
            light.color, dark.color,
            "light/dark color tokens must differ"
        );
    }

    #[test]
    fn current_returns_default_when_no_context() {
        let theme = Theme::current();
        assert_eq!(theme.color, Theme::default_light().color);
    }

    #[test]
    fn current_returns_provided_theme() {
        let dark = Theme::default_dark();
        dark.clone().provide();
        let got = Theme::current();
        assert_eq!(got.color, dark.color);
    }
}
