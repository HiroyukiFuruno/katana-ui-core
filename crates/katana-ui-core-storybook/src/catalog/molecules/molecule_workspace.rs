use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![drag_and_drop_story(), closeable_tab_strip_story()]
}

fn drag_and_drop_story() -> StoryExample {
    let source = molecule::CLOSEABLE_TAB_DRAG_TAG;
    let root = molecule::List::new("DragAndDrop")
        .child(atom::Badge::new("source: tab-a"))
        .child(atom::Badge::new(format!("payload: {source}")))
        .child(atom::Badge::new("target: tab-b"))
        .child(atom::Badge::new("indicator: before / inside / after"))
        .child(atom::Badge::new("keyboard: Space Arrow Space"));
    let target = UiStateId::new("state:DragAndDrop:storybook");
    let logs = vec![
        log(
            &target,
            "drag_start",
            "idle",
            "event=drag_start payload=tab-a",
        ),
        log(
            &target,
            "drag_over",
            "target=tab-b",
            "event=drag_over indicator=after",
        ),
        log(&target, "drop", "effect=move", "event=drop committed=true"),
    ];
    StoryCatalog::interactive_story("drag-and-drop", root, logs)
}

fn closeable_tab_strip_story() -> StoryExample {
    let mut bar = molecule::CloseableTabStrip::new("CloseableTabStrip")
        .group(molecule::CloseableTabGroup::new("docs", "Docs"))
        .tab(molecule::CloseableTab::new("works", "works").pinned(true))
        .tab(
            molecule::CloseableTab::new("inbox", "受信トレイ")
                .dirty(true)
                .group_id("docs"),
        )
        .tab(molecule::CloseableTab::new("settings", "Settings").icon("gear"))
        .active_tab_id("inbox");
    let target = bar.state().state_id.clone();
    let select = bar.apply_action(molecule::CloseableTabStripAction::SelectTab {
        tab_id: molecule::CloseableTabId::new("settings"),
    });
    let close = bar.apply_action(molecule::CloseableTabStripAction::CloseTab {
        tab_id: molecule::CloseableTabId::new("inbox"),
    });
    let overflow = bar.apply_action(molecule::CloseableTabStripAction::OpenOverflow {
        hidden_tab_ids: vec![molecule::CloseableTabId::new("settings")],
    });
    let logs = vec![
        event_log(&target, "select_tab", "active=inbox", select.as_slice()),
        event_log(&target, "close_tab", "dirty=true", close.as_slice()),
        event_log(&target, "open_overflow", "hidden=1", overflow.as_slice()),
    ];
    StoryCatalog::interactive_story("closeable-tab-strip", bar, logs)
}

fn event_log(
    target: &UiStateId,
    action: &str,
    before: &str,
    events: &[molecule::CloseableTabStripEvent],
) -> UiCallbackLog {
    let names = events
        .iter()
        .map(molecule::CloseableTabStripEvent::name)
        .collect::<Vec<_>>()
        .join(",");
    log(target, action, before, format!("events={names}"))
}

fn log(
    target: &UiStateId,
    action: &str,
    before: impl Into<String>,
    after: impl Into<String>,
) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
