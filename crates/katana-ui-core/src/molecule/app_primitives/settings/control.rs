use crate::render_model::UiNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsControlKind {
    Toggle,
    Select,
    Combo,
    Input,
    TextArea,
    Number,
    Chips,
    Radio,
    ColorPicker,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsDirtyVisualization {
    None,
    Marker,
    Highlight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsListDensity {
    Compact,
    Default,
    Spacious,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsValue {
    Bool(bool),
    Text(String),
    Number(i64),
    Color {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    },
    List(Vec<String>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsControlOption {
    pub value: String,
    pub label: String,
}

impl SettingsControlOption {
    #[must_use]
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsControl {
    Toggle {
        checked: bool,
    },
    Select {
        options: Vec<SettingsControlOption>,
        selected: String,
    },
    Combo {
        options: Vec<SettingsControlOption>,
        query: String,
        selected: Option<String>,
    },
    Input {
        value: String,
    },
    TextArea {
        value: String,
    },
    Number {
        value: i64,
        min: i64,
        max: i64,
    },
    Chips {
        values: Vec<String>,
    },
    Radio {
        options: Vec<SettingsControlOption>,
        selected: String,
    },
    ColorPicker {
        color: SettingsValue,
    },
    Custom(Box<UiNode>),
}

impl SettingsControl {
    #[must_use]
    pub const fn kind(&self) -> SettingsControlKind {
        match self {
            Self::Toggle { .. } => SettingsControlKind::Toggle,
            Self::Select { .. } => SettingsControlKind::Select,
            Self::Combo { .. } => SettingsControlKind::Combo,
            Self::Input { .. } => SettingsControlKind::Input,
            Self::TextArea { .. } => SettingsControlKind::TextArea,
            Self::Number { .. } => SettingsControlKind::Number,
            Self::Chips { .. } => SettingsControlKind::Chips,
            Self::Radio { .. } => SettingsControlKind::Radio,
            Self::ColorPicker { .. } => SettingsControlKind::ColorPicker,
            Self::Custom(_) => SettingsControlKind::Custom,
        }
    }

    #[must_use]
    pub fn custom(node: impl Into<UiNode>) -> Self {
        Self::Custom(Box::new(node.into()))
    }

    #[must_use]
    pub fn value(&self) -> SettingsValue {
        match self {
            Self::Toggle { checked } => SettingsValue::Bool(*checked),
            Self::Select { selected, .. }
            | Self::Input { value: selected }
            | Self::TextArea { value: selected } => SettingsValue::Text(selected.clone()),
            Self::Combo {
                selected, query, ..
            } => selected
                .clone()
                .map(SettingsValue::Text)
                .unwrap_or_else(|| SettingsValue::Text(query.clone())),
            Self::Number { value, .. } => SettingsValue::Number(*value),
            Self::Chips { values } => SettingsValue::List(values.clone()),
            Self::Radio { selected, .. } => SettingsValue::Text(selected.clone()),
            Self::ColorPicker { color } => color.clone(),
            Self::Custom(_) => SettingsValue::None,
        }
    }

    pub fn set_value(&mut self, value: SettingsValue) -> bool {
        match (self, value) {
            (Self::Toggle { checked }, SettingsValue::Bool(value)) => *checked = value,
            (Self::Select { selected, .. }, SettingsValue::Text(value))
            | (Self::Input { value: selected }, SettingsValue::Text(value))
            | (Self::TextArea { value: selected }, SettingsValue::Text(value))
            | (Self::Radio { selected, .. }, SettingsValue::Text(value)) => *selected = value,
            (Self::Combo { selected, .. }, SettingsValue::Text(value)) => *selected = Some(value),
            (Self::Number { value, min, max }, SettingsValue::Number(next)) => {
                *value = next.clamp(*min, *max);
            }
            (Self::Chips { values }, SettingsValue::List(next)) => *values = next,
            (Self::ColorPicker { color }, value @ SettingsValue::Color { .. }) => *color = value,
            _ => return false,
        }
        true
    }
}
