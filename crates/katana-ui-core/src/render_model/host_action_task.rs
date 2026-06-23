use super::{
    UI_TASK_SET_STATE_ACTION_ID, UI_TASK_TOGGLE_ACTION_ID, UiContextMenuItem, UiHostActionPayload,
    UiHostActionPlan, UiHostActionSpec, UiNode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTaskMarker {
    Empty,
    Done,
    Progress,
    Blocked,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlTarget {
    pub node_id: String,
    pub row_index: usize,
    pub state_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlAction {
    pub node_id: String,
    pub row_index: usize,
    pub current_marker: UiTaskMarker,
    pub state_id: String,
    pub menu_items: Vec<UiTaskControlMenuItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlMenuItem {
    pub item_id: String,
    pub marker: UiTaskMarker,
    pub label: String,
    pub checked: bool,
    pub host_action: Option<UiHostActionSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlStateAction {
    pub node_id: String,
    pub row_index: usize,
    pub state_id: String,
    pub marker: UiTaskMarker,
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
