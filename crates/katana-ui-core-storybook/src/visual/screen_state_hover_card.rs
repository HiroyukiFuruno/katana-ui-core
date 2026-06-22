use super::screen_state::StorybookScreenState;
use katana_ui_core::molecule::{HoverCard, HoverCardAction, HoverCardEvent, PopoverActionSlot};
use katana_ui_core::render_model::UiNodeId;

const OPEN_DELAY_MS: u16 = 0;
const CLOSE_DELAY_MS: u16 = 220;
const SLOT_ACTION_ID: &str = "configure-action";

impl StorybookScreenState {
    pub(in crate::visual) fn register_hover_card_open(&mut self) {
        let event = hover_card_event(HoverCardAction::AnchorPointerEntered);
        if event != HoverCardEvent::Opened {
            return;
        }
        self.action_count += 1;
        self.last_action = "hover_card_open";
        self.last_event = "hover_card_opened";
        self.last_setting = "interaction.open";
        self.last_setting_value = "true";
        self.state_label = "open=true";
    }

    pub(in crate::visual) fn register_hover_card_hover(&mut self) {
        let event = hover_card_event(HoverCardAction::AnchorPointerEntered);
        if event != HoverCardEvent::Opened {
            return;
        }
        self.action_count += 1;
        self.preview_hovered = true;
        self.last_action = "hover_card_hover";
        self.last_event = "hover_card_opened";
        self.last_setting = "interaction.hovered";
        self.last_setting_value = "true";
        self.state_label = "hover=true open=true";
    }

    pub(in crate::visual) fn register_hover_card_focus(&mut self) {
        let event = hover_card_event(HoverCardAction::AnchorFocused);
        if event != HoverCardEvent::Opened {
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "hover_card_focus";
        self.last_event = "hover_card_opened";
        self.last_setting = "interaction.focused";
        self.last_setting_value = "true";
        self.state_label = "focus=true open=true";
    }

    pub(in crate::visual) fn register_hover_card_inner_focus_keep_open(&mut self) {
        let event = hover_card_event(HoverCardAction::InnerFocusEntered(UiNodeId::new(
            SLOT_ACTION_ID,
        )));
        if event != HoverCardEvent::KeptOpen {
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "hover_card_inner_focus";
        self.last_event = "hover_card_kept_open";
        self.last_setting = "hover_card.slot_action";
        self.last_setting_value = "visible";
        self.state_label = "inner_focus=kept_open";
    }
}

fn hover_card_event(action: HoverCardAction) -> HoverCardEvent {
    let mut hover_card = hover_card_fixture();
    if matches!(action, HoverCardAction::InnerFocusEntered(_)) {
        let _ = hover_card.apply_hover_card_action(HoverCardAction::AnchorPointerEntered);
    }
    hover_card.apply_hover_card_action(action)
}

fn hover_card_fixture() -> HoverCard {
    HoverCard::new("Capability")
        .open_delay_ms(OPEN_DELAY_MS)
        .close_delay_ms(CLOSE_DELAY_MS)
        .pointer_follow(true)
        .slot_action(PopoverActionSlot::new(SLOT_ACTION_ID, "Configure"))
}
