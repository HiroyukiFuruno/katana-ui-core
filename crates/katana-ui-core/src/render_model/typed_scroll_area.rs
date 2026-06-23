use super::super::common_types::{UiAlignItems, UiDimension};
use super::{UiRect, UiScrollbarPlacement, UiScrollbarVisibility};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiScrollAreaAxis {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiScrollAreaProps {
    pub axis: UiScrollAreaAxis,
    pub offset_x: u32,
    pub offset_y: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub content_width: u32,
    pub content_height: u32,
    pub scrollbar_visibility: UiScrollbarVisibility,
    pub scrollbar_placement: UiScrollbarPlacement,
    pub edge_threshold: u32,
    pub gap: UiDimension,
    pub alignment: UiAlignItems,
    pub visible_rect: UiRect,
}

impl Default for UiScrollAreaProps {
    fn default() -> Self {
        Self {
            axis: UiScrollAreaAxis::Vertical,
            offset_x: 0,
            offset_y: 0,
            viewport_width: 0,
            viewport_height: 0,
            content_width: 0,
            content_height: 0,
            scrollbar_visibility: UiScrollbarVisibility::Auto,
            scrollbar_placement: UiScrollbarPlacement::Reserved,
            edge_threshold: 0,
            gap: UiDimension::Px(0),
            alignment: UiAlignItems::Start,
            visible_rect: UiRect::default(),
        }
    }
}
