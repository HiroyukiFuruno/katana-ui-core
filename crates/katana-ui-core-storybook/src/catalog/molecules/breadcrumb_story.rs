use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::{atom, molecule};

pub(super) fn story() -> StoryExample {
    let breadcrumb = molecule::Breadcrumb::new("Breadcrumb")
        .item(molecule::ChoiceItem::new("root", "Root"))
        .item(molecule::ChoiceItem::new("src", "src"))
        .item(molecule::ChoiceItem::new("lib", "lib.rs"))
        .crumb_action("breadcrumb_click")
        .long_list(true)
        .open(true)
        .placement("bottom-start")
        .selected_index(0)
        .value("root")
        .child(atom::Text::new("Root"))
        .child(atom::Text::new("src"))
        .child(atom::Text::new("lib.rs"));
    let target = breadcrumb.state_id().clone();
    let mut probe = breadcrumb.clone();
    let result = probe.apply_action(&UiAction::select_box_selected(target.clone(), 2));
    let mut logs = result.callback_log;
    logs.push(UiCallbackLog::new(
        target,
        "breadcrumb_click",
        "interaction.selected_index=0 route=root",
        "event=route_changed interaction.selected_index=2 route=lib",
    ));

    StoryCatalog::interactive_story("breadcrumb", breadcrumb, logs)
}
