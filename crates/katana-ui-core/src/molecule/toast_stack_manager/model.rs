use super::{ActiveToast, ToastDedupStrategy, ToastPosition, ToastStackDirection, ToastStackEvent};
use crate::render_model::UiNodeKind;
use std::collections::VecDeque;

const DEFAULT_MAX_VISIBLE_TOASTS: usize = 3;
const DEFAULT_TOAST_DURATION_MS: u64 = 5_000;
const DEFAULT_STACK_GAP: u16 = 8;
const DEFAULT_MAX_QUEUED_TOASTS: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToastStackManager {
    pub(super) state_id: crate::render_model::UiStateId,
    pub(super) options: ToastStackOptions,
    pub(super) state: ToastStackState,
    callback_log: Vec<ToastStackEvent>,
}

impl ToastStackManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_id: crate::render_model::UiStateId::next_for(UiNodeKind::ToastStackManager),
            options: ToastStackOptions::default(),
            state: ToastStackState::default(),
            callback_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn options(mut self, value: ToastStackOptions) -> Self {
        self.options = value;
        self
    }

    #[must_use]
    pub const fn state(&self) -> &ToastStackState {
        &self.state
    }

    #[must_use]
    pub fn callback_log(&self) -> &[ToastStackEvent] {
        &self.callback_log
    }

    #[must_use]
    pub fn visual_contract(&self) -> ToastStackVisualContract {
        ToastStackVisualContract {
            position: self.options.position,
            stack_direction: self.options.position.stack_direction(),
            stack_gap: self.options.stack_gap,
            enter_direction: self.options.enter_direction,
            exit_direction: self.options.exit_direction,
        }
    }

    pub(super) fn record(&mut self, event: ToastStackEvent) -> Vec<ToastStackEvent> {
        self.callback_log.push(event.clone());
        vec![event]
    }

    pub(super) fn promote(&mut self) -> Vec<ToastStackEvent> {
        let mut events = Vec::new();
        while self.state.visible.len() < self.options.max_visible {
            let Some(payload) = self.state.queued.pop_front() else {
                break;
            };
            events.extend(self.show(payload));
        }
        events
    }

    pub(super) fn show(&mut self, payload: super::ToastPayload) -> Vec<ToastStackEvent> {
        self.state.visible.push_back(ActiveToast::new(
            payload.clone(),
            self.options.default_duration_ms,
        ));
        self.record(ToastStackEvent::ToastShown { id: payload.id })
    }

    pub(super) fn queue(&mut self, payload: super::ToastPayload) -> Vec<ToastStackEvent> {
        let mut events = self.enforce_queue_cap(&payload.id);
        self.state.queued.push_back(payload.clone());
        events.extend(self.record(ToastStackEvent::ToastQueued { id: payload.id }));
        events
    }

    fn enforce_queue_cap(&mut self, incoming_id: &str) -> Vec<ToastStackEvent> {
        if self.options.max_queued == 0 {
            return self.record(ToastStackEvent::ToastQueueOverflow {
                dropped_id: incoming_id.to_string(),
            });
        }
        if self.state.queued.len() < self.options.max_queued {
            return Vec::new();
        }
        let dropped_id = self
            .state
            .queued
            .pop_front()
            .map_or_else(|| incoming_id.to_string(), |it| it.id);
        self.record(ToastStackEvent::ToastQueueOverflow { dropped_id })
    }
}

impl Default for ToastStackManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToastStackOptions {
    pub position: ToastPosition,
    pub max_visible: usize,
    pub dedup_strategy: ToastDedupStrategy,
    pub default_duration_ms: u64,
    pub pause_on_hover: bool,
    pub stack_gap: u16,
    pub enter_direction: ToastStackDirection,
    pub exit_direction: ToastStackDirection,
    pub replace_resets_duration: bool,
    pub max_queued: usize,
}

impl Default for ToastStackOptions {
    fn default() -> Self {
        Self {
            position: ToastPosition::TopEnd,
            max_visible: DEFAULT_MAX_VISIBLE_TOASTS,
            dedup_strategy: ToastDedupStrategy::None,
            default_duration_ms: DEFAULT_TOAST_DURATION_MS,
            pause_on_hover: true,
            stack_gap: DEFAULT_STACK_GAP,
            enter_direction: ToastStackDirection::Down,
            exit_direction: ToastStackDirection::Up,
            replace_resets_duration: true,
            max_queued: DEFAULT_MAX_QUEUED_TOASTS,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToastStackState {
    pub visible: VecDeque<ActiveToast>,
    pub queued: VecDeque<super::ToastPayload>,
    pub paused: bool,
    pub hover_count: usize,
    pub focus_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToastStackVisualContract {
    pub position: ToastPosition,
    pub stack_direction: ToastStackDirection,
    pub stack_gap: u16,
    pub enter_direction: ToastStackDirection,
    pub exit_direction: ToastStackDirection,
}
