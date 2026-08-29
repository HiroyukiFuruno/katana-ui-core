use crate::command_chrome::{
    EguiCommandChromeActionFrame, EguiCommandChromeFrameRecord, EguiCommandChromeSearchFrameRecord,
};
use crate::context_menu::EguiContextMenuFrameRecord;
use crate::text_command_surface::accesskit_evidence::{AccessKitEvidence, AccessKitTargetClass};

use super::interaction_locator_types::{KucInteractionActionClass, LocatorTarget};
use super::interaction_locator_utils::evidence_for;

pub(super) fn append_toolbar_targets(
    targets: &mut Vec<LocatorTarget>,
    record: &EguiCommandChromeFrameRecord,
    action_class: KucInteractionActionClass,
    evidence: &[AccessKitEvidence],
) {
    for action in &record.actions {
        append_action_target(targets, action, action_class, evidence);
        if record.dropdown.is_none() && action.secondary_trigger_bounds.is_some() {
            append_action_target(
                targets,
                action,
                KucInteractionActionClass::DropdownTrigger,
                evidence,
            );
        }
    }
    if let Some(dropdown) = record.dropdown.as_ref() {
        if let Some(action) = record
            .actions
            .iter()
            .find(|action| action.action_id == dropdown.action_id)
        {
            append_action_target(
                targets,
                action,
                KucInteractionActionClass::DropdownTrigger,
                evidence,
            );
        }
        for item in &dropdown.items {
            if let Some(accesskit) = evidence_for(
                evidence,
                &item.item_id,
                KucInteractionActionClass::DropdownItem,
                item.disabled,
            ) {
                targets.push(LocatorTarget {
                    action_identity: item.item_id.clone(),
                    action_class: KucInteractionActionClass::DropdownItem,
                    disabled: item.disabled,
                    evidence: accesskit,
                });
            }
        }
    }
}

const TEXT_SURFACE_CONTEXT_TARGET_ID: &str = "kuc.text-surface.context-target";

pub(super) fn append_text_surface_context_target(
    targets: &mut Vec<LocatorTarget>,
    evidence: &[AccessKitEvidence],
) {
    if let Some(accesskit) = evidence_for(
        evidence,
        TEXT_SURFACE_CONTEXT_TARGET_ID,
        KucInteractionActionClass::TextSurfaceContextTarget,
        false,
    ) {
        targets.push(LocatorTarget {
            action_identity: TEXT_SURFACE_CONTEXT_TARGET_ID.to_owned(),
            action_class: KucInteractionActionClass::TextSurfaceContextTarget,
            disabled: false,
            evidence: accesskit,
        });
    } else if evidence.iter().any(|value| {
        value.target_identity == TEXT_SURFACE_CONTEXT_TARGET_ID
            && value.target_class == AccessKitTargetClass::TextSurfaceContextTarget
    }) && let Some(accesskit) = evidence_for(
        evidence,
        TEXT_SURFACE_CONTEXT_TARGET_ID,
        KucInteractionActionClass::TextSurfaceContextTarget,
        true,
    ) {
        targets.push(LocatorTarget {
            action_identity: TEXT_SURFACE_CONTEXT_TARGET_ID.to_owned(),
            action_class: KucInteractionActionClass::TextSurfaceContextTarget,
            disabled: true,
            evidence: accesskit,
        });
    }
}

pub(super) fn append_search_targets(
    targets: &mut Vec<LocatorTarget>,
    record: &EguiCommandChromeSearchFrameRecord,
    evidence: &[AccessKitEvidence],
) {
    for control in &record.controls {
        if let Some(accesskit) = evidence_for(
            evidence,
            &control.control_id,
            KucInteractionActionClass::SearchControl,
            control.disabled,
        ) {
            targets.push(LocatorTarget {
                action_identity: control.control_id.clone(),
                action_class: KucInteractionActionClass::SearchControl,
                disabled: control.disabled,
                evidence: accesskit,
            });
        }
    }
}

pub(super) fn append_context_menu_targets(
    targets: &mut Vec<LocatorTarget>,
    record: &EguiContextMenuFrameRecord,
    evidence: &[AccessKitEvidence],
) {
    for item in &record.items {
        if let Some(accesskit) = evidence_for(
            evidence,
            &item.id,
            KucInteractionActionClass::ContextMenuItem,
            item.disabled,
        ) {
            targets.push(LocatorTarget {
                action_identity: item.id.clone(),
                action_class: KucInteractionActionClass::ContextMenuItem,
                disabled: item.disabled,
                evidence: accesskit,
            });
        }
    }
}

pub(super) fn append_action_target(
    targets: &mut Vec<LocatorTarget>,
    action: &EguiCommandChromeActionFrame,
    action_class: KucInteractionActionClass,
    evidence: &[AccessKitEvidence],
) {
    if let Some(accesskit) =
        evidence_for(evidence, &action.action_id, action_class, action.disabled)
    {
        targets.push(LocatorTarget {
            action_identity: action.action_id.clone(),
            action_class,
            disabled: action.disabled,
            evidence: accesskit,
        });
    }
}
