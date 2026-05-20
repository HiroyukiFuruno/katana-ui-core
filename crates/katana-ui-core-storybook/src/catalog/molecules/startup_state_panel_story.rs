use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule::{self, StartupState, StartupStatePanelAction};

const STARTUP_PROGRESS: u8 = 64;

pub(super) fn example() -> StoryExample {
    let presets = presets();
    let root = presets.iter().fold(
        molecule::List::new("Startup state panel presets"),
        |list, preset| list.child(preset.preview()),
    );

    StoryCatalog::interactive_story("startup-state-panel", root, callback_logs())
}

fn presets() -> [StartupPreset; 4] {
    [
        StartupPreset {
            name: "app boot",
            state: StartupState::Idle,
            version_label: Some("v0.1.0"),
        },
        StartupPreset {
            name: "session init",
            state: StartupState::loading(None, Some("Preparing session")),
            version_label: None,
        },
        StartupPreset {
            name: "update install",
            state: StartupState::loading(Some(STARTUP_PROGRESS), Some("Installing update")),
            version_label: Some("build 2026.05"),
        },
        StartupPreset {
            name: "error retry",
            state: StartupState::error("Workspace failed to open", true, true),
            version_label: Some("v0.1.0"),
        },
    ]
}

fn callback_logs() -> Vec<UiCallbackLog> {
    let mut panel = molecule::StartupStatePanel::new("Startup action source").state(
        StartupState::loading(Some(STARTUP_PROGRESS), Some("Loading workspace")),
    );
    let target = panel.state_id().clone();
    let changed = panel.apply_action(StartupStatePanelAction::SetState(StartupState::error(
        "Workspace failed to open",
        true,
        true,
    )));
    let retried = panel.apply_action(StartupStatePanelAction::Retry);
    let canceled = panel.apply_action(StartupStatePanelAction::Cancel);

    vec![
        UiCallbackLog::new(
            target.clone(),
            "startup_state_error",
            "state=Loading progress=64 label=Loading workspace",
            format!("events={changed:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "startup_retry",
            "state=Error retry=true",
            format!("events={retried:?}"),
        ),
        UiCallbackLog::new(
            target,
            "startup_cancel",
            "state=Error cancel=true",
            format!("events={canceled:?}"),
        ),
    ]
}

struct StartupPreset {
    name: &'static str,
    state: StartupState,
    version_label: Option<&'static str>,
}

impl StartupPreset {
    fn preview(&self) -> molecule::StartupStatePanel {
        let options = molecule::StartupStatePanelOptions::default()
            .live_region_label(self.name)
            .version_label(self.version_label);
        molecule::StartupStatePanel::new(self.preview_label())
            .state(self.state.clone())
            .option(options)
    }

    fn preview_label(&self) -> String {
        format!(
            "preset={} state={} version={}",
            self.name,
            state_name(&self.state),
            self.version_label.unwrap_or("none")
        )
    }
}

fn state_name(state: &StartupState) -> &'static str {
    match state {
        StartupState::Idle => "Idle",
        StartupState::Loading { progress: None, .. } => "LoadingIndeterminate",
        StartupState::Loading {
            progress: Some(_), ..
        } => "LoadingDeterminate",
        StartupState::Error { .. } => "Error",
    }
}
