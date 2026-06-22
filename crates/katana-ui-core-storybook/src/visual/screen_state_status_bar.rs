use super::screen_state::StorybookScreenState;
use katana_ui_core::molecule::{
    StatusBar, StatusBarAction, StatusBarDensity, StatusBarEvent, StatusBarMode,
    StatusBarPopoverSpec, StatusBarSegment,
};

impl StorybookScreenState {
    pub(in crate::visual) fn register_status_bar_segment_click(&mut self, index: usize) {
        let id = segment_id(index);
        let events =
            apply_core_status_bar_action(StatusBarAction::PressSegment { id: id.to_string() });
        self.action_count += 1;
        self.status_bar_open_segment_index = Some(index);
        self.last_action = "status_bar_segment_popover";
        self.last_event = status_bar_event_label(&events);
        self.last_setting = "status_bar.open_popover";
        self.last_setting_value = open_popover_value(index);
        self.state_label = open_popover_state(index);
    }

    pub(in crate::visual) fn register_status_bar_segment_hover(&mut self, index: usize) {
        let id = segment_id(index);
        let events =
            apply_core_status_bar_action(StatusBarAction::ShowTooltip { id: id.to_string() });
        self.action_count += 1;
        self.status_bar_hovered_segment_index = Some(index);
        self.preview_hovered = true;
        self.last_action = "status_bar_segment_hover";
        self.last_event = status_bar_event_label(&events);
        self.state_label = hover_state(index);
    }

    pub(in crate::visual) fn register_status_bar_segment_focus(&mut self, index: usize) {
        self.action_count += 1;
        self.status_bar_focused_segment_index = Some(index);
        self.button_focused = true;
        self.last_action = "status_bar_segment_focus";
        self.last_event = "focus";
        self.state_label = focus_state(index);
    }

    pub(in crate::visual) fn register_status_bar_keyboard_activate(&mut self) {
        if !self.button_focused {
            self.last_action = "status_bar_keyboard_without_focus";
            self.last_event = "status_bar_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        let index = self.status_bar_focused_segment_index.unwrap_or(0);
        let id = segment_id(index);
        let events =
            apply_core_status_bar_action(StatusBarAction::ActivateSegment { id: id.to_string() });
        self.action_count += 1;
        self.status_bar_open_segment_index = Some(index);
        self.last_action = "status_bar_keyboard_activate";
        self.last_event = status_bar_event_label(&events);
        self.last_setting = "status_bar.open_popover";
        self.last_setting_value = open_popover_value(index);
        self.state_label = open_popover_state(index);
    }
}

fn apply_core_status_bar_action(action: StatusBarAction) -> Vec<StatusBarEvent> {
    let mut status_bar = StatusBar::new("Storybook status")
        .mode(StatusBarMode::MultiSegment)
        .density(StatusBarDensity::Compact)
        .segment(
            StatusBarSegment::new("branch", "main")
                .popover(StatusBarPopoverSpec::new("Branch", "main is ahead by 1")),
        )
        .segment(
            StatusBarSegment::new("usage", "42%")
                .popover(StatusBarPopoverSpec::new("Usage", "Token budget")),
        )
        .segment(
            StatusBarSegment::new("progress", "Indexing")
                .popover(StatusBarPopoverSpec::new("Progress", "Indexing workspace")),
        );
    status_bar.apply_action(&action)
}

fn open_popover_value(index: usize) -> &'static str {
    match index {
        0 => "branch",
        1 => "usage",
        _ => "progress",
    }
}

fn segment_id(index: usize) -> &'static str {
    open_popover_value(index)
}

fn open_popover_state(index: usize) -> &'static str {
    match index {
        0 => "open_popover=branch",
        1 => "open_popover=usage",
        _ => "open_popover=progress",
    }
}

fn hover_state(index: usize) -> &'static str {
    match index {
        0 => "tooltip=branch",
        1 => "tooltip=usage",
        _ => "tooltip=progress",
    }
}

fn focus_state(index: usize) -> &'static str {
    match index {
        0 => "focus=branch",
        1 => "focus=usage",
        _ => "focus=progress",
    }
}

fn status_bar_event_label(events: &[StatusBarEvent]) -> &'static str {
    if events
        .iter()
        .any(|event| matches!(event, StatusBarEvent::SegmentPopoverOpened { .. }))
    {
        return "status_bar_popover_opened";
    }
    if events
        .iter()
        .any(|event| matches!(event, StatusBarEvent::SegmentTooltipShown { .. }))
    {
        return "status_bar_tooltip_shown";
    }
    if events
        .iter()
        .any(|event| matches!(event, StatusBarEvent::Dismissed))
    {
        return "status_bar_dismissed";
    }
    "status_bar_segment_pressed"
}
