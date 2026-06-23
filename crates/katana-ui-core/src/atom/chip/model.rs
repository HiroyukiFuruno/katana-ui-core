use super::{ChipEvent, ChipSize, ChipTone, ChipVariant};
use crate::render_model::{
    UiInteractionState, UiNode, UiNodeKind, UiSize, UiStateId, UiTone, UiVariant,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static CHIP_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chip {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) leading_icon: Option<String>,
    pub(super) trailing_icon: Option<String>,
    pub(super) tone: ChipTone,
    pub(super) variant: ChipVariant,
    pub(super) size: ChipSize,
    pub(super) interactive: bool,
    pub(super) selected: bool,
    pub(super) disabled: bool,
    pub(super) dismissible: bool,
    pub(super) accessibility_label: String,
    pub(super) focused: bool,
    pub(super) callback_log: Vec<ChipEvent>,
}

impl Chip {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: next_chip_state_id(),
            leading_icon: None,
            trailing_icon: None,
            tone: ChipTone::Neutral,
            variant: ChipVariant::Soft,
            size: ChipSize::Default,
            interactive: false,
            selected: false,
            disabled: false,
            dismissible: false,
            accessibility_label: String::new(),
            focused: false,
            callback_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn leading_icon(mut self, value: impl Into<String>) -> Self {
        self.leading_icon = Some(value.into());
        self
    }

    #[must_use]
    pub fn trailing_icon(mut self, value: impl Into<String>) -> Self {
        self.trailing_icon = Some(value.into());
        self
    }

    #[must_use]
    pub fn tone(mut self, value: ChipTone) -> Self {
        self.tone = value;
        self
    }

    #[must_use]
    pub fn variant(mut self, value: ChipVariant) -> Self {
        self.variant = value;
        self
    }

    #[must_use]
    pub fn size(mut self, value: ChipSize) -> Self {
        self.size = value;
        self
    }

    #[must_use]
    pub fn interactive(mut self, value: bool) -> Self {
        self.interactive = value;
        self
    }

    #[must_use]
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    #[must_use]
    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn focused(mut self, value: bool) -> Self {
        self.focused = value;
        self
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn leading_icon_value(&self) -> Option<&str> {
        self.leading_icon.as_deref()
    }

    #[must_use]
    pub fn trailing_icon_value(&self) -> Option<&str> {
        self.trailing_icon.as_deref()
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub const fn interactive_value(&self) -> bool {
        self.interactive
    }

    #[must_use]
    pub const fn selected_value(&self) -> bool {
        self.selected
    }

    #[must_use]
    pub const fn focused_value(&self) -> bool {
        self.focused
    }

    #[must_use]
    pub fn accessibility_label_value(&self) -> &str {
        &self.accessibility_label
    }

    #[must_use]
    pub fn callback_log(&self) -> &[ChipEvent] {
        &self.callback_log
    }

    #[must_use]
    pub fn theme_token_key(&self) -> String {
        format!(
            "chip.{}.{}",
            self.variant.token_name(),
            self.tone.token_name()
        )
    }

    #[must_use]
    pub fn interaction_state(&self) -> UiInteractionState {
        UiInteractionState {
            has_selection: self.selected,
            focused: self.focused,
            item_count: 1,
            value: self.label.clone(),
            ..UiInteractionState::default()
        }
    }
}

impl From<Chip> for UiNode {
    fn from(value: Chip) -> Self {
        let interaction = value.interaction_state();
        let tone = value.tone;
        let variant = value.variant;
        let size = value.size;
        let disabled = value.disabled;
        let focusable = value.interactive || value.dismissible;
        let selected = value.selected;
        UiNode::from_state(UiNodeKind::Chip, value.label, value.state_id)
            .tone(tone.into())
            .variant(variant.into())
            .size(size.into())
            .disabled(disabled)
            .focusable(focusable)
            .accessibility_label(value.accessibility_label)
            .checked(selected)
            .interaction(interaction)
    }
}

fn next_chip_state_id() -> UiStateId {
    let sequence = CHIP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    UiStateId::new(format!("state:Chip:{sequence}"))
}

impl From<ChipTone> for UiTone {
    fn from(value: ChipTone) -> Self {
        match value {
            ChipTone::Neutral | ChipTone::Muted => Self::Neutral,
            ChipTone::Accent => Self::Accent,
            ChipTone::Success => Self::Success,
            ChipTone::Warning => Self::Warning,
            ChipTone::Danger => Self::Danger,
        }
    }
}

impl From<ChipVariant> for UiVariant {
    fn from(value: ChipVariant) -> Self {
        match value {
            ChipVariant::Solid => Self::Filled,
            ChipVariant::Soft => Self::Plain,
            ChipVariant::Outline => Self::Outline,
            ChipVariant::Ghost => Self::Text,
        }
    }
}

impl From<ChipSize> for UiSize {
    fn from(value: ChipSize) -> Self {
        match value {
            ChipSize::Compact => Self::Small,
            ChipSize::Default => Self::Medium,
            ChipSize::Large => Self::Large,
        }
    }
}
