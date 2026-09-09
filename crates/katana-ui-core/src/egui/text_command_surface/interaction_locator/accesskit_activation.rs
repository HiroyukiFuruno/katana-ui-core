use super::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucOpaqueInteractionRequest,
};

impl KucInteractionLocator {
    /// Resolves one current actionable AccessKit node and issues its click request exactly once.
    pub fn request_accesskit_activation(
        &self,
        action_identity: &str,
        action_class: KucInteractionActionClass,
    ) -> Result<KucOpaqueInteractionRequest, KucInteractionLocatorError> {
        let targets = self
            .targets
            .iter()
            .filter(|target| {
                target.action_identity == action_identity && target.action_class == action_class
            })
            .collect::<Vec<_>>();
        let [target] = targets.as_slice() else {
            return Err(if targets.is_empty() {
                KucInteractionLocatorError::Missing
            } else {
                KucInteractionLocatorError::Ambiguous
            });
        };
        let key = (target.action_identity.clone(), target.action_class);
        if !self.requested.borrow_mut().insert(key.clone()) {
            return Err(KucInteractionLocatorError::Duplicate);
        }
        if self.hidden.contains(&key) {
            return Err(KucInteractionLocatorError::Hidden);
        }
        if target.disabled {
            return Err(KucInteractionLocatorError::Disabled);
        }
        if self.ambiguous_bounds.contains(&target.evidence.bounds) {
            return Err(KucInteractionLocatorError::Ambiguous);
        }
        Ok(KucOpaqueInteractionRequest {
            root_identity: self.root_identity.clone(),
            state_revision: self.state_revision,
            correlation_fingerprint: self.correlation_fingerprint.clone(),
            events: vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node: target.evidence.response_id.accesskit_id(),
                    data: None,
                },
            )],
            queued: false,
        })
    }
}
