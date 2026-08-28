use super::super::command_chrome_script::CommandChromeScriptFrame;
use katana_ui_core::atom::TextAreaEvent;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeDropdownCloseReason, CommandChromeSearchEvent, CommandChromeToolbarEvent,
    FloatingCommandToolbarCloseReason, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::structured::SearchControlStripEvent;
use katana_ui_core::text_surface::TextSurfaceEvent;
use std::io;

pub(super) const FRAME_COUNT: usize = 34;
const TOOLBAR_ACTION_COUNT: usize = 13;
const FLOATING_ACTION_COUNT: usize = 3;
const SEARCH_CONTROL_COUNT: usize = 9;
const DROPDOWN_ITEM_COUNT: usize = 17;
const FLOATING_RETURN_TARGET: &str = "storybook-return-target";

pub(super) fn assert_fixture_inventory(frame: &CommandChromeScriptFrame) {
    assert_eq!(TOOLBAR_ACTION_COUNT, frame.toolbar.record.actions.len());
    assert_eq!(SEARCH_CONTROL_COUNT, frame.search.record.controls.len());
}

pub(super) fn assert_initial_floating_surface(frame: &CommandChromeScriptFrame) -> io::Result<()> {
    let record = frame
        .floating
        .record
        .as_ref()
        .ok_or_else(|| io::Error::other("initial fixture must show the floating toolbar"))?;
    assert_eq!(FLOATING_ACTION_COUNT, record.toolbar.actions.len());
    assert!(record.tooltip_bounds.is_none());
    Ok(())
}

pub(super) fn assert_hover_tooltip(frame: &CommandChromeScriptFrame) -> io::Result<()> {
    let record = frame
        .floating
        .record
        .as_ref()
        .ok_or_else(|| io::Error::other("hovered fixture must retain the floating toolbar"))?;
    assert!(record.tooltip_bounds.is_some());
    assert!(record.tooltip_raster_identity.is_some());
    Ok(())
}

pub(super) fn assert_no_command(frames: &[CommandChromeScriptFrame], action_id: &str) {
    assert!(frames.iter().all(|frame| {
        frame.toolbar.events.iter().all(|event| {
            !matches!(
                event,
                CommandChromeToolbarEvent::CommandActivated { action_id: actual }
                    if actual.as_str() == action_id
            )
        })
    }));
}

pub(super) fn assert_no_toolbar_command(frame: &CommandChromeScriptFrame) {
    assert!(
        frame
            .toolbar
            .events
            .iter()
            .all(|event| { !matches!(event, CommandChromeToolbarEvent::CommandActivated { .. }) })
    );
}

pub(super) fn assert_toolbar_command(frame: &CommandChromeScriptFrame, action_id: &str) {
    assert!(frame.toolbar.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::CommandActivated { action_id: actual }
                if actual.as_str() == action_id
        )
    }));
}

pub(super) fn assert_dropdown_opened(frame: &CommandChromeScriptFrame) -> io::Result<()> {
    assert!(frame.toolbar.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::SplitDropdownOpened { action_id, .. }
                | CommandChromeToolbarEvent::DropdownOpened { action_id, .. }
                if action_id.as_str() == "code-block"
        )
    }));
    let dropdown = frame
        .toolbar
        .record
        .dropdown
        .as_ref()
        .ok_or_else(|| io::Error::other("opened dropdown must have a frame record"))?;
    assert_eq!(DROPDOWN_ITEM_COUNT, dropdown.items.len());
    Ok(())
}

pub(super) fn assert_dropdown_focus(frame: &CommandChromeScriptFrame, item_id: &str) {
    assert!(frame.toolbar.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownFocusChanged {
                action_id,
                item_id: actual,
            } if action_id.as_str() == "code-block" && actual.as_str() == item_id
        )
    }));
}

pub(super) fn assert_dropdown_item_activated(frame: &CommandChromeScriptFrame, item_id: &str) {
    assert!(frame.toolbar.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownItemActivated {
                action_id,
                item_id: actual,
            } if action_id.as_str() == "code-block" && actual.as_str() == item_id
        )
    }));
}

pub(super) fn assert_dropdown_closed(
    frame: &CommandChromeScriptFrame,
    expected_reason: CommandChromeDropdownCloseReason,
) {
    assert!(frame.toolbar.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownClosed { action_id, reason }
                if action_id.as_str() == "code-block" && *reason == expected_reason
        )
    }));
}

pub(super) fn assert_floating_command(frame: &CommandChromeScriptFrame, action_id: &str) {
    assert!(frame.floating.events.iter().any(|event| {
        matches!(
            event,
            FloatingCommandToolbarEvent::Toolbar {
                event: CommandChromeToolbarEvent::CommandActivated { action_id: actual },
            } if actual.as_str() == action_id
        )
    }));
}

pub(super) fn assert_floating_event(
    frame: &CommandChromeScriptFrame,
    expected: FloatingCommandToolbarEvent,
) {
    assert!(frame.floating.events.contains(&expected));
}

pub(super) fn assert_floating_closed(
    frame: &CommandChromeScriptFrame,
    expected_reason: FloatingCommandToolbarCloseReason,
) {
    assert!(frame.floating.events.iter().any(|event| {
        matches!(
            event,
            FloatingCommandToolbarEvent::Closed { reason } if *reason == expected_reason
        )
    }));
    assert!(frame.floating.events.iter().any(|event| {
        matches!(
            event,
            FloatingCommandToolbarEvent::FocusReturnRequested { target }
                if target.as_str() == FLOATING_RETURN_TARGET
        )
    }));
}

pub(super) fn assert_search_event(
    frame: &CommandChromeScriptFrame,
    expected: impl Fn(&SearchControlStripEvent) -> bool,
) {
    assert!(frame.search.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeSearchEvent::Strip { event } if expected(event)
        )
    }));
}

pub(super) fn assert_text_event(
    frame: &CommandChromeScriptFrame,
    expected: impl Fn(&TextAreaEvent) -> bool,
) {
    assert!(
        frame
            .search
            .text_events
            .iter()
            .any(|event| { matches!(event, TextSurfaceEvent::TextArea(event) if expected(event)) })
    );
}

pub(super) fn artifact_hashes(
    frame: &CommandChromeScriptFrame,
) -> (String, String, Option<(String, String)>, String, String) {
    let floating = frame.floating.artifact.as_ref().map(|artifact| {
        (
            artifact.frame_record_hash.clone(),
            artifact.paint_plan_hash.clone(),
        )
    });
    (
        frame.toolbar.artifact.frame_record_hash.clone(),
        frame.toolbar.artifact.paint_plan_hash.clone(),
        floating,
        frame.search.artifact.frame_record_hash.clone(),
        frame.search.artifact.paint_plan_hash.clone(),
    )
}
