use super::state::AtomState;
use crate::interaction::drag_and_drop::{
    DndRect, DropIndicatorKind, DropIndicatorOrientation, DropIndicatorVisual,
};
use crate::render_model::{UiCommonProps, UiDropIndicatorProps, UiNode, UiNodeKind, UiTone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DropIndicator {
    label: String,
    state: AtomState,
    props: UiDropIndicatorProps,
}

impl DropIndicator {
    #[must_use]
    pub fn new(kind: DropIndicatorKind, anchor_rect: DndRect) -> Self {
        Self {
            label: "drop indicator".to_string(),
            state: AtomState::enabled(UiNodeKind::DropIndicator),
            props: UiDropIndicatorProps::new(kind, anchor_rect),
        }
    }

    #[must_use]
    pub fn visual(mut self, value: DropIndicatorVisual) -> Self {
        self.props.visual = value;
        self
    }

    #[must_use]
    pub fn orientation(mut self, value: DropIndicatorOrientation) -> Self {
        self.props.orientation = value;
        self
    }

    #[must_use]
    pub fn tone(mut self, value: UiTone) -> Self {
        self.props.tone = value;
        self.state.tone = value;
        self
    }

    #[must_use]
    pub fn visible(mut self, value: bool) -> Self {
        self.state.common.visible = value;
        self
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.state.disabled = value.disabled;
        self.state.focusable = value.focusable;
        self.state.accessibility_label = value.accessibility_label.clone();
        self.state.common = value;
        self
    }
}

impl From<DropIndicator> for UiNode {
    fn from(value: DropIndicator) -> Self {
        value
            .state
            .node(UiNodeKind::DropIndicator, value.label)
            .drop_indicator(value.props)
    }
}
