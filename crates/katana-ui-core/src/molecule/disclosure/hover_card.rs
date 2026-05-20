use super::rich_content::{PopoverActionSlot, PopoverSlots};
use crate::interaction::placement::Point;
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverCard {
    pub(super) label: String,
    pub(super) open: bool,
    open_delay_ms: u16,
    close_delay_ms: u16,
    elapsed_ms: u16,
    delay_state: HoverCardDelayState,
    pointer_follow: bool,
    pointer_anchor: Option<Point>,
    pub(super) slots: PopoverSlots,
    pub(super) slot_actions: Vec<PopoverActionSlot>,
}

impl HoverCard {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            open: false,
            open_delay_ms: 0,
            close_delay_ms: 0,
            elapsed_ms: 0,
            delay_state: HoverCardDelayState::Idle,
            pointer_follow: false,
            pointer_anchor: None,
            slots: PopoverSlots::default(),
            slot_actions: Vec::new(),
        }
    }

    #[must_use]
    pub fn open_delay_ms(mut self, value: u16) -> Self {
        self.open_delay_ms = value;
        self
    }

    #[must_use]
    pub fn close_delay_ms(mut self, value: u16) -> Self {
        self.close_delay_ms = value;
        self
    }

    #[must_use]
    pub fn pointer_follow(mut self, value: bool) -> Self {
        self.pointer_follow = value;
        self
    }

    #[must_use]
    pub fn slot_action(mut self, value: PopoverActionSlot) -> Self {
        self.slot_actions.push(value);
        self
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.open
    }

    #[must_use]
    pub fn delay_state(&self) -> HoverCardDelayState {
        self.delay_state
    }

    #[must_use]
    pub fn pointer_follow_model(&self) -> bool {
        self.pointer_follow
    }

    pub fn apply_hover_card_action(&mut self, action: HoverCardAction) -> HoverCardEvent {
        match action {
            HoverCardAction::AnchorPointerEntered => self.open_from_anchor(),
            HoverCardAction::AnchorPointerLeft => self.schedule_close(),
            HoverCardAction::CardPointerEntered => self.pause_close(),
            HoverCardAction::CardPointerLeft => self.schedule_close(),
            HoverCardAction::AnchorFocused => self.open_from_anchor(),
            HoverCardAction::AnchorBlurred => self.schedule_close(),
            HoverCardAction::InnerFocusEntered(node_id) => self.keep_open_for_focus(node_id),
            HoverCardAction::InnerFocusLeft(_node_id) => self.schedule_close(),
            HoverCardAction::PointerMoved(point) => self.move_pointer_anchor(point),
            HoverCardAction::TimerElapsed(delta_ms) => self.advance_timer(delta_ms),
        }
    }

    fn open_from_anchor(&mut self) -> HoverCardEvent {
        self.elapsed_ms = 0;
        if self.open_delay_ms == 0 {
            self.open = true;
            self.delay_state = HoverCardDelayState::Idle;
            return HoverCardEvent::Opened;
        }
        self.delay_state = HoverCardDelayState::Opening;
        HoverCardEvent::OpenScheduled
    }

    fn schedule_close(&mut self) -> HoverCardEvent {
        self.elapsed_ms = 0;
        self.delay_state = HoverCardDelayState::Closing;
        HoverCardEvent::CloseScheduled
    }

    fn pause_close(&mut self) -> HoverCardEvent {
        if self.open {
            self.delay_state = HoverCardDelayState::PausedClose;
        }
        HoverCardEvent::KeptOpen
    }

    fn keep_open_for_focus(&mut self, _node_id: UiNodeId) -> HoverCardEvent {
        self.open = true;
        self.delay_state = HoverCardDelayState::PausedClose;
        HoverCardEvent::KeptOpen
    }

    fn move_pointer_anchor(&mut self, point: Point) -> HoverCardEvent {
        if !self.pointer_follow {
            return HoverCardEvent::Ignored;
        }
        self.pointer_anchor = Some(point);
        HoverCardEvent::PointerAnchorUpdated
    }

    fn advance_timer(&mut self, delta_ms: u16) -> HoverCardEvent {
        match self.delay_state {
            HoverCardDelayState::Opening => self.advance_open(delta_ms),
            HoverCardDelayState::Closing => self.advance_close(delta_ms),
            HoverCardDelayState::PausedClose => HoverCardEvent::KeptOpen,
            HoverCardDelayState::Idle => HoverCardEvent::DelayPending,
        }
    }

    fn advance_open(&mut self, delta_ms: u16) -> HoverCardEvent {
        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
        if self.elapsed_ms < self.open_delay_ms {
            return HoverCardEvent::DelayPending;
        }
        self.open = true;
        self.delay_state = HoverCardDelayState::Idle;
        HoverCardEvent::Opened
    }

    fn advance_close(&mut self, delta_ms: u16) -> HoverCardEvent {
        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
        if self.elapsed_ms < self.close_delay_ms {
            return HoverCardEvent::DelayPending;
        }
        self.open = false;
        self.delay_state = HoverCardDelayState::Idle;
        HoverCardEvent::Closed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoverCardAction {
    AnchorPointerEntered,
    AnchorPointerLeft,
    CardPointerEntered,
    CardPointerLeft,
    AnchorFocused,
    AnchorBlurred,
    InnerFocusEntered(UiNodeId),
    InnerFocusLeft(UiNodeId),
    PointerMoved(Point),
    TimerElapsed(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoverCardDelayState {
    Idle,
    Opening,
    Closing,
    PausedClose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoverCardEvent {
    OpenScheduled,
    Opened,
    CloseScheduled,
    Closed,
    DelayPending,
    KeptOpen,
    PointerAnchorUpdated,
    Ignored,
}
