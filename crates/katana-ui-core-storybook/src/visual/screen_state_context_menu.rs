use super::interaction_spec::StorybookInteractionSpec;
use super::screen_state::StorybookScreenState;
use katana_ui_core::widget::molecules::{
    ContextMenu, ContextMenuAction, ContextMenuAnchor, ContextMenuCloseReason, ContextMenuEvent,
    ContextMenuItem, ContextMenuItemKind,
};

const STORYBOOK_CONTEXT_ANCHOR_X: i32 = 192;
const STORYBOOK_CONTEXT_ANCHOR_Y: i32 = 128;

impl StorybookScreenState {
    pub(in crate::visual) fn register_context_menu(&mut self, page: &str) {
        if page != "tree-view" && page != "context-menu" {
            return;
        }
        self.action_count += 1;
        if page == "tree-view" {
            self.last_action = "tree_context_menu";
            self.last_event = "tree_context_opened";
            self.last_setting = "empty_area_context_menu";
            self.last_setting_value = "visible";
            self.state_label = "context_menu=open";
            return;
        }
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.last_setting = spec.option;
        self.last_setting_value = spec.after;
        self.state_label = spec.state;
    }

    pub(in crate::visual) fn register_context_menu_submenu(&mut self) {
        let event = context_menu_event(ContextMenuAction::OpenSubmenu { path: vec![2] });
        self.action_count += 1;
        self.last_action = "context_menu_open_submenu";
        self.last_event = event.name();
        self.last_setting = "context_menu.items";
        self.last_setting_value = "insert.submenu=open";
        self.state_label = "context_menu.submenu=[2]";
    }

    pub(in crate::visual) fn register_context_menu_select_link(&mut self) {
        let event = context_menu_event(ContextMenuAction::Activate { path: vec![2, 1] });
        self.action_count += 1;
        self.last_action = "context_menu_select_item";
        self.last_event = event.name();
        self.last_setting = "context_menu.command";
        self.last_setting_value = "link";
        self.state_label = "context_menu.selected=[2,1]";
    }

    pub(in crate::visual) fn register_context_menu_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "context_menu_focus";
        self.last_event = "context_menu_focused";
        self.last_setting = "context_menu.focus";
        self.last_setting_value = "insert";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_context_menu_keyboard_select(&mut self) {
        if !self.button_focused {
            self.last_action = "context_menu_keyboard_without_focus";
            self.last_event = "context_menu_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        let event = context_menu_event(ContextMenuAction::Activate { path: vec![1] });
        self.action_count += 1;
        self.last_action = "context_menu_keyboard_select";
        self.last_event = event.name();
        self.last_setting = "context_menu.command";
        self.last_setting_value = "copy";
        self.state_label = "context_menu.selected=[1]";
    }

    pub(in crate::visual) fn register_context_menu_outside_dismiss(&mut self) {
        let event = context_menu_event(ContextMenuAction::Close {
            reason: ContextMenuCloseReason::OutsideClick,
        });
        self.action_count += 1;
        self.last_action = "context_menu_outside_dismiss";
        self.last_event = event.name();
        self.last_setting = "context_menu.dismiss";
        self.last_setting_value = "outside";
        self.state_label = "context_menu=closed";
    }
}

fn context_menu_event(action: ContextMenuAction) -> ContextMenuEvent {
    let mut menu = ContextMenu::new("storybook-context")
        .anchor(storybook_context_menu_anchor())
        .items(storybook_context_menu_items());
    menu.apply_context_action(&ContextMenuAction::Open {
        anchor: storybook_context_menu_anchor(),
    });
    menu.apply_context_action(&action)
}

fn storybook_context_menu_anchor() -> ContextMenuAnchor {
    ContextMenuAnchor::Pointer {
        x: STORYBOOK_CONTEXT_ANCHOR_X,
        y: STORYBOOK_CONTEXT_ANCHOR_Y,
    }
}

fn storybook_context_menu_items() -> Vec<ContextMenuItem> {
    vec![
        ContextMenuItem::action("edit", "編集"),
        ContextMenuItem::action("copy", "Copy").shortcut("Cmd+C"),
        ContextMenuItem::new("insert", "Insert", ContextMenuItemKind::Submenu)
            .child(ContextMenuItem::action("table", "Table"))
            .child(ContextMenuItem::action("link", "Link")),
        ContextMenuItem::new("divider", "", ContextMenuItemKind::Divider),
        ContextMenuItem::action("delete", "Delete").destructive(true),
    ]
}
