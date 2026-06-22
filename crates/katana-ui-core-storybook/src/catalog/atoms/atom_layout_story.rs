use super::{StoryCatalog, StoryExample};
use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionSource, UiCallbackLog};
use katana_ui_core::render_model::{UiSize, UiStateId, UiVisualRole};

pub(super) fn divider() -> StoryExample {
    let mut divider = atom::Divider::new("Divider")
        .visual_role(UiVisualRole::Separator)
        .focusable(true)
        .value("inset=true");
    let target = divider.state_id().clone();
    let mut logs = divider
        .apply_action(&set_value_action(&target, "inset=true"))
        .callback_log;
    logs.push(log(
        &target,
        "divider_resize",
        "inset=false orientation=horizontal",
        "event=divider_changed inset=true orientation=horizontal",
    ));
    StoryCatalog::interactive_story("divider", divider, logs)
}

pub(super) fn spacer() -> StoryExample {
    let mut spacer = atom::Spacer::new("Spacer")
        .visual_role(UiVisualRole::Separator)
        .size(UiSize::Large)
        .focusable(true)
        .value("gap=large");
    let target = spacer.state_id().clone();
    let mut logs = spacer
        .apply_action(&set_value_action(&target, "gap=large"))
        .callback_log;
    logs.push(log(
        &target,
        "spacer_resize",
        "gap=medium flexible=false",
        "event=spacer_changed gap=large flexible=true",
    ));
    StoryCatalog::interactive_story("spacer", spacer, logs)
}

fn set_value_action(target: &UiStateId, value: &str) -> UiAction {
    UiAction::SetValue {
        target: target.clone(),
        value: value.to_string(),
        source: UiActionSource::Generic,
        progress: None,
        color_drag: None,
    }
}

fn log(
    target: &UiStateId,
    action: &str,
    before: impl Into<String>,
    after: impl Into<String>,
) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
