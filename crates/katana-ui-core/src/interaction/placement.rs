use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

mod geometry;
pub use geometry::{Point, Rect, Size};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnchorKind {
    NodeRect { id: UiNodeId, rect: Rect },
    VirtualRect(Rect),
    Pointer(Point),
}

impl AnchorKind {
    #[must_use]
    pub fn node_rect(id: UiNodeId, rect: Rect) -> Self {
        Self::NodeRect { id, rect }
    }

    #[must_use]
    pub const fn virtual_rect(rect: Rect) -> Self {
        Self::VirtualRect(rect)
    }

    #[must_use]
    pub const fn pointer(point: Point) -> Self {
        Self::Pointer(point)
    }

    pub(crate) fn rect(&self) -> Rect {
        match self {
            Self::NodeRect { rect, .. } | Self::VirtualRect(rect) => *rect,
            Self::Pointer(point) => Rect::new(point.x, point.y, 0, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

impl Placement {
    #[must_use]
    pub const fn flipped(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::TopStart => Self::BottomStart,
            Self::TopEnd => Self::BottomEnd,
            Self::Bottom => Self::Top,
            Self::BottomStart => Self::TopStart,
            Self::BottomEnd => Self::TopEnd,
            Self::Left => Self::Right,
            Self::LeftStart => Self::RightStart,
            Self::LeftEnd => Self::RightEnd,
            Self::Right => Self::Left,
            Self::RightStart => Self::LeftStart,
            Self::RightEnd => Self::LeftEnd,
        }
    }

    pub(crate) fn is_vertical(self) -> bool {
        matches!(
            self,
            Self::Top
                | Self::TopStart
                | Self::TopEnd
                | Self::Bottom
                | Self::BottomStart
                | Self::BottomEnd
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlacementConsumer {
    Tooltip,
    Popover,
    HoverCard,
    ContextMenu,
    Menu,
    MenuButton,
    SelectBox,
    ComboBox,
}

impl PlacementConsumer {
    #[must_use]
    pub const fn default_priority(self) -> [Placement; 2] {
        match self {
            Self::Tooltip | Self::HoverCard => [Placement::Top, Placement::Bottom],
            Self::Popover
            | Self::ContextMenu
            | Self::Menu
            | Self::MenuButton
            | Self::SelectBox
            | Self::ComboBox => [Placement::BottomStart, Placement::TopStart],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementRequest {
    pub anchor: AnchorKind,
    pub preferred: Placement,
    pub priority: Vec<Placement>,
    pub offset: i32,
    pub panel_size: Size,
    pub viewport: Rect,
    pub clamp_margin: i32,
    pub arrow_size: Option<u32>,
}

impl PlacementRequest {
    #[must_use]
    pub fn new(anchor: AnchorKind, preferred: Placement, panel_size: Size, viewport: Rect) -> Self {
        Self {
            anchor,
            preferred,
            priority: Vec::new(),
            offset: 0,
            panel_size,
            viewport,
            clamp_margin: 0,
            arrow_size: None,
        }
    }

    #[must_use]
    pub fn priority(mut self, value: impl IntoIterator<Item = Placement>) -> Self {
        self.priority = value.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn offset(mut self, value: i32) -> Self {
        self.offset = value;
        self
    }

    #[must_use]
    pub const fn clamp_margin(mut self, value: i32) -> Self {
        self.clamp_margin = value;
        self
    }

    #[must_use]
    pub const fn arrow_size(mut self, value: u32) -> Self {
        self.arrow_size = Some(value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementResult {
    pub placement_used: Placement,
    pub position: Point,
    pub arrow_offset: Option<i32>,
    pub clamped: bool,
}

mod engine;
pub use engine::PlacementEngine;
