use super::{ScrollArea, ScrollAxis, ScrollbarPlacement, ScrollbarVisibility};
use crate::render_model::{
    UiNode, UiNodeKind, UiRect, UiScrollAreaAxis, UiScrollAreaProps, UiScrollbarPlacement,
    UiScrollbarVisibility,
};

impl From<ScrollArea> for UiNode {
    fn from(value: ScrollArea) -> Self {
        let scroll_area = UiScrollAreaProps {
            axis: value.axis.into(),
            offset_x: value.offset_x,
            offset_y: value.offset_y,
            viewport_width: value.viewport_width,
            viewport_height: value.viewport_height,
            content_width: value.content_width,
            content_height: value.content_height,
            scrollbar_visibility: value.scrollbar_visibility.into(),
            scrollbar_placement: value.scrollbar_placement.into(),
            edge_threshold: value.edge_threshold,
            visible_rect: UiRect::new(
                value.offset_x as i32,
                value.offset_y as i32,
                value.viewport_width,
                value.viewport_height,
            ),
        };
        let mut node = UiNode::from_state(UiNodeKind::ScrollArea, "ScrollArea", value.state_id)
            .interaction(value.interaction)
            .scroll_area(scroll_area);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

impl From<ScrollAxis> for UiScrollAreaAxis {
    fn from(value: ScrollAxis) -> Self {
        match value {
            ScrollAxis::Vertical => Self::Vertical,
            ScrollAxis::Horizontal => Self::Horizontal,
            ScrollAxis::Both => Self::Both,
        }
    }
}

impl From<ScrollbarVisibility> for UiScrollbarVisibility {
    fn from(value: ScrollbarVisibility) -> Self {
        match value {
            ScrollbarVisibility::Auto => Self::Auto,
            ScrollbarVisibility::Always => Self::Always,
            ScrollbarVisibility::Hidden => Self::Hidden,
        }
    }
}

impl From<ScrollbarPlacement> for UiScrollbarPlacement {
    fn from(value: ScrollbarPlacement) -> Self {
        match value {
            ScrollbarPlacement::Reserved => Self::Reserved,
            ScrollbarPlacement::Overlay => Self::Overlay,
        }
    }
}
