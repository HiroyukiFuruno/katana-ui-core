use super::focus_request::TextSurfaceFocusRequestToken;
use super::props::TextSurfacePoint;
use super::scroll_request_types::TextSurfaceScrollRequestToken;
use crate::atom::TextAreaState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceScrollBounds {
    pub max_x: i32,
    pub max_y: i32,
}

impl TextSurfaceScrollBounds {
    #[must_use]
    pub fn from_extents(
        content_width: u32,
        content_height: u32,
        viewport_width: u32,
        viewport_height: u32,
    ) -> Self {
        Self {
            max_x: scroll_maximum(content_width, viewport_width),
            max_y: scroll_maximum(content_height, viewport_height),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceState {
    pub text_area: TextAreaState,
    pub pointer_anchor: Option<TextSurfacePoint>,
    pub scroll_x: i32,
    pub scroll_y: i32,
    pub scroll_bounds: Option<TextSurfaceScrollBounds>,
    pub last_scroll_request_token: Option<TextSurfaceScrollRequestToken>,
    pub last_focus_request_token: Option<TextSurfaceFocusRequestToken>,
}

impl TextSurfaceState {
    #[must_use]
    pub fn new(text_area: TextAreaState, scroll_x: i32, scroll_y: i32) -> Self {
        Self {
            text_area,
            pointer_anchor: None,
            scroll_x,
            scroll_y,
            scroll_bounds: None,
            last_scroll_request_token: None,
            last_focus_request_token: None,
        }
    }
}

fn scroll_maximum(content_extent: u32, viewport_extent: u32) -> i32 {
    i32::try_from(content_extent.saturating_sub(viewport_extent)).unwrap_or(i32::MAX)
}
