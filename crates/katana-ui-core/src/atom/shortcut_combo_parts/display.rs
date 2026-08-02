use super::key_model::{KeyKind, KeyModifiers, NamedKey, RuntimePlatform, ShortcutSeparator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RenderPurpose {
    Visual,
    Accessible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Modifier {
    Command,
    Control,
    Alt,
    Shift,
    Meta,
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
        Modifier::Command,
    );
    push_modifier(
        &mut sequence,
        modifiers.control,
        platform,
        purpose,
        Modifier::Control,
    );
    push_modifier(
        &mut sequence,
        modifiers.alt,
        platform,
        purpose,
        Modifier::Alt,
    );
    push_modifier(
        &mut sequence,
        modifiers.shift,
        platform,
        purpose,
        Modifier::Shift,
    );
    push_modifier(
        &mut sequence,
        modifiers.meta,
        platform,
        purpose,
        Modifier::Meta,
    );
    sequence
}

fn push_modifier(
    sequence: &mut Vec<String>,
    enabled: bool,
    platform: RuntimePlatform,
    purpose: RenderPurpose,
    modifier: Modifier,
) {
    if enabled {
        sequence.push(modifier_text(platform, purpose, modifier));
    }
}

fn modifier_text(platform: RuntimePlatform, purpose: RenderPurpose, modifier: Modifier) -> String {
    match (platform, purpose, modifier) {
        (_, RenderPurpose::Accessible, Modifier::Command) => "Command".into(),
        (_, RenderPurpose::Accessible, Modifier::Control) => "Control".into(),
        (_, RenderPurpose::Accessible, Modifier::Alt) => "Alt".into(),
        (_, RenderPurpose::Accessible, Modifier::Shift) => "Shift".into(),
        (_, RenderPurpose::Accessible, Modifier::Meta) => "Meta".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, Modifier::Command) => "⌘".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, Modifier::Control) => "⌃".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, Modifier::Alt) => "⌥".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, Modifier::Shift) => "⇧".into(),
        (RuntimePlatform::MacOS, RenderPurpose::Visual, Modifier::Meta) => "⌘".into(),
        (RuntimePlatform::Windows, RenderPurpose::Visual, Modifier::Command) => "Ctrl".into(),
        (RuntimePlatform::Windows, RenderPurpose::Visual, Modifier::Meta) => "Win".into(),
        (RuntimePlatform::Linux, RenderPurpose::Visual, Modifier::Command) => "Ctrl".into(),
        (RuntimePlatform::Linux, RenderPurpose::Visual, Modifier::Meta) => "Super".into(),
        (_, RenderPurpose::Visual, Modifier::Control) => "Ctrl".into(),
        (_, RenderPurpose::Visual, Modifier::Alt) => "Alt".into(),
        (_, RenderPurpose::Visual, Modifier::Shift) => "Shift".into(),
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
