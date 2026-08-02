use katana_ui_core::molecule::{
    CommandPalette, DiagnosticsList, EmptyState, EmptyStateAction, EmptyStateActionId,
    EmptyStateAlignment, EmptyStateContractViolation, EmptyStateEvent, EmptyStateSize,
    EmptyStateTone, SearchBox, SelectionList, TreeView,
};
use katana_ui_core::render_model::{UiNodeKind, UiTone, UiTree, UiVariant};

#[test]
fn heading_is_required_and_media_is_exclusive() {
    assert_eq!(
        Err(EmptyStateContractViolation::MissingHeading),
        EmptyState::new(" ").validate()
    );
    assert_eq!(
        Err(EmptyStateContractViolation::IconAndIllustrationConflict),
        EmptyState::new("No files")
            .icon("folder")
            .illustration("empty-folder")
            .validate()
    );
    assert_eq!(Ok(()), EmptyState::new("No files").validate());
}

#[test]
fn primary_and_secondary_actions_emit_typed_events() {
    let empty = EmptyState::new("No results")
        .primary_action(EmptyStateAction::new("create", "Create"))
        .secondary_action(EmptyStateAction::new("docs", "Open docs"));

    assert_eq!(
        Some(EmptyStateEvent::Actioned {
            id: EmptyStateActionId::Primary,
            action_id: "create".to_string()
        }),
        empty.apply_action(EmptyStateActionId::Primary)
    );
    assert_eq!(
        Some(EmptyStateEvent::Actioned {
            id: EmptyStateActionId::Secondary,
            action_id: "docs".to_string()
        }),
        empty.apply_action(EmptyStateActionId::Secondary)
    );
}

#[test]
fn missing_actions_do_not_render_action_children() {
    let tree = UiTree::new(EmptyState::new("Clean").body("No diagnostics"));

    assert_eq!(UiNodeKind::EmptyState, tree.root().kind());
    assert_eq!(1, tree.root().children().len());
    assert_eq!("No diagnostics", tree.root().children()[0].props().label);
}

#[test]
fn actions_render_as_distinct_button_atoms() {
    let tree = UiTree::new(
        EmptyState::new("No results")
            .primary_action(EmptyStateAction::new("create", "Create"))
            .secondary_action(EmptyStateAction::new("docs", "Open docs")),
    );
    let buttons = tree
        .root()
        .children()
        .iter()
        .filter(|it| it.kind() == UiNodeKind::Button)
        .collect::<Vec<_>>();

    assert_eq!(2, buttons.len());
    assert_eq!(UiVariant::Filled, buttons[0].props().variant);
    assert_eq!(UiTone::Accent, buttons[0].props().tone);
    assert_eq!(UiVariant::Text, buttons[1].props().variant);
    assert_eq!(UiTone::Neutral, buttons[1].props().tone);
}

#[test]
fn tone_does_not_change_layout_snapshot() {
    let neutral = EmptyState::new("No files")
        .body("Drop a file")
        .size(EmptyStateSize::Large)
        .alignment(EmptyStateAlignment::Leading)
        .tone(EmptyStateTone::Neutral)
        .layout_snapshot();
    let danger = EmptyState::new("No files")
        .body("Drop a file")
        .size(EmptyStateSize::Large)
        .alignment(EmptyStateAlignment::Leading)
        .tone(EmptyStateTone::Danger)
        .layout_snapshot();

    assert_eq!(neutral, danger);
}

#[test]
fn size_alignment_and_actions_are_reflected_in_layout_snapshot() {
    let compact = EmptyState::new("No files")
        .size(EmptyStateSize::Compact)
        .alignment(EmptyStateAlignment::Center)
        .layout_snapshot();
    let large = EmptyState::new("No files")
        .size(EmptyStateSize::Large)
        .alignment(EmptyStateAlignment::Leading)
        .body("Drop a file")
        .primary_action(EmptyStateAction::new("drop", "Drop file"))
        .secondary_action(EmptyStateAction::new("docs", "Open docs"))
        .layout_snapshot();

    assert_ne!(compact, large);
    assert_eq!(EmptyStateSize::Large, large.size);
    assert_eq!(EmptyStateAlignment::Leading, large.alignment);
    assert!(large.has_body);
    assert_eq!(2, large.action_count);

    let default = EmptyState::new("Default").layout_snapshot();
    assert_eq!(EmptyStateSize::Default, default.size);
    assert_eq!(360, default.heading_rect.width + 80);
}

#[test]
fn live_region_payload_is_exposed_as_accessibility_label() {
    let empty = EmptyState::new("No results").tone(EmptyStateTone::Warning);
    let tree = UiTree::new(empty.clone());

    assert_eq!("Warning: No results", empty.announce_payload());
    assert_eq!(
        "Warning: No results",
        tree.root().props().accessibility_label
    );
}

#[test]
fn all_empty_state_tone_and_size_conversions_are_total() {
    assert_eq!(
        ["Neutral", "Subtle", "Accent", "Warning", "Danger"],
        [
            EmptyStateTone::Neutral,
            EmptyStateTone::Subtle,
            EmptyStateTone::Accent,
            EmptyStateTone::Warning,
            EmptyStateTone::Danger,
        ]
        .map(EmptyStateTone::announce_label)
    );
    assert_eq!(
        [
            UiTone::Neutral,
            UiTone::Accent,
            UiTone::Accent,
            UiTone::Warning,
            UiTone::Danger,
        ],
        [
            EmptyStateTone::Neutral,
            EmptyStateTone::Subtle,
            EmptyStateTone::Accent,
            EmptyStateTone::Warning,
            EmptyStateTone::Danger,
        ]
        .map(UiTone::from)
    );
    assert_eq!(
        [
            katana_ui_core::render_model::UiSize::Small,
            katana_ui_core::render_model::UiSize::Medium,
            katana_ui_core::render_model::UiSize::Large,
        ],
        [
            EmptyStateSize::Compact,
            EmptyStateSize::Default,
            EmptyStateSize::Large,
        ]
        .map(katana_ui_core::render_model::UiSize::from)
    );
}

#[test]
fn icon_illustration_leading_alignment_and_state_id_render_independently() {
    let icon = EmptyState::new("Icon")
        .icon("folder")
        .alignment(EmptyStateAlignment::Leading);
    let icon_id = icon.state_id().clone();
    let icon_tree = UiTree::new(icon);
    assert_eq!(icon_id, icon_tree.root().props().state_id);
    assert!(
        icon_tree
            .root()
            .children()
            .iter()
            .any(|node| node.kind() == UiNodeKind::Icon && node.props().label == "folder")
    );
    assert_eq!(
        katana_ui_core::render_model::UiJustifyContent::Start,
        icon_tree.root().props().common.justify_content
    );

    let illustration_tree =
        UiTree::new(EmptyState::new("Illustration").illustration("empty-folder"));
    assert!(
        illustration_tree
            .root()
            .children()
            .iter()
            .any(|node| node.kind() == UiNodeKind::Icon && node.props().label == "empty-folder")
    );
}

#[test]
fn empty_state_embeds_without_parent_state_conflict() {
    let tree = UiTree::new(
        SelectionList::new("Filtered").child(
            EmptyState::new("No matching items")
                .primary_action(EmptyStateAction::new("clear", "Clear filter")),
        ),
    );
    let empty = &tree.root().children()[0];

    assert_eq!(UiNodeKind::EmptyState, empty.kind());
    assert_ne!(tree.root().props().state_id, empty.props().state_id);
}

#[test]
fn empty_state_embeds_in_required_empty_hosts_with_distinct_state() {
    assert_embedded_empty_state(UiTree::new(
        DiagnosticsList::new("Diagnostics").empty_slot(EmptyState::new("No diagnostics")),
    ));
    assert_embedded_empty_state(UiTree::new(
        TreeView::new("Tree").child(EmptyState::new("No files")),
    ));
    assert_embedded_empty_state(UiTree::new(
        CommandPalette::new("Commands").child(EmptyState::new("No commands")),
    ));
    assert_embedded_empty_state(UiTree::new(
        SearchBox::new("Search").child(EmptyState::new("No results")),
    ));
}

fn assert_embedded_empty_state(tree: UiTree) {
    let empty = tree
        .root()
        .children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::EmptyState);
    assert!(
        empty.is_some(),
        "required host must render EmptyState child"
    );
    let Some(empty) = empty else {
        return;
    };

    assert_ne!(tree.root().props().state_id, empty.props().state_id);
}
