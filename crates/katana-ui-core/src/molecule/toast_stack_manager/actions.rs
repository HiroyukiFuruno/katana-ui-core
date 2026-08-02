use super::{
    ToastDedupStrategy, ToastDismissReason, ToastPayload, ToastReplaceKind, ToastStackAction,
    ToastStackEvent, ToastStackManager,
};

impl ToastStackManager {
    #[must_use]
    pub fn apply_action(&mut self, action: ToastStackAction) -> Vec<ToastStackEvent> {
        match action {
            ToastStackAction::Enqueue(payload) => self.enqueue(payload),
            ToastStackAction::Dismiss(id) => self.dismiss(id, ToastDismissReason::Manual),
            ToastStackAction::DismissAll => self.dismiss_all(),
            ToastStackAction::PauseHover(hovered) => self.set_hover(hovered),
            ToastStackAction::FocusInside(focused) => self.set_focus(focused),
            ToastStackAction::Resume => self.resume(),
            ToastStackAction::Tick(elapsed_ms) => self.tick(elapsed_ms),
            ToastStackAction::ActivateToastAction {
                toast_id,
                action_id,
            } => self.dismiss(toast_id, ToastDismissReason::Action(action_id)),
        }
    }

    fn enqueue(&mut self, payload: ToastPayload) -> Vec<ToastStackEvent> {
        if let Some(events) = self.replace_existing(payload.clone()) {
            return events;
        }
        if self.state.visible.len() < self.options.max_visible {
            return self.show(payload);
        }
        self.queue(payload)
    }

    fn dismiss(&mut self, id: String, reason: ToastDismissReason) -> Vec<ToastStackEvent> {
        let Some(index) = self.state.visible.iter().position(|it| it.payload.id == id) else {
            return Vec::new();
        };
        let removed = self.state.visible[index].clone();
        let _ = self.state.visible.remove(index);
        let mut events = self.record(ToastStackEvent::ToastDismissed {
            id: removed.payload.id,
            reason,
        });
        events.extend(self.promote());
        events
    }

    fn dismiss_all(&mut self) -> Vec<ToastStackEvent> {
        let ids: Vec<String> = self
            .state
            .visible
            .drain(..)
            .map(|it| it.payload.id)
            .collect();
        self.state.queued.clear();
        ids.into_iter()
            .flat_map(|id| {
                self.record(ToastStackEvent::ToastDismissed {
                    id,
                    reason: ToastDismissReason::DismissAll,
                })
            })
            .collect()
    }

    fn tick(&mut self, elapsed_ms: u64) -> Vec<ToastStackEvent> {
        if self.state.paused {
            return Vec::new();
        }
        let mut timed_out = Vec::new();
        for toast in &mut self.state.visible {
            if let Some(remaining) = toast.remaining_duration_ms.as_mut() {
                *remaining = remaining.saturating_sub(elapsed_ms);
                if *remaining == 0 {
                    timed_out.push(toast.payload.id.clone());
                }
            }
        }
        timed_out
            .into_iter()
            .flat_map(|id| self.timeout(id))
            .collect()
    }

    fn timeout(&mut self, id: String) -> Vec<ToastStackEvent> {
        let mut events = self.record(ToastStackEvent::ToastTimedOut { id: id.clone() });
        events.extend(self.dismiss(id, ToastDismissReason::Timeout));
        events
    }

    fn set_hover(&mut self, hovered: bool) -> Vec<ToastStackEvent> {
        self.state.hover_count = usize::from(hovered);
        self.sync_pause_state()
    }

    fn set_focus(&mut self, focused: bool) -> Vec<ToastStackEvent> {
        self.state.focus_count = usize::from(focused);
        self.sync_pause_state()
    }

    fn resume(&mut self) -> Vec<ToastStackEvent> {
        self.state.hover_count = 0;
        self.state.focus_count = 0;
        self.sync_pause_state()
    }

    fn sync_pause_state(&mut self) -> Vec<ToastStackEvent> {
        let paused = self.options.pause_on_hover
            && (self.state.hover_count > 0 || self.state.focus_count > 0);
        if paused == self.state.paused {
            return Vec::new();
        }
        self.state.paused = paused;
        if paused {
            self.record(ToastStackEvent::ToastPaused)
        } else {
            self.record(ToastStackEvent::ToastResumed)
        }
    }

    fn replace_existing(&mut self, payload: ToastPayload) -> Option<Vec<ToastStackEvent>> {
        match self.options.dedup_strategy {
            ToastDedupStrategy::None => None,
            ToastDedupStrategy::ById | ToastDedupStrategy::ByIdAndSeverity => self
                .replace_visible(payload.clone())
                .or_else(|| self.replace_queued(payload)),
        }
    }

    fn replace_visible(&mut self, payload: ToastPayload) -> Option<Vec<ToastStackEvent>> {
        let index = self
            .state
            .visible
            .iter()
            .position(|it| self.matches(&it.payload, &payload))?;
        let remaining = self.state.visible[index].remaining_duration_ms;
        let mut replacement =
            super::ActiveToast::new(payload.clone(), self.options.default_duration_ms);
        if !self.options.replace_resets_duration {
            replacement.remaining_duration_ms = remaining;
        }
        self.state.visible[index] = replacement;
        Some(self.record(ToastStackEvent::ToastReplaced {
            id: payload.id,
            kind: ToastReplaceKind::Visible,
        }))
    }

    fn replace_queued(&mut self, payload: ToastPayload) -> Option<Vec<ToastStackEvent>> {
        let index = self
            .state
            .queued
            .iter()
            .position(|it| self.matches(it, &payload))?;
        self.state.queued[index] = payload.clone();
        Some(self.record(ToastStackEvent::ToastReplaced {
            id: payload.id,
            kind: ToastReplaceKind::Queued,
        }))
    }

    fn matches(&self, current: &ToastPayload, incoming: &ToastPayload) -> bool {
        current.id == incoming.id
            && (self.options.dedup_strategy == ToastDedupStrategy::ById
                || current.severity == incoming.severity)
    }
}
