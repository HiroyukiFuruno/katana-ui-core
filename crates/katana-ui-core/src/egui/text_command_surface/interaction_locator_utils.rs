use super::interaction_locator_types::{KucInteractionActionClass, LocatorTarget};
use crate::egui::text_command_surface::accesskit_evidence::{AccessKitEvidence, AccessKitTargetClass};

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
        KucInteractionActionClass::Toolbar => AccessKitTargetClass::Toolbar,
        KucInteractionActionClass::FloatingToolbar => AccessKitTargetClass::FloatingToolbar,
        KucInteractionActionClass::DropdownTrigger => AccessKitTargetClass::DropdownTrigger,
        KucInteractionActionClass::DropdownItem => AccessKitTargetClass::DropdownItem,
        KucInteractionActionClass::SearchControl => AccessKitTargetClass::SearchControl,
        KucInteractionActionClass::ContextMenuItem => AccessKitTargetClass::ContextMenuItem,
    }
}

pub(super) fn center(bounds: crate::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}
