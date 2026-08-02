use katana_ui_core::atom::{Chip, ChipKeyboardInput};
use katana_ui_core::molecule::{
    ChipGroup, ChipGroupAction, ChipGroupEvent, ChipGroupFocusTarget, ChipGroupOverflow,
};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

#[test]
fn menu_overflow_distinguishes_fitting_and_hidden_chips() {
    let fitting = ChipGroup::new("Tags")
        .overflow(ChipGroupOverflow::Menu)
        .wrap(false)
        .available_width(200)
        .chip(Chip::new("one"), 40);
    assert!(!fitting.layout().overflow_trigger_visible());
    assert!(fitting.overflow_menu().is_none());

    let mut overflowing = ChipGroup::new("Tags")
        .overflow(ChipGroupOverflow::Menu)
        .wrap(false)
        .available_width(80)
        .overflow_trigger_width(24)
        .gap(8)
        .chip(Chip::new("one"), 48)
        .chip(Chip::new("two"), 48);
    let layout = overflowing.layout();
    assert!(layout.overflow_trigger_visible());
    assert_eq!(1, layout.visible_chip_ids().len());
    assert_eq!(1, layout.hidden_chip_ids().len());
    assert!(overflowing.overflow_menu().is_some());
    let tree = UiTree::new(overflowing.clone());
    assert_eq!(2, tree.root().children().len());
    assert_eq!(UiNodeKind::Chip, tree.root().children()[1].kind());

    let events = overflowing.apply_action(ChipGroupAction::OpenOverflow);
    assert!(matches!(
        events.as_slice(),
        [ChipGroupEvent::OverflowOpened { hidden_chip_ids }] if hidden_chip_ids.len() == 1
    ));
    assert!(overflowing.overflow_open());
}

#[test]
fn scroll_and_reorder_actions_enforce_their_modes() {
    let mut plain = ChipGroup::new("Plain")
        .chip(Chip::new("one"), 40)
        .chip(Chip::new("two"), 40);
    assert!(plain.apply_action(ChipGroupAction::OpenOverflow).is_empty());
    assert!(
        plain
            .apply_action(ChipGroupAction::ScrollHorizontal { offset: 12 })
            .is_empty()
    );
    assert!(
        plain
            .apply_action(ChipGroupAction::Reorder { from: 0, to: 1 })
            .is_empty()
    );

    let mut interactive = ChipGroup::new("Interactive")
        .overflow(ChipGroupOverflow::ScrollHorizontal)
        .reorder(true)
        .chip(Chip::new("one"), 40)
        .chip(Chip::new("two"), 40);
    assert_eq!(
        vec![ChipGroupEvent::Scrolled { offset: 32 }],
        interactive.apply_action(ChipGroupAction::ScrollHorizontal { offset: 32 })
    );
    assert_eq!(32, interactive.layout().scroll_offset());

    let events = interactive.apply_action(ChipGroupAction::Reorder { from: 0, to: 1 });
    assert!(matches!(
        events.as_slice(),
        [ChipGroupEvent::ChipReordered { from: 0, to: 1, .. }]
    ));
    assert_eq!("two", interactive.chips()[0].chip().label());
}

#[test]
fn keyboard_dismiss_requires_focus_and_dismissibility() {
    let mut unfocused = ChipGroup::new("Tags").chip(Chip::new("one").dismissible(true), 40);
    assert!(
        unfocused
            .dismiss_focused_with_keyboard(ChipKeyboardInput::Delete)
            .is_empty()
    );

    let mut fixed = ChipGroup::new("Tags").chip(Chip::new("fixed").focused(true), 40);
    assert!(
        fixed
            .dismiss_focused_with_keyboard(ChipKeyboardInput::Delete)
            .is_empty()
    );

    let mut only =
        ChipGroup::new("Tags").chip(Chip::new("only").dismissible(true).focused(true), 40);
    let events = only.dismiss_focused_with_keyboard(ChipKeyboardInput::Backspace);
    assert!(matches!(
        events.as_slice(),
        [ChipGroupEvent::ChipDismissed {
            focus_target: ChipGroupFocusTarget::PriorFocusHolder,
            ..
        }]
    ));
}

#[test]
fn keyboard_dismiss_moves_focus_to_the_previous_chip() {
    let mut group = ChipGroup::new("Tags")
        .chip(Chip::new("first").dismissible(true), 40)
        .chip(Chip::new("second").dismissible(true).focused(true), 40);
    let first_id = group.chips()[0].chip().state_id().clone();

    let events = group.dismiss_focused_with_keyboard(ChipKeyboardInput::Delete);

    assert!(matches!(
        events.as_slice(),
        [ChipGroupEvent::ChipDismissed {
            focus_target: ChipGroupFocusTarget::Chip(id),
            ..
        }] if id == &first_id
    ));
    assert!(group.chips()[0].chip().focused_value());
    assert_eq!(events, group.callback_log());
}
