use super::{StartupState, StartupStatePanel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartupStatePanelAction {
    SetState(StartupState),
    Retry,
    Cancel,
    SetReducedMotion(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartupStatePanelEvent {
    StartupStateChanged {
        from: StartupState,
        to: StartupState,
    },
    StartupRetried,
    StartupCanceled,
}

pub(super) fn apply(
    panel: &mut StartupStatePanel,
    action: StartupStatePanelAction,
) -> Vec<StartupStatePanelEvent> {
    match action {
        StartupStatePanelAction::SetState(next) => set_state(panel, next),
        StartupStatePanelAction::Retry => vec![StartupStatePanelEvent::StartupRetried],
        StartupStatePanelAction::Cancel => vec![StartupStatePanelEvent::StartupCanceled],
        StartupStatePanelAction::SetReducedMotion(value) => {
            panel.options.reduced_motion = value;
            Vec::new()
        }
    }
}

fn set_state(panel: &mut StartupStatePanel, next: StartupState) -> Vec<StartupStatePanelEvent> {
    if panel.state == next {
        return Vec::new();
    }
    let from = panel.state.clone();
    panel.state = next.clone();
    vec![StartupStatePanelEvent::StartupStateChanged { from, to: next }]
}
