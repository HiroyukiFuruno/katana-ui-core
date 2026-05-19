use super::{StartupState, StartupStatePanel};
use crate::atom::{Button, LoadingDots, ProgressBar, Text};
use crate::interaction::{MotionContext, MotionDisableContext, MotionResolver};
use crate::render_model::{
    UiAnimationState, UiInteractionState, UiLoadingProps, UiNode, UiNodeKind, UiProgressMode,
    UiVisualRole,
};

const RETRY_LABEL: &str = "Retry";
const CANCEL_LABEL: &str = "Cancel";
const IDLE_LABEL: &str = "Idle";

impl From<StartupStatePanel> for UiNode {
    fn from(value: StartupStatePanel) -> Self {
        let mut node = root_node(&value);
        match &value.state {
            StartupState::Idle => node = node.child(Text::new(IDLE_LABEL)),
            StartupState::Loading { label, .. } => node = render_loading(node, &value, label),
            StartupState::Error {
                message,
                retry,
                cancel,
            } => node = render_error(node, message, *retry, *cancel),
        }
        node
    }
}

fn root_node(value: &StartupStatePanel) -> UiNode {
    let progress = value.state.progress_percent();
    let role = value.accessibility_role();
    UiNode::from_state(
        UiNodeKind::StartupStatePanel,
        value.label.clone(),
        value.state_id.clone(),
    )
    .visual_role(UiVisualRole::Status)
    .loading(matches!(value.state, StartupState::Loading { .. }))
    .progress(progress.is_some(), progress.unwrap_or_default())
    .loading_indicator(loading_props(
        progress,
        &value.options.live_region_label,
        value.options.reduced_motion,
    ))
    .interaction(interaction_state(value, progress))
    .accessibility_label(accessibility_label(role, value.live_region_label_model()))
}

fn render_loading(node: UiNode, value: &StartupStatePanel, label: &Option<String>) -> UiNode {
    let label_text = label.as_deref().unwrap_or(value.live_region_label_model());
    let progress = value.state.progress_percent();
    node.child(
        ProgressBar::new(label_text)
            .progress(progress.is_some(), progress.unwrap_or_default())
            .loading_label(label_text)
            .reduced_motion(value.options.reduced_motion)
            .animation_state(animation_state(value))
            .speed_ms(animation_speed(value)),
    )
    .child(
        LoadingDots::new(label_text)
            .loading_label(label_text)
            .reduced_motion(value.options.reduced_motion)
            .animation_state(animation_state(value))
            .speed_ms(animation_speed(value)),
    )
}

fn render_error(node: UiNode, message: &str, retry: bool, cancel: bool) -> UiNode {
    let mut rendered = node.child(Text::new(message));
    if retry {
        rendered = rendered.child(Button::new(RETRY_LABEL).focusable(true));
    }
    if cancel {
        rendered = rendered.child(Button::new(CANCEL_LABEL));
    }
    rendered
}

fn interaction_state(value: &StartupStatePanel, progress: Option<u8>) -> UiInteractionState {
    UiInteractionState {
        active: matches!(value.state, StartupState::Loading { .. }),
        focused: matches!(value.state, StartupState::Error { retry: true, .. }),
        reduced_motion: value.options.reduced_motion,
        value: progress.unwrap_or_default().to_string(),
        ..UiInteractionState::default()
    }
}

fn loading_props(progress: Option<u8>, label: &str, reduced_motion: bool) -> UiLoadingProps {
    UiLoadingProps {
        mode: if progress.is_some() {
            UiProgressMode::Determinate
        } else {
            UiProgressMode::Indeterminate
        },
        label: label.to_string(),
        animation_state: if reduced_motion {
            UiAnimationState::Idle
        } else {
            UiAnimationState::Running
        },
        speed_ms: UiLoadingProps::default().speed_ms,
        reduced_motion,
        ..UiLoadingProps::default()
    }
}

fn animation_state(value: &StartupStatePanel) -> UiAnimationState {
    if motion_snapshot(value).instant {
        return UiAnimationState::Idle;
    }
    UiAnimationState::Running
}

fn animation_speed(value: &StartupStatePanel) -> u16 {
    motion_snapshot(value).duration_ms
}

fn motion_snapshot(value: &StartupStatePanel) -> crate::interaction::MotionSnapshot {
    MotionResolver::compute(
        &value.options.motion,
        MotionContext {
            reduced_motion: value.options.reduced_motion,
            surface: MotionDisableContext::Test,
        },
    )
}

fn accessibility_label(role: &str, label: &str) -> String {
    if role == "alert" {
        return role.to_string();
    }
    format!("{role}: {label}")
}
