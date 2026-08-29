use sha2::{Digest, Sha256};

pub(crate) struct RootEventCorrelationFingerprint;

impl RootEventCorrelationFingerprint {
    pub(crate) fn compose(
        root_identity: &str,
        state_revision: u64,
        event_batch_fingerprint: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.root-event-correlation/v1\0");
        hasher.update(root_identity.as_bytes());
        hasher.update([0]);
        hasher.update(state_revision.to_le_bytes());
        hasher.update([0]);
        hasher.update(event_batch_fingerprint.as_bytes());
        hex::encode(hasher.finalize())
    }
}
