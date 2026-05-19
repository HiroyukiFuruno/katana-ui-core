use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl UiRect {
    #[must_use]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiScrollbarVisibility {
    Always,
    #[default]
    Auto,
    Hidden,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiScrollbarPlacement {
    Overlay,
    #[default]
    Reserved,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiScrollbarDragState {
    pub dragging: bool,
    pub pointer_id: Option<u64>,
    pub origin_y: i32,
    pub origin_offset: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiScrollbarModel {
    pub visibility: UiScrollbarVisibility,
    pub placement: UiScrollbarPlacement,
    pub track_bounds: UiRect,
    pub thumb_bounds: UiRect,
    pub offset: u32,
    pub drag_state: UiScrollbarDragState,
}

impl UiScrollbarModel {
    #[must_use]
    pub const fn new(
        visibility: UiScrollbarVisibility,
        placement: UiScrollbarPlacement,
        track_bounds: UiRect,
        thumb_bounds: UiRect,
        offset: u32,
    ) -> Self {
        Self {
            visibility,
            placement,
            track_bounds,
            thumb_bounds,
            offset,
            drag_state: UiScrollbarDragState {
                dragging: false,
                pointer_id: None,
                origin_y: 0,
                origin_offset: offset,
            },
        }
    }

    #[must_use]
    pub const fn dragging(mut self, pointer_id: u64, origin_y: i32) -> Self {
        self.drag_state = UiScrollbarDragState {
            dragging: true,
            pointer_id: Some(pointer_id),
            origin_y,
            origin_offset: self.offset,
        };
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiPanelProps {
    pub scroll_y: u32,
    pub viewport_height: u32,
    pub content_height: u32,
    pub vertical_scrollbar_visible: bool,
    pub vertical_scrollbar: UiScrollbarModel,
}

impl UiPanelProps {
    #[must_use]
    pub fn vertical_scroll(
        scroll_y: u32,
        viewport_height: u32,
        content_height: u32,
        visible: bool,
    ) -> Self {
        Self {
            scroll_y,
            viewport_height,
            content_height,
            vertical_scrollbar_visible: visible,
            vertical_scrollbar: UiScrollbarModel::new(
                if visible {
                    UiScrollbarVisibility::Always
                } else {
                    UiScrollbarVisibility::Hidden
                },
                UiScrollbarPlacement::Reserved,
                UiRect::new(0, 0, 0, viewport_height),
                UiRect::new(0, scroll_y as i32, 0, viewport_height.min(content_height)),
                scroll_y,
            ),
        }
    }

    #[must_use]
    pub fn scrollbar(mut self, value: UiScrollbarModel) -> Self {
        self.vertical_scrollbar_visible =
            !matches!(value.visibility, UiScrollbarVisibility::Hidden);
        self.scroll_y = value.offset;
        self.vertical_scrollbar = value;
        self
    }
}
