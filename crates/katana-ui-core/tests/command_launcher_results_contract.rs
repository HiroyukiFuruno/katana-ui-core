use katana_ui_core::atom::{KeyCombo, KeyKind, KeyModifiers, NamedKey, ShortcutCombo};
use katana_ui_core::interaction::{RowHeightProvider, VirtualizationConfig};
use katana_ui_core::molecule::{
    CommandKeyboardInput, CommandLauncherAction, CommandLauncherEvent, CommandPalette,
    CommandResultRow, HighlightMove,
};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

#[test]
fn result_row_preserves_visual_metadata_without_domain_action() {
    let palette = CommandPalette::new("Commands")
        .result_row(primary_row())
        .highlighted_index(Some(0));
    let tree = UiTree::new(palette);
    let root = tree.root();
    let row = &root.children()[0];

    assert_eq!(UiNodeKind::CommandPalette, root.kind());
    assert_eq!(UiNodeKind::CommandResultRow, row.kind());
    assert_eq!("format", row.props().command_result.id);
    assert_eq!("Format Document", row.props().label);
    assert_eq!("Rust Analyzer", row.props().command_result.secondary_label);
    assert_eq!("code", row.props().command_result.icon);
    assert_eq!("editor", row.props().command_result.provider_id);
    assert_eq!("source", row.props().command_result.group_id);
    assert!(!row.props().disabled);
    assert!(
        row.children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::ShortcutCombo)
    );
}

#[test]
fn query_change_updates_state_and_highlights_first_enabled_row() {
    let mut palette = CommandPalette::new("Commands")
        .result_row(disabled_row())
        .result_row(primary_row());
    let events = palette.apply_launcher_action(CommandLauncherAction::SetQuery("theme".into()));

    assert_eq!("theme", palette.query_model());
    assert_eq!(Some(1), palette.command_highlighted_index_model());
    assert_eq!(
        events,
        vec![
            CommandLauncherEvent::QueryChanged("theme".to_string()),
            CommandLauncherEvent::ResultHighlighted {
                index: Some(1),
                id: Some("format".to_string())
            }
        ]
    );
}

#[test]
fn keyboard_moves_highlight_and_disabled_row_does_not_execute() {
    let mut palette = CommandPalette::new("Commands")
        .result_row(primary_row())
        .result_row(disabled_row())
        .result_row(CommandResultRow::new("open", "Open File"))
        .highlighted_index(Some(0));

    assert_eq!(
        vec![CommandLauncherEvent::ResultHighlighted {
            index: Some(1),
            id: Some("locked".to_string())
        }],
        palette.apply_launcher_action(CommandLauncherAction::Keyboard(
            CommandKeyboardInput::ArrowDown
        ))
    );
    assert!(
        palette
            .apply_launcher_action(CommandLauncherAction::Keyboard(CommandKeyboardInput::Enter))
            .is_empty()
    );

    assert_eq!(
        vec![CommandLauncherEvent::ResultHighlighted {
            index: Some(2),
            id: Some("open".to_string())
        }],
        palette.apply_launcher_action(CommandLauncherAction::MoveHighlight(HighlightMove::Last))
    );
    assert_eq!(
        vec![CommandLauncherEvent::ResultExecuted {
            id: "open".to_string()
        }],
        palette.apply_launcher_action(CommandLauncherAction::SelectHighlighted)
    );
}

#[test]
fn keyboard_escape_closes_command_palette() {
    let mut palette = CommandPalette::new("Commands").result_row(primary_row());

    assert_eq!(
        vec![CommandLauncherEvent::Closed],
        palette.apply_launcher_action(CommandLauncherAction::Keyboard(
            CommandKeyboardInput::Escape
        ))
    );
}

#[test]
fn keyboard_and_direct_execution_cover_all_navigation_and_rejection_paths() {
    let mut palette = CommandPalette::new("Commands")
        .result_row(primary_row())
        .result_row(disabled_row())
        .result_row(CommandResultRow::new("open", "Open File"))
        .highlighted_index(Some(2));

    for (input, expected_index) in [
        (CommandKeyboardInput::ArrowUp, Some(1)),
        (CommandKeyboardInput::Home, Some(0)),
        (CommandKeyboardInput::End, Some(2)),
    ] {
        let events = palette.apply_launcher_action(CommandLauncherAction::Keyboard(input));
        assert_eq!(expected_index, palette.command_highlighted_index_model());
        assert!(matches!(
            events.as_slice(),
            [CommandLauncherEvent::ResultHighlighted { .. }]
        ));
    }

    assert_eq!(
        vec![CommandLauncherEvent::ResultExecuted {
            id: "open".to_string()
        }],
        palette.apply_launcher_action(CommandLauncherAction::Execute("open".to_string()))
    );
    assert!(
        palette
            .apply_launcher_action(CommandLauncherAction::Execute("locked".to_string()))
            .is_empty()
    );
    assert!(
        palette
            .apply_launcher_action(CommandLauncherAction::Execute("missing".to_string()))
            .is_empty()
    );

    let mut empty = CommandPalette::new("Empty");
    assert!(
        empty
            .apply_launcher_action(CommandLauncherAction::MoveHighlight(
                HighlightMove::Previous
            ))
            .iter()
            .any(|event| matches!(
                event,
                CommandLauncherEvent::ResultHighlighted {
                    index: None,
                    id: None
                }
            ))
    );
    assert!(
        empty
            .apply_launcher_action(CommandLauncherAction::SelectHighlighted)
            .is_empty()
    );
    assert_eq!(
        vec![CommandLauncherEvent::Closed],
        empty.apply_launcher_action(CommandLauncherAction::Close)
    );
}

#[test]
fn virtualization_range_keeps_highlighted_row_reachable() {
    let mut palette = CommandPalette::new("Commands");
    for index in 0..50 {
        palette = palette.result_row(CommandResultRow::new(
            format!("command-{index}"),
            format!("Command {index}"),
        ));
    }
    let palette = palette
        .highlighted_index(Some(40))
        .virtualization(VirtualizationConfig {
            enabled: true,
            total_count: 50,
            viewport_offset: 0,
            viewport_height: 30,
            overscan: 0,
            row_height_provider: RowHeightProvider::Fixed { height: 10 },
            keep_focused_in_window: true,
            focused_index: Some(40),
        });
    let ranges = palette
        .command_virtual_range_model()
        .into_iter()
        .collect::<Vec<_>>();

    assert_eq!(1, ranges.len(), "virtual range is available");
    assert!((ranges[0].start..ranges[0].end).contains(&40));
    assert_eq!(50, ranges[0].aria_set_size);
    assert!(
        ranges[0]
            .rows
            .iter()
            .any(|it| it.index == 40 && it.aria_pos_in_set == 41)
    );
}

#[test]
fn virtualization_updates_with_actions_and_supports_all_height_providers() {
    let providers = [
        RowHeightProvider::Fixed { height: 10 },
        RowHeightProvider::Variable {
            row_heights: vec![10; 50],
            fallback_height: 10,
        },
        RowHeightProvider::Estimated {
            estimated_height: 10,
            measured_overrides: Vec::new(),
        },
    ];

    for provider in providers {
        let mut palette = CommandPalette::new("Commands");
        for index in 0..50 {
            palette = palette.result_row(CommandResultRow::new(
                format!("command-{index}"),
                format!("Command {index}"),
            ));
        }
        palette = palette
            .highlighted_index(Some(40))
            .virtualization(VirtualizationConfig {
                enabled: true,
                total_count: 0,
                viewport_offset: 0,
                viewport_height: 30,
                overscan: 0,
                row_height_provider: provider,
                keep_focused_in_window: false,
                focused_index: None,
            });

        assert_eq!(50, palette.result_rows_model().len());
        assert!(
            palette
                .apply_launcher_action(CommandLauncherAction::SetQuery("command-4".to_string()))
                .iter()
                .any(|event| matches!(
                    event,
                    CommandLauncherEvent::ResultHighlighted { index: Some(4), .. }
                ))
        );
        assert_eq!(Some(4), palette.command_highlighted_index_model());
        assert!(
            palette
                .command_virtual_range_model()
                .is_some_and(|range| (range.start..range.end).contains(&4))
        );
    }
}

#[test]
fn virtualization_keeps_an_already_visible_highlight_and_renders_disabled_reason() {
    let palette = CommandPalette::new("Commands")
        .result_row(disabled_row())
        .highlighted_index(Some(0))
        .virtualization(VirtualizationConfig {
            enabled: true,
            total_count: 1,
            viewport_offset: 0,
            viewport_height: 30,
            overscan: 0,
            row_height_provider: RowHeightProvider::Fixed { height: 10 },
            keep_focused_in_window: true,
            focused_index: Some(0),
        });
    let tree = UiTree::new(palette);
    let row = &tree.root().children()[0];

    assert_eq!(
        "Locked command disabled: Workspace is readonly",
        row.props().accessibility_label
    );
    assert!(
        row.children()
            .iter()
            .any(|child| child.props().label == "Workspace is readonly")
    );
}

fn primary_row() -> CommandResultRow {
    CommandResultRow::new("format", "Format Document")
        .secondary_label("Rust Analyzer")
        .icon("code")
        .provider_id("editor")
        .group_id("source")
        .shortcut(ShortcutCombo::new(
            "Format",
            KeyCombo::new(
                KeyModifiers::command_shift(),
                KeyKind::Named(NamedKey::Enter),
            ),
        ))
}

fn disabled_row() -> CommandResultRow {
    CommandResultRow::new("locked", "Locked command").disabled("Workspace is readonly")
}
