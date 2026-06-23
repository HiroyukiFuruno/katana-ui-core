use super::super::{
    WorkspaceTab, WorkspaceTabBar, WorkspaceTabBarEvent, WorkspaceTabId, WorkspaceTabKey,
    WorkspaceTabKeyboardInput, WorkspaceTabKeyboardShortcut,
};

#[test]
fn keyboard_digit_selects_nth_visible_tab() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("first", "First"))
        .tab(WorkspaceTab::new("second", "Second"))
        .tab(WorkspaceTab::new("third", "Third"))
        .active_tab_id("first");
    let visible = vec![
        WorkspaceTabId::new("first"),
        WorkspaceTabId::new("second"),
        WorkspaceTabId::new("third"),
    ];
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Digit(2),
        true,
        false,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("second")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("second")
        }],
        events
    );
}

#[test]
fn keyboard_ctrl_w_requests_dirty_close_for_active_tab() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("draft", "Draft").dirty(true))
        .active_tab_id("draft");
    let visible = vec![WorkspaceTabId::new("draft")];

    let events = bar.apply_keyboard_input(WorkspaceTabKeyboardInput::CloseActiveTab, &visible);

    assert_eq!(
        Some(&WorkspaceTabId::new("draft")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        Some(&WorkspaceTabId::new("draft")),
        bar.state().pending_close_confirm.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabCloseRequested {
            tab_id: WorkspaceTabId::new("draft")
        }],
        events
    );
}

#[test]
fn keyboard_ctrl_tab_cycles_visible_tabs() {
    let mut bar = tab_bar("second");
    let visible = visible_tabs();
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Tab,
        true,
        false,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("third")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("third")
        }],
        events
    );
}

#[test]
fn keyboard_shift_ctrl_tab_cycles_backwards() {
    let mut bar = tab_bar("first");
    let visible = visible_tabs();
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Tab,
        true,
        true,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("third")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("third")
        }],
        events
    );
}

#[test]
fn keyboard_digit_zero_selects_last_visible_tab() {
    let mut bar = tab_bar("first");
    let visible = visible_tabs();
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Digit(0),
        true,
        false,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("third")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("third")
        }],
        events
    );
}

fn tab_bar(active_tab_id: &str) -> WorkspaceTabBar {
    WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("first", "First"))
        .tab(WorkspaceTab::new("second", "Second"))
        .tab(WorkspaceTab::new("third", "Third"))
        .active_tab_id(active_tab_id)
}

fn visible_tabs() -> Vec<WorkspaceTabId> {
    vec![
        WorkspaceTabId::new("first"),
        WorkspaceTabId::new("second"),
        WorkspaceTabId::new("third"),
    ]
}
