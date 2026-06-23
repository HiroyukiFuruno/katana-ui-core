use super::action_model::ToolbarAction;
use super::group_model::ToolbarGroup;
use super::identifiers::{ToolbarActionId, ToolbarGroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarGroupDivider {
    before_action_id: ToolbarActionId,
    from_group: Option<ToolbarGroupId>,
    to_group: Option<ToolbarGroupId>,
}

impl ToolbarGroupDivider {
    #[must_use]
    pub fn before_action_id(&self) -> &ToolbarActionId {
        &self.before_action_id
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarGroupLayout;

impl ToolbarGroupLayout {
    #[must_use]
    pub fn visible_group_dividers(
        actions: &[ToolbarAction],
        groups: &[ToolbarGroup],
    ) -> Vec<ToolbarGroupDivider> {
        actions
            .windows(2)
            .filter_map(|window| divider_for_boundary(&window[0], &window[1], groups))
            .collect()
    }
}

fn divider_for_boundary(
    previous: &ToolbarAction,
    next: &ToolbarAction,
    groups: &[ToolbarGroup],
) -> Option<ToolbarGroupDivider> {
    let from_group = previous.group_id_model().cloned();
    let to_group = next.group_id_model().cloned();
    if from_group == to_group || !divider_enabled(to_group.as_ref(), groups) {
        return None;
    }
    Some(ToolbarGroupDivider {
        before_action_id: next.id().clone(),
        from_group,
        to_group,
    })
}

fn divider_enabled(group_id: Option<&ToolbarGroupId>, groups: &[ToolbarGroup]) -> bool {
    group_id
        .and_then(|id| groups.iter().find(|group| group.id() == id))
        .is_none_or(ToolbarGroup::divider_model)
}
