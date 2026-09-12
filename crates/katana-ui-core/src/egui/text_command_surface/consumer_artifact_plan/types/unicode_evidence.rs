use super::support::json_bytes;
use super::{ConsumerArtifactLeafId, ConsumerArtifactPlanError};
use crate::egui::text_command_surface::{
    KucUnicodeColorGlyphEvidenceCapture, KucUnicodeColorGlyphEvidenceOptions,
};
use crate::text_raster::PlatformFontSha256;
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
    let mut options = KucUnicodeColorGlyphEvidenceOptions::default();
    pin_first_readable_emoji_candidate(&mut options.config);
    options
}

fn pin_first_readable_emoji_candidate(config: &mut crate::text_raster::PlatformTextRasterConfig) {
    if !config.emoji_candidate_sha256.is_empty() {
        return;
    }
    let Some((path, hash)) = config.emoji_candidates.iter().find_map(|path| {
        std::fs::read(path)
            .ok()
            .map(|bytes| (path.clone(), PlatformFontSha256::digest(&bytes)))
    }) else {
        return;
    };
    /* WHY: consumer artifactはhost固有のfont policyを注入しないため、KUCが読み込めた候補だけをpinする。 */
    config.emoji_candidates = vec![path];
    config.emoji_candidate_sha256 = vec![hash];
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
    use super::{
        pin_first_readable_emoji_candidate, remove_catalog_face_source_path, unicode_evidence_value,
    };
    use crate::egui::text_command_surface::consumer_artifact_plan::ConsumerArtifactPlanError;
    use crate::text_raster::PlatformFontSha256;

    fn test_raster_config(
        emoji_candidates: Vec<std::path::PathBuf>,
        emoji_candidate_sha256: Vec<PlatformFontSha256>,
    ) -> crate::text_raster::PlatformTextRasterConfig {
        crate::text_raster::PlatformTextRasterConfig {
            proportional_candidates: Vec::new(),
            monospace_candidates: Vec::new(),
            emoji_candidates,
            emoji_candidate_sha256,
            cache_capacity: 1,
        }
    }

    #[test]
    fn pin_first_readable_emoji_candidate_skips_unreadable_candidates() {
        let path = std::env::temp_dir().join(format!(
            "kuc-unicode-evidence-readable-{}",
            std::process::id()
        ));
        std::fs::write(&path, b"test emoji font").expect("test font should be readable");
        let missing = path.with_extension("missing");
        let mut config = test_raster_config(vec![missing, path.clone()], Vec::new());

        pin_first_readable_emoji_candidate(&mut config);

        assert_eq!(config.emoji_candidates, vec![path.clone()]);
        assert_eq!(
            config.emoji_candidate_sha256,
            vec![PlatformFontSha256::digest(b"test emoji font")]
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn pin_first_readable_emoji_candidate_leaves_unreadable_candidates_unpinned() {
        let missing = std::env::temp_dir().join(format!(
            "kuc-unicode-evidence-missing-{}",
            std::process::id()
        ));
        let mut config = test_raster_config(vec![missing.clone()], Vec::new());

        pin_first_readable_emoji_candidate(&mut config);

        assert_eq!(config.emoji_candidates, vec![missing]);
        assert!(config.emoji_candidate_sha256.is_empty());
    }

    #[test]
    fn pin_first_readable_emoji_candidate_preserves_an_existing_pin() {
        let candidate = std::env::temp_dir().join(format!(
            "kuc-unicode-evidence-pinned-{}",
            std::process::id()
        ));
        let existing_hash = PlatformFontSha256::digest(b"existing pin");
        let mut config = test_raster_config(vec![candidate.clone()], vec![existing_hash]);

        pin_first_readable_emoji_candidate(&mut config);

        assert_eq!(config.emoji_candidates, vec![candidate]);
        assert_eq!(config.emoji_candidate_sha256, vec![existing_hash]);
    }

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
