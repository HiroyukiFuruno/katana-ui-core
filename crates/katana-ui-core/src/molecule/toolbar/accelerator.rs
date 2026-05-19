use super::action_model::ToolbarAction;
use super::identifiers::ToolbarActionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyModifier {
    Command,
    Control,
    CommandOrControl,
    Shift,
    Alt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyCombo {
    key: String,
    modifiers: Vec<KeyModifier>,
}

impl KeyCombo {
    #[must_use]
    pub fn new(key: impl Into<String>, modifiers: Vec<KeyModifier>) -> Self {
        Self {
            key: normalize_key(key),
            modifiers,
        }
    }

    #[must_use]
    pub fn command_or_control(key: impl Into<String>) -> Self {
        Self::new(key, vec![KeyModifier::CommandOrControl])
    }

    #[must_use]
    pub fn matches_input(&self, input: &ToolbarKeyInput) -> bool {
        self.key == input.key
            && self.modifiers.len() == input.modifiers.len()
            && self
                .modifiers
                .iter()
                .all(|modifier| modifier_matches(*modifier, &input.modifiers))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarKeyInput {
    key: String,
    modifiers: Vec<KeyModifier>,
}

impl ToolbarKeyInput {
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: normalize_key(key),
            modifiers: Vec::new(),
        }
    }

    #[must_use]
    pub fn command_or_control(mut self) -> Self {
        self.modifiers.push(KeyModifier::CommandOrControl);
        self
    }

    #[must_use]
    pub fn with_modifier(mut self, value: KeyModifier) -> Self {
        self.modifiers.push(value);
        self
    }
}

pub(super) fn matching_action_id(
    actions: &[ToolbarAction],
    input: &ToolbarKeyInput,
) -> Option<(ToolbarActionId, KeyCombo)> {
    actions.iter().find_map(|action| {
        if action.split_state().primary_disabled() || action.disabled_model() {
            return None;
        }
        let combo = action.accelerator_model()?;
        combo
            .matches_input(input)
            .then(|| (action.id().clone(), combo.clone()))
    })
}

fn normalize_key(value: impl Into<String>) -> String {
    value.into().to_lowercase()
}

fn modifier_matches(required: KeyModifier, actual: &[KeyModifier]) -> bool {
    match required {
        KeyModifier::CommandOrControl => actual.iter().any(|it| {
            matches!(
                it,
                KeyModifier::CommandOrControl | KeyModifier::Command | KeyModifier::Control
            )
        }),
        value => actual.contains(&value),
    }
}
