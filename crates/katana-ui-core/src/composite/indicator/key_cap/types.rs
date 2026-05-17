/// Named special keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedKey {
    Enter,
    Escape,
    Tab,
    Space,
    Backspace,
    Delete,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

/// A single key label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyLabel {
    Cmd,
    Ctrl,
    Shift,
    Alt,
    Option,
    Super,
    Char(char),
    Named(NamedKey),
}

/// Size of KeyCap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyCapSize {
    Sm,
    #[default]
    Md,
}

/// Tone of KeyCap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyCapTone {
    #[default]
    Neutral,
    Subtle,
}

/// Properties for a single `KeyCap`.
#[derive(Debug, Clone)]
pub struct KeyCapProps {
    pub key: KeyLabel,
    pub size: KeyCapSize,
    pub tone: KeyCapTone,
}

/// Properties for `KeyCombo`.
#[derive(Debug, Clone)]
pub struct KeyComboProps {
    pub keys: Vec<KeyLabel>,
    pub size: KeyCapSize,
    pub tone: KeyCapTone,
}
