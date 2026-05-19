use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule;
use katana_ui_core::molecule::{
    StartupState, StartupStatePanelAction, WindowControlButtonGroupAction,
    WindowControlButtonGroupOptions, WindowControlKind, WindowControlSize, WindowControlVisibility,
    WindowControlsPosition,
};

const STARTUP_PROGRESS: u8 = 64;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        window_control_button_group_story(),
        startup_state_panel_story(),
    ]
}

fn window_control_button_group_story() -> StoryExample {
    let mut group = molecule::WindowControlButtonGroup::new("Window control button group").options(
        WindowControlButtonGroupOptions {
            controls: vec![
                WindowControlKind::Close,
                WindowControlKind::Minimize,
                WindowControlKind::Maximize,
            ],
            position: WindowControlsPosition::Leading,
            visibility: WindowControlVisibility::FullscreenHover,
            size: WindowControlSize::Compact,
        },
    );
    let target = group.state().state_id().clone();
    let fullscreen = group.apply_action(WindowControlButtonGroupAction::SetFullscreen(true));
    let hover = group.apply_action(WindowControlButtonGroupAction::SetHover(true));
    let close = group.apply_action(WindowControlButtonGroupAction::Press(
        WindowControlKind::Close,
    ));
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "window_controls_fullscreen",
            "fullscreen=false visible=true",
            format!("events={fullscreen:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "window_controls_hover",
            "visible=false",
            format!("events={hover:?}"),
        ),
        UiCallbackLog::new(
            target,
            "window_control_press",
            "pressed=None",
            format!("events={close:?}"),
        ),
    ];
    StoryCatalog::interactive_story("window-control-button-group", group, logs)
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
