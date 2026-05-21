use super::{
    HOVER_CARD_CLOSE_DELAY_MS, HOVER_CARD_OPEN_DELAY_MS, StoryCatalog, StoryExample, UiCallbackLog,
    molecule,
};

pub(super) fn hover_card_story() -> StoryExample {
    let slots = molecule::PopoverSlots::new()
        .heading("Capability")
        .body("Shows rich hover and focus content")
        .footer("Keeps open while the card is focused")
        .action(molecule::PopoverActionSlot::new(
            "configure-action",
            "Configure",
        ));
    let mut hover_card = molecule::HoverCard::new("Hover card")
        .open_delay_ms(HOVER_CARD_OPEN_DELAY_MS)
        .close_delay_ms(HOVER_CARD_CLOSE_DELAY_MS)
        .pointer_follow(true)
        .slots(slots);
    let opened =
        hover_card.apply_hover_card_action(molecule::HoverCardAction::AnchorPointerEntered);
    let kept = hover_card.apply_hover_card_action(molecule::HoverCardAction::CardPointerEntered);
    let target = katana_ui_core::render_model::UiStateId::new("state:HoverCard:storybook");
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "hover_card_open",
            "open=false",
            format!("event={opened:?}"),
        ),
        UiCallbackLog::new(
            target,
            "hover_card_keep_open",
            "close=scheduled",
            format!("event={kept:?}"),
        ),
    ];
    StoryCatalog::interactive_story("hover-card", hover_card, logs)
}
