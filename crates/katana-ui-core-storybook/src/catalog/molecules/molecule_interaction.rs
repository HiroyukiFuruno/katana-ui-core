use super::super::{StoryCatalog, StoryExample};
mod accordion_story;
mod hover_card_story;
mod modal_story;
mod popover_story;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::placement::Placement;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::{UiDismissAction, UiNodeId, UiStateId, UiTone};
use katana_ui_core::{atom, layout, molecule};

const TOOLTIP_DELAY_MS: u16 = 240;
const TOOLTIP_MAX_WIDTH: u16 = 280;
const FIRST_OPTION_INDEX: usize = 0;
const SECOND_OPTION_INDEX: usize = 1;
const HOVER_CARD_OPEN_DELAY_MS: u16 = 100;
const HOVER_CARD_CLOSE_DELAY_MS: u16 = 50;
const ACCORDION_TREE_DEPTH: u8 = 2;
const POPOVER_OFFSET_X: i16 = 12;
const POPOVER_OFFSET_Y: i16 = 8;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        menu_story(),
        tooltip_story(),
        modal_story::modal_story(),
        accordion_story::accordion_story(),
        combo_box_story(),
        menu_button_story(),
        modal_story::modal_overlay_story(),
        notification_toast_story(),
        popover_story::popover_story(),
        hover_card_story::hover_card_story(),
        segmented_toggle_story(),
        select_box_story(),
    ]
}

fn menu_story() -> StoryExample {
    let menu = molecule::Menu::new("Menu")
        .child(atom::Button::new("Open"))
        .child(atom::Button::new("Close"))
        .child(atom::Button::new("Disabled").disabled(true));
    let node: katana_ui_core::render_model::UiNode = menu.clone().into();
    let target = node.props().state_id.clone();
    let logs = vec![
        UiCallbackLog::new(target.clone(), "menu_open", "open=false", "open=true"),
        UiCallbackLog::new(target.clone(), "menu_close", "open=true", "open=false"),
        UiCallbackLog::new(
            target.clone(),
            "menu_select",
            "selected=none",
            "selected=open",
        ),
        UiCallbackLog::new(
            target,
            "menu_shortcut_activate",
            "shortcut=Cmd+O",
            "selected=open",
        ),
    ];
    StoryCatalog::interactive_story("menu", menu, logs)
}

fn tooltip_story() -> StoryExample {
    let mut tooltip = molecule::Tooltip::new("Tooltip")
        .hover_trigger(true)
        .delay_ms(TOOLTIP_DELAY_MS)
        .max_width(TOOLTIP_MAX_WIDTH)
        .child(atom::Icon::new("Info"))
        .child(atom::Text::new("Hint"));
    let target = tooltip.state_id().clone();
    let result = tooltip.apply_action(&UiAction::hover(target, true));
    StoryCatalog::interactive_story("tooltip", tooltip, result.callback_log)
}

fn combo_box_story() -> StoryExample {
    let combo = molecule::ComboBox::new("Combo box")
        .item(molecule::ChoiceItem::new("one", "One"))
        .item(molecule::ChoiceItem::new("two", "Two"))
        .child(atom::Input::new("Search"))
        .child(atom::Text::new("Option"));
    let target = combo.state_id().clone();
    let mut logs = vec![UiCallbackLog::new(
        target.clone(),
        "combo_state_read",
        "open=false query=empty selected=none",
        "open=false query=empty selected=none",
    )];
    logs.push(UiCallbackLog::new(
        target.clone(),
        "combo_filter",
        "query=empty",
        "query=tw",
    ));
    let mut log_select = combo.clone().open(true).value("tw");
    let result = log_select.apply_action(&UiAction::select_box_selected(
        target.clone(),
        SECOND_OPTION_INDEX,
    ));
    logs.extend(result.callback_log);
    logs.push(UiCallbackLog::new(
        target.clone(),
        "combo_select",
        "selected=none",
        "selected=two",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "combo_reset",
        "query=tw selected=two",
        "query=empty selected=none",
    ));
    let typed: katana_ui_core::render_model::UiNode = combo.clone().into();
    let typed_state = typed.props().interaction.summary();
    let harness = layout::Column::new()
        .child(combo)
        .child(
            layout::Row::new()
                .child(atom::Button::new("state read"))
                .child(atom::Button::new("filter"))
                .child(atom::Button::new("select two"))
                .child(atom::Button::new("reset")),
        )
        .child(atom::Text::new(format!(
            "typed state: state_id={} {}",
            target.as_str(),
            typed_state
        )));
    StoryCatalog::interactive_story("combo-box", harness, logs)
}

fn menu_button_story() -> StoryExample {
    let mut menu = molecule::MenuButton::new("Menu button")
        .open(true)
        .item(molecule::ChoiceItem::new("open", "Open"))
        .child(atom::Button::new("Trigger"))
        .child(molecule::Menu::new("Menu"));
    let target = menu.state_id().clone();
    let result = menu.apply_action(&UiAction::select_box_selected(target, FIRST_OPTION_INDEX));
    StoryCatalog::interactive_story("menu-button", menu, result.callback_log)
}

fn notification_toast_story() -> StoryExample {
    let mut toast = molecule::NotificationToast::new("Notification")
        .open(true)
        .severity(UiTone::Warning)
        .timer_summary("duration=5000ms")
        .dismiss_action(UiDismissAction::Available)
        .child(atom::Badge::new("Info"))
        .child(atom::Text::new("Message"))
        .child(atom::Button::new("Undo"));
    let target = toast.state_id().clone();
    let result = toast.apply_action(&UiAction::dismiss(target));
    StoryCatalog::interactive_story("notification-toast", toast, result.callback_log)
}

fn segmented_toggle_story() -> StoryExample {
    let mut segmented = molecule::SegmentedToggle::new("Segmented toggle")
        .item(molecule::ChoiceItem::new("preview", "Preview"))
        .item(molecule::ChoiceItem::new("code", "Code"))
        .selected_index(1)
        .child(atom::Toggle::new("Preview"))
        .child(atom::Toggle::new("Code"));
    let target = segmented.state_id().clone();
    let result = segmented.apply_action(&UiAction::segmented_toggle_selected(target, 0));
    StoryCatalog::interactive_story("segmented-toggle", segmented, result.callback_log)
}

fn select_box_story() -> StoryExample {
    let select = molecule::SelectBox::new("Select box")
        .placement("bottom-start")
        .item(molecule::ChoiceItem::new("light", "Light"))
        .item(molecule::ChoiceItem::new("dark", "Dark"))
        .item(molecule::ChoiceItem::new("system", "System"))
        .child(atom::Button::new("Trigger"))
        .child(molecule::List::new("Options"));
    let target = select.state_id().clone();
    let mut logs = vec![UiCallbackLog::new(
        target.clone(),
        "select_state_read",
        "open=false selected=none",
        "open=false selected=none",
    )];
    logs.push(UiCallbackLog::new(
        target.clone(),
        "select_open",
        "open=false",
        "open=true",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "select_close",
        "open=true",
        "open=false",
    ));
    let mut log_select = select.clone();
    let result = log_select.apply_action(&UiAction::select_box_selected(
        target.clone(),
        SECOND_OPTION_INDEX,
    ));
    logs.extend(result.callback_log);
    logs.push(UiCallbackLog::new(
        target.clone(),
        "select_option",
        "selected=none",
        "selected=dark",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "select_reset",
        "selected=dark",
        "selected=none",
    ));
    let typed: katana_ui_core::render_model::UiNode = select.clone().into();
    let typed_state = typed.props().interaction.summary();
    let harness = layout::Column::new()
        .child(select)
        .child(
            layout::Row::new()
                .child(atom::Button::new("state read"))
                .child(atom::Button::new("open"))
                .child(atom::Button::new("close"))
                .child(atom::Button::new("select dark"))
                .child(atom::Button::new("reset")),
        )
        .child(atom::Text::new(format!(
            "typed state: state_id={} {}",
            target.as_str(),
            typed_state
        )));
    StoryCatalog::interactive_story("select-box", harness, logs)
}
