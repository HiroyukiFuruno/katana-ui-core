#[path = "shortcut_combo_parts/display.rs"]
mod display;
#[path = "shortcut_combo_parts/key_model.rs"]
mod key_model;

use crate::render_model::{UiNode, UiNodeKind, UiShortcutProps, UiSize, UiStateId, UiTone};
use display::{RenderPurpose, default_separator, sequence};
pub use key_model::{
    KeyCombo, KeyKind, KeyModifiers, NamedKey, RuntimePlatform, ShortcutPlatform, ShortcutSeparator,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCombo {
    label: String,
    state_id: UiStateId,
    combo: KeyCombo,
    separator: Option<ShortcutSeparator>,
    platform_display: ShortcutPlatform,
    size: UiSize,
    tone: UiTone,
    accessibility_label: Option<String>,
}

pub trait ShortcutPlatformProvider {
    fn runtime_platform(&self) -> RuntimePlatform;
}

impl ShortcutCombo {
    #[must_use]
    pub fn new(label: impl Into<String>, combo: KeyCombo) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::ShortcutCombo),
            combo,
            separator: None,
            platform_display: ShortcutPlatform::Auto,
            size: UiSize::Medium,
            tone: UiTone::Neutral,
            accessibility_label: None,
        }
    }

    #[must_use]
    pub fn platform_display(mut self, value: ShortcutPlatform) -> Self {
        self.platform_display = value;
        self
    }

    #[must_use]
    pub fn separator(mut self, value: ShortcutSeparator) -> Self {
        self.separator = Some(value);
        self
    }

    #[must_use]
    pub fn size(mut self, value: UiSize) -> Self {
        self.size = value;
        self
    }

    #[must_use]
    pub fn tone(mut self, value: UiTone) -> Self {
        self.tone = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn visual_text(&self, runtime: RuntimePlatform) -> String {
        sequence(
            self.combo.modifiers,
            self.combo.key,
            self.resolved_platform(runtime),
            RenderPurpose::Visual,
        )
        .join(
            self.separator
                .unwrap_or_else(|| default_separator(self.resolved_platform(runtime)))
                .as_str(),
        )
    }

    #[must_use]
    pub fn visual_text_with_provider(&self, provider: &impl ShortcutPlatformProvider) -> String {
        self.visual_text(provider.runtime_platform())
    }

    #[must_use]
    pub fn accessibility_text(&self, runtime: RuntimePlatform) -> String {
        self.accessibility_label.clone().unwrap_or_else(|| {
            sequence(
                self.combo.modifiers,
                self.combo.key,
                self.resolved_platform(runtime),
                RenderPurpose::Accessible,
            )
            .join(" + ")
        })
    }

    #[must_use]
    pub const fn combo(&self) -> &KeyCombo {
        &self.combo
    }

    fn resolved_platform(&self, runtime: RuntimePlatform) -> RuntimePlatform {
        match self.platform_display {
            ShortcutPlatform::Auto => runtime,
            ShortcutPlatform::MacOS => RuntimePlatform::MacOS,
            ShortcutPlatform::Windows => RuntimePlatform::Windows,
            ShortcutPlatform::Linux => RuntimePlatform::Linux,
        }
    }
}

impl From<ShortcutCombo> for UiNode {
    fn from(value: ShortcutCombo) -> Self {
        let combo = value.visual_text(RuntimePlatform::MacOS);
        let accessibility_label = value.accessibility_text(RuntimePlatform::MacOS);
        UiNode::from_state(UiNodeKind::ShortcutCombo, value.label, value.state_id)
            .shortcut(UiShortcutProps {
                platform: format!("{:?}", value.platform_display),
                combo,
            })
            .size(value.size)
            .tone(value.tone)
            .accessibility_label(accessibility_label)
    }
}
