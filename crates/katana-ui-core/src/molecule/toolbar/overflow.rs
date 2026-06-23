use super::action_model::ToolbarAction;
use super::group_model::ToolbarGroup;
use super::identifiers::{ToolbarActionId, ToolbarPriority};
use super::options::ToolbarStrategy;
use super::overflow_sections::{ToolbarOverflowSection, overflow_menu_sections};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasuredToolbarAction {
    action_id: ToolbarActionId,
    width: u32,
    priority: ToolbarPriority,
}

impl MeasuredToolbarAction {
    #[must_use]
    pub fn new(
        action_id: impl Into<ToolbarActionId>,
        width: u32,
        priority: ToolbarPriority,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            width,
            priority,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarOverflowInput {
    available_width: u32,
    overflow_trigger_width: u32,
    strategy: ToolbarStrategy,
    actions: Vec<MeasuredToolbarAction>,
}

impl ToolbarOverflowInput {
    #[must_use]
    pub fn new(
        available_width: u32,
        overflow_trigger_width: u32,
        strategy: ToolbarStrategy,
        actions: Vec<MeasuredToolbarAction>,
    ) -> Self {
        Self {
            available_width,
            overflow_trigger_width,
            strategy,
            actions,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarOverflowPlan {
    visible_action_ids: Vec<ToolbarActionId>,
    hidden_action_ids: Vec<ToolbarActionId>,
    overflow_trigger_visible: bool,
}

impl ToolbarOverflowPlan {
    #[must_use]
    pub fn visible_action_ids(&self) -> Vec<&str> {
        self.visible_action_ids
            .iter()
            .map(ToolbarActionId::as_str)
            .collect()
    }

    #[must_use]
    pub fn hidden_action_ids(&self) -> Vec<&str> {
        self.hidden_action_ids
            .iter()
            .map(ToolbarActionId::as_str)
            .collect()
    }

    #[must_use]
    pub const fn overflow_trigger_visible(&self) -> bool {
        self.overflow_trigger_visible
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarOverflowPlanner;

impl ToolbarOverflowPlanner {
    #[must_use]
    pub fn plan(input: &ToolbarOverflowInput) -> ToolbarOverflowPlan {
        let mut visible_indices: Vec<usize> = (0..input.actions.len()).collect();
        let mut hidden_action_ids = Vec::new();
        let total_width = visible_width(&visible_indices, &input.actions);
        if total_width <= input.available_width {
            return plan_from_indices(input, visible_indices, hidden_action_ids, false);
        }
        let reserved_width = match input.strategy {
            ToolbarStrategy::Menu => input.overflow_trigger_width,
            ToolbarStrategy::Hide | ToolbarStrategy::Custom => 0,
        };
        while visible_width(&visible_indices, &input.actions) + reserved_width
            > input.available_width
        {
            let Some(index_to_hide) = next_hidden_index(&visible_indices, &input.actions) else {
                break;
            };
            visible_indices.retain(|index| *index != index_to_hide);
            hidden_action_ids.push(input.actions[index_to_hide].action_id.clone());
        }
        let trigger_visible =
            input.strategy == ToolbarStrategy::Menu && !hidden_action_ids.is_empty();
        plan_from_indices(input, visible_indices, hidden_action_ids, trigger_visible)
    }

    #[must_use]
    pub fn overflow_menu_sections(
        actions: &[ToolbarAction],
        groups: &[ToolbarGroup],
    ) -> Vec<ToolbarOverflowSection> {
        overflow_menu_sections(actions, groups)
    }
}

fn visible_width(indices: &[usize], actions: &[MeasuredToolbarAction]) -> u32 {
    indices.iter().map(|index| actions[*index].width).sum()
}

fn next_hidden_index(indices: &[usize], actions: &[MeasuredToolbarAction]) -> Option<usize> {
    indices.iter().copied().min_by(|left, right| {
        let left_action = &actions[*left];
        let right_action = &actions[*right];
        left_action
            .priority
            .cmp(&right_action.priority)
            .then_with(|| right.cmp(left))
    })
}

fn plan_from_indices(
    input: &ToolbarOverflowInput,
    visible_indices: Vec<usize>,
    hidden_action_ids: Vec<ToolbarActionId>,
    overflow_trigger_visible: bool,
) -> ToolbarOverflowPlan {
    ToolbarOverflowPlan {
        visible_action_ids: visible_indices
            .iter()
            .map(|index| input.actions[*index].action_id.clone())
            .collect(),
        hidden_action_ids,
        overflow_trigger_visible,
    }
}
