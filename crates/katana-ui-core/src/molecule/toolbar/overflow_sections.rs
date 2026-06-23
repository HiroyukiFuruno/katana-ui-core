use super::action_model::ToolbarAction;
use super::group_model::ToolbarGroup;
use super::identifiers::{ToolbarActionId, ToolbarGroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarOverflowSection {
    group_id: Option<ToolbarGroupId>,
    label: Option<String>,
    action_ids: Vec<ToolbarActionId>,
}

impl ToolbarOverflowSection {
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn new(
        group_id: Option<ToolbarGroupId>,
        label: Option<String>,
        action_id: ToolbarActionId,
    ) -> Self {
        Self {
            group_id,
            label,
            action_ids: vec![action_id],
        }
    }
}

pub(super) fn overflow_menu_sections(
    actions: &[ToolbarAction],
    groups: &[ToolbarGroup],
) -> Vec<ToolbarOverflowSection> {
    let mut sections = Vec::new();
    for action in actions {
        let group_id = action.group_id_model().cloned();
        if sections
            .last()
            .is_some_and(|section: &ToolbarOverflowSection| section.group_id == group_id)
        {
            push_to_last_section(&mut sections, action.id().clone());
            continue;
        }
        sections.push(ToolbarOverflowSection::new(
            group_id.clone(),
            group_label(group_id.as_ref(), groups),
            action.id().clone(),
        ));
    }
    sections
}

fn push_to_last_section(sections: &mut [ToolbarOverflowSection], action_id: ToolbarActionId) {
    if let Some(section) = sections.last_mut() {
        section.action_ids.push(action_id);
    }
}

fn group_label(group_id: Option<&ToolbarGroupId>, groups: &[ToolbarGroup]) -> Option<String> {
    group_id
        .and_then(|id| groups.iter().find(|group| group.id() == id))
        .and_then(ToolbarGroup::label_model)
        .cloned()
}
