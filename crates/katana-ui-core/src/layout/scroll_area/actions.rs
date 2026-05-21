use super::{
    ScrollArea, ScrollAreaAction, ScrollAreaEvent, ScrollAxis, ScrollEdge, ScrollRejectionReason,
    ScrollbarVisibility,
};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiRect, UiScrollbarVisibility};

impl ScrollArea {
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

    pub(super) fn clamp_offsets(&mut self) {
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

    fn scroll_by(&mut self, dx: i32, dy: i32) -> Vec<ScrollAreaEvent> {
        if !self.allows_delta(dx, dy) {
            return vec![self.rejected(ScrollRejectionReason::AxisMismatch)];
        }
        if !self.has_overflow_for_delta(dx, dy) {
            return vec![self.rejected(ScrollRejectionReason::NoOverflow)];
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

    const fn axis_allows_x(&self) -> bool {
        matches!(self.axis, ScrollAxis::Horizontal | ScrollAxis::Both)
    }

    const fn axis_allows_y(&self) -> bool {
        matches!(self.axis, ScrollAxis::Vertical | ScrollAxis::Both)
    }

    fn allows_delta(&self, dx: i32, dy: i32) -> bool {
        (dx == 0 || self.axis_allows_x()) && (dy == 0 || self.axis_allows_y())
    }

    fn has_overflow_for_delta(&self, dx: i32, dy: i32) -> bool {
        (dx != 0 && self.max_x() > 0) || (dy != 0 && self.max_y() > 0)
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
                ScrollAreaAction::SetScrollbarVisibility(ScrollbarVisibility::from(*visibility)),
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

impl From<UiScrollbarVisibility> for ScrollbarVisibility {
    fn from(value: UiScrollbarVisibility) -> Self {
        match value {
            UiScrollbarVisibility::Auto => Self::Auto,
            UiScrollbarVisibility::Always => Self::Always,
            UiScrollbarVisibility::Hidden => Self::Hidden,
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
