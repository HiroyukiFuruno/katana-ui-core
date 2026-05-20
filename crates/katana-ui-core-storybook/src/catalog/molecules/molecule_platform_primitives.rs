use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule;
use katana_ui_core::molecule::{
    StartupState, StartupStatePanelAction, WindowControlButtonGroupAction,
    WindowControlButtonGroupOptions, WindowControlKind, WindowControlSize, WindowControlVisibility,
    WindowControlsPosition,
};

const STARTUP_PROGRESS: u8 = 64;
const WINDOW_CONTROL_TARGET: &str = "state:WindowControlButtonGroup:storybook";

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        window_control_button_group_story(),
        startup_state_panel_story(),
    ]
}

fn window_control_button_group_story() -> StoryExample {
    let presets = window_control_presets();
    let root = presets.iter().fold(
        molecule::List::new("Window control button group presets"),
        |list, preset| list.child(preset.preview()),
    );
    let logs = presets
        .iter()
        .map(WindowControlPreset::callback_log)
        .collect();

    StoryCatalog::interactive_story("window-control-button-group", root, logs)
}

fn window_control_presets() -> [WindowControlPreset; 5] {
    [
        WindowControlPreset {
            name: "macOS",
            action: "window_control_press",
            options: WindowControlButtonGroupOptions {
                controls: desktop_controls(),
                position: WindowControlsPosition::Leading,
                visibility: WindowControlVisibility::Always,
                size: WindowControlSize::Compact,
            },
            action_kind: WindowControlButtonGroupAction::Press(WindowControlKind::Close),
        },
        WindowControlPreset {
            name: "Windows",
            action: "window_controls_trailing_press",
            options: WindowControlButtonGroupOptions {
                controls: desktop_controls(),
                position: WindowControlsPosition::Trailing,
                visibility: WindowControlVisibility::Always,
                size: WindowControlSize::Default,
            },
            action_kind: WindowControlButtonGroupAction::Press(WindowControlKind::Maximize),
        },
        WindowControlPreset {
            name: "Linux",
            action: "window_controls_hover",
            options: WindowControlButtonGroupOptions {
                controls: vec![
                    WindowControlKind::Minimize,
                    WindowControlKind::Maximize,
                    WindowControlKind::Restore,
                    WindowControlKind::Close,
                ],
                position: WindowControlsPosition::Auto,
                visibility: WindowControlVisibility::Hover,
                size: WindowControlSize::Tall,
            },
            action_kind: WindowControlButtonGroupAction::SetHover(true),
        },
        WindowControlPreset {
            name: "fullscreen hover",
            action: "window_controls_fullscreen",
            options: WindowControlButtonGroupOptions {
                controls: desktop_controls(),
                position: WindowControlsPosition::Leading,
                visibility: WindowControlVisibility::FullscreenHover,
                size: WindowControlSize::Compact,
            },
            action_kind: WindowControlButtonGroupAction::SetFullscreen(true),
        },
        WindowControlPreset {
            name: "close only",
            action: "window_controls_close_only",
            options: WindowControlButtonGroupOptions {
                controls: vec![WindowControlKind::Close],
                position: WindowControlsPosition::Trailing,
                visibility: WindowControlVisibility::Always,
                size: WindowControlSize::Default,
            },
            action_kind: WindowControlButtonGroupAction::Press(WindowControlKind::Close),
        },
    ]
}

fn desktop_controls() -> Vec<WindowControlKind> {
    vec![
        WindowControlKind::Close,
        WindowControlKind::Minimize,
        WindowControlKind::Maximize,
    ]
}

struct WindowControlPreset {
    name: &'static str,
    action: &'static str,
    options: WindowControlButtonGroupOptions,
    action_kind: WindowControlButtonGroupAction,
}

impl WindowControlPreset {
    fn preview(&self) -> molecule::WindowControlButtonGroup {
        molecule::WindowControlButtonGroup::new(self.preview_label()).options(self.options.clone())
    }

    fn preview_label(&self) -> String {
        format!(
            "preset={} position={:?} size={:?} controls={} visibility={:?}",
            self.name,
            self.options.position,
            self.options.size,
            self.control_names(),
            self.options.visibility
        )
    }

    fn callback_log(&self) -> UiCallbackLog {
        let target = katana_ui_core::render_model::UiStateId::new(WINDOW_CONTROL_TARGET);
        let mut group =
            molecule::WindowControlButtonGroup::new(self.name).options(self.options.clone());
        let events = group.apply_action(self.action_kind);
        let after = if matches!(
            self.options.visibility,
            WindowControlVisibility::FullscreenHover
        ) {
            let hover_events = group.apply_action(WindowControlButtonGroupAction::SetHover(true));
            format!("events={events:?}+{hover_events:?}")
        } else {
            format!("events={events:?}")
        };

        UiCallbackLog::new(target, self.action, self.before_state(), after)
    }

    fn before_state(&self) -> String {
        format!(
            "preset={} position={:?} size={:?} controls={} visibility={:?} state=visible",
            self.name,
            self.options.position,
            self.options.size,
            self.control_names(),
            self.options.visibility
        )
    }

    fn control_names(&self) -> String {
        self.options
            .controls
            .iter()
            .map(|it| format!("{it:?}"))
            .collect::<Vec<_>>()
            .join("+")
    }
}

fn startup_state_panel_story() -> StoryExample {
    let mut panel = molecule::StartupStatePanel::new("Startup state panel").state(
        StartupState::loading(Some(STARTUP_PROGRESS), Some("Loading workspace")),
    );
    let target = panel.state_id().clone();
    let changed = panel.apply_action(StartupStatePanelAction::SetState(StartupState::error(
        "Workspace failed to open",
        true,
        true,
    )));
    let retried = panel.apply_action(StartupStatePanelAction::Retry);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "startup_state_error",
            "state=Loading progress=64",
            format!("events={changed:?}"),
        ),
        UiCallbackLog::new(
            target,
            "startup_retry",
            "state=Error",
            format!("events={retried:?}"),
        ),
    ];
    StoryCatalog::interactive_story("startup-state-panel", panel, logs)
}
