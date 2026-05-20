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
    let bar = closeable_tab_strip_base();
    let target = bar.state().state_id.clone();
    let logs = closeable_tab_strip_logs(&target);
    StoryCatalog::interactive_story("closeable-tab-strip", bar, logs)
}

fn closeable_tab_strip_base() -> molecule::CloseableTabStrip {
    molecule::CloseableTabStrip::new("CloseableTabStrip")
        .group(molecule::TabGroup::new("docs", "Docs"))
        .group(molecule::TabGroup::new("preview", "Preview"))
        .tab(molecule::CloseableTab::new("works", "default"))
        .tab(molecule::CloseableTab::new("pinned", "pinned").pinned(true))
        .tab(molecule::CloseableTab::new("overflow", "overflow").icon("more"))
        .tab(
            molecule::CloseableTab::new("inbox", "groups")
                .dirty(true)
                .group_id("docs"),
        )
        .tab(molecule::CloseableTab::new("settings", "dirty").dirty(true))
        .tab(molecule::CloseableTab::new("dragging", "dragging"))
        .active_tab_id("inbox")
}

fn closeable_tab_strip_logs(target: &UiStateId) -> Vec<UiCallbackLog> {
    vec![
        closeable_action_log(target, "add_tab", "tabs=6", "tabs=7 event=tab_added"),
        closeable_core_action_log(target, "delete_tab", "tabs=6", close_events()),
        closeable_core_action_log(target, "pin_tab", "pinned=false", pin_events()),
        closeable_action_log(
            target,
            "dirty_toggle",
            "dirty=false",
            "dirty=true event=tab_dirty_changed",
        ),
        closeable_core_action_log(target, "group_toggle", "group=docs", group_events()),
        closeable_core_action_log(target, "drag_tab", "index=4", drag_events()),
        closeable_core_action_log(target, "overflow_open", "hidden=0", overflow_events()),
    ]
}

fn close_events() -> Vec<molecule::CloseableTabStripEvent> {
    let mut bar = closeable_tab_strip_base();
    bar.apply_action(molecule::CloseableTabStripAction::CloseTab {
        tab_id: molecule::CloseableTabId::new("dragging"),
    })
}

fn pin_events() -> Vec<molecule::CloseableTabStripEvent> {
    let mut bar = closeable_tab_strip_base();
    bar.apply_action(molecule::CloseableTabStripAction::PinTab {
        tab_id: molecule::CloseableTabId::new("settings"),
    })
}

fn group_events() -> Vec<molecule::CloseableTabStripEvent> {
    let mut bar = closeable_tab_strip_base();
    bar.apply_action(molecule::CloseableTabStripAction::MoveToGroup {
        tab_id: molecule::CloseableTabId::new("settings"),
        target: molecule::TabGroupTarget::Existing("preview".into()),
    })
}

fn drag_events() -> Vec<molecule::CloseableTabStripEvent> {
    let mut bar = closeable_tab_strip_base();
    bar.apply_action(molecule::CloseableTabStripAction::MoveTab {
        tab_id: molecule::CloseableTabId::new("dragging"),
        to_visual_index: 2,
    })
}

fn overflow_events() -> Vec<molecule::CloseableTabStripEvent> {
    let mut bar = closeable_tab_strip_base();
    bar.apply_action(molecule::CloseableTabStripAction::OpenOverflow {
        hidden_tab_ids: vec![molecule::CloseableTabId::new("settings")],
    })
}

fn closeable_core_action_log(
    target: &UiStateId,
    action: &str,
    before: &str,
    events: Vec<molecule::CloseableTabStripEvent>,
) -> UiCallbackLog {
    event_log(target, action, before, events.as_slice())
}

fn closeable_action_log(
    target: &UiStateId,
    action: &str,
    before: impl Into<String>,
    after: impl Into<String>,
) -> UiCallbackLog {
    log(target, action, before, after)
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
