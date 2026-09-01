use super::targets::center;
use super::types::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionSelector, KucOpaqueClickContinuation, KucOpaqueClickContinuationError,
    KucOpaqueInteractionRequest, KucSearchTraceContinuationError, OpaqueClickPhase,
};

impl KucInteractionLocator {
    pub(super) fn click_event(
        &self,
        selector: &KucInteractionSelector,
        phase: OpaqueClickPhase,
    ) -> Result<egui::Event, KucInteractionLocatorError> {
        let key = (selector.action_identity.clone(), selector.action_class);
        if self.hidden.contains(&key) {
            return Err(KucInteractionLocatorError::Hidden);
        }
        let mut selected = None;
        for target in self.targets.iter().filter(|target| {
            target.action_identity == selector.action_identity
                && target.action_class == selector.action_class
        }) {
            if selected.is_some() {
                return Err(KucInteractionLocatorError::Ambiguous);
            }
            selected = Some(target);
        }
        let target = selected.ok_or(KucInteractionLocatorError::Missing)?;
        if target.disabled {
            return Err(KucInteractionLocatorError::Disabled);
        }
        if self.ambiguous_bounds.contains(&target.evidence.bounds) {
            return Err(KucInteractionLocatorError::Ambiguous);
        }
        let point = center(target.evidence.bounds);
        match phase {
            OpaqueClickPhase::Aim => Ok(egui::Event::PointerMoved(point)),
            OpaqueClickPhase::Press | OpaqueClickPhase::Release => Ok(egui::Event::PointerButton {
                pos: point,
                button: if selector.action_class
                    == KucInteractionActionClass::TextSurfaceContextTarget
                {
                    egui::PointerButton::Secondary
                } else {
                    egui::PointerButton::Primary
                },
                pressed: matches!(phase, OpaqueClickPhase::Press),
                modifiers: egui::Modifiers::default(),
            }),
        }
    }

    pub(super) fn search_control_request(
        &self,
        suffix: &str,
    ) -> Result<KucOpaqueInteractionRequest, KucSearchTraceContinuationError> {
        let controls = self
            .targets
            .iter()
            .filter(|target| {
                target.action_class == KucInteractionActionClass::SearchControl
                    && target
                        .action_identity
                        .rsplit_once(':')
                        .is_some_and(|(_, id)| id == suffix)
            })
            .map(|target| target.action_identity.clone())
            .collect::<Vec<_>>();
        let [identity] = controls.as_slice() else {
            return Err(KucSearchTraceContinuationError::Unavailable);
        };
        self.request(KucInteractionSelector::new(
            identity.clone(),
            KucInteractionActionClass::SearchControl,
        ))
        .map_err(KucSearchTraceContinuationError::Interaction)
    }
}
impl KucOpaqueClickContinuation {
    pub fn apply_to_raw_input_once(
        &mut self,
        input: &mut egui::RawInput,
    ) -> Result<(), KucOpaqueClickContinuationError> {
        if self.applied {
            return Err(KucOpaqueClickContinuationError::AlreadyApplied);
        }
        input.events.push(self.event.clone());
        self.applied = true;
        Ok(())
    }

    pub fn advance(
        self,
        locator: &KucInteractionLocator,
    ) -> Result<Option<Self>, KucOpaqueClickContinuationError> {
        if !self.applied {
            return Err(KucOpaqueClickContinuationError::NotApplied);
        }
        if self.root_identity != locator.root_identity {
            return Err(KucOpaqueClickContinuationError::RootMismatch);
        }
        if locator.frame_serial != self.frame_serial.saturating_add(1) {
            return Err(KucOpaqueClickContinuationError::FrameDiscontinuity);
        }
        let phase = match self.phase {
            OpaqueClickPhase::Aim => OpaqueClickPhase::Press,
            OpaqueClickPhase::Press => OpaqueClickPhase::Release,
            OpaqueClickPhase::Release => return Ok(None),
        };
        let event = locator
            .click_event(&self.selector, phase)
            .map_err(KucOpaqueClickContinuationError::Interaction)?;
        Ok(Some(Self {
            root_identity: self.root_identity,
            frame_serial: locator.frame_serial,
            selector: self.selector,
            event,
            phase,
            applied: false,
        }))
    }
}
