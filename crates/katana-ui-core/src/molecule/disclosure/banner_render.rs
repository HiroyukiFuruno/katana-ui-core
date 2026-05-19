use super::banner::Banner;
use super::banner_types::BannerDensity;
use crate::atom::{Button, Icon, Text};
use crate::render_model::{
    UiDismissAction, UiInteractionState, UiNode, UiNodeKind, UiSize, UiStatusProps, UiVariant,
};

impl From<Banner> for UiNode {
    fn from(value: Banner) -> Self {
        let contract = value.visual_contract();
        let status = UiStatusProps {
            severity: contract.tone,
            variant: UiVariant::Filled,
            dismiss_action: dismiss_action(value.dismissible),
            leading_icon: contract.icon.clone().unwrap_or_default(),
        };
        let mut node = UiNode::from_state(
            UiNodeKind::Banner,
            value.message.clone(),
            value.state_id.clone(),
        )
        .visible(value.state.visible)
        .status(status)
        .tone(contract.tone)
        .size(value.density.into())
        .interaction(value.interaction_state())
        .accessibility_label(value.announce_text());
        if let Some(icon) = contract.icon {
            node = node.child(Icon::new(icon));
        }
        node = node.child(Text::new(value.message));
        for action in value.actions {
            node = node.child(Button::new(action.label).disabled(action.disabled));
        }
        node
    }
}

impl Banner {
    pub(super) fn interaction_state(&self) -> UiInteractionState {
        UiInteractionState {
            open: self.state.details_open,
            item_count: self.actions.len(),
            value: self.visual_contract().role.as_str().to_string(),
            ..UiInteractionState::default()
        }
    }

    pub(super) fn announce_text(&self) -> String {
        match &self.title {
            Some(title) => format!("{title}: {}", self.message),
            None => self.message.clone(),
        }
    }
}

fn dismiss_action(dismissible: bool) -> UiDismissAction {
    if dismissible {
        UiDismissAction::Available
    } else {
        UiDismissAction::None
    }
}

impl From<BannerDensity> for UiSize {
    fn from(value: BannerDensity) -> Self {
        match value {
            BannerDensity::Compact => Self::Small,
            BannerDensity::Default => Self::Medium,
        }
    }
}
