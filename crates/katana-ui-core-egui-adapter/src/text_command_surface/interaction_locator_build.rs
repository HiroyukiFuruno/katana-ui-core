use super::super::super::accesskit_evidence::BoundAccessKitEvidence;
use super::super::super::types::EguiTextCommandSurfaceOutput;
use super::KucRootEventBatchContext;
use super::interaction_locator_appenders::{
    append_context_menu_targets, append_search_targets, append_text_surface_context_target,
    append_toolbar_targets,
};
use super::interaction_locator_types::KucInteractionActionClass;
use super::interaction_locator_types::KucInteractionLocator;
use super::interaction_locator_utils::overlapping_enabled_bounds;

pub(super) fn build_from_output(
    root_identity: &str,
    context: &KucRootEventBatchContext,
    output: &EguiTextCommandSurfaceOutput,
    bound_evidence: &BoundAccessKitEvidence,
) -> KucInteractionLocator {
    let mut targets = Vec::new();
    let mut hidden = std::collections::HashSet::new();
    let evidence = applicable_evidence(
        bound_evidence.matches(context, root_identity),
        bound_evidence.entries(),
    );
    if let Some(toolbar) = output.toolbar.as_ref() {
        append_toolbar_targets(
            &mut targets,
            &toolbar.record,
            KucInteractionActionClass::Toolbar,
            evidence,
        );
        for item_id in &toolbar.record.hidden_item_ids {
            hidden.insert((item_id.clone(), KucInteractionActionClass::DropdownItem));
        }
    }
    if let Some(floating) = output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
    {
        append_toolbar_targets(
            &mut targets,
            &floating.toolbar,
            KucInteractionActionClass::FloatingToolbar,
            evidence,
        );
    }
    if let Some(search) = output.search.as_ref() {
        append_search_targets(&mut targets, &search.record, evidence);
    }
    if output.context_menu.is_some()
        && output
            .context_menu
            .as_ref()
            .is_none_or(|value| value.record.is_none())
    {
        append_text_surface_context_target(&mut targets, evidence);
    }
    if let Some(menu) = output
        .context_menu
        .as_ref()
        .and_then(|value| value.record.as_ref())
    {
        append_context_menu_targets(&mut targets, menu, evidence);
    }
    /* WHY: Hidden metadata can outlive the presentation that produced it. A
    target recorded by the current bound AccessKit frame is authoritative. */
    hidden.retain(|key| {
        !targets
            .iter()
            .any(|target| target.action_identity == key.0 && target.action_class == key.1)
    });
    KucInteractionLocator {
        root_identity: root_identity.to_owned(),
        state_revision: context.state_revision(),
        correlation_fingerprint: context.correlation_fingerprint().to_owned(),
        ambiguous_bounds: overlapping_enabled_bounds(&targets),
        targets,
        hidden,
        requested: std::cell::RefCell::new(Default::default()),
    }
}

fn applicable_evidence(
    matches: bool,
    entries: &[super::super::super::accesskit_evidence::AccessKitEvidence],
) -> &[super::super::super::accesskit_evidence::AccessKitEvidence] {
    if matches { entries } else { &[] }
}

#[cfg(test)]
mod evidence_tests {
    use super::*;

    #[test]
    fn mismatched_evidence_is_discarded() {
        assert!(applicable_evidence(false, &[]).is_empty());
        assert!(applicable_evidence(true, &[]).is_empty());
    }
}
