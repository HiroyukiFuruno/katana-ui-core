use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![drag_and_drop_story(), closeable_tab_strip_story()]
}

fn drag_and_drop_story() -> StoryExample {
    let target = UiStateId::new("state:DragAndDrop:storybook");
    let presets = drag_and_drop_presets();
    let root = presets
        .iter()
        .fold(molecule::List::new("DragAndDrop"), |root, preset| {
            root.child(atom::Badge::new(preset.preview_label()))
        });
    let logs = presets
        .iter()
        .map(|preset| preset.callback_log(&target))
        .collect();
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

fn drag_and_drop_presets() -> [DragAndDropPreset; 5] {
    [
        DragAndDropPreset {
            name: "reorder list",
            payload: "katana-ui-core/list-row:item-02",
            target: "list:item-04",
            accept: "Accept(move, indicator=after)",
            autoscroll: "edge=24 speed=medium",
            keyboard_draggable: true,
            events: "DragStart > DragMove > DragEnter > Drop > DragEnd(committed=true)",
            action: "reorder_list_drop",
        },
        DragAndDropPreset {
            name: "file drop",
            payload: "os/file-list:3 files",
            target: "drop-zone:imports",
            accept: "Accept(copy, indicator=outline)",
            autoscroll: "off",
            keyboard_draggable: false,
            events: "DragEnter > DragMove > Drop > DragEnd(committed=true)",
            action: "file_drop_accept",
        },
        DragAndDropPreset {
            name: "tab reorder",
            payload: molecule::CLOSEABLE_TAB_DRAG_TAG,
            target: "tab:settings",
            accept: "Accept(move, indicator=before)",
            autoscroll: "edge=16 speed=slow",
            keyboard_draggable: true,
            events: "DragStart > DragMove > DragEnter > Drop > DragEnd(committed=true)",
            action: "tab_reorder_drop",
        },
        DragAndDropPreset {
            name: "attachment drop",
            payload: "consumer/chat-attachment:image.png",
            target: "composer:attachments",
            accept: "Accept(copy, indicator=inside)",
            autoscroll: "edge=32 speed=fast",
            keyboard_draggable: false,
            events: "DragEnter > DragMove > Drop > DragEnd(committed=true)",
            action: "attachment_drop_accept",
        },
        DragAndDropPreset {
            name: "keyboard drag",
            payload: "katana-ui-core/list-row:item-01",
            target: "list:item-03",
            accept: "Accept(move, indicator=after)",
            autoscroll: "edge=24 speed=keyboard",
            keyboard_draggable: true,
            events: "DragStart(Space) > DragMove(ArrowDown) > DragEnter > DragCancel(Esc) > DragEnd(committed=false)",
            action: "keyboard_drag_cancel",
        },
    ]
}

struct DragAndDropPreset {
    name: &'static str,
    payload: &'static str,
    target: &'static str,
    accept: &'static str,
    autoscroll: &'static str,
    keyboard_draggable: bool,
    events: &'static str,
    action: &'static str,
}

impl DragAndDropPreset {
    fn preview_label(&self) -> String {
        format!(
            "preset={} payload={} target={} accept={} autoscroll={} keyboard_draggable={} events={}",
            self.name,
            self.payload,
            self.target,
            self.accept,
            self.autoscroll,
            self.keyboard_draggable,
            self.events
        )
    }

    fn callback_log(&self, target: &UiStateId) -> UiCallbackLog {
        log(
            target,
            self.action,
            format!(
                "preset={} accept=pending autoscroll={} keyboard_draggable={}",
                self.name, self.autoscroll, self.keyboard_draggable
            ),
            format!(
                "preset={} payload={} accept={} autoscroll={} keyboard_draggable={} events={} target={}",
                self.name,
                self.payload,
                self.accept,
                self.autoscroll,
                self.keyboard_draggable,
                self.events,
                self.target
            ),
        )
    }
}
