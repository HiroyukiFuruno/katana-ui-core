use super::types::{KucInteractionActionClass, LocatorTarget};
use super::{
    AccessKitEvidence, AccessKitTargetClass, EguiCommandChromeActionFrame,
    EguiCommandChromeFrameRecord, EguiCommandChromeSearchFrameRecord, EguiContextMenuFrameRecord,
};

pub(super) fn append_toolbar_targets(
    targets: &mut Vec<LocatorTarget>,
    record: &EguiCommandChromeFrameRecord,
    action_class: KucInteractionActionClass,
    evidence: &[AccessKitEvidence],
) {
    for action in &record.actions {
        if !action.primary_dropdown_trigger {
            append_action_target(targets, action, action_class, evidence);
        }
        if record.dropdown.is_none()
            && (action.primary_dropdown_trigger || action.secondary_trigger_bounds.is_some())
        {
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

pub(super) const TEXT_SURFACE_CONTEXT_TARGET_ID: &str = "kuc.text-surface.context-target";

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

pub(super) fn append_search_text_input_target(
    targets: &mut Vec<LocatorTarget>,
    record: &EguiCommandChromeSearchFrameRecord,
    evidence: &[AccessKitEvidence],
) {
    if let Some(accesskit) = evidence_for(
        evidence,
        &record.query.hit_target,
        KucInteractionActionClass::TextInput,
        record.query.frame.accessibility.root.disabled,
    ) {
        targets.push(LocatorTarget {
            action_identity: record.query.hit_target.clone(),
            action_class: KucInteractionActionClass::TextInput,
            disabled: record.query.frame.accessibility.root.disabled,
            evidence: accesskit,
        });
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

pub(super) fn append_generic_targets(
    targets: &mut Vec<LocatorTarget>,
    evidence: &[AccessKitEvidence],
) {
    for (target_class, action_class) in [
        (
            AccessKitTargetClass::StatusBarSegment,
            KucInteractionActionClass::StatusBarSegment,
        ),
        (
            AccessKitTargetClass::DiagnosticsScope,
            KucInteractionActionClass::DiagnosticsScope,
        ),
        (
            AccessKitTargetClass::DiagnosticsSeverityFilter,
            KucInteractionActionClass::DiagnosticsSeverityFilter,
        ),
        (
            AccessKitTargetClass::DiagnosticsItem,
            KucInteractionActionClass::DiagnosticsItem,
        ),
        (
            AccessKitTargetClass::DiagnosticsFix,
            KucInteractionActionClass::DiagnosticsFix,
        ),
    ] {
        for value in evidence
            .iter()
            .filter(|value| value.target_class == target_class)
        {
            targets.push(LocatorTarget {
                action_identity: value.target_identity.clone(),
                action_class,
                disabled: value.disabled,
                evidence: value.clone(),
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

pub(super) fn overlapping_enabled_bounds(
    targets: &[LocatorTarget],
) -> Vec<crate::render_model::UiRect> {
    let mut overlapping = Vec::new();
    for (index, target) in targets.iter().enumerate() {
        if target.disabled || overlapping.contains(&target.evidence.bounds) {
            continue;
        }
        if targets[index + 1..]
            .iter()
            .any(|other| !other.disabled && other.evidence.bounds == target.evidence.bounds)
        {
            overlapping.push(target.evidence.bounds);
        }
    }
    overlapping
}

pub(super) fn evidence_for(
    evidence: &[AccessKitEvidence],
    action_identity: &str,
    action_class: KucInteractionActionClass,
    disabled: bool,
) -> Option<AccessKitEvidence> {
    evidence
        .iter()
        .find(|value| {
            value.target_identity == action_identity
                && value.target_class == accesskit_class(action_class)
                && value.disabled == disabled
        })
        .cloned()
}

pub(super) fn accesskit_class(class: KucInteractionActionClass) -> AccessKitTargetClass {
    match class {
        KucInteractionActionClass::TextSurfaceContextTarget => {
            AccessKitTargetClass::TextSurfaceContextTarget
        }
        KucInteractionActionClass::TextInput => AccessKitTargetClass::TextInput,
        KucInteractionActionClass::Toolbar => AccessKitTargetClass::Toolbar,
        KucInteractionActionClass::FloatingToolbar => AccessKitTargetClass::FloatingToolbar,
        KucInteractionActionClass::DropdownTrigger => AccessKitTargetClass::DropdownTrigger,
        KucInteractionActionClass::DropdownItem => AccessKitTargetClass::DropdownItem,
        KucInteractionActionClass::SearchControl => AccessKitTargetClass::SearchControl,
        KucInteractionActionClass::ContextMenuItem => AccessKitTargetClass::ContextMenuItem,
        KucInteractionActionClass::StatusBarSegment => AccessKitTargetClass::StatusBarSegment,
        KucInteractionActionClass::DiagnosticsScope => AccessKitTargetClass::DiagnosticsScope,
        KucInteractionActionClass::DiagnosticsSeverityFilter => {
            AccessKitTargetClass::DiagnosticsSeverityFilter
        }
        KucInteractionActionClass::DiagnosticsItem => AccessKitTargetClass::DiagnosticsItem,
        KucInteractionActionClass::DiagnosticsFix => AccessKitTargetClass::DiagnosticsFix,
    }
}

pub(super) fn center(bounds: crate::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}
