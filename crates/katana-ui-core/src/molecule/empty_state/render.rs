use super::{EmptyState, EmptyStateAction};
use crate::atom::{Button, Icon, Text};
use crate::render_model::{UiCommonProps, UiJustifyContent, UiNode, UiNodeKind, UiTone, UiVariant};

impl From<EmptyState> for UiNode {
    fn from(value: EmptyState) -> Self {
        let mut node = UiNode::from_state(
            UiNodeKind::EmptyState,
            value.heading.clone(),
            value.state_id.clone(),
        )
        .common(common_props(&value))
        .accessibility_label(value.announce_payload())
        .tone(value.tone.into())
        .size(value.size.into());
        if let Some(icon) = value.icon {
            node = node.child(Icon::new(icon));
        }
        if let Some(illustration) = value.illustration {
            node = node.child(Icon::new(illustration));
        }
        if let Some(body) = value.body {
            node = node.child(Text::new(body));
        }
        if let Some(action) = value.primary_action {
            node = node.child(primary_button(action));
        }
        if let Some(action) = value.secondary_action {
            node = node.child(secondary_button(action));
        }
        node
    }
}

fn common_props(value: &EmptyState) -> UiCommonProps {
    UiCommonProps::default().justify_content(match value.alignment {
        super::EmptyStateAlignment::Center => UiJustifyContent::Center,
        super::EmptyStateAlignment::Leading => UiJustifyContent::Start,
    })
}

fn primary_button(action: EmptyStateAction) -> Button {
    Button::new(action.label)
        .variant(UiVariant::Filled)
        .tone(UiTone::Accent)
        .focusable(true)
}

fn secondary_button(action: EmptyStateAction) -> Button {
    Button::new(action.label)
        .variant(UiVariant::Text)
        .tone(UiTone::Neutral)
        .focusable(true)
}
