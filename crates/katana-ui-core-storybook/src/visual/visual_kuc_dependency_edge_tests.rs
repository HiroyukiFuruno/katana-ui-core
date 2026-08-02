use katana_ui_core::interaction::MotionDurationToken;
use katana_ui_core::molecule::{
    CloseableTab, CloseableTabGroup, CloseableTabId, CloseableTabStrip, CloseableTabStripAction,
    CloseableTabStripEvent, ContextMenuItem, ContextMenuKeyboardInput, ContextMenuKeyboardIntent,
    ContextMenuKeyboardNavigator, ContextMenuTypeAheadBuffer, SettingsControl, SettingsField,
    SettingsList, SettingsListHitTestInput, SettingsListHitTestResult, SettingsSection,
};
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeKind};
use katana_ui_core::theme::MotionTokenSet;

#[test]
fn kuc_dependency_edge_contracts_execute_in_the_storybook_runtime() {
    assert_eq!(
        0,
        MotionTokenSet::default().duration(MotionDurationToken::Instant)
    );

    let items = vec![
        ContextMenuItem::action("first", "First"),
        ContextMenuItem::action("second", "Second"),
    ];
    for (input, expected) in [
        (
            ContextMenuKeyboardInput::ArrowDown,
            ContextMenuKeyboardIntent::MoveTo(0),
        ),
        (
            ContextMenuKeyboardInput::ArrowUp,
            ContextMenuKeyboardIntent::MoveTo(1),
        ),
        (
            ContextMenuKeyboardInput::Home,
            ContextMenuKeyboardIntent::MoveTo(0),
        ),
        (
            ContextMenuKeyboardInput::End,
            ContextMenuKeyboardIntent::MoveTo(1),
        ),
        (
            ContextMenuKeyboardInput::Enter,
            ContextMenuKeyboardIntent::Activate,
        ),
        (
            ContextMenuKeyboardInput::Space,
            ContextMenuKeyboardIntent::Activate,
        ),
        (
            ContextMenuKeyboardInput::Escape,
            ContextMenuKeyboardIntent::Close,
        ),
        (
            ContextMenuKeyboardInput::ArrowRight,
            ContextMenuKeyboardIntent::OpenSubmenu,
        ),
        (
            ContextMenuKeyboardInput::ArrowLeft,
            ContextMenuKeyboardIntent::CloseSubmenu,
        ),
    ] {
        assert_eq!(
            expected,
            ContextMenuKeyboardNavigator::intent(&items, None, &input)
        );
    }
    assert_eq!(
        ContextMenuKeyboardIntent::MoveTo(1),
        ContextMenuKeyboardNavigator::intent(&items, Some(0), &ContextMenuKeyboardInput::ArrowDown,)
    );
    assert_eq!(
        ContextMenuKeyboardIntent::MoveTo(0),
        ContextMenuKeyboardNavigator::intent(&items, Some(1), &ContextMenuKeyboardInput::ArrowUp,)
    );
    assert_eq!(
        ContextMenuKeyboardIntent::MoveTo(1),
        ContextMenuKeyboardNavigator::intent(
            &items,
            Some(usize::MAX),
            &ContextMenuKeyboardInput::ArrowDown,
        )
    );
    let mut typeahead = ContextMenuTypeAheadBuffer::new(100);
    assert_eq!("f", typeahead.push("f", 10));
    assert_eq!("s", typeahead.push("s", 200));

    let settings = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General")
            .field(SettingsField::new(
                "toggle",
                "Toggle",
                SettingsControl::Toggle { checked: false },
            ))
            .footer("Restart required"),
    );
    assert!(settings.content_height() > 0);
    let section_input = SettingsListHitTestInput {
        pointer_x: 200,
        pointer_y: 75,
        scroll_offset_y: 0,
    };
    assert_ne!(
        SettingsListHitTestResult::None,
        settings.hit_test(section_input)
    );
    assert!(settings.action_for_hit(section_input).is_some());
    assert_eq!(UiCursor::Pointer, settings.cursor_for_hit(section_input));
    assert_eq!(2, settings.hit_targets(320).len());
    assert!(settings.hit_target_for_field("toggle", 320).is_some());
    assert!(settings.hit_target_for_field("missing", 320).is_none());
    assert!(settings.hit_target_for_section("general", 320).is_some());
    assert!(settings.hit_target_for_section("missing", 320).is_none());
    assert!(settings.hit_target_for_hit(section_input, 320).is_some());
    assert!(
        settings
            .interaction_for_hit(section_input, 320)
            .target
            .is_some()
    );
    let outside_input = SettingsListHitTestInput {
        pointer_x: 0,
        pointer_y: u32::MAX,
        scroll_offset_y: 0,
    };
    assert_eq!(
        SettingsListHitTestResult::None,
        settings.interaction_for_hit(outside_input, 320).result
    );

    let mut tabs = CloseableTabStrip::new("Workspace").tab(CloseableTab::new("tab", "Tab"));
    assert_eq!(
        vec![CloseableTabStripEvent::TabPinChanged {
            tab_id: CloseableTabId::new("tab"),
            pinned: true,
        }],
        tabs.apply_action(CloseableTabStripAction::PinTab {
            tab_id: CloseableTabId::new("tab"),
        })
    );

    let node = UiNode::from(
        CloseableTabStrip::new("Workspace")
            .tab(CloseableTab::new("orphan", "Orphan").group_id("missing")),
    );
    assert_eq!(1, node.children().len());
    assert_eq!(UiNodeKind::CloseableTab, node.children()[0].kind());

    let mut grouped = CloseableTabStrip::new("Workspace")
        .group(CloseableTabGroup::new("docs", "Docs"))
        .tab(
            CloseableTab::new("pinned", "Pinned")
                .pinned(true)
                .group_id("docs"),
        );
    assert_eq!(
        vec![CloseableTabStripEvent::TabGroupChanged {
            tab_id: CloseableTabId::new("pinned"),
            group_id: None,
        }],
        grouped.apply_action(CloseableTabStripAction::PinTab {
            tab_id: CloseableTabId::new("pinned"),
        })
    );

    let mut dirty_group = CloseableTabStrip::new("Workspace")
        .group(CloseableTabGroup::new("drafts", "Drafts"))
        .tab(
            CloseableTab::new("draft", "Draft")
                .dirty(true)
                .group_id("drafts"),
        );
    assert_eq!(
        vec![CloseableTabStripEvent::TabCloseRequested {
            tab_id: CloseableTabId::new("draft"),
        }],
        dirty_group.apply_action(CloseableTabStripAction::CloseGroup {
            group_id: "drafts".into(),
        })
    );
    assert_eq!(1, dirty_group.options().groups.len());
    assert!(
        dirty_group
            .apply_action(CloseableTabStripAction::CloseGroup {
                group_id: "missing".into(),
            })
            .is_empty()
    );
}
