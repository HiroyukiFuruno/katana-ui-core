use super::{UiIconProps, UiNode, UiNodeKind, UiSlotPlacement, UiSlotSpec, UiTree};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSvgIconRenderPlan {
    pub node_kind: UiNodeKind,
    pub label: String,
    pub slot_label: String,
    pub placement: Option<UiSlotPlacement>,
    pub svg_source: String,
    pub view_box: String,
    pub path_summary: String,
    pub paint_policy: super::UiSvgPaintPolicy,
    pub role: String,
    pub color_token: String,
    pub theme_token: String,
    pub callback: String,
}

impl UiSvgIconRenderPlan {
    #[must_use]
    pub fn collect_from_tree(tree: &UiTree) -> Vec<Self> {
        let mut plans = Vec::new();
        Self::collect_from_node(tree.root(), &mut plans);
        plans
    }

    fn collect_from_node(node: &UiNode, plans: &mut Vec<Self>) {
        let props = node.props();
        Self::push_icon(node, "", None, &props.icon, "", plans);
        if let Some(slot) = &props.text_entry.leading_slot {
            Self::push_slot_icon(node, slot, plans);
        }
        if props.text_entry.trailing_icon_buttons.is_empty()
            && let Some(slot) = &props.text_entry.trailing_slot
        {
            Self::push_slot_icon(node, slot, plans);
        }
        for slot in &props.text_entry.trailing_icon_buttons {
            Self::push_slot_icon(node, slot, plans);
        }
        for child in node.children() {
            Self::collect_from_node(child, plans);
        }
    }

    fn push_slot_icon(node: &UiNode, slot: &UiSlotSpec, plans: &mut Vec<Self>) {
        let Some(icon) = &slot.icon else {
            return;
        };
        let callback = slot
            .action
            .as_ref()
            .map_or("", |action| action.callback.as_str());
        Self::push_icon(
            node,
            slot.label.as_str(),
            Some(slot.placement),
            icon,
            callback,
            plans,
        );
    }

    fn push_icon(
        node: &UiNode,
        slot_label: &str,
        placement: Option<UiSlotPlacement>,
        icon: &UiIconProps,
        callback: &str,
        plans: &mut Vec<Self>,
    ) {
        if icon.svg_source.trim().is_empty() {
            return;
        }
        plans.push(Self {
            node_kind: node.kind(),
            label: node.props().label.clone(),
            slot_label: slot_label.to_string(),
            placement,
            svg_source: icon.svg_source.clone(),
            view_box: icon.view_box.clone(),
            path_summary: icon.path_summary.clone(),
            paint_policy: icon.paint_policy,
            role: icon.role.clone(),
            color_token: icon.color_token.clone(),
            theme_token: icon.theme_token.clone(),
            callback: callback.to_string(),
        });
    }
}
