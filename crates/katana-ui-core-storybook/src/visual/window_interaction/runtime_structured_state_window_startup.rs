use super::{RuntimeStructuredUpdate, StartupStateRuntimeState, WindowControlRuntimeState};

impl WindowControlRuntimeState {
    pub(in crate::visual) fn press_close(&mut self) -> RuntimeStructuredUpdate {
        self.pressed_close = window_control_press_is_close();
        RuntimeStructuredUpdate::new(
            "window_control_press",
            "window_control_pressed",
            if self.pressed_close {
                "pressed=Close"
            } else {
                "pressed=unknown"
            },
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
            if self.hover_visible {
                "visible=true"
            } else {
                "visible=false"
            },
        )
    }

    pub(in crate::visual) fn keyboard_restore(&mut self) -> RuntimeStructuredUpdate {
        self.keyboard_restore = window_control_keyboard_is_restore();
        RuntimeStructuredUpdate::new(
            "window_control_keyboard_restore",
            "window_control_pressed",
            if self.keyboard_restore {
                "pressed=Restore"
            } else {
                "pressed=unknown"
            },
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
            if self.error {
                "retry=true"
            } else {
                "retry=false"
            },
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
            if self.retried {
                "retry=requested"
            } else {
                "retry=ignored"
            },
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
