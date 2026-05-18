use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureTriggerArea {
    IconOnly,
    IconAndText,
    #[default]
    WholeElement,
    TextOnly,
}
