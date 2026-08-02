use super::{
    UI_TASK_SET_STATE_ACTION_ID, UI_TASK_TOGGLE_ACTION_ID, UiContextMenuItem, UiHostActionPayload,
    UiHostActionPlan, UiHostActionSpec, UiNode, UiTaskControlAction, UiTaskControlMenuItem,
    UiTaskControlStateAction, UiTaskControlTarget, UiTaskMarker,
};

impl UiTaskMarker {
    pub const ALL: [Self; 4] = [Self::Empty, Self::Done, Self::Progress, Self::Blocked];

    #[must_use]
    pub fn from_marker(value: &str) -> Option<Self> {
        match value {
            "[ ]" => Some(Self::Empty),
            "[x]" | "[X]" => Some(Self::Done),
            "[/]" => Some(Self::Progress),
            "[-]" => Some(Self::Blocked),
            _ => None,
        }
    }

    #[must_use]
    pub const fn marker(self) -> &'static str {
        match self {
            Self::Empty => "[ ]",
            Self::Done => "[x]",
            Self::Progress => "[/]",
            Self::Blocked => "[-]",
        }
    }

    #[must_use]
    pub const fn context_menu_item_id(self) -> &'static str {
        match self {
            Self::Empty => "ui.task.state.empty",
            Self::Done => "ui.task.state.done",
            Self::Progress => "ui.task.state.progress",
            Self::Blocked => "ui.task.state.blocked",
        }
    }

    #[must_use]
    pub fn from_context_menu_item_id(value: &str) -> Option<Self> {
        match value {
            "ui.task.state.empty" => Some(Self::Empty),
            "ui.task.state.done" => Some(Self::Done),
            "ui.task.state.progress" => Some(Self::Progress),
            "ui.task.state.blocked" => Some(Self::Blocked),
            _ => None,
        }
    }
}

impl UiHostActionPlan {
    #[must_use]
    pub fn task_control_target(&self) -> Option<UiTaskControlTarget> {
        UiTaskControlTarget::from_plan(self)
    }

    #[must_use]
    pub fn task_control_action(&self, current_marker: &str) -> Option<UiTaskControlAction> {
        UiTaskControlAction::from_plan(self, current_marker)
    }

    #[must_use]
    pub fn task_control_action_from_root(&self, root: &UiNode) -> Option<UiTaskControlAction> {
        UiTaskControlAction::from_plan_root(self, root)
    }

    #[must_use]
    pub fn task_control_state_action(&self) -> Option<UiTaskControlStateAction> {
        UiTaskControlStateAction::from_plan(self)
    }
}

impl UiTaskControlTarget {
    fn from_plan(plan: &UiHostActionPlan) -> Option<Self> {
        if plan.action_id != UI_TASK_TOGGLE_ACTION_ID {
            return None;
        }
        let UiHostActionPayload::TaskControl(payload) = &plan.typed_payload else {
            return None;
        };
        Some(Self {
            node_id: payload.node_id.clone(),
            row_index: payload.row_index,
            state_id: payload.state_id.clone(),
        })
    }
}

impl UiTaskControlAction {
    fn from_plan(plan: &UiHostActionPlan, current_marker: &str) -> Option<Self> {
        let target = UiTaskControlTarget::from_plan(plan)?;
        Self::from_target(target, current_marker, Vec::new())
    }

    fn from_plan_root(plan: &UiHostActionPlan, root: &UiNode) -> Option<Self> {
        let target = UiTaskControlTarget::from_plan(plan)?;
        let target_node = find_node(root, plan.target.as_str())?;
        let current_marker = task_marker_value(target_node)?;
        let menu_items = task_menu_items_from_node(target_node);
        Self::from_target(target, current_marker, menu_items)
    }

    fn from_target(
        target: UiTaskControlTarget,
        current_marker: &str,
        menu_items: Vec<UiTaskControlMenuItem>,
    ) -> Option<Self> {
        Some(Self {
            node_id: target.node_id,
            row_index: target.row_index,
            current_marker: UiTaskMarker::from_marker(current_marker)?,
            state_id: target.state_id,
            menu_items,
        })
    }
}

impl UiTaskControlStateAction {
    fn from_plan(plan: &UiHostActionPlan) -> Option<Self> {
        if plan.action_id != UI_TASK_SET_STATE_ACTION_ID {
            return None;
        }
        let UiHostActionPayload::TaskControlState(payload) = &plan.typed_payload else {
            return None;
        };
        Some(Self {
            node_id: payload.node_id.clone(),
            row_index: payload.row_index,
            state_id: payload.state_id.clone(),
            marker: UiTaskMarker::from_marker(&payload.marker)?,
        })
    }
}

fn task_menu_items(items: &[UiContextMenuItem]) -> Vec<UiTaskControlMenuItem> {
    items
        .iter()
        .filter_map(|item| {
            let (marker, host_action) = task_marker_from_host_action(item).map_or_else(
                || UiTaskMarker::from_context_menu_item_id(&item.id).map(|marker| (marker, None)),
                |(marker, action)| Some((marker, Some(action))),
            )?;
            Some(UiTaskControlMenuItem {
                item_id: item.id.clone(),
                marker,
                label: item.label.clone(),
                checked: item.checked,
                host_action,
            })
        })
        .collect()
}

fn task_marker_from_host_action(
    item: &UiContextMenuItem,
) -> Option<(UiTaskMarker, UiHostActionSpec)> {
    let action = item.host_action.as_ref()?;
    if action.action_id != UI_TASK_SET_STATE_ACTION_ID {
        return None;
    }
    let UiHostActionPayload::TaskControlState(payload) = &action.typed_payload else {
        return None;
    };
    Some((UiTaskMarker::from_marker(&payload.marker)?, action.clone()))
}

fn task_marker_value(node: &UiNode) -> Option<&str> {
    let value = node.props().interaction.value.as_str();
    if UiTaskMarker::from_marker(value).is_some() {
        return Some(value);
    }
    node.children().iter().find_map(task_marker_value)
}

fn task_menu_items_from_node(node: &UiNode) -> Vec<UiTaskControlMenuItem> {
    let items = task_menu_items(&node.props().context_menu.items);
    if !items.is_empty() {
        return items;
    }
    node.children()
        .iter()
        .find_map(|child| {
            let items = task_menu_items_from_node(child);
            (!items.is_empty()).then_some(items)
        })
        .unwrap_or_default()
}

fn find_node<'a>(node: &'a UiNode, node_id: &str) -> Option<&'a UiNode> {
    if node.id().as_str() == node_id {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_node(child, node_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_model::{
        UiContextMenuItemKind, UiContextMenuProps, UiHostActionPayload, UiInteractionState,
        UiNodeId, UiNodeKind,
    };

    #[test]
    fn marker_parsing_covers_blocked_and_invalid_identifiers() {
        assert_eq!(
            Some(UiTaskMarker::Blocked),
            UiTaskMarker::from_marker("[-]")
        );
        assert_eq!(
            Some(UiTaskMarker::Blocked),
            UiTaskMarker::from_context_menu_item_id("ui.task.state.blocked")
        );
        assert_eq!(
            None,
            UiTaskMarker::from_context_menu_item_id("ui.task.state.unknown")
        );
    }

    #[test]
    fn task_accessors_reject_wrong_actions_payloads_and_markers() {
        let unrelated = UiHostActionPlan::new(
            UiNodeId::new("target"),
            UiHostActionSpec::command("other", "Other"),
        );
        assert!(unrelated.task_control_target().is_none());
        assert!(unrelated.task_control_state_action().is_none());

        let wrong_toggle_payload = UiHostActionPlan::new(
            UiNodeId::new("target"),
            UiHostActionSpec::command(UI_TASK_TOGGLE_ACTION_ID, "Toggle"),
        );
        assert!(wrong_toggle_payload.task_control_target().is_none());

        let wrong_state_payload = UiHostActionPlan::new(
            UiNodeId::new("target"),
            UiHostActionSpec::command(UI_TASK_SET_STATE_ACTION_ID, "State"),
        );
        assert!(wrong_state_payload.task_control_state_action().is_none());

        let toggle = UiHostActionPlan::new(
            UiNodeId::new("target"),
            UiHostActionSpec::task_control("Toggle", "list", 1),
        );
        assert!(toggle.task_control_action("invalid").is_none());

        let invalid_state = UiHostActionPlan::new(
            UiNodeId::new("target"),
            UiHostActionSpec::task_control_state("State", "list", 1, "invalid"),
        );
        assert!(invalid_state.task_control_state_action().is_none());
    }

    #[test]
    fn task_root_lookup_rejects_missing_or_invalid_markers_and_finds_nested_target() {
        let plan = UiHostActionPlan::new(
            UiNodeId::new("target"),
            UiHostActionSpec::task_control("Toggle", "list", 1),
        );
        let missing = UiNode::new(UiNodeKind::Column, "root");
        assert!(plan.task_control_action_from_root(&missing).is_none());

        let invalid = UiNode::new(UiNodeKind::Column, "root").child(
            UiNode::new(UiNodeKind::Row, "target")
                .stable_node_id("target")
                .interaction(UiInteractionState {
                    value: "invalid".to_string(),
                    ..UiInteractionState::default()
                }),
        );
        assert!(plan.task_control_action_from_root(&invalid).is_none());

        let nested = UiNode::new(UiNodeKind::Column, "root").child(
            UiNode::new(UiNodeKind::Row, "wrapper").child(
                UiNode::new(UiNodeKind::Checkbox, "target")
                    .stable_node_id("target")
                    .interaction(UiInteractionState {
                        value: "[x]".to_string(),
                        ..UiInteractionState::default()
                    }),
            ),
        );
        assert_eq!(
            Some(UiTaskMarker::Done),
            plan.task_control_action_from_root(&nested)
                .map(|action| action.current_marker)
        );
    }

    #[test]
    fn task_menu_items_support_typed_legacy_and_recursive_fallbacks() {
        let valid_action = UiHostActionSpec::task_control_state("Done", "list", 1, "[x]");
        let wrong_action = UiHostActionSpec::command("other", "Other");
        let wrong_payload = UiHostActionSpec::command(UI_TASK_SET_STATE_ACTION_ID, "State")
            .typed_payload(UiHostActionPayload::None);
        let items = vec![
            UiContextMenuItem::new("typed", "Done", UiContextMenuItemKind::Radio)
                .host_action(valid_action),
            UiContextMenuItem::new("ui.task.state.empty", "Empty", UiContextMenuItemKind::Radio),
            UiContextMenuItem::new(
                "ui.task.state.blocked",
                "Blocked",
                UiContextMenuItemKind::Radio,
            )
            .host_action(wrong_action),
            UiContextMenuItem::new(
                "ui.task.state.progress",
                "Progress",
                UiContextMenuItemKind::Radio,
            )
            .host_action(wrong_payload),
            UiContextMenuItem::new("ignored", "Ignored", UiContextMenuItemKind::Action)
                .host_action(UiHostActionSpec::task_control_state(
                    "Invalid", "list", 1, "invalid",
                )),
        ];

        let converted = task_menu_items(&items);
        assert_eq!(4, converted.len());
        assert!(converted[0].host_action.is_some());
        assert!(converted[1].host_action.is_none());
        assert_eq!(UiTaskMarker::Blocked, converted[2].marker);
        assert_eq!(UiTaskMarker::Progress, converted[3].marker);

        let child = UiNode::new(UiNodeKind::ContextMenu, "menu").context_menu(UiContextMenuProps {
            items,
            ..UiContextMenuProps::default()
        });
        let root = UiNode::new(UiNodeKind::Column, "root").child(child);
        assert_eq!(4, task_menu_items_from_node(&root).len());
    }
}
