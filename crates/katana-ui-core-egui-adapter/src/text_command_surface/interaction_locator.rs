//! Generic current-frame interaction requests for the retained command surface.

use super::super::types::EguiTextCommandSurfaceOutput;
use super::root_event::KucRootEventBatchContext;

#[path = "interaction_locator_appenders.rs"]
mod interaction_locator_appenders;
#[path = "interaction_locator_build.rs"]
mod interaction_locator_build;
#[path = "interaction_locator_types.rs"]
mod interaction_locator_types;
#[path = "interaction_locator_utils.rs"]
mod interaction_locator_utils;

#[cfg(test)]
#[path = "interaction_locator_tests.rs"]
mod interaction_locator_tests;

use interaction_locator_build::build_from_output;
#[cfg(test)]
use interaction_locator_utils::accesskit_class;
use interaction_locator_utils::center;

use interaction_locator_types::KucInteractionRequestError::*;
pub use interaction_locator_types::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueInteractionRequest,
};

impl KucInteractionLocator {
    pub(super) fn from_output(
        root_identity: &str,
        context: &KucRootEventBatchContext,
        output: &EguiTextCommandSurfaceOutput,
        bound_evidence: &super::super::accesskit_evidence::BoundAccessKitEvidence,
    ) -> Self {
        build_from_output(root_identity, context, output, bound_evidence)
    }

    pub fn request(
        &self,
        selector: KucInteractionSelector,
    ) -> Result<KucOpaqueInteractionRequest, KucInteractionLocatorError> {
        let key = (
            selector.action_identity().to_owned(),
            selector.action_class(),
        );
        if !self.requested.borrow_mut().insert(key.clone()) {
            return Err(KucInteractionLocatorError::Duplicate);
        }
        if self.hidden.contains(&key) {
            return Err(KucInteractionLocatorError::Hidden);
        }
        let mut matching = self.targets.iter().filter(|target| {
            target.action_identity == selector.action_identity()
                && target.action_class == selector.action_class()
        });
        let target = matching.next().ok_or(KucInteractionLocatorError::Missing)?;
        if target.disabled {
            return Err(KucInteractionLocatorError::Disabled);
        }
        if matching.next().is_some() || self.ambiguous_bounds.contains(&target.evidence.bounds) {
            return Err(KucInteractionLocatorError::Ambiguous);
        }
        let point = center(target.evidence.bounds);
        let modifiers = egui::Modifiers::default();
        Ok(KucOpaqueInteractionRequest {
            root_identity: self.root_identity.clone(),
            state_revision: self.state_revision,
            correlation_fingerprint: self.correlation_fingerprint.clone(),
            events: vec![
                egui::Event::PointerMoved(point),
                egui::Event::PointerButton {
                    pos: point,
                    button: if selector.action_class()
                        == KucInteractionActionClass::TextSurfaceContextTarget
                    {
                        egui::PointerButton::Secondary
                    } else {
                        egui::PointerButton::Primary
                    },
                    pressed: true,
                    modifiers,
                },
                egui::Event::PointerButton {
                    pos: point,
                    button: if selector.action_class()
                        == KucInteractionActionClass::TextSurfaceContextTarget
                    {
                        egui::PointerButton::Secondary
                    } else {
                        egui::PointerButton::Primary
                    },
                    pressed: false,
                    modifiers,
                },
            ],
            queued: false,
        })
    }

    /// Requests the generic physical context-menu opener for this frame's text surface.
    ///
    /// The target identity and geometry are resolved and retained inside KUC. The caller only
    /// receives an opaque request that can be applied to the next root input frame.
    pub fn request_context_open(
        &self,
    ) -> Result<KucOpaqueInteractionRequest, KucInteractionLocatorError> {
        self.request(KucInteractionSelector::new(
            TEXT_SURFACE_CONTEXT_TARGET_ID,
            KucInteractionActionClass::TextSurfaceContextTarget,
        ))
    }

    pub fn queue_request(
        &self,
        mut request: KucOpaqueInteractionRequest,
        input: &mut egui::RawInput,
    ) -> Result<(), KucInteractionRequestError> {
        if request.root_identity != self.root_identity {
            return Err(RootMismatch);
        }
        if request.state_revision != self.state_revision
            || request.correlation_fingerprint != self.correlation_fingerprint
        {
            return Err(Stale);
        }
        request.apply_to_raw_input_once(input)
    }
}

impl KucOpaqueInteractionRequest {
    /// Applies the opaque pointer sequence exactly once.
    pub fn apply_to_raw_input_once(
        &mut self,
        input: &mut egui::RawInput,
    ) -> Result<(), KucInteractionRequestError> {
        if self.queued {
            return Err(AlreadyQueued);
        }
        input.events.append(&mut self.events);
        self.queued = true;
        Ok(())
    }
}

impl std::fmt::Debug for KucInteractionLocator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucInteractionLocator(..)")
    }
}

const TEXT_SURFACE_CONTEXT_TARGET_ID: &str = "kuc.text-surface.context-target";
