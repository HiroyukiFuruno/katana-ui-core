use super::support::json_bytes;
use super::{ConsumerArtifactLeafId, ConsumerArtifactPlanError};
use crate::egui::text_command_surface::{
    KucUnicodeColorGlyphEvidenceCapture, KucUnicodeColorGlyphEvidenceOptions,
};
use sha2::{Digest, Sha256};

const STAGE_UNICODE_EVIDENCE_DOMAIN: &[u8] = b"kuc.consumer-artifact.unicode-evidence.v1";

pub(super) fn capture_unicode_evidence(
    options: KucUnicodeColorGlyphEvidenceOptions,
) -> Result<Vec<u8>, ConsumerArtifactPlanError> {
    let unicode = KucUnicodeColorGlyphEvidenceCapture::capture(options)
        .map_err(|error| ConsumerArtifactPlanError::UnicodeEvidence(error.to_string()))?;
    json_bytes(&unicode)
}

pub(super) fn artifact_unicode_evidence_options() -> KucUnicodeColorGlyphEvidenceOptions {
    KucUnicodeColorGlyphEvidenceOptions::default()
}

pub(super) fn bind_unicode_evidence(
    unicode_evidence: &[u8],
    stage_id: &str,
    leaf: &ConsumerArtifactLeafId,
    root_revision: u64,
    root_record_hash: &str,
    accesskit_snapshot_hash: &str,
    receipt_fingerprint: &str,
) -> String {
    let mut hasher = Sha256::new();
    for field in [
        STAGE_UNICODE_EVIDENCE_DOMAIN,
        unicode_evidence,
        stage_id.as_bytes(),
        leaf.as_str().as_bytes(),
        &root_revision.to_be_bytes(),
        root_record_hash.as_bytes(),
        accesskit_snapshot_hash.as_bytes(),
        receipt_fingerprint.as_bytes(),
    ] {
        hasher.update((field.len() as u64).to_be_bytes());
        hasher.update(field);
    }
    hex::encode(hasher.finalize())
}
