use super::typed_panel::UiRect;
use crate::interaction::drag_and_drop::{
    DndRect, DropIndicatorKind, DropIndicatorOrientation, DropIndicatorVisual,
};
use crate::render_model::{common_types::UiCursor, props::UiTone, tree::UiNode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiDragHandleProps {
    pub cursor_hint: UiCursor,
    pub accessibility_label: String,
}

impl Default for UiDragHandleProps {
    fn default() -> Self {
        Self {
            cursor_hint: UiCursor::Grab,
            accessibility_label: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiDropIndicatorProps {
    pub kind: DropIndicatorKind,
    pub visual: DropIndicatorVisual,
    pub orientation: DropIndicatorOrientation,
    pub tone: UiTone,
    pub anchor_rect: UiRect,
}

impl UiDropIndicatorProps {
    #[must_use]
    pub fn new(kind: DropIndicatorKind, anchor_rect: DndRect) -> Self {
        Self {
            kind,
            visual: DropIndicatorVisual::Line,
            orientation: DropIndicatorOrientation::Vertical,
            tone: UiTone::Accent,
            anchor_rect: rect_from_dnd(anchor_rect),
        }
    }
}

impl Default for UiDropIndicatorProps {
    fn default() -> Self {
        Self::new(DropIndicatorKind::None, DndRect::new(0.0, 0.0, 0.0, 0.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiDragPreviewProps {
    pub icon: String,
    pub count_badge: usize,
    pub opacity_percent: u8,
}

impl Default for UiDragPreviewProps {
    fn default() -> Self {
        Self {
            icon: String::new(),
            count_badge: 0,
            opacity_percent: 88,
        }
    }
}

impl UiNode {
    #[must_use]
    pub fn drag_handle(mut self, value: UiDragHandleProps) -> Self {
        self.props.drag_handle = value;
        self
    }

    #[must_use]
    pub fn drop_indicator(mut self, value: UiDropIndicatorProps) -> Self {
        self.props.drop_indicator = value;
        self
    }

    #[must_use]
    pub fn drag_preview(mut self, value: UiDragPreviewProps) -> Self {
        self.props.drag_preview = value;
        self
    }
}

fn rect_from_dnd(rect: DndRect) -> UiRect {
    UiRect::new(
        rect.x.round() as i32,
        rect.y.round() as i32,
        rect.width.round().max(0.0) as u32,
        rect.height.round().max(0.0) as u32,
    )
}
