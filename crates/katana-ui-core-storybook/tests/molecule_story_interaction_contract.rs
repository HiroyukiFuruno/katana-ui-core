use katana_ui_core_storybook::{StoryCatalog, StoryDetailContent};

const REQUIRED_INTERACTIVE_MOLECULES: [(&str, &str); 15] = [
    ("card", "click"),
    ("tooltip", "hover_start"),
    ("modal", "modal_escape"),
    ("accordion", "accordion_toggle"),
    ("combo-box", "select_box_selected"),
    ("menu-button", "select_box_selected"),
    ("notification-toast", "dismiss"),
    ("popover", "popover_open"),
    ("search-box", "search_submitted"),
    ("segmented-toggle", "segmented_toggle_selected"),
    ("select-box", "select_box_selected"),
    ("modal-overlay", "modal_escape"),
    ("code-diff", "code_diff_mode_changed"),
    ("color-picker-rgba", "color_drag"),
    ("tree-view", "click"),
];

#[test]
fn molecule_story_pages_expose_component_specific_action_history() {
    let examples = StoryCatalog.examples();

    for (page, action) in REQUIRED_INTERACTIVE_MOLECULES {
        let example = examples.iter().find(|it| it.page == page);
        assert!(example.is_some(), "{page} story is missing");
        let Some(example) = example else {
            continue;
        };

        assert!(
            example.callback_logs.iter().any(|it| it.action == action),
            "{page} lacks {action} action"
        );
    }
}

#[test]
fn modal_story_pages_expose_specific_action_event_evidence() {
    let examples = StoryCatalog.examples();
    let modal = examples
        .iter()
        .find(|it| it.page == "modal")
        .expect("modal story is missing");
    let overlay = examples
        .iter()
        .find(|it| it.page == "modal-overlay")
        .expect("modal-overlay story is missing");

    for action in ["modal_escape", "modal_focus_return", "modal_parent_block"] {
        assert!(
            modal.callback_logs.iter().any(|it| it.action == action),
            "modal lacks {action} action"
        );
    }
    for event in [
        "NativeWindowOpened",
        "ModalEscaped",
        "FocusReturned",
        "ParentInteractionBlocked",
    ] {
        assert!(
            modal
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "modal lacks {event} event evidence"
        );
    }
    for action in [
        "modal_backdrop_click",
        "modal_escape",
        "modal_focus_trap",
        "modal_focus_return",
        "modal_dismiss_disabled",
    ] {
        assert!(
            overlay.callback_logs.iter().any(|it| it.action == action),
            "modal-overlay lacks {action} action"
        );
    }
    for event in [
        "OverlayBackdropClosed",
        "OverlayEscaped",
        "FocusTrapCycled",
        "FocusReturned",
        "DismissBlocked",
    ] {
        assert!(
            overlay
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "modal-overlay lacks {event} event evidence"
        );
    }
}

#[test]
fn popover_story_page_exposes_specific_action_event_evidence() {
    let examples = StoryCatalog.examples();
    let popover = examples
        .iter()
        .find(|it| it.page == "popover")
        .expect("popover story is missing");
    let details = StoryDetailContent::from_example(popover);

    for action in [
        "popover_open",
        "popover_outside_close",
        "popover_escape_close",
        "popover_auto_flip",
        "popover_focus_return",
        "popover_slot_action",
    ] {
        assert!(
            popover.callback_logs.iter().any(|it| it.action == action),
            "popover lacks {action} action"
        );
        assert!(
            details.settings.contains(action),
            "popover settings lacks {action} action evidence"
        );
    }
    for event in [
        "PopoverOpened",
        "PopoverOutsideClosed",
        "PopoverEscapeClosed",
        "PopoverAutoFlipped",
        "PopoverFocusReturned",
        "PopoverSlotActionInvoked",
    ] {
        assert!(
            popover
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "popover lacks {event} event evidence"
        );
        assert!(
            details.settings.contains(event),
            "popover settings lacks {event} event evidence"
        );
    }
    for setting in [
        "option=",
        "action=",
        "event=",
        "state=",
        "preset=",
        "anchor=node:toolbar.more-actions",
        "placement=bottom-start",
        "auto_flip=BottomStart>TopStart",
        "offset=12,8",
        "width=320px",
        "outside_close=true",
        "escape_close=true",
        "focus_handling=FirstInteractive",
        "slot=heading/body/footer/action",
    ] {
        assert!(
            details.settings.contains(setting),
            "popover settings lacks {setting}"
        );
    }
}
