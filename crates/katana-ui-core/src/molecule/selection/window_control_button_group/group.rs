use super::{
    WindowControlButtonGroupAction, WindowControlButtonGroupEvent, WindowControlButtonGroupOptions,
    WindowControlButtonGroupState, WindowControlKind, WindowControlSize,
};
use crate::render_model::{UiCursor, UiDimension, UiNode, UiNodeKind, UiVariant, UiVisualRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowControlButtonGroup {
    label: String,
    options: WindowControlButtonGroupOptions,
    state: WindowControlButtonGroupState,
}

impl WindowControlButtonGroup {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let options = WindowControlButtonGroupOptions::default();
        Self {
            label: label.into(),
            state: WindowControlButtonGroupState::new(&options),
            options,
        }
    }

    #[must_use]
    pub fn options(mut self, options: WindowControlButtonGroupOptions) -> Self {
        self.state.sync_options(&options);
        self.options = options;
        self
    }

    pub fn apply_action(
        &mut self,
        action: WindowControlButtonGroupAction,
    ) -> Vec<WindowControlButtonGroupEvent> {
        self.state.apply(action, &self.options)
    }

    #[must_use]
    pub fn options_ref(&self) -> &WindowControlButtonGroupOptions {
        &self.options
    }

    #[must_use]
    pub fn state(&self) -> &WindowControlButtonGroupState {
        &self.state
    }
}

impl From<WindowControlButtonGroup> for UiNode {
    fn from(value: WindowControlButtonGroup) -> Self {
        let state_id = value.state.clone().into_state_id();
        let mut node =
            UiNode::from_state(UiNodeKind::WindowControlButtonGroup, value.label, state_id)
                .visible(value.state.visible())
                .size(value.options.size.ui_size())
                .justify_content(value.options.position.justify_content())
                .interaction(value.state.interaction(&value.options))
                .style_class(value.options.position.style_class())
                .style_class(value.options.visibility.style_class());

        for control in value.options.controls {
            node = node.child(control_node(
                control,
                value.options.size,
                value.state.visible(),
            ));
        }
        node
    }
}

fn control_node(kind: WindowControlKind, size: WindowControlSize, visible: bool) -> UiNode {
    UiNode::new(UiNodeKind::Button, format!("{kind:?}"))
        .visible(visible)
        .width(UiDimension::px(size.pixels()))
        .height(UiDimension::px(size.pixels()))
        .size(size.ui_size())
        .variant(UiVariant::Icon)
        .visual_role(UiVisualRole::Control)
        .focusable(true)
        .cursor(UiCursor::Pointer)
        .accessibility_label(format!("Window control: {kind:?}"))
}
