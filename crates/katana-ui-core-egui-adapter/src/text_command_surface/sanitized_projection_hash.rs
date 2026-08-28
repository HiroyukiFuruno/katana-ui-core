use sha2::{Digest, Sha256};

pub(crate) struct SanitizedProjectionHash;

impl SanitizedProjectionHash {
    pub(crate) const SHA256_BYTES: usize = 32;

    pub(crate) fn hash_len_prefixed_text(digest: &mut Sha256, value: &str) {
        digest.update(value.len().to_le_bytes());
        digest.update(value.as_bytes());
    }

    pub(crate) fn hash_len_prefixed_bytes(digest: &mut Sha256, value: &[u8]) {
        digest.update(value.len().to_le_bytes());
        digest.update(value);
    }

    pub(crate) fn sha256_signature(value: &[u8]) -> [u8; Self::SHA256_BYTES] {
        Sha256::digest(value).into()
    }

    pub(crate) fn sha256_signature_hex(value: &[u8]) -> String {
        Self::sha256_signature(value)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    }
}
