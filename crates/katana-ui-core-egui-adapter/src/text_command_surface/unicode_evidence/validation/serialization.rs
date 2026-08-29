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
        .map_err(|error| KucUnicodeColorGlyphEvidenceError::Serialization(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_pixels_is_stable_and_sensitive_to_rgba_content() {
        let first = hash_pixels(&[[1, 2, 3, 4]]);
        assert_eq!(first, hash_pixels(&[[1, 2, 3, 4]]));
        assert_ne!(first, hash_pixels(&[[1, 2, 3, 5]]));
        assert_eq!(first.len(), 64);
    }
}
