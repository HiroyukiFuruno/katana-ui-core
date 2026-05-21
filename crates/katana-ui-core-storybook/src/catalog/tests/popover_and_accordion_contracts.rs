use super::*;

#[test]
fn popover_story_exposes_operable_settings_presets_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "popover")
        .ok_or("popover page missing")?;
    let labels = page_children(&examples, "popover").ok_or("popover page missing")?;
    let details = StoryDetailContent::from_example(story);
    let popover = &story.tree.root().props().popover;

    assert_eq!(
        &[
            "anchor",
            "placement",
            "auto flip",
            "offset width",
            "outside+escape close",
            "focus handling",
            "slot content"
        ],
        StoryPresetLabels::for_page("popover")
    );
    for preset in StoryPresetLabels::for_page("popover") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "popover preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "popover details lack preset {preset}"
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
        "focus_return=trigger:popover-anchor",
        "slot=heading/body/footer/action",
    ] {
        assert!(
            details.settings.contains(setting),
            "popover settings inspector lacks {setting}"
        );
    }
    assert_eq!("node:toolbar.more-actions", popover.anchor.as_str());
    assert_eq!(UiPopoverPlacement::BottomStart, popover.placement);
    assert_eq!(12, popover.offset_x);
    assert_eq!(8, popover.offset_y);
    assert_eq!("320px", popover.width.as_str());
    assert!(popover.dismiss_on_outside_click);
    assert!(popover.dismiss_on_escape);
    assert_eq!("Quick actions", popover.heading.as_str());
    assert_eq!("Operate on the current selection", popover.body.as_str());
    assert_eq!("Esc closes and returns focus", popover.footer.as_str());
    assert_eq!(1, popover.action_count);
    assert_eq!(
        UiPopoverFocusManagement::FirstInteractive,
        popover.focus_management
    );
    assert_eq!(
        &[
            UiPopoverPlacement::BottomStart,
            UiPopoverPlacement::TopStart
        ],
        popover.auto_flip_priority.as_slice()
    );
    for action in [
        "popover_open",
        "popover_outside_close",
        "popover_escape_close",
        "popover_auto_flip",
        "popover_focus_return",
        "popover_slot_action",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "popover callback log lacks action {action}"
        );
        assert!(
            details.settings.contains(action),
            "popover settings inspector lacks action {action}"
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
            story
                .callback_logs
                .iter()
                .any(|it| it.after.contains(event)),
            "popover callback log lacks event {event}"
        );
    }
    Ok(())
}

#[test]
fn accordion_story_exposes_presets_settings_and_logs() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "accordion")
        .ok_or("accordion page missing")?;
    let labels = page_children(&examples, "accordion").ok_or("accordion page missing")?;
    let details = StoryDetailContent::from_example(story);

    assert_eq!(
        &[
            "closed",
            "open",
            "disabled",
            "controlled",
            "multiple",
            "tree mode",
            "reduced motion",
            "trigger areas"
        ],
        StoryPresetLabels::for_page("accordion")
    );
    for preset in StoryPresetLabels::for_page("accordion") {
        assert!(
            labels.iter().any(|it| it.contains(preset)),
            "accordion preview lacks preset {preset}"
        );
        assert!(
            details.preset.contains(preset),
            "accordion detail preset lacks {preset}"
        );
    }
    for setting in [
        "expanded",
        "disabled",
        "controlled",
        "multiple",
        "indicator",
        "trigger_area",
        "toggle_icon",
        "tree_mode",
        "depth",
        "selected",
        "show_lines",
        "reduced_motion",
        "body_border",
    ] {
        assert!(
            details.settings.contains(setting),
            "accordion settings inspector lacks {setting}"
        );
    }
    for action in [
        "accordion_toggle",
        "accordion_trigger_area",
        "accordion_controlled_request",
        "accordion_group_toggle",
        "accordion_disabled_block",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "accordion callback log lacks action {action}"
        );
    }
    Ok(())
}

#[test]
fn catalog_contains_single_independent_context_menu_story() {
    let examples = StoryCatalog.examples();
    let count = examples
        .iter()
        .filter(|it| it.page == "context-menu")
        .count();

    assert_eq!(1, count);
}
