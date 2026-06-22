use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::UiNode;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

const FILES_ID: &str = "files";
const SETTINGS_ID: &str = "settings";

pub(super) fn story() -> StoryExample {
    let menu = molecule::SideMenu::new("Side menu")
        .item(molecule::ChoiceItem::new(FILES_ID, "Files"))
        .item(molecule::ChoiceItem::new(SETTINGS_ID, "Settings"))
        .open(true)
        .hover_expansion(true)
        .selected_index(0)
        .value(FILES_ID)
        .child(atom::Text::new("Files route selected"))
        .child(atom::Badge::new("Hover expand enabled"));
    let state = menu_state(&menu);
    let target = menu.state_id().clone();
    let logs = vec![
        callback_log(&target, "side_menu_state_read", &state, &state),
        callback_log(
            &target,
            "select_box_selected",
            &format!("selected_index=0 route={}", FILES_ID),
            &format!("selected_index=1 route={}", SETTINGS_ID),
        ),
        callback_log(
            &target,
            "side_menu_hover_expand",
            &format!(
                "hover_expansion={} hovered={}",
                menu.hover_expansion_model(),
                false
            ),
            "hover_expansion=true hovered=true",
        ),
    ];

    StoryCatalog::interactive_story("side-menu", menu, logs)
}

fn menu_state(menu: &molecule::SideMenu) -> String {
    let node: UiNode = menu.clone().into();
    let props = &node.props().interaction;
    format!(
        "open={} collapsed={} selected_index={} route={} hover_expansion={} hover={}",
        props.open,
        !props.open,
        props.selected_index,
        props.value,
        menu.hover_expansion_model(),
        props.hovered
    )
}

fn callback_log(target: &UiStateId, action: &str, before: &str, after: &str) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
