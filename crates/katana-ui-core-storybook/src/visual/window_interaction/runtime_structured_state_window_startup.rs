use super::{RuntimeStructuredUpdate, StartupStateRuntimeState, WindowControlRuntimeState};

impl WindowControlRuntimeState {
    pub(in crate::visual) fn press_close(&mut self) -> RuntimeStructuredUpdate {
        self.pressed_close = window_control_press_is_close();
        RuntimeStructuredUpdate::new(
            "window_control_press",
            "window_control_pressed",
            window_control_close_label(self.pressed_close),
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        RuntimeStructuredUpdate::new("window_control_focus", "focus", "focus=Close")
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hover_visible = window_control_hover_makes_visible();
        RuntimeStructuredUpdate::new(
            "window_control_hover",
            "window_control_visibility_changed",
            window_control_visibility_label(self.hover_visible),
        )
    }

    pub(in crate::visual) fn keyboard_restore(&mut self) -> RuntimeStructuredUpdate {
        self.keyboard_restore = window_control_keyboard_is_restore();
        RuntimeStructuredUpdate::new(
            "window_control_keyboard_restore",
            "window_control_pressed",
            window_control_restore_label(self.keyboard_restore),
        )
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "window_control.position" => self.position_trailing = true,
            "window_control.size" => self.size_tall = true,
            "window_control.controls" => self.controls_close_only = true,
            "window_control.visibility" => self.visibility_hover = true,
            _ => {}
        }
    }
}

fn window_control_press_is_close() -> bool {
    use katana_ui_core::molecule::selection::window_control_button_group::{
        WindowControlButtonGroup, WindowControlButtonGroupAction, WindowControlButtonGroupEvent,
        WindowControlKind,
    };

    let mut group = WindowControlButtonGroup::new("Window controls");
    let events = group.apply_action(WindowControlButtonGroupAction::Press(
        WindowControlKind::Close,
    ));
    events
        == [WindowControlButtonGroupEvent::ControlPressed {
            which: WindowControlKind::Close,
        }]
}

fn window_control_hover_makes_visible() -> bool {
    use katana_ui_core::molecule::selection::window_control_button_group::{
        WindowControlButtonGroup, WindowControlButtonGroupAction, WindowControlButtonGroupEvent,
        WindowControlButtonGroupOptions, WindowControlVisibility,
    };

    let mut group =
        WindowControlButtonGroup::new("Window controls").options(WindowControlButtonGroupOptions {
            visibility: WindowControlVisibility::Hover,
            ..WindowControlButtonGroupOptions::default()
        });
    let events = group.apply_action(WindowControlButtonGroupAction::SetHover(true));
    group.state().visible()
        && events == [WindowControlButtonGroupEvent::VisibilityChanged { visible: true }]
}

fn window_control_keyboard_is_restore() -> bool {
    use katana_ui_core::molecule::selection::window_control_button_group::{
        WindowControlButtonGroup, WindowControlButtonGroupAction, WindowControlButtonGroupEvent,
        WindowControlKind,
    };

    let mut group = WindowControlButtonGroup::new("Window controls");
    let events = group.apply_action(WindowControlButtonGroupAction::Press(
        WindowControlKind::Restore,
    ));
    events
        == [WindowControlButtonGroupEvent::ControlPressed {
            which: WindowControlKind::Restore,
        }]
}

impl StartupStateRuntimeState {
    pub(in crate::visual) fn preview_error(&mut self) -> RuntimeStructuredUpdate {
        self.error = startup_state_set_error_changed();
        RuntimeStructuredUpdate::new(
            "startup_state_error",
            "startup_state_changed",
            startup_error_label(self.error),
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        RuntimeStructuredUpdate::new("startup_state_focus", "focus", "focus=retry")
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hovered = true;
        RuntimeStructuredUpdate::new("startup_state_hover", "hover_start", "hover=retry")
    }

    pub(in crate::visual) fn keyboard_retry(&mut self) -> RuntimeStructuredUpdate {
        self.retried = startup_state_retry_event();
        RuntimeStructuredUpdate::new(
            "startup_state_keyboard_retry",
            "startup_retried",
            startup_retry_label(self.retried),
        )
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "startup_state.state" => self.error = true,
            "startup_state.retry" => self.retried = true,
            "startup_state.cancel" => self.canceled = true,
            _ => {}
        }
    }
}

const fn window_control_close_label(pressed: bool) -> &'static str {
    if pressed {
        "pressed=Close"
    } else {
        "pressed=unknown"
    }
}

const fn window_control_visibility_label(visible: bool) -> &'static str {
    if visible {
        "visible=true"
    } else {
        "visible=false"
    }
}

const fn window_control_restore_label(restored: bool) -> &'static str {
    if restored {
        "pressed=Restore"
    } else {
        "pressed=unknown"
    }
}

const fn startup_error_label(error: bool) -> &'static str {
    if error { "retry=true" } else { "retry=false" }
}

const fn startup_retry_label(retried: bool) -> &'static str {
    if retried {
        "retry=requested"
    } else {
        "retry=ignored"
    }
}

fn startup_state_set_error_changed() -> bool {
    use katana_ui_core::molecule::structured::startup_state_panel::{
        StartupState, StartupStatePanel, StartupStatePanelAction, StartupStatePanelEvent,
    };

    let mut panel = StartupStatePanel::new("Startup");
    let events = panel.apply_action(StartupStatePanelAction::SetState(StartupState::error(
        "Could not open workspace",
        true,
        true,
    )));
    matches!(
        events.as_slice(),
        [StartupStatePanelEvent::StartupStateChanged {
            from: StartupState::Idle,
            to: StartupState::Error { retry: true, .. }
        }]
    )
}

fn startup_state_retry_event() -> bool {
    use katana_ui_core::molecule::structured::startup_state_panel::{
        StartupState, StartupStatePanel, StartupStatePanelAction, StartupStatePanelEvent,
    };

    let mut panel = StartupStatePanel::new("Startup").state(StartupState::error(
        "Could not open workspace",
        true,
        true,
    ));
    let events = panel.apply_action(StartupStatePanelAction::Retry);
    events == [StartupStatePanelEvent::StartupRetried]
}

#[cfg(test)]
mod tests {
    use super::{
        WindowControlRuntimeState, startup_error_label, startup_retry_label,
        window_control_close_label, window_control_restore_label, window_control_visibility_label,
    };

    #[test]
    fn window_and_startup_labels_cover_fallbacks_and_unknown_window_options_are_noops() {
        assert_eq!("pressed=Close", window_control_close_label(true));
        assert_eq!("pressed=unknown", window_control_close_label(false));
        assert_eq!("visible=true", window_control_visibility_label(true));
        assert_eq!("visible=false", window_control_visibility_label(false));
        assert_eq!("pressed=Restore", window_control_restore_label(true));
        assert_eq!("pressed=unknown", window_control_restore_label(false));
        assert_eq!("retry=true", startup_error_label(true));
        assert_eq!("retry=false", startup_error_label(false));
        assert_eq!("retry=requested", startup_retry_label(true));
        assert_eq!("retry=ignored", startup_retry_label(false));

        let mut state = WindowControlRuntimeState::default();
        state.apply_option("unknown.setting");
        assert_eq!(WindowControlRuntimeState::default(), state);
    }
}
