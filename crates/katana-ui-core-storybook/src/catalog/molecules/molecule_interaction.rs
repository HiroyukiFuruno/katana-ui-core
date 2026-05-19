use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::{atom, molecule};

const TOOLTIP_DELAY_MS: u16 = 240;
const TOOLTIP_MAX_WIDTH: u16 = 280;
const FIRST_OPTION_INDEX: usize = 0;
const SECOND_OPTION_INDEX: usize = 1;
const CONTEXT_MENU_X: i32 = 192;
const CONTEXT_MENU_Y: i32 = 128;
const CONTEXT_MENU_MIN_WIDTH: u32 = 240;
const CONTEXT_MENU_MAX_HEIGHT: u32 = 260;
const CONTEXT_MENU_DELAY_MS: u16 = 180;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story(
            "menu",
            molecule::Menu::new("Menu")
                .child(atom::Button::new("Open"))
                .child(atom::Button::new("Close")),
        ),
        context_menu_story(),
        tooltip_story(),
        modal_story(),
        accordion_story(),
        combo_box_story(),
        menu_button_story(),
        modal_overlay_story(),
        notification_toast_story(),
        popover_story(),
        segmented_toggle_story(),
        select_box_story(),
    ]
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

fn modal_story() -> StoryExample {
    let mut modal = molecule::Modal::new("Modal")
        .open(true)
        .title("Preferences")
        .panel_size("medium")
        .footer("Cancel / Save")
        .escape_dismiss(true)
        .child(atom::Text::new("Body"))
        .child(atom::Button::new("Close"));
    let target = modal.state_id().clone();
    let result = modal.apply_action(&UiAction::modal_escape(target));
    StoryCatalog::interactive_story("modal", modal, result.callback_log)
}

fn accordion_story() -> StoryExample {
    let mut accordion = molecule::Accordion::new("Accordion")
        .trigger_area(molecule::DisclosureTriggerArea::IconAndText)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .child(atom::Button::new("Toggle"))
        .child(atom::Text::new("Panel"));
    let target = accordion.state_id().clone();
    let result = accordion.apply_action(&UiAction::accordion_toggle(target));
    StoryCatalog::interactive_story("accordion", accordion, result.callback_log)
}

fn combo_box_story() -> StoryExample {
    let mut combo = molecule::ComboBox::new("Combo box")
        .open(true)
        .item(molecule::ChoiceItem::new("one", "One"))
        .item(molecule::ChoiceItem::new("two", "Two"))
        .child(atom::Input::new("Search"))
        .child(atom::Text::new("Option"));
    let target = combo.state_id().clone();
    let result = combo.apply_action(&UiAction::select_box_selected(target, SECOND_OPTION_INDEX));
    StoryCatalog::interactive_story("combo-box", combo, result.callback_log)
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

fn context_menu_story() -> StoryExample {
    let mut menu = molecule::ContextMenu::new("ContextMenu")
        .anchor(context_menu_anchor())
        .placement_used(molecule::ContextMenuPlacement::BelowStart)
        .min_width(CONTEXT_MENU_MIN_WIDTH)
        .max_height(CONTEXT_MENU_MAX_HEIGHT)
        .submenu_open_delay_ms(CONTEXT_MENU_DELAY_MS)
        .focus_return_target("editor.surface")
        .items(context_menu_items())
        .child(atom::Text::new(
            "visual-marker: pointer anchor + floating menu",
        ))
        .child(atom::Badge::new("state-marker: open highlighted_path=[1]"))
        .child(atom::KeyCap::new("Cmd+C").platform("macos").combo("Cmd+C"));
    let target = menu.state_id().clone();
    let logs = context_menu_logs(&mut menu, target);
    StoryCatalog::interactive_story("context-menu", menu, logs)
}

fn context_menu_items() -> Vec<molecule::ContextMenuItem> {
    vec![
        molecule::ContextMenuItem::new("editing", "編集", molecule::ContextMenuItemKind::Section)
            .child(molecule::ContextMenuItem::action("cut", "Cut").shortcut("Cmd+X"))
            .child(molecule::ContextMenuItem::action("copy", "Copy").shortcut("Cmd+C")),
        molecule::ContextMenuItem::action("copy", "Copy").shortcut("Cmd+C"),
        molecule::ContextMenuItem::new("insert", "Insert", molecule::ContextMenuItemKind::Submenu)
            .child(molecule::ContextMenuItem::action("table", "Table"))
            .child(molecule::ContextMenuItem::action("link", "Link")),
        molecule::ContextMenuItem::new("divider", "", molecule::ContextMenuItemKind::Divider),
        molecule::ContextMenuItem::new("wrap", "Wrap line", molecule::ContextMenuItemKind::Toggle)
            .checked(true),
        molecule::ContextMenuItem::new(
            "scope",
            "Selection only",
            molecule::ContextMenuItemKind::Radio,
        )
        .checked(true),
        molecule::ContextMenuItem::action("delete", "Delete").destructive(true),
    ]
}

fn context_menu_logs(
    menu: &mut molecule::ContextMenu,
    target: katana_ui_core::render_model::UiStateId,
) -> Vec<UiCallbackLog> {
    let opened = menu.apply_context_action(&molecule::ContextMenuAction::Open {
        anchor: context_menu_anchor(),
    });
    let highlighted =
        menu.apply_context_action(&molecule::ContextMenuAction::Highlight { path: vec![1] });
    let submenu =
        menu.apply_context_action(&molecule::ContextMenuAction::OpenSubmenu { path: vec![2] });
    let selected =
        menu.apply_context_action(&molecule::ContextMenuAction::Activate { path: vec![6] });
    vec![
        context_menu_log(
            &target,
            "context_menu_open",
            "open=false",
            &opened,
            "open=true",
        ),
        context_menu_log(
            &target,
            "context_menu_highlight",
            "highlight=[]",
            &highlighted,
            "highlight=[1]",
        ),
        context_menu_log(
            &target,
            "context_menu_submenu",
            "submenu=closed",
            &submenu,
            "submenu=[2]",
        ),
        context_menu_log(
            &target,
            "context_menu_select",
            "command=pending",
            &selected,
            "command=delete open=false",
        ),
    ]
}

fn context_menu_anchor() -> molecule::ContextMenuAnchor {
    molecule::ContextMenuAnchor::Pointer {
        x: CONTEXT_MENU_X,
        y: CONTEXT_MENU_Y,
    }
}

fn context_menu_log(
    target: &katana_ui_core::render_model::UiStateId,
    action: &str,
    before: &str,
    event: &molecule::ContextMenuEvent,
    after: &str,
) -> UiCallbackLog {
    UiCallbackLog::new(
        target.clone(),
        action,
        before,
        format!("event={} state={after}", event.name()),
    )
}

fn modal_overlay_story() -> StoryExample {
    let mut overlay = molecule::ModalOverlay::new("Modal overlay")
        .open(true)
        .escape_dismiss(true)
        .focus_trap(true)
        .focus_return("trigger")
        .child(molecule::Modal::new("Dialog"))
        .child(atom::Button::new("Dismiss"));
    let target = overlay.state_id().clone();
    let result = overlay.apply_action(&UiAction::modal_escape(target));
    StoryCatalog::interactive_story("modal-overlay", overlay, result.callback_log)
}

fn notification_toast_story() -> StoryExample {
    let mut toast = molecule::NotificationToast::new("Notification")
        .open(true)
        .child(atom::Badge::new("Info"))
        .child(atom::Text::new("Message"));
    let target = toast.state_id().clone();
    let result = toast.apply_action(&UiAction::dismiss(target));
    StoryCatalog::interactive_story("notification-toast", toast, result.callback_log)
}

fn popover_story() -> StoryExample {
    let mut popover = molecule::Popover::new("Popover")
        .open(true)
        .placement("bottom-start")
        .width("320px")
        .escape_dismiss(true)
        .child(atom::Button::new("Anchor"))
        .child(atom::Text::new("Content"));
    let target = popover.state_id().clone();
    let result = popover.apply_action(&UiAction::modal_escape(target));
    StoryCatalog::interactive_story("popover", popover, result.callback_log)
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
    let mut select = molecule::SelectBox::new("Select box")
        .open(true)
        .placement("bottom-start")
        .item(molecule::ChoiceItem::new("light", "Light"))
        .item(molecule::ChoiceItem::new("dark", "Dark"))
        .child(atom::Button::new("Trigger"))
        .child(molecule::List::new("Options"));
    let target = select.state_id().clone();
    let result = select.apply_action(&UiAction::select_box_selected(target, SECOND_OPTION_INDEX));
    StoryCatalog::interactive_story("select-box", select, result.callback_log)
}
