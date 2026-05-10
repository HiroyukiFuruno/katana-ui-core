use super::types::{KeyLabel, NamedKey};

pub(super) fn key_display(key: &KeyLabel, is_mac: bool) -> String {
    match key {
        KeyLabel::Cmd => {
            if is_mac {
                "⌘".into()
            } else {
                "Ctrl".into()
            }
        }
        KeyLabel::Ctrl => {
            if is_mac {
                "⌃".into()
            } else {
                "Ctrl".into()
            }
        }
        KeyLabel::Shift => {
            if is_mac {
                "⇧".into()
            } else {
                "Shift".into()
            }
        }
        KeyLabel::Alt => {
            if is_mac {
                "⌥".into()
            } else {
                "Alt".into()
            }
        }
        KeyLabel::Option => {
            if is_mac {
                "⌥".into()
            } else {
                "Alt".into()
            }
        }
        KeyLabel::Super => {
            if is_mac {
                "⌘".into()
            } else {
                "Super".into()
            }
        }
        KeyLabel::Char(c) => c.to_uppercase().collect(),
        KeyLabel::Named(n) => named_key_display(n),
    }
}

fn named_key_display(n: &NamedKey) -> String {
    match n {
        NamedKey::Enter => "↩".into(),
        NamedKey::Escape => "Esc".into(),
        NamedKey::Tab => "⇥".into(),
        NamedKey::Space => "Space".into(),
        NamedKey::Backspace => "⌫".into(),
        NamedKey::Delete => "Del".into(),
        NamedKey::ArrowUp => "↑".into(),
        NamedKey::ArrowDown => "↓".into(),
        NamedKey::ArrowLeft => "←".into(),
        NamedKey::ArrowRight => "→".into(),
        NamedKey::F1 => "F1".into(),
        NamedKey::F2 => "F2".into(),
        NamedKey::F3 => "F3".into(),
        NamedKey::F4 => "F4".into(),
        NamedKey::F5 => "F5".into(),
        NamedKey::F6 => "F6".into(),
        NamedKey::F7 => "F7".into(),
        NamedKey::F8 => "F8".into(),
        NamedKey::F9 => "F9".into(),
        NamedKey::F10 => "F10".into(),
        NamedKey::F11 => "F11".into(),
        NamedKey::F12 => "F12".into(),
    }
}
