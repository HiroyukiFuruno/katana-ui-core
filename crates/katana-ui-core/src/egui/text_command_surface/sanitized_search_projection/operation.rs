use super::types::{
    SanitizedSearchOperationSlot, SanitizedSearchProjectionBuildError, SanitizedSearchTarget,
};
use sha2::{Digest, Sha256};

pub(crate) struct SanitizedSearchOperation {
    pub(crate) enabled: bool,
    pub(crate) current: bool,
    pub(crate) target: Option<SanitizedSearchTarget>,
}

impl SanitizedSearchOperation {
    pub(crate) fn new() -> Self {
        Self {
            enabled: false,
            current: false,
            target: None,
        }
    }

    pub(crate) fn validate(
        &self,
        slot: SanitizedSearchOperationSlot,
    ) -> Result<(), SanitizedSearchProjectionBuildError> {
        if self.enabled && self.target.is_none() {
            return Err(
                SanitizedSearchProjectionBuildError::EnabledOperationWithoutTarget {
                    operation: slot,
                },
            );
        }
        Ok(())
    }

    pub(crate) fn hash_into(&self, hasher: &mut Sha256) {
        hasher.update([u8::from(self.enabled)]);
        hasher.update([u8::from(self.current)]);
        match &self.target {
            Some(target) => {
                hasher.update([1]);
                hasher.update(target.stable_signature());
            }
            None => hasher.update([0]),
        }
    }
}

impl std::fmt::Debug for SanitizedSearchOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.target.is_some();
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .field("有効", &self.enabled)
            .finish_non_exhaustive()
    }
}
