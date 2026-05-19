use katana_ui_core::molecule::{
    EmptyState, EmptyStateAction, EmptyStateActionId, EmptyStateAlignment,
    EmptyStateContractViolation, EmptyStateEvent, EmptyStateSize, EmptyStateTone, SelectionList,
};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

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
