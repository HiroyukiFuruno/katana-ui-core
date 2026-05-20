use crate::render_model::{UiCommonProps, UiDragPreviewProps, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragPreview {
    label: String,
    state_id: UiStateId,
    common: UiCommonProps,
    props: UiDragPreviewProps,
}

impl DragPreview {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::DragPreview),
            common: UiCommonProps::default(),
            props: UiDragPreviewProps::default(),
        }
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.props.icon = value.into();
        self
    }

    #[must_use]
    pub fn count_badge(mut self, value: usize) -> Self {
        self.props.count_badge = value;
        self
    }

    #[must_use]
    pub fn opacity_percent(mut self, value: u8) -> Self {
        self.props.opacity_percent = value.min(100);
        self
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.common = value;
        self
    }
}

impl From<DragPreview> for UiNode {
    fn from(value: DragPreview) -> Self {
        UiNode::from_state(UiNodeKind::DragPreview, value.label, value.state_id)
            .common(value.common)
            .drag_preview(value.props)
    }
}
