use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiInteractionState, UiNode, UiNodeKind, UiRect, UiScrollAreaAxis, UiScrollAreaProps,
    UiScrollbarPlacement, UiScrollbarVisibility, UiStateId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAxis {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollbarVisibility {
    Auto,
    Always,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollbarPlacement {
    Reserved,
    Overlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAreaAction {
    ScrollTo { x: u32, y: u32 },
    ScrollBy { dx: i32, dy: i32 },
    ScrollIntoView { target_rect: UiRect },
    SetScrollbarVisibility(ScrollbarVisibility),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAreaEvent {
    Scrolled {
        target: UiStateId,
        x: u32,
        y: u32,
    },
    ScrollEdgeReached {
        target: UiStateId,
        edge: ScrollEdge,
    },
    ScrollCommandRejected {
        target: UiStateId,
        reason: ScrollRejectionReason,
    },
}

impl ScrollAreaEvent {
    #[must_use]
    pub fn target(&self) -> &UiStateId {
        match self {
            Self::Scrolled { target, .. }
            | Self::ScrollEdgeReached { target, .. }
            | Self::ScrollCommandRejected { target, .. } => target,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollEdge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollRejectionReason {
    AxisMismatch,
    InvalidExtent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollArea {
    state_id: UiStateId,
    children: Vec<UiNode>,
    interaction: UiInteractionState,
    axis: ScrollAxis,
    offset_x: u32,
    offset_y: u32,
    viewport_width: u32,
    viewport_height: u32,
    content_width: u32,
    content_height: u32,
    scrollbar_visibility: ScrollbarVisibility,
    scrollbar_placement: ScrollbarPlacement,
    edge_threshold: u32,
}

impl ScrollArea {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::ScrollArea),
            children: Vec::new(),
            interaction: UiInteractionState::default(),
            axis: ScrollAxis::Vertical,
            offset_x: 0,
            offset_y: 0,
            viewport_width: 0,
            viewport_height: 0,
            content_width: 0,
            content_height: 0,
            scrollbar_visibility: ScrollbarVisibility::Auto,
            scrollbar_placement: ScrollbarPlacement::Reserved,
            edge_threshold: 0,
        }
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn axis(mut self, value: ScrollAxis) -> Self {
        self.axis = value;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn viewport(mut self, width: u32, height: u32) -> Self {
        self.viewport_width = width;
        self.viewport_height = height;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn content_extent(mut self, width: u32, height: u32) -> Self {
        self.content_width = width;
        self.content_height = height;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn offset(mut self, x: u32, y: u32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self.clamp_offsets();
        self
    }

    #[must_use]
    pub fn scrollbar_visibility(mut self, value: ScrollbarVisibility) -> Self {
        self.scrollbar_visibility = value;
        self
    }

    #[must_use]
    pub fn scrollbar_placement(mut self, value: ScrollbarPlacement) -> Self {
        self.scrollbar_placement = value;
        self
    }

    #[must_use]
    pub fn edge_threshold(mut self, value: u32) -> Self {
        self.edge_threshold = value;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub const fn offset_x(&self) -> u32 {
        self.offset_x
    }

    #[must_use]
    pub const fn offset_y(&self) -> u32 {
        self.offset_y
    }

    pub fn apply_scroll_action(&mut self, action: ScrollAreaAction) -> Vec<ScrollAreaEvent> {
        match action {
            ScrollAreaAction::ScrollTo { x, y } => self.scroll_to(x, y),
            ScrollAreaAction::ScrollBy { dx, dy } => self.scroll_by(dx, dy),
            ScrollAreaAction::ScrollIntoView { target_rect } => self.scroll_into_view(target_rect),
            ScrollAreaAction::SetScrollbarVisibility(value) => {
                self.scrollbar_visibility = value;
                vec![self.scrolled_event()]
            }
        }
    }

    fn scroll_by(&mut self, dx: i32, dy: i32) -> Vec<ScrollAreaEvent> {
        if !self.allows_delta(dx, dy) {
            return vec![self.rejected(ScrollRejectionReason::AxisMismatch)];
        }
        let x = add_delta(self.offset_x, dx);
        let y = add_delta(self.offset_y, dy);
        self.scroll_to(x, y)
    }

    fn scroll_to(&mut self, x: u32, y: u32) -> Vec<ScrollAreaEvent> {
        let old_x = self.offset_x;
        let old_y = self.offset_y;
        self.offset_x = if self.axis_allows_x() {
            x.min(self.max_x())
        } else {
            0
        };
        self.offset_y = if self.axis_allows_y() {
            y.min(self.max_y())
        } else {
            0
        };
        self.interaction.value = format!("offset={},{}", self.offset_x, self.offset_y);
        let mut events = Vec::new();
        if old_x != self.offset_x || old_y != self.offset_y {
            events.push(self.scrolled_event());
        }
        events.extend(self.edge_events());
        events
    }

    fn scroll_into_view(&mut self, target_rect: UiRect) -> Vec<ScrollAreaEvent> {
        if self.viewport_width == 0 || self.viewport_height == 0 {
            return vec![self.rejected(ScrollRejectionReason::InvalidExtent)];
        }
        let x = into_view_offset(
            self.offset_x,
            self.viewport_width,
            target_rect.x.max(0) as u32,
            target_rect.width,
        );
        let y = into_view_offset(
            self.offset_y,
            self.viewport_height,
            target_rect.y.max(0) as u32,
            target_rect.height,
        );
        self.scroll_to(x, y)
    }

    fn clamp_offsets(&mut self) {
        self.offset_x = if self.axis_allows_x() {
            self.offset_x.min(self.max_x())
        } else {
            0
        };
        self.offset_y = if self.axis_allows_y() {
            self.offset_y.min(self.max_y())
        } else {
            0
        };
        self.interaction.value = format!("offset={},{}", self.offset_x, self.offset_y);
    }

    const fn axis_allows_x(&self) -> bool {
        matches!(self.axis, ScrollAxis::Horizontal | ScrollAxis::Both)
    }

    const fn axis_allows_y(&self) -> bool {
        matches!(self.axis, ScrollAxis::Vertical | ScrollAxis::Both)
    }

    fn allows_delta(&self, dx: i32, dy: i32) -> bool {
        (dx == 0 || self.axis_allows_x()) && (dy == 0 || self.axis_allows_y())
    }

    const fn max_x(&self) -> u32 {
        self.content_width.saturating_sub(self.viewport_width)
    }

    const fn max_y(&self) -> u32 {
        self.content_height.saturating_sub(self.viewport_height)
    }

    fn scrolled_event(&self) -> ScrollAreaEvent {
        ScrollAreaEvent::Scrolled {
            target: self.state_id.clone(),
            x: self.offset_x,
            y: self.offset_y,
        }
    }

    fn edge_events(&self) -> Vec<ScrollAreaEvent> {
        let mut events = Vec::new();
        if self.offset_y == 0 && self.axis_allows_y() {
            events.push(self.edge_event(ScrollEdge::Top));
        }
        if self.offset_y == self.max_y() && self.max_y() > 0 && self.axis_allows_y() {
            events.push(self.edge_event(ScrollEdge::Bottom));
        }
        if self.offset_x == 0 && self.axis_allows_x() {
            events.push(self.edge_event(ScrollEdge::Left));
        }
        if self.offset_x == self.max_x() && self.max_x() > 0 && self.axis_allows_x() {
            events.push(self.edge_event(ScrollEdge::Right));
        }
        events
    }

    fn edge_event(&self, edge: ScrollEdge) -> ScrollAreaEvent {
        ScrollAreaEvent::ScrollEdgeReached {
            target: self.state_id.clone(),
            edge,
        }
    }

    fn rejected(&self, reason: ScrollRejectionReason) -> ScrollAreaEvent {
        ScrollAreaEvent::ScrollCommandRejected {
            target: self.state_id.clone(),
            reason,
        }
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentAction for ScrollArea {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.interaction.clone();
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        let events = match action {
            UiAction::ScrollTo { x, y, .. } => {
                self.apply_scroll_action(ScrollAreaAction::ScrollTo { x: *x, y: *y })
            }
            UiAction::ScrollBy { dx, dy, .. } => {
                self.apply_scroll_action(ScrollAreaAction::ScrollBy { dx: *dx, dy: *dy })
            }
            UiAction::ScrollIntoView { target_rect, .. } => {
                self.apply_scroll_action(ScrollAreaAction::ScrollIntoView {
                    target_rect: *target_rect,
                })
            }
            UiAction::SetScrollbarVisibility { visibility, .. } => self.apply_scroll_action(
                ScrollAreaAction::SetScrollbarVisibility((*visibility).into()),
            ),
            _ => Vec::new(),
        };
        if events.is_empty() {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        let after = self.interaction.clone();
        UiActionResult {
            target: self.state_id.clone(),
            handled: !events
                .iter()
                .all(|it| matches!(it, ScrollAreaEvent::ScrollCommandRejected { .. })),
            before,
            after,
            callback_log: vec![crate::interaction::UiCallbackLog::new(
                self.state_id.clone(),
                action.name(),
                "scroll_area",
                events_summary(&events),
            )],
        }
    }
}

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

impl From<UiScrollbarVisibility> for ScrollbarVisibility {
    fn from(value: UiScrollbarVisibility) -> Self {
        match value {
            UiScrollbarVisibility::Auto => Self::Auto,
            UiScrollbarVisibility::Always => Self::Always,
            UiScrollbarVisibility::Hidden => Self::Hidden,
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

fn add_delta(value: u32, delta: i32) -> u32 {
    if delta.is_negative() {
        value.saturating_sub(delta.unsigned_abs())
    } else {
        value.saturating_add(delta as u32)
    }
}

fn into_view_offset(offset: u32, viewport: u32, target_start: u32, target_extent: u32) -> u32 {
    let target_end = target_start.saturating_add(target_extent);
    let viewport_end = offset.saturating_add(viewport);
    if target_start < offset {
        target_start
    } else if target_end > viewport_end {
        target_end.saturating_sub(viewport)
    } else {
        offset
    }
}

fn events_summary(events: &[ScrollAreaEvent]) -> String {
    events
        .iter()
        .map(|it| match it {
            ScrollAreaEvent::Scrolled { x, y, .. } => format!("Scrolled({x},{y})"),
            ScrollAreaEvent::ScrollEdgeReached { edge, .. } => {
                format!("ScrollEdgeReached({edge:?})")
            }
            ScrollAreaEvent::ScrollCommandRejected { reason, .. } => {
                format!("ScrollCommandRejected({reason:?})")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
