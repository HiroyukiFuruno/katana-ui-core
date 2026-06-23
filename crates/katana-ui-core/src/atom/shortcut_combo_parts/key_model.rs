use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModifiers {
    pub command: bool,
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

impl KeyModifiers {
    #[must_use]
    pub const fn command_shift() -> Self {
        Self {
            command: true,
            shift: true,
            control: false,
            alt: false,
            meta: false,
        }
    }

    #[must_use]
    pub const fn control_shift() -> Self {
        Self {
            command: false,
            control: true,
            shift: true,
            alt: false,
            meta: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NamedKey {
    Enter,
    Escape,
    Tab,
    Space,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Plus,
    Minus,
    Function(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyKind {
    Char(char),
    Named(NamedKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyCombo {
    pub(super) modifiers: KeyModifiers,
    pub(super) key: KeyKind,
}

impl KeyCombo {
    #[must_use]
    pub const fn new(modifiers: KeyModifiers, key: KeyKind) -> Self {
        Self { modifiers, key }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutPlatform {
    #[default]
    Auto,
    MacOS,
    Windows,
    Linux,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimePlatform {
    MacOS,
    Windows,
    Linux,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutSeparator {
    Plus,
    Space,
    Arrow,
    None,
}

impl ShortcutSeparator {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Plus => "+",
            Self::Space => " ",
            Self::Arrow => " -> ",
            Self::None => "",
        }
    }
}
