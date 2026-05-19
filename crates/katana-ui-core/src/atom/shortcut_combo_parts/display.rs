use super::key_model::{KeyKind, KeyModifiers, NamedKey, RuntimePlatform, ShortcutSeparator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RenderPurpose {
    Visual,
    Accessible,
}

pub(super) fn default_separator(platform: RuntimePlatform) -> ShortcutSeparator {
    match platform {
        RuntimePlatform::MacOS => ShortcutSeparator::None,
        RuntimePlatform::Windows | RuntimePlatform::Linux => ShortcutSeparator::Plus,
    }
}

pub(super) fn sequence(
    modifiers: KeyModifiers,
    key: KeyKind,
    platform: RuntimePlatform,
    purpose: RenderPurpose,
) -> Vec<String> {
    let mut sequence = modifier_sequence(modifiers, platform, purpose);
    sequence.push(key_text(key, purpose));
    sequence
}

fn modifier_sequence(
    modifiers: KeyModifiers,
    platform: RuntimePlatform,
    purpose: RenderPurpose,
) -> Vec<String> {
    let mut sequence = Vec::new();
    push_modifier(
        &mut sequence,
        modifiers.command,
        platform,
        purpose,
        "command",
    );
    push_modifier(
        &mut sequence,
        modifiers.control,
        platform,
        purpose,
        "control",
    );
    push_modifier(&mut sequence, modifiers.alt, platform, purpose, "alt");
    push_modifier(&mut sequence, modifiers.shift, platform, purpose, "shift");
    push_modifier(&mut sequence, modifiers.meta, platform, purpose, "meta");
    sequence
}

fn push_modifier(
    sequence: &mut Vec<String>,
    enabled: bool,
    platform: RuntimePlatform,
    purpose: RenderPurpose,
    modifier: &str,
) {
    if enabled {
        sequence.push(modifier_text(platform, purpose, modifier));
    }
}

fn modifier_text(platform: RuntimePlatform, purpose: RenderPurpose, modifier: &str) -> String {
    match (platform, purpose, modifier) {
        (_, RenderPurpose::Accessible, "command") => "Command".into(),
        (_, RenderPurpose::Accessible, "control") => "Control".into(),
        (_, RenderPurpose::Accessible, "alt") => "Alt".into(),
        (_, RenderPurpose::Accessible, "shift") => "Shift".into(),
        (_, RenderPurpose::Accessible, "meta") => "Meta".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, "command") => "⌘".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, "control") => "⌃".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, "alt") => "⌥".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, "shift") => "⇧".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, "meta") => "⌘".into(),
        (RuntimePlatform::Windows, RenderPurpose::Visual, "command") => "Ctrl".into(),
        (RuntimePlatform::Windows, RenderPurpose::Visual, "meta") => "Win".into(),
        (RuntimePlatform::Linux, RenderPurpose::Visual, "command") => "Ctrl".into(),
        (RuntimePlatform::Linux, RenderPurpose::Visual, "meta") => "Super".into(),
        (_, RenderPurpose::Visual, "control") => "Ctrl".into(),
        (_, RenderPurpose::Visual, "alt") => "Alt".into(),
        (_, RenderPurpose::Visual, "shift") => "Shift".into(),
        _ => modifier.into(),
    }
}

fn key_text(key: KeyKind, purpose: RenderPurpose) -> String {
    match key {
        KeyKind::Char(value) => value.to_ascii_uppercase().to_string(),
        KeyKind::Named(value) => named_key_text(value, purpose),
    }
}

fn named_key_text(key: NamedKey, purpose: RenderPurpose) -> String {
    match (key, purpose) {
        (NamedKey::Escape, RenderPurpose::Visual) => "Esc".into(),
        (NamedKey::Escape, RenderPurpose::Accessible) => "Escape".into(),
        (NamedKey::ArrowUp, _) => "Arrow Up".into(),
        (NamedKey::ArrowDown, _) => "Arrow Down".into(),
        (NamedKey::ArrowLeft, _) => "Arrow Left".into(),
        (NamedKey::ArrowRight, _) => "Arrow Right".into(),
        (NamedKey::PageUp, _) => "Page Up".into(),
        (NamedKey::PageDown, _) => "Page Down".into(),
        (NamedKey::Function(value), _) => format!("F{}", value),
        (value, _) => format!("{value:?}"),
    }
}
