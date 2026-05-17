use super::UiNodeId;
use crate::theme::ThemeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTreeDiff {
    pub changed: Vec<UiNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiCommand {
    pub target: UiNodeId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderContext {
    pub theme_id: ThemeId,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl RenderContext {
    #[must_use]
    pub fn new(theme_id: ThemeId, viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            theme_id,
            viewport_width,
            viewport_height,
        }
    }
}
