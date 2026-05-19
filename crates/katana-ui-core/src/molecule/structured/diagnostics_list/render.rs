use super::{DiagnosticsList, DiagnosticsListPlanner};
use crate::atom::Text;
use crate::render_model::{UiNode, UiNodeKind};

impl From<DiagnosticsList> for UiNode {
    fn from(value: DiagnosticsList) -> Self {
        let mut node = UiNode::from_state(UiNodeKind::DiagnosticsList, value.label, value.state_id);
        if value.state.loading {
            return render_optional_slot(node, value.loading_slot);
        }
        let visible = DiagnosticsListPlanner::visible_items(&value.items, &value.options);
        if visible.is_empty() {
            return render_optional_slot(node, value.empty_slot);
        }
        for item in visible {
            node = node.child(Text::new(format!("{:?}: {}", item.severity, item.message)));
            if value.state.expanded_ids.contains(&item.id)
                && let Some(preview) = item.fix_preview.clone()
            {
                node = node.child(preview.diff);
            }
        }
        if value.state.bulk_preview_open
            && let Some(preview) = value.bulk_preview
        {
            node = node.child(preview);
        }
        node
    }
}

fn render_optional_slot(node: UiNode, slot: Option<UiNode>) -> UiNode {
    if let Some(slot) = slot {
        return node.child(slot);
    }
    node
}
