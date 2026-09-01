use super::error::MotionArtifactError;
use super::types::MotionArtifactSettings;
use crate::egui::FullRootArtifact;
use sha2::{Digest, Sha256};

pub(super) fn validate_settings(
    settings: MotionArtifactSettings,
) -> Result<(), MotionArtifactError> {
    if settings.expected_frame_count == 0
        || settings.width == 0
        || settings.height == 0
        || settings.fps_numerator == 0
        || settings.fps_denominator == 0
    {
        Err(MotionArtifactError::InvalidSettings)
    } else {
        Ok(())
    }
}

pub(super) fn expected_stage_name(index: usize) -> String {
    format!(
        "{}{:0width$}",
        super::constants::STAGE_NAME_PREFIX,
        index,
        width = super::constants::STAGE_NAME_WIDTH
    )
}

pub(super) fn validate_provenance(receipt: &FullRootArtifact) -> Result<(), MotionArtifactError> {
    let manifest = std::fs::read(receipt.manifest_path()).map_err(io_error)?;
    let value: serde_json::Value = match serde_json::from_slice(&manifest) {
        Ok(value) => value,
        Err(error) => return Err(MotionArtifactError::Json(error.to_string())),
    };
    let object = value.as_object().ok_or(MotionArtifactError::Json(
        "root provenance is not an object".into(),
    ))?;
    let matches = object.get("width").and_then(serde_json::Value::as_u64)
        == Some(u64::from(receipt.width()))
        && object.get("height").and_then(serde_json::Value::as_u64)
            == Some(u64::from(receipt.height()))
        && object
            .get("root_record_hash")
            .and_then(serde_json::Value::as_str)
            == Some(receipt.root_record_hash())
        && object.get("pixel_hash").and_then(serde_json::Value::as_str)
            == Some(receipt.pixel_hash())
        && object.get("png_sha256").and_then(serde_json::Value::as_str)
            == Some(receipt.png_sha256());
    if matches {
        Ok(())
    } else {
        Err(MotionArtifactError::MissingProvenance(
            receipt.manifest_path().to_path_buf(),
        ))
    }
}

pub(super) fn hash_sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub(super) fn io_error(error: std::io::Error) -> MotionArtifactError {
    MotionArtifactError::Io(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::FullRootArtifact;
    use crate::egui::motion_artifact_writer::error::MotionArtifactError;
    use sha2::{Digest, Sha256};

    fn manifest_path_for(temp: &std::path::Path, stage: &str) -> std::path::PathBuf {
        temp.join(format!("{stage}.manifest.json"))
    }

    fn receipt(
        temp: &std::path::Path,
        stage: &str,
        width: u32,
        height: u32,
        pixel_hash: &str,
        root_record_hash: &str,
        png_sha256: &str,
    ) -> FullRootArtifact {
        FullRootArtifact::from_test_parts(
            stage.to_owned(),
            temp.join(format!("{stage}.png")),
            manifest_path_for(temp, stage),
            width,
            height,
            root_record_hash.to_owned(),
            pixel_hash.to_owned(),
            png_sha256.to_owned(),
        )
    }

    fn write_manifest(path: &std::path::Path, text: &str) {
        std::fs::write(path, text).expect("manifest test fixture should write");
    }

    #[test]
    fn expected_stage_name_is_zero_padded() {
        assert_eq!(expected_stage_name(0), "frame-000");
        assert_eq!(expected_stage_name(7), "frame-007");
    }

    #[test]
    fn io_error_converts_io_issue_to_motion_error() {
        let converted = io_error(std::io::Error::other("io fail"));
        assert!(matches!(converted, MotionArtifactError::Io(text) if text == "io fail"));
    }

    #[test]
    fn hash_sha256_matches_reference() {
        assert_eq!(
            hash_sha256(b"motion"),
            hex::encode(Sha256::digest(b"motion"))
        );
    }

    #[test]
    fn validate_settings_rejects_invalid_values() {
        assert!(validate_settings(MotionArtifactSettings::new(0, 1, 1)).is_err());
        assert!(
            validate_settings(MotionArtifactSettings {
                expected_frame_count: 1,
                width: 0,
                height: 1,
                fps_numerator: 1,
                fps_denominator: 1
            })
            .is_err()
        );
        assert!(
            validate_settings(MotionArtifactSettings {
                expected_frame_count: 1,
                width: 1,
                height: 1,
                fps_numerator: 1,
                fps_denominator: 1
            })
            .is_ok()
        );
    }

    #[test]
    fn validates_full_provenance_roundtrip() {
        let root = std::env::temp_dir().join("kuc-motion-validation");
        std::fs::create_dir_all(&root).expect("temp directory should create");
        let record = receipt(
            &root,
            "frame-000",
            2,
            1,
            "pixel-hash",
            "record-hash",
            "png-hash",
        );
        write_manifest(
            record.manifest_path(),
            &serde_json::json!({
                "width": 2,
                "height": 1,
                "root_record_hash": "record-hash",
                "pixel_hash": "pixel-hash",
                "png_sha256": "png-hash",
            })
            .to_string(),
        );
        assert!(validate_provenance(&record).is_ok());
    }

    #[test]
    fn validates_provenance_rejects_mismatch_and_non_object() {
        let temp = std::env::temp_dir().join("kuc-motion-validation-invalid");
        std::fs::create_dir_all(&temp).expect("temp directory should create");
        let record = receipt(
            &temp,
            "frame-000",
            2,
            1,
            "pixel-hash",
            "record-hash",
            "png-hash",
        );
        write_manifest(record.manifest_path(), "[]");
        assert!(matches!(
            validate_provenance(&record),
            Err(MotionArtifactError::Json(_))
        ));
        write_manifest(record.manifest_path(), "{");
        assert!(matches!(
            validate_provenance(&record),
            Err(MotionArtifactError::Json(_))
        ));
        let wrong = FullRootArtifact::from_test_parts(
            "frame-000".to_owned(),
            temp.join("frame-000.png"),
            temp.join("frame-000.manifest.json"),
            2,
            1,
            "other-record".to_owned(),
            "pixel-hash".to_owned(),
            "png-hash".to_owned(),
        );
        write_manifest(
            wrong.manifest_path(),
            &serde_json::json!({
                "width": 2,
                "height": 1,
                "root_record_hash": "record-hash",
                "pixel_hash": "pixel-hash",
                "png_sha256": "png-hash",
            })
            .to_string(),
        );
        assert!(matches!(
            validate_provenance(&wrong),
            Err(MotionArtifactError::MissingProvenance(_))
        ));
    }
}
