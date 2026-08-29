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
    let bytes = serde_json::to_vec(artifact).map_err(serialization_error)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn serialization_error(error: serde_json::Error) -> KucUnicodeColorGlyphEvidenceError {
    KucUnicodeColorGlyphEvidenceError::Serialization(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_serialization_errors_map_to_the_typed_error() {
        let result = serde_json::from_slice::<serde_json::Value>(b"").map_err(serialization_error);
        assert!(matches!(
            result,
            Err(KucUnicodeColorGlyphEvidenceError::Serialization(_))
        ));
    }
}
