use super::state::AtomState;
use crate::render_model::{
    UiCommonProps, UiCursor, UiDragHandleProps, UiNode, UiNodeKind, UiStateId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragHandle {
    label: String,
    state: AtomState,
    props: UiDragHandleProps,
}

impl DragHandle {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let props = UiDragHandleProps {
            accessibility_label: label.clone(),
            ..UiDragHandleProps::default()
        };
        Self {
            label,
            state: AtomState::enabled(UiNodeKind::DragHandle),
            props,
        }
        .focusable(true)
        .cursor_hint(UiCursor::Grab)
    }

    #[must_use]
    pub fn cursor_hint(mut self, value: UiCursor) -> Self {
        self.props.cursor_hint = value;
        self.state.common.cursor = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        let label = value.into();
        self.props.accessibility_label = label.clone();
        self.state.accessibility_label = label.clone();
        self.state.common.accessibility_label = label;
        self
    }

    #[must_use]
    pub fn focusable(mut self, value: bool) -> Self {
        self.state.focusable = value;
        self.state.common.focusable = value;
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

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }
}

impl From<DragHandle> for UiNode {
    fn from(value: DragHandle) -> Self {
        value
            .state
            .node(UiNodeKind::DragHandle, value.label)
            .drag_handle(value.props)
    }
}
