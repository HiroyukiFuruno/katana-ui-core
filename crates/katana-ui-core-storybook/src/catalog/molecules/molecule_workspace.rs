#[path = "drag_and_drop_story.rs"]
mod drag_and_drop_story;
use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule;
use katana_ui_core::render_model::{UiIconProps, UiStateId, UiSvgPaintPolicy};

const CLOSEABLE_TAB_DRAG_TARGET_INDEX: usize = 2;
const STORYBOOK_TAB_ICON_SVG: &str = "<svg viewBox=\"0 0 16 16\"><path d=\"M3 3h10v10H3z\"/></svg>";

pub(super) fn examples() -> Vec<StoryExample> {
    vec![drag_and_drop_story(), closeable_tab_strip_story()]
}

fn drag_and_drop_story() -> StoryExample {
    drag_and_drop_story::story()
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
        .tab(
            molecule::CloseableTab::new("overflow", "overflow")
                .svg_icon(storybook_tab_icon("overflow")),
        )
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
        closeable_core_action_log(target, "add_tab", "tabs=6", add_events()),
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

fn storybook_tab_icon(role: impl Into<String>) -> UiIconProps {
    UiIconProps::new(STORYBOOK_TAB_ICON_SVG)
        .role(role)
        .paint_policy(UiSvgPaintPolicy::CurrentColor)
}

fn add_events() -> Vec<molecule::CloseableTabStripEvent> {
    let mut bar = closeable_tab_strip_base();
    bar.apply_action(molecule::CloseableTabStripAction::AddTab {
        tab: molecule::CloseableTab::new("notes", "notes"),
        activate: true,
    })
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
        to_visual_index: CLOSEABLE_TAB_DRAG_TARGET_INDEX,
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
