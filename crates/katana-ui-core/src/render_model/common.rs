use super::common_types::{
    UiAlignItems, UiBorder, UiCursor, UiDimension, UiDisplay, UiEdgeInsets, UiJustifyContent,
    UiLayoutAxis, UiOverflow, UiPointerEvents, UiPosition, UiZIndex,
};
use super::{UiHostActionSpec, UiInteractivePreset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiCommonProps {
    pub visible: bool,
    pub disabled: bool,
    pub focusable: bool,
    pub selectable: bool,
    pub width: UiDimension,
    pub height: UiDimension,
    pub min_width: UiDimension,
    pub max_width: UiDimension,
    pub min_height: UiDimension,
    pub max_height: UiDimension,
    pub border: UiBorder,
    pub hover_border: UiBorder,
    pub padding: UiEdgeInsets,
    pub margin: UiEdgeInsets,
    pub display: UiDisplay,
    pub layout_axis: UiLayoutAxis,
    pub gap: UiDimension,
    pub overflow: UiOverflow,
    pub position: UiPosition,
    pub align_items: UiAlignItems,
    pub justify_content: UiJustifyContent,
    pub tab_index: Option<i16>,
    pub z_index: UiZIndex,
    pub cursor: UiCursor,
    pub pointer_events: UiPointerEvents,
    pub accessibility_label: String,
    pub theme_slot: String,
    pub semantic_node_id: String,
    pub host_actions: Vec<UiHostActionSpec>,
}

impl UiCommonProps {
    #[must_use]
    pub fn visible(mut self, value: bool) -> Self {
        self.visible = value;
        self
    }

    #[must_use]
    pub fn width(mut self, value: UiDimension) -> Self {
        self.width = value;
        self
    }

    #[must_use]
    pub fn height(mut self, value: UiDimension) -> Self {
        self.height = value;
        self
    }

    #[must_use]
    pub fn border(mut self, value: UiBorder) -> Self {
        self.border = value;
        self
    }

    #[must_use]
    pub fn hover_border(mut self, value: UiBorder) -> Self {
        self.hover_border = value;
        self
    }

    #[must_use]
    pub fn padding(mut self, value: UiEdgeInsets) -> Self {
        self.padding = value;
        self
    }

    #[must_use]
    pub fn margin(mut self, value: UiEdgeInsets) -> Self {
        self.margin = value;
        self
    }

    #[must_use]
    pub fn display(mut self, value: UiDisplay) -> Self {
        self.display = value;
        self
    }

    #[must_use]
    pub fn layout_axis(mut self, value: UiLayoutAxis) -> Self {
        self.layout_axis = value;
        self
    }

    #[must_use]
    pub fn gap(mut self, value: UiDimension) -> Self {
        self.gap = value;
        self
    }

    #[must_use]
    pub fn overflow(mut self, value: UiOverflow) -> Self {
        self.overflow = value;
        self
    }

    #[must_use]
    pub fn position(mut self, value: UiPosition) -> Self {
        self.position = value;
        self
    }

    #[must_use]
    pub fn align_items(mut self, value: UiAlignItems) -> Self {
        self.align_items = value;
        self
    }

    #[must_use]
    pub fn justify_content(mut self, value: UiJustifyContent) -> Self {
        self.justify_content = value;
        self
    }

    #[must_use]
    pub fn tab_index(mut self, value: i16) -> Self {
        self.tab_index = Some(value);
        self
    }

    #[must_use]
    pub fn z_index(mut self, value: UiZIndex) -> Self {
        self.z_index = value;
        self
    }

    #[must_use]
    pub fn cursor(mut self, value: UiCursor) -> Self {
        self.cursor = value;
        self
    }

    #[must_use]
    pub fn pointer_events(mut self, value: UiPointerEvents) -> Self {
        self.pointer_events = value;
        self
    }

    #[must_use]
    pub fn selectable(mut self, value: bool) -> Self {
        self.selectable = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn theme_slot(mut self, value: impl Into<String>) -> Self {
        self.theme_slot = value.into();
        self
    }

    #[must_use]
    pub fn semantic_node_id(mut self, value: impl Into<String>) -> Self {
        self.semantic_node_id = value.into();
        self
    }

    #[must_use]
    pub fn host_action(mut self, value: UiHostActionSpec) -> Self {
        self = UiInteractivePreset::control().apply_to_common_defaults(self);
        self.host_actions.push(value);
        self
    }
}

impl Default for UiCommonProps {
    fn default() -> Self {
        Self {
            visible: true,
            disabled: false,
            focusable: false,
            selectable: false,
            width: UiDimension::Auto,
            height: UiDimension::Auto,
            min_width: UiDimension::Auto,
            max_width: UiDimension::Auto,
            min_height: UiDimension::Auto,
            max_height: UiDimension::Auto,
            border: UiBorder::default(),
            hover_border: UiBorder::default(),
            padding: UiEdgeInsets::default(),
            margin: UiEdgeInsets::default(),
            display: UiDisplay::Block,
            layout_axis: UiLayoutAxis::Unspecified,
            gap: UiDimension::Px(0),
            overflow: UiOverflow::Visible,
            position: UiPosition::Static,
            align_items: UiAlignItems::Center,
            justify_content: UiJustifyContent::Start,
            tab_index: None,
            z_index: UiZIndex::Auto,
            cursor: UiCursor::Default,
            pointer_events: UiPointerEvents::Auto,
            accessibility_label: String::new(),
            theme_slot: String::new(),
            semantic_node_id: String::new(),
            host_actions: Vec::new(),
        }
    }
}
