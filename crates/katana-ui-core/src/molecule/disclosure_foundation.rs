use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureTriggerArea {
    IconOnly,
    IconAndText,
    #[default]
    WholeElement,
    TextOnly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureIndicatorPosition {
    Leading,
    #[default]
    Trailing,
    None,
}

impl From<DisclosureIndicatorPosition> for String {
    fn from(value: DisclosureIndicatorPosition) -> Self {
        match value {
            DisclosureIndicatorPosition::Leading => "leading",
            DisclosureIndicatorPosition::Trailing => "trailing",
            DisclosureIndicatorPosition::None => "none",
        }
        .to_string()
    }
}
