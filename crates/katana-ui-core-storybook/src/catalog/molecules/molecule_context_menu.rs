use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::{atom, molecule};

const CONTEXT_MENU_X: i32 = 192;
const CONTEXT_MENU_Y: i32 = 128;
const CONTEXT_MENU_MIN_WIDTH: u32 = 240;
const CONTEXT_MENU_MAX_HEIGHT: u32 = 260;
const CONTEXT_MENU_DELAY_MS: u16 = 180;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![context_menu_story()]
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
        .child(atom::Text::new("preset: 編集器右クリック"))
        .child(atom::Text::new("preset: explorer 空領域"))
        .child(atom::Text::new("preset: tab bar"))
        .child(atom::Text::new("preset: message 行"))
        .child(atom::Text::new("preset: leading icon + shortcut"))
        .child(atom::Badge::new("settings: anchor / placement / item kind"))
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
        molecule::ContextMenuItem::action("copy", "Copy")
            .leading_icon("<svg data-icon=\"copy\"/>")
            .shortcut("Cmd+C"),
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
        .checked(true)
        .radio_group("selection-scope"),
        molecule::ContextMenuItem::action("delete", "Delete").destructive(true),
        molecule::ContextMenuItem::action("locked", "Locked action").disabled(true),
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
            "open=false anchor=Pointer(192,128)",
            &opened,
            "open=true placement=BelowStart",
        ),
        context_menu_log(
            &target,
            "context_menu_highlight",
            "highlight=[] item_kind=Action",
            &highlighted,
            "highlight=[1]",
        ),
        context_menu_log(
            &target,
            "context_menu_submenu",
            "submenu=closed item_kind=Submenu",
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

#[cfg(test)]
mod tests {
    use super::context_menu_story;
    use crate::catalog::StoryPresetLabels;
    use katana_ui_core::render_model::{UiContextMenuItemKind, UiNodeKind};

    #[test]
    fn context_menu_story_exposes_required_presets_items_and_logs() {
        let story = context_menu_story();
        let props = story.tree.root().props();

        assert_eq!(UiNodeKind::ContextMenu, story.tree.root().kind());
        assert_eq!(
            &[
                "編集器右クリック",
                "explorer 空領域",
                "tab bar",
                "message 行",
                "leading icon + shortcut"
            ],
            StoryPresetLabels::for_page("context-menu")
        );
        for kind in [
            UiContextMenuItemKind::Action,
            UiContextMenuItemKind::Toggle,
            UiContextMenuItemKind::Radio,
            UiContextMenuItemKind::Submenu,
            UiContextMenuItemKind::Section,
            UiContextMenuItemKind::Divider,
        ] {
            assert!(props.context_menu.items.iter().any(|it| it.kind == kind));
        }
        assert!(
            props
                .context_menu
                .items
                .iter()
                .any(|it| !it.leading_icon.is_empty())
        );
        assert!(
            props
                .context_menu
                .items
                .iter()
                .any(|it| !it.shortcut.is_empty())
        );
        assert!(props.context_menu.items.iter().any(|it| it.disabled));
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.action == "context_menu_open")
        );
        assert!(
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains("event="))
        );
    }
}
