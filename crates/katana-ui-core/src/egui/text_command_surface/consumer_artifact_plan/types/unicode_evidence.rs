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
    let mut published = unicode_evidence_value(unicode)?;
    remove_catalog_face_source_path(&mut published)?;
    json_bytes(&published)
}

fn unicode_evidence_value<T: serde::Serialize>(
    unicode: T,
) -> Result<serde_json::Value, ConsumerArtifactPlanError> {
    serde_json::to_value(unicode)
        .map_err(|error| ConsumerArtifactPlanError::UnicodeEvidence(error.to_string()))
}

fn remove_catalog_face_source_path(
    published: &mut serde_json::Value,
) -> Result<(), ConsumerArtifactPlanError> {
    let catalog_face = published
        .get_mut("catalog_face")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| {
            ConsumerArtifactPlanError::UnicodeEvidence(
                "Unicode evidence catalog face is missing".to_owned(),
            )
        })?;
    if catalog_face.remove("source_file_path").is_none() {
        return Err(ConsumerArtifactPlanError::UnicodeEvidence(
            "Unicode evidence catalog face source path is missing".to_owned(),
        ));
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::{remove_catalog_face_source_path, unicode_evidence_value};
    use crate::egui::text_command_surface::consumer_artifact_plan::ConsumerArtifactPlanError;

    #[test]
    fn published_unicode_evidence_removes_the_host_font_path() {
        let mut published = serde_json::json!({
            "catalog_face": {
                "family": "Noto Color Emoji",
                "source_file_path": "/Users/example/Library/Fonts/NotoColorEmoji.ttf",
                "raw_file_sha256": "hash"
            }
        });

        remove_catalog_face_source_path(&mut published).expect("path should be removed");

        assert!(published["catalog_face"]["source_file_path"].is_null());
        assert_eq!(
            published["catalog_face"]["raw_file_sha256"],
            serde_json::json!("hash")
        );
    }

    #[test]
    fn published_unicode_evidence_rejects_a_missing_catalog_face() {
        let mut published = serde_json::json!({});

        assert!(matches!(
            remove_catalog_face_source_path(&mut published),
            Err(ConsumerArtifactPlanError::UnicodeEvidence(message))
                if message == "Unicode evidence catalog face is missing"
        ));
    }

    #[test]
    fn published_unicode_evidence_rejects_a_catalog_face_without_a_path() {
        let mut published = serde_json::json!({"catalog_face": {"family": "Noto Color Emoji"}});

        assert!(matches!(
            remove_catalog_face_source_path(&mut published),
            Err(ConsumerArtifactPlanError::UnicodeEvidence(message))
                if message == "Unicode evidence catalog face source path is missing"
        ));
    }

    struct FailingSerialization;

    impl serde::Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(<S::Error as serde::ser::Error>::custom(
                "intentional Unicode failure",
            ))
        }
    }

    #[test]
    fn unicode_evidence_value_maps_serialization_failure_to_a_typed_error() {
        assert!(matches!(
            unicode_evidence_value(FailingSerialization),
            Err(ConsumerArtifactPlanError::UnicodeEvidence(message))
                if message == "intentional Unicode failure"
        ));
    }
}
