use super::super::SanitizedSearchProjection;
use super::super::sanitized_command_projection::SanitizedCommandProjection;
use super::super::sanitized_context_projection::SanitizedContextMenuProjection;
use super::super::sanitized_document_root_style::SanitizedDocumentRootStyleKey;
use super::super::sanitized_tab_projection::SanitizedTabProjection;
use sha2::{Digest, Sha256};

pub(super) const SEARCH_PROJECTION_FINGERPRINT_LENGTH: usize = 32;

/// Opaque stable identity supplied by the host.
pub struct SanitizedDocumentRootIdentity {
    bytes: Box<[u8]>,
}

impl SanitizedDocumentRootIdentity {
    /// Creates an identity without assigning meaning to its bytes.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into().into_boxed_slice(),
        }
    }

    pub(crate) fn same_identity(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }

    pub(crate) fn stable_fingerprint(&self) -> String {
        hex::encode(Sha256::digest(&self.bytes))
    }
}

impl std::fmt::Debug for SanitizedDocumentRootIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SanitizedDocumentRootIdentity(..)")
    }
}

/// The complete public input for one retained document root.
pub struct SanitizedDocumentRootInput {
    pub(crate) revision: u64,
    pub(crate) identity: SanitizedDocumentRootIdentity,
    pub(crate) snapshot: String,
    pub(crate) readonly: bool,
    pub(crate) style: SanitizedDocumentRootStyleKey,
    pub(crate) command_projection: Option<SanitizedCommandProjection>,
    pub(crate) floating_command_projection: Option<SanitizedCommandProjection>,
    pub(crate) search_projection: Option<SanitizedSearchProjection>,
    pub(crate) context_projection: Option<SanitizedContextMenuProjection>,
    pub(crate) tab_projection: Option<SanitizedTabProjection>,
    pub(crate) search_projection_fingerprint: Option<[u8; SEARCH_PROJECTION_FINGERPRINT_LENGTH]>,
}
