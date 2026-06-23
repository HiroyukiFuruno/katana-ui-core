use super::{
    SplitPane, SplitPaneAction, SplitPaneEvent, SplitPaneRejectionReason, SplitPaneResizeMode,
    SplitPaneResizeSource,
};

impl SplitPane {
    #[must_use]
    pub fn apply_split_action_sequence(
        &mut self,
        actions: impl IntoIterator<Item = SplitPaneAction>,
    ) -> Vec<SplitPaneEvent> {
        actions
            .into_iter()
            .flat_map(|action| self.apply_split_action(action))
            .collect()
    }

    #[must_use]
    pub fn apply_split_action(&mut self, action: SplitPaneAction) -> Vec<SplitPaneEvent> {
        match action {
            SplitPaneAction::StartResize => self.start_resize(),
            SplitPaneAction::SetRatio(percent) => {
                self.set_ratio_with_event(percent, SplitPaneResizeSource::Pointer)
            }
            SplitPaneAction::ResizeBy {
                delta_percent,
                source,
            } => self.resize_by(delta_percent, source),
            SplitPaneAction::ResetRatio => self.reset_ratio_with_event(),
            SplitPaneAction::EndResize => self.end_resize(),
        }
    }

    pub(super) fn can_resize_from(&self, source: SplitPaneResizeSource) -> bool {
        match self.resize_mode_value() {
            SplitPaneResizeMode::PointerOnly => matches!(source, SplitPaneResizeSource::Pointer),
            SplitPaneResizeMode::KeyboardOnly => matches!(source, SplitPaneResizeSource::Keyboard),
            SplitPaneResizeMode::PointerAndKeyboard => true,
            SplitPaneResizeMode::Disabled => false,
        }
    }

    fn start_resize(&mut self) -> Vec<SplitPaneEvent> {
        if !self.can_resize_from(SplitPaneResizeSource::Pointer) {
            return self.rejected_event();
        }
        self.interaction.dragging = true;
        vec![SplitPaneEvent::ResizeStarted {
            target: self.state_id.clone(),
        }]
    }

    fn end_resize(&mut self) -> Vec<SplitPaneEvent> {
        self.interaction.dragging = false;
        vec![SplitPaneEvent::ResizeEnded {
            target: self.state_id.clone(),
        }]
    }

    fn resize_by(
        &mut self,
        delta_percent: i8,
        source: SplitPaneResizeSource,
    ) -> Vec<SplitPaneEvent> {
        let current = i16::from(self.ratio_percent_value());
        let requested = current + i16::from(delta_percent);
        let percent = u8::try_from(requested.clamp(0, 100)).unwrap_or(0);
        self.set_ratio_with_event(percent, source)
    }

    fn reset_ratio_with_event(&mut self) -> Vec<SplitPaneEvent> {
        if matches!(self.resize_mode_value(), SplitPaneResizeMode::Disabled) {
            return self.rejected_event();
        }
        let percent = self.reset_percent;
        self.set_ratio_percent(percent);
        self.interaction.dismiss_reason.clear();
        vec![SplitPaneEvent::RatioChanged {
            target: self.state_id.clone(),
            ratio_percent: percent,
            clamped: false,
            source: SplitPaneResizeSource::Pointer,
        }]
    }

    pub(super) fn set_ratio_with_event(
        &mut self,
        percent: u8,
        source: SplitPaneResizeSource,
    ) -> Vec<SplitPaneEvent> {
        if !self.can_resize_from(source) {
            return self.rejected_event();
        }
        let clamped = self.clamped(percent);
        self.set_ratio_percent(percent);
        self.interaction.dismiss_reason = if clamped == percent {
            String::new()
        } else {
            format!("clamped:{percent}->{clamped}")
        };
        vec![SplitPaneEvent::RatioChanged {
            target: self.state_id.clone(),
            ratio_percent: clamped,
            clamped: clamped != percent,
            source,
        }]
    }

    fn rejected_event(&mut self) -> Vec<SplitPaneEvent> {
        let reason = if matches!(self.resize_mode_value(), SplitPaneResizeMode::Disabled) {
            SplitPaneRejectionReason::ResizeDisabled
        } else {
            SplitPaneRejectionReason::SourceNotAllowed
        };
        self.interaction.dismiss_reason = format!("ResizeRejected:{reason:?}");
        vec![SplitPaneEvent::ResizeRejected {
            target: self.state_id.clone(),
            reason,
        }]
    }
}
