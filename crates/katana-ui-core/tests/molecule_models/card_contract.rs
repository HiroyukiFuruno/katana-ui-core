use katana_ui_core::atom::{Button, Text};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::Card;
use katana_ui_core::render_model::{UiNodeKind, UiSize, UiTree, UiVariant};

#[test]
fn card_owns_layout_options_and_named_regions() {
    let card = Card::new("Summary")
        .variant(UiVariant::Outline)
        .padding(UiSize::Large)
        .interactive(true)
        .header(Text::new("Header"))
        .child(Text::new("Body"))
        .footer(Button::new("Open"));

    let tree = UiTree::new(card);

    assert_eq!(UiNodeKind::Card, tree.root().kind());
    assert_eq!(UiVariant::Outline, tree.root().props().variant);
    assert_eq!(UiSize::Large, tree.root().props().size);
    assert!(tree.root().props().focusable);
    assert_eq!(3, tree.root().children().len());
}

#[test]
fn interactive_card_click_updates_owned_state_and_event_log() {
    let mut card = Card::new("Summary").interactive(true);
    let action = UiAction::click(card.state_id().clone());

    let result = card.apply_action(&action);
    let tree = UiTree::new(card);

    assert!(result.handled);
    assert_eq!("click", result.callback_log[0].action);
    assert!(result.after.has_selection);
    assert!(tree.root().props().interaction.has_selection);
}

#[test]
fn non_interactive_card_ignores_click_without_selection_mutation() {
    let mut card = Card::new("Summary");
    let action = UiAction::click(card.state_id().clone());

    let result = card.apply_action(&action);
    let tree = UiTree::new(card);

    assert!(!result.handled);
    assert!(result.callback_log.is_empty());
    assert!(!result.after.has_selection);
    assert!(!tree.root().props().interaction.has_selection);
    assert!(!tree.root().props().focusable);
}
