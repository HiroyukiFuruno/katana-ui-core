use super::{
    UiBorder, UiCommonProps, UiCursor, UiDimension, UiDisplay, UiEdgeInsets, UiHostActionSpec,
    UiJustifyContent, UiNode, UiPointerEvents, UiPosition, UiZIndex,
};

impl UiNode {
    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.props.common = value;
        self.sync_legacy_common_fields()
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.props.disabled = value;
        self.props.common.disabled = value;
        self
    }

    #[must_use]
    pub fn focusable(mut self, value: bool) -> Self {
        self.props.focusable = value;
        self.props.common.focusable = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        let label = value.into();
        self.props.accessibility_label = label.clone();
        self.props.common.accessibility_label = label;
        self
    }

    #[must_use]
    pub fn visible(mut self, value: bool) -> Self {
        self.props.common.visible = value;
        self
    }

    #[must_use]
    pub fn width(mut self, value: UiDimension) -> Self {
        self.props.common.width = value;
        self
    }

    #[must_use]
    pub fn height(mut self, value: UiDimension) -> Self {
        self.props.common.height = value;
        self
    }

    #[must_use]
    pub fn border(mut self, value: UiBorder) -> Self {
        self.props.common.border = value;
        self
    }

    #[must_use]
    pub fn hover_border(mut self, value: UiBorder) -> Self {
        self.props.common.hover_border = value;
        self
    }

    #[must_use]
    pub fn margin(mut self, value: UiEdgeInsets) -> Self {
        self.props.common.margin = value;
        self
    }

    #[must_use]
    pub fn display(mut self, value: UiDisplay) -> Self {
        self.props.common.display = value;
        self
    }

    #[must_use]
    pub fn position(mut self, value: UiPosition) -> Self {
        self.props.common.position = value;
        self
    }

    #[must_use]
    pub fn justify_content(mut self, value: UiJustifyContent) -> Self {
        self.props.common.justify_content = value;
        self
    }

    #[must_use]
    pub fn tab_index(mut self, value: i16) -> Self {
        self.props.common.tab_index = Some(value);
        self
    }

    #[must_use]
    pub fn z_index(mut self, value: UiZIndex) -> Self {
        self.props.common.z_index = value;
        self
    }

    #[must_use]
    pub fn cursor(mut self, value: UiCursor) -> Self {
        self.props.common.cursor = value;
        self
    }

    #[must_use]
    pub fn pointer_events(mut self, value: UiPointerEvents) -> Self {
        self.props.common.pointer_events = value;
        self
    }

    #[must_use]
    pub fn host_action(mut self, value: UiHostActionSpec) -> Self {
        self.props.common = self.props.common.host_action(value);
        self
    }

    #[must_use]
    pub fn command_action(
        mut self,
        action_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        self.props.common = self
            .props
            .common
            .host_action(UiHostActionSpec::command(action_id, label));
        self
    }

    #[must_use]
    pub fn surface_control_action(
        mut self,
        action_id: impl Into<String>,
        label: impl Into<String>,
        node_id: impl Into<String>,
    ) -> Self {
        self.props.common = self
            .props
            .common
            .host_action(UiHostActionSpec::surface_control_for(
                action_id, label, node_id,
            ));
        self
    }

    #[must_use]
    pub fn task_control_action(
        mut self,
        label: impl Into<String>,
        node_id: impl Into<String>,
        row_index: usize,
    ) -> Self {
        self.props.common = self
            .props
            .common
            .host_action(UiHostActionSpec::task_control(label, node_id, row_index));
        self
    }

    #[must_use]
    pub fn has_host_action(&self) -> bool {
        !self.props.common.host_actions.is_empty()
    }

    #[must_use]
    pub fn selectable(mut self, value: bool) -> Self {
        self.props.common.selectable = value;
        self
    }

    fn sync_legacy_common_fields(mut self) -> Self {
        self.props.disabled = self.props.common.disabled;
        self.props.focusable = self.props.common.focusable;
        self.props.accessibility_label = self.props.common.accessibility_label.clone();
        self
    }
}
