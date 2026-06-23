use katana_ui_core::atom::{Chip, ChipAction, ChipEvent, ChipKeyboardInput, ChipTone, ChipVariant};
use katana_ui_core::molecule::{
    AttachmentChip, AttachmentChipAction as AAction, AttachmentChipEvent as AEvent, AttachmentKind,
    AttachmentProgress, AttachmentStatus as AStatus, ChipGroup, ChipGroupAction, ChipGroupEvent,
    ChipGroupFocusTarget, ChipGroupOverflow,
};
use katana_ui_core::render_model::UiStateId;

#[test]
fn chip_keeps_typed_options_and_keyboard_dismisses() {
    let mut backspace_chip = Chip::new("Filter")
        .leading_icon("filter")
        .tone(ChipTone::Danger)
        .variant(ChipVariant::Outline)
        .dismissible(true)
        .accessibility_label("Remove filter")
        .focused(true);
    let backspace_id = backspace_chip.state_id().clone();
    let mut delete_chip = Chip::new("Delete").dismissible(true).focused(true);
    let delete_id = delete_chip.state_id().clone();

    assert_eq!(Some("filter"), backspace_chip.leading_icon_value());
    assert_eq!("chip.outline.danger", backspace_chip.theme_token_key());
    assert_eq!(
        vec![ChipEvent::ChipDismissed { id: backspace_id }],
        backspace_chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Backspace))
    );
    assert_eq!(
        vec![ChipEvent::ChipDismissed { id: delete_id }],
        delete_chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Delete))
    );
}

#[test]
fn disabled_chip_suppresses_press_and_dismiss() {
    let mut chip = Chip::new("Disabled")
        .interactive(true)
        .dismissible(true)
        .disabled(true)
        .focused(true);

    let blocked = [
        chip.apply_action(ChipAction::Press).is_empty(),
        chip.apply_action(ChipAction::Dismiss).is_empty(),
        chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Delete))
            .is_empty(),
    ];
    assert_eq!([true, true, true], blocked);
}

#[test]
fn attachment_chip_emits_status_transitions_and_retry_reset() {
    let mut attachment = AttachmentChip::new(AttachmentKind::File, "report.pdf")
        .progress(AttachmentProgress::from_basis_points(4_200));
    let id = attachment.chip().state_id().clone();

    let uploading = attachment.apply_action(AAction::TransitionStatus(AStatus::Uploading));
    let _ = attachment.apply_action(AAction::TransitionStatus(AStatus::Ready));
    let _ = attachment.apply_action(AAction::TransitionStatus(AStatus::Error));
    let retry = attachment.apply_action(AAction::Retry);

    assert_eq!(
        vec![status_changed(&id, AStatus::Pending, AStatus::Uploading)],
        uploading
    );
    assert_eq!(AStatus::Pending, attachment.status_value());
    assert_eq!(
        vec![
            AEvent::Retry { id: id.clone() },
            status_changed(&id, AStatus::Error, AStatus::Pending),
        ],
        retry
    );
}

#[test]
fn chip_group_menu_overflow_calculates_hidden_chips() {
    let first = Chip::new("first");
    let second = Chip::new("second");
    let third = Chip::new("third");
    let second_id = second.state_id().clone();
    let third_id = third.state_id().clone();
    let mut group = ChipGroup::new("filters")
        .chip(first, 40)
        .chip(second, 40)
        .chip(third, 40)
        .gap(5)
        .available_width(95)
        .overflow_trigger_width(20)
        .wrap(false)
        .overflow(ChipGroupOverflow::Menu);

    let layout = group.layout();
    let events = group.apply_action(ChipGroupAction::OpenOverflow);
    let menu = group.overflow_menu();

    assert_eq!(
        [second_id.clone(), third_id.clone()],
        layout.hidden_chip_ids()
    );
    assert!(menu.as_ref().is_some_and(|it| it.children().len() == 2));
    assert_eq!(
        vec![ChipGroupEvent::OverflowOpened {
            hidden_chip_ids: vec![second_id, third_id],
        }],
        events
    );
}

#[test]
fn chip_group_scroll_state_and_reorder_are_explicit() {
    let first = Chip::new("first");
    let first_id = first.state_id().clone();
    let second = Chip::new("second");
    let second_id = second.state_id().clone();
    let mut static_group = ChipGroup::new("static")
        .chip(first.clone(), 40)
        .chip(second.clone(), 40);
    let mut group = ChipGroup::new("filters")
        .chip(first, 40)
        .chip(second, 40)
        .overflow(ChipGroupOverflow::ScrollHorizontal)
        .reorder(true);

    assert!(
        static_group
            .apply_action(ChipGroupAction::Reorder { from: 0, to: 1 })
            .is_empty()
    );
    assert_eq!(
        vec![ChipGroupEvent::Scrolled { offset: 24 }],
        group.apply_action(ChipGroupAction::ScrollHorizontal { offset: 24 })
    );
    assert_eq!(24, group.layout().scroll_offset());
    assert_eq!(
        vec![ChipGroupEvent::ChipReordered {
            chip_id: first_id,
            from: 0,
            to: 1,
        }],
        group.apply_action(ChipGroupAction::Reorder { from: 0, to: 1 })
    );
    assert_eq!(second_id, group.chips()[0].chip().state_id().clone());
}

#[test]
fn chip_group_keyboard_dismiss_moves_focus_to_previous_chip() {
    let first = Chip::new("first");
    let first_id = first.state_id().clone();
    let second = Chip::new("second").dismissible(true).focused(true);
    let second_id = second.state_id().clone();
    let mut group = ChipGroup::new("filters").chip(first, 40).chip(second, 40);

    let events = group.dismiss_focused_with_keyboard(ChipKeyboardInput::Backspace);

    assert_eq!(
        vec![ChipGroupEvent::ChipDismissed {
            chip_id: second_id,
            focus_target: ChipGroupFocusTarget::Chip(first_id),
        }],
        events
    );
    assert!(group.chips()[0].chip().focused_value());
}

fn status_changed(id: &UiStateId, previous: AStatus, current: AStatus) -> AEvent {
    AEvent::StatusChanged {
        id: id.clone(),
        previous,
        current,
    }
}
