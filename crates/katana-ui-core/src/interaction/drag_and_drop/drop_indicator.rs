use super::geometry::DndRect;
use crate::render_model::UiTone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropIndicatorKind {
    #[default]
    None,
    Before,
    After,
    Inside,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropIndicatorVisual {
    #[default]
    Line,
    Outline,
    Glow,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropIndicatorOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropIndicator {
    pub kind: DropIndicatorKind,
    pub visual: DropIndicatorVisual,
    pub orientation: DropIndicatorOrientation,
    pub tone: UiTone,
    pub anchor_rect: DndRect,
}

impl DropIndicator {
    #[must_use]
    pub const fn new(kind: DropIndicatorKind, anchor_rect: DndRect) -> Self {
        Self {
            kind,
            visual: DropIndicatorVisual::Line,
            orientation: DropIndicatorOrientation::Vertical,
            tone: UiTone::Accent,
            anchor_rect,
        }
    }

    #[must_use]
    pub const fn hidden(anchor_rect: DndRect) -> Self {
        Self::new(DropIndicatorKind::None, anchor_rect)
    }
}
