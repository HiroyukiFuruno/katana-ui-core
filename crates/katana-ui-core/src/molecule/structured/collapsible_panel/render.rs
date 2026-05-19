use super::{CollapsiblePanel, PanelMode};
use crate::render_model::{
    UiCommonProps, UiDimension, UiInteractionState, UiNode, UiNodeKind, UiPosition, UiZIndex,
};

const EXPANDED_MODE_INDEX: usize = 0;
const ICON_ONLY_MODE_INDEX: usize = 1;
const COLLAPSED_MODE_INDEX: usize = 2;
const FLOATING_OVERLAY_MODE_INDEX: usize = 3;

impl From<CollapsiblePanel> for UiNode {
    fn from(value: CollapsiblePanel) -> Self {
        let common = common_props(&value);
        let interaction = interaction_state(&value);
        let mut node =
            UiNode::from_state(UiNodeKind::CollapsiblePanel, value.label, value.state_id)
                .common(common)
                .interaction(interaction);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

fn common_props(value: &CollapsiblePanel) -> UiCommonProps {
    let props = UiCommonProps::default()
        .visible(value.rendered_mode() != PanelMode::Collapsed)
        .width(UiDimension::Px(value.state.width.current));
    if value.rendered_mode() == PanelMode::FloatingOverlay {
        props
            .position(UiPosition::Absolute)
            .z_index(UiZIndex::Value(CollapsiblePanel::OVERLAY_Z_INDEX))
    } else {
        props
    }
}

fn interaction_state(value: &CollapsiblePanel) -> UiInteractionState {
    UiInteractionState {
        open: value.rendered_mode() != PanelMode::Collapsed,
        selected_index: mode_index(value.rendered_mode()),
        value: value.state.width.current.to_string(),
        hovered: value.state.hover_open,
        ..UiInteractionState::default()
    }
}

fn mode_index(mode: PanelMode) -> usize {
    match mode {
        PanelMode::Expanded => EXPANDED_MODE_INDEX,
        PanelMode::IconOnly => ICON_ONLY_MODE_INDEX,
        PanelMode::Collapsed => COLLAPSED_MODE_INDEX,
        PanelMode::FloatingOverlay => FLOATING_OVERLAY_MODE_INDEX,
    }
}
