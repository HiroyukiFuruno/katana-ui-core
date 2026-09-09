use super::{ConsumerArtifactLeafId, ConsumerArtifactPlanError};

/// Non-serializable, one-shot acknowledgement bound to one retained root and issued stage.
pub struct ConsumerArtifactForwardingReceipt {
    pub(super) leaf: ConsumerArtifactLeafId,
    pub(super) stage_id: String,
    pub(super) root_revision: u64,
    pub(super) root_identity_fingerprint: String,
    pub(super) consumed: bool,
    pub(super) fingerprint: String,
}

impl ConsumerArtifactForwardingReceipt {
    pub(super) fn consume_once(
        mut self,
        root_identity_fingerprint: &str,
        leaf: &ConsumerArtifactLeafId,
        stage_id: &str,
        root_revision: u64,
    ) -> Result<(), ConsumerArtifactPlanError> {
        if self.consumed {
            return Err(ConsumerArtifactPlanError::ReceiptReuse);
        }
        if self.root_identity_fingerprint != root_identity_fingerprint
            || &self.leaf != leaf
            || self.stage_id != stage_id
            || self.root_revision != root_revision
        {
            return Err(ConsumerArtifactPlanError::ReceiptCrossBind);
        }
        self.consumed = true;
        let _ = &self.fingerprint;
        Ok(())
    }
}

impl std::fmt::Debug for ConsumerArtifactForwardingReceipt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ConsumerArtifactForwardingReceipt(..)")
    }
}
