use super::{DiagnosticsList, DiagnosticsListPlanner};
use crate::atom::{Chip, ChipSize, ChipTone, ChipVariant, Text};
use crate::interaction::VirtualRange;
use crate::molecule::virtualization::MoleculeVirtualization;
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind};

impl From<DiagnosticsList> for UiNode {
    fn from(value: DiagnosticsList) -> Self {
        let mut node = UiNode::from_state(
            UiNodeKind::DiagnosticsList,
            value.label.clone(),
            value.state_id.clone(),
        );
        if value.state.loading {
            return render_optional_slot(node, value.loading_slot);
        }
        let visible = DiagnosticsListPlanner::visible_items(&value.items, &value.options);
        let range = value.virtual_range_model();
        node = node.interaction(interaction_state(&value, &visible, range.as_ref()));
        if visible.is_empty() {
            return render_optional_slot(node, value.empty_slot);
        }
        for severity in super::DiagnosticSeverity::all() {
            node = node.child(severity_chip(severity, &value));
        }
        for item in virtual_visible_items(visible, range.as_ref()) {
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

fn severity_chip(severity: super::DiagnosticSeverity, value: &DiagnosticsList) -> Chip {
    Chip::new(format!("{severity:?}"))
        .tone(severity_tone(severity))
        .variant(ChipVariant::Soft)
        .size(ChipSize::Compact)
        .interactive(true)
        .selected(value.options.severity_filter.contains(&severity))
}

fn severity_tone(severity: super::DiagnosticSeverity) -> ChipTone {
    match severity {
        super::DiagnosticSeverity::Error => ChipTone::Danger,
        super::DiagnosticSeverity::Warning => ChipTone::Warning,
        super::DiagnosticSeverity::Info => ChipTone::Accent,
        super::DiagnosticSeverity::Hint => ChipTone::Muted,
    }
}

fn render_optional_slot(node: UiNode, slot: Option<UiNode>) -> UiNode {
    if let Some(slot) = slot {
        return node.child(slot);
    }
    node
}

fn interaction_state(
    value: &DiagnosticsList,
    visible: &[&super::DiagnosticItem],
    range: Option<&VirtualRange>,
) -> UiInteractionState {
    let selected_index = value
        .state
        .selected_id
        .as_ref()
        .and_then(|id| visible.iter().position(|it| &it.id == id));
    let base = UiInteractionState {
        has_selection: value.state.selected_id.is_some() && !visible.is_empty(),
        selected_index: selected_index.unwrap_or_default(),
        item_count: visible.len(),
        open: value.state.bulk_preview_open,
        ..UiInteractionState::default()
    };
    MoleculeVirtualization::interaction(base, range)
}

fn virtual_visible_items<'a>(
    visible: Vec<&'a super::DiagnosticItem>,
    range: Option<&VirtualRange>,
) -> Vec<&'a super::DiagnosticItem> {
    MoleculeVirtualization::slice_by_range(visible, range)
}
