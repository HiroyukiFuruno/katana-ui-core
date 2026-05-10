mod ops;
mod types;
mod view;

pub use types::{KeyCapProps, KeyCapSize, KeyCapTone, KeyComboProps, KeyLabel, NamedKey};

use crate::theme::Theme;
use crate::theme::color::Color;
use ops::key_display;
use view::{bg_color, border_color, font_size, padding, text_color};

/// Resolved visual properties for a single KeyCap.
#[derive(Debug, Clone)]
pub struct ResolvedKeyCap {
    pub display: String,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub bg_color: Color,
    pub text_color: Color,
    pub border_color: Color,
}

/// Resolved visual properties for KeyCombo.
#[derive(Debug, Clone)]
pub struct ResolvedKeyCombo {
    pub caps: Vec<ResolvedKeyCap>,
}

/// Builder for a single KeyCap.
#[derive(Debug, Clone)]
pub struct KeyCap {
    props: KeyCapProps,
}

impl KeyCap {
    #[must_use]
    pub fn new(key: KeyLabel) -> Self {
        Self {
            props: KeyCapProps {
                key,
                size: KeyCapSize::default(),
                tone: KeyCapTone::default(),
            },
        }
    }

    #[must_use]
    pub fn size(mut self, size: KeyCapSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn tone(mut self, tone: KeyCapTone) -> Self {
        self.props.tone = tone;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedKeyCap {
        let is_mac = cfg!(target_os = "macos");
        Self::resolve_key(
            &self.props.key,
            self.props.size,
            self.props.tone,
            theme,
            is_mac,
        )
    }

    fn resolve_key(
        key: &KeyLabel,
        size: KeyCapSize,
        tone: KeyCapTone,
        theme: &Theme,
        is_mac: bool,
    ) -> ResolvedKeyCap {
        ResolvedKeyCap {
            display: key_display(key, is_mac),
            font_size: font_size(size),
            pad_v: padding(size).0,
            pad_h: padding(size).1,
            bg_color: bg_color(tone, theme),
            text_color: text_color(theme),
            border_color: border_color(theme),
        }
    }
}

/// Builder for KeyCombo (sequence of keys).
#[derive(Debug, Clone)]
pub struct KeyCombo {
    props: KeyComboProps,
}

impl KeyCombo {
    #[must_use]
    pub fn new(keys: Vec<KeyLabel>) -> Self {
        Self {
            props: KeyComboProps {
                keys,
                size: KeyCapSize::default(),
                tone: KeyCapTone::default(),
            },
        }
    }

    #[must_use]
    pub fn size(mut self, size: KeyCapSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn tone(mut self, tone: KeyCapTone) -> Self {
        self.props.tone = tone;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedKeyCombo {
        let is_mac = cfg!(target_os = "macos");
        let caps = self
            .props
            .keys
            .iter()
            .map(|key| KeyCap::resolve_key(key, self.props.size, self.props.tone, theme, is_mac))
            .collect();
        ResolvedKeyCombo { caps }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn cmd_display_mac_vs_non_mac() {
        assert_eq!(ops::key_display(&KeyLabel::Cmd, true), "⌘");
        assert_eq!(ops::key_display(&KeyLabel::Cmd, false), "Ctrl");
    }

    #[test]
    fn shift_display_mac_vs_non_mac() {
        assert_eq!(ops::key_display(&KeyLabel::Shift, true), "⇧");
        assert_eq!(ops::key_display(&KeyLabel::Shift, false), "Shift");
    }

    #[test]
    fn char_key_uppercased() {
        assert_eq!(ops::key_display(&KeyLabel::Char('p'), false), "P");
    }

    #[test]
    fn named_f1() {
        assert_eq!(
            ops::key_display(&KeyLabel::Named(NamedKey::F1), false),
            "F1"
        );
    }

    #[test]
    fn key_combo_resolves_all_caps() {
        let theme = Theme::default_light();
        let combo = KeyCombo::new(vec![KeyLabel::Cmd, KeyLabel::Shift, KeyLabel::Char('p')]);
        let r = combo.resolve(&theme);
        assert_eq!(r.caps.len(), 3);
    }
}
