mod types;
mod view;

pub use types::StatusSeverity;

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use std::rc::Rc;
use types::StatusBarProps;

const DEFAULT_GAP: f32 = crate::floem_view::GAP_SM;
const DEFAULT_PADDING: f32 = crate::floem_view::GAP_SM;
const DEFAULT_HEIGHT: f32 = 38.0;
const ERROR_WARNING_SUCCESS_ALPHA: u8 = 30;
const INFO_ALPHA: u8 = 24;
#[cfg(test)]
const TEST_HEIGHT: f32 = 50.0;
#[cfg(test)]
const TEST_PADDING: f32 = 12.0;
#[cfg(test)]
const TEST_GAP: f32 = 4.0;

pub struct StatusBar {
    props: types::StatusBarProps,
}

/// Resolved `StatusBar` values, excluding slot views.
#[derive(Debug, Clone)]
pub struct ResolvedStatusBar {
    pub severity: StatusSeverity,
    pub icon: &'static str,
    pub icon_color: Color,
    pub bar_color: Color,
    pub message: String,
    pub text_color: Color,
    pub height: f32,
    pub padding: f32,
    pub gap: f32,
    pub action_label: Option<String>,
}

fn noop_action() {}

impl StatusBar {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            props: StatusBarProps {
                message: message.into(),
                severity: StatusSeverity::Info,
                trailing: None,
                action_label: None,
                on_action: Rc::new(noop_action),
                height: None,
                padding: None,
                gap: None,
            },
        }
    }

    /// Sets a severity.
    pub fn severity(mut self, severity: StatusSeverity) -> Self {
        self.props.severity = severity;
        self
    }

    /// Sets optional trailing content.
    pub fn trailing<V>(mut self, trailing: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.trailing = Some(trailing.into_any());
        self
    }

    /// Sets optional action label shown as the built-in action button.
    pub fn action_label(mut self, label: impl Into<String>) -> Self {
        self.props.action_label = Some(label.into());
        self
    }

    /// Sets action callback for the built-in action button.
    pub fn on_action(mut self, on_action: impl Fn() + 'static) -> Self {
        self.props.on_action = Rc::new(on_action);
        self
    }

    /// Sets explicit bar height.
    pub fn height(mut self, height: f32) -> Self {
        self.props.height = Some(height);
        self
    }

    /// Sets internal horizontal / vertical padding.
    pub fn padding(mut self, padding: f32) -> Self {
        self.props.padding = Some(padding);
        self
    }

    /// Sets internal gap.
    pub fn gap(mut self, gap: f32) -> Self {
        self.props.gap = Some(gap);
        self
    }

    /// Applies theme-resolved state.
    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedStatusBar {
        let icon_color = icon_color(self.props.severity, theme);
        let bar_color = match self.props.severity {
            StatusSeverity::Error => Color {
                a: ERROR_WARNING_SUCCESS_ALPHA,
                ..theme.color.danger
            },
            StatusSeverity::Warning => Color {
                a: ERROR_WARNING_SUCCESS_ALPHA,
                ..theme.color.warning
            },
            StatusSeverity::Success => Color {
                a: ERROR_WARNING_SUCCESS_ALPHA,
                ..theme.color.success
            },
            StatusSeverity::Info => Color {
                a: INFO_ALPHA,
                ..theme.color.accent
            },
        };

        ResolvedStatusBar {
            severity: self.props.severity,
            icon: status_icon(self.props.severity),
            icon_color,
            bar_color,
            message: self.props.message.clone(),
            text_color: theme.color.text,
            height: self.props.height.unwrap_or(DEFAULT_HEIGHT),
            padding: self.props.padding.unwrap_or(DEFAULT_PADDING),
            gap: self.props.gap.unwrap_or(DEFAULT_GAP),
            action_label: self.props.action_label.clone(),
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new("")
    }
}

fn status_icon(severity: StatusSeverity) -> &'static str {
    match severity {
        StatusSeverity::Error => "⚠",
        StatusSeverity::Warning => "⭑",
        StatusSeverity::Success => "✓",
        StatusSeverity::Info => "ⓘ",
    }
}

fn icon_color(severity: StatusSeverity, theme: &Theme) -> Color {
    match severity {
        StatusSeverity::Error => theme.color.danger,
        StatusSeverity::Warning => theme.color.warning,
        StatusSeverity::Success => theme.color.success,
        StatusSeverity::Info => theme.color.accent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use floem::views::label;

    #[test]
    fn default_severity_is_info() {
        let bar = StatusBar::new("done");
        let theme = Theme::default_light();
        let resolved = bar.resolve(&theme);
        assert_eq!(resolved.severity, StatusSeverity::Info);
    }

    #[test]
    fn can_customize_layout_values() {
        let theme = Theme::default_light();
        let resolved = StatusBar::new("save")
            .on_action(|| {})
            .height(TEST_HEIGHT)
            .padding(TEST_PADDING)
            .gap(TEST_GAP)
            .resolve(&theme);
        assert_eq!(resolved.height, TEST_HEIGHT);
        assert_eq!(resolved.padding, TEST_PADDING);
        assert_eq!(resolved.gap, TEST_GAP);
    }

    #[test]
    fn can_set_trailing_without_panic() {
        let theme = Theme::default_light();
        let _ = StatusBar::new("with trailing")
            .trailing(label(|| "dummy"))
            .resolve(&theme);
    }
}
