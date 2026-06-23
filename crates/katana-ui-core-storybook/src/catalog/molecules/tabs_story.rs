use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::{atom, molecule};

pub(super) fn story() -> StoryExample {
    let tabs = molecule::Tabs::new("Tabs")
        .item(molecule::ChoiceItem::new("preview", "Preview"))
        .item(molecule::ChoiceItem::new("output", "Output"))
        .item(molecule::ChoiceItem::new("settings", "Settings"))
        .icon_action("tabs_icon_action")
        .selected_index(0)
        .value("preview")
        .child(atom::Text::new("Preview tab"))
        .child(atom::Text::new("Output panel"))
        .child(atom::Text::new("Settings panel"));
    let target = tabs.state_id().clone();
    let mut probe = tabs.clone();
    let result = probe.apply_action(&UiAction::select_box_selected(target.clone(), 1));
    let mut logs = result.callback_log;
    logs.push(UiCallbackLog::new(
        target,
        "tab_select",
        "selected_index=0 tab=preview",
        "event=tab_changed selected_index=1 tab=output",
    ));

    StoryCatalog::interactive_story("tabs", tabs, logs)
}
