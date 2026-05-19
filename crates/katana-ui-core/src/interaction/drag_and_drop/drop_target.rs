use super::{
    AutoScrollPolicy, DndPoint, DndRect, DragData, DropEffect, DropIndicator, DropIndicatorKind,
    DropIndicatorOrientation,
};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

const BEFORE_THRESHOLD_RATIO: f32 = 0.25;
const AFTER_THRESHOLD_RATIO: f32 = 0.75;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DropTargetActions {
    pub on_enter: String,
    pub on_over: String,
    pub on_leave: String,
    pub on_drop: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropTarget {
    pub node_id: UiNodeId,
    pub accepted_tags: Vec<String>,
    pub effect: DropEffect,
    pub indicator_orientation: DropIndicatorOrientation,
    pub auto_scroll: AutoScrollPolicy,
    pub actions: DropTargetActions,
}

impl DropTarget {
    #[must_use]
    pub fn new(node_id: UiNodeId) -> Self {
        Self {
            node_id,
            accepted_tags: Vec::new(),
            effect: DropEffect::Move,
            indicator_orientation: DropIndicatorOrientation::Vertical,
            auto_scroll: AutoScrollPolicy::default(),
            actions: DropTargetActions::default(),
        }
    }

    #[must_use]
    pub fn accepted_tag(mut self, value: impl Into<String>) -> Self {
        self.accepted_tags.push(value.into());
        self
    }

    #[must_use]
    pub fn effect(mut self, value: DropEffect) -> Self {
        self.effect = value;
        self
    }

    #[must_use]
    pub fn auto_scroll(mut self, value: AutoScrollPolicy) -> Self {
        self.auto_scroll = value;
        self
    }

    #[must_use]
    pub fn accepts_data(&self, data: &DragData) -> bool {
        self.accepted_tags.iter().any(|tag| tag == &data.tag)
    }

    #[must_use]
    pub fn accept(&self, data: &DragData, position: DndPoint, rect: DndRect) -> DropAcceptance {
        if !self.accepts_data(data) || !rect.contains(position) {
            return DropAcceptance::Reject;
        }
        DropAcceptance::Accept {
            effect: self.effect,
            indicator: DropIndicator {
                kind: indicator_kind(position, rect, self.indicator_orientation),
                orientation: self.indicator_orientation,
                ..DropIndicator::new(DropIndicatorKind::Inside, rect)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DropAcceptance {
    Reject,
    Accept {
        effect: DropEffect,
        indicator: DropIndicator,
    },
}

impl DropAcceptance {
    #[must_use]
    pub fn indicator(&self) -> Option<&DropIndicator> {
        match self {
            Self::Reject => None,
            Self::Accept { indicator, .. } if indicator.kind == DropIndicatorKind::None => None,
            Self::Accept { indicator, .. } => Some(indicator),
        }
    }

    #[must_use]
    pub fn indicator_kind(&self) -> Option<DropIndicatorKind> {
        self.indicator().map(|indicator| indicator.kind)
    }

    #[must_use]
    pub const fn effect(&self) -> DropEffect {
        match self {
            Self::Reject => DropEffect::None,
            Self::Accept { effect, .. } => *effect,
        }
    }
}

fn indicator_kind(
    position: DndPoint,
    rect: DndRect,
    orientation: DropIndicatorOrientation,
) -> DropIndicatorKind {
    let ratio = match orientation {
        DropIndicatorOrientation::Vertical => rect.vertical_ratio(position),
        DropIndicatorOrientation::Horizontal => rect.horizontal_ratio(position),
    };
    if ratio <= BEFORE_THRESHOLD_RATIO {
        DropIndicatorKind::Before
    } else if ratio >= AFTER_THRESHOLD_RATIO {
        DropIndicatorKind::After
    } else {
        DropIndicatorKind::Inside
    }
}
