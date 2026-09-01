use super::super::constants::RGBA_CHANNEL_COUNT;
use super::super::model::KucUnicodeColorGlyphEvidence;
use super::super::types::KucUnicodeColorGlyphEvidenceError;
use sha2::{Digest, Sha256};

pub(super) fn hash_pixels(pixels: &[[u8; RGBA_CHANNEL_COUNT]]) -> String {
    let bytes = pixels.iter().flatten().copied().collect::<Vec<_>>();
    hex::encode(Sha256::digest(bytes))
}

pub(super) fn canonical_hash(
    artifact: &KucUnicodeColorGlyphEvidence,
) -> Result<String, KucUnicodeColorGlyphEvidenceError> {
    serde_json::to_vec(artifact)
        .map(|bytes| hex::encode(Sha256::digest(bytes)))
        .map_err(serialization_error)
}

fn serialization_error(error: serde_json::Error) -> KucUnicodeColorGlyphEvidenceError {
    KucUnicodeColorGlyphEvidenceError::Serialization(error.to_string())
}

#[cfg(test)]
#[path = "serialization_tests.rs"]
mod serialization_tests;
