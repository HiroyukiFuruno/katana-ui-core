use super::super::super::support::{
    json_bytes, preflight_output, sha256, validate_decoded_png, write_manifest,
    write_manifest_with_forced_serialization_failure, write_manifest_with_forced_write_failure,
};
use super::super::super::*;
use super::super::temp_dir;

struct FailingJsonSerialization;

impl serde::Serialize for FailingJsonSerialization {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(<S::Error as serde::ser::Error>::custom(
            "intentional serialization failure",
        ))
    }
}

#[test]
fn json_bytes_maps_serialization_failure_to_a_typed_artifact_error() {
    assert!(matches!(
        json_bytes(&FailingJsonSerialization),
        Err(ConsumerArtifactPlanError::Artifact(message))
            if message == "intentional serialization failure"
    ));
}

#[test]
fn support_validates_artifacts_receipts_and_manifest_output() {
    let output = temp_dir("support");
    let artifact = crate::egui::FullRootArtifact::from_test_parts(
        "bad".to_owned(),
        output.join("invalid.png"),
        output.join("bad.consumer-artifact.json"),
        1,
        1,
        "record".to_owned(),
        "pixel".to_owned(),
        "sha".to_owned(),
    );
    std::fs::write(artifact.png_path(), b"not-a-png").expect("write invalid png");
    assert_eq!(
        validate_decoded_png(&artifact),
        Err(ConsumerArtifactPlanError::InvalidPng(
            artifact.png_path().to_path_buf()
        ))
    );
    let missing_artifact = crate::egui::FullRootArtifact::from_test_parts(
        "missing".to_owned(),
        output.join("missing.png"),
        output.join("missing.consumer-artifact.json"),
        1,
        1,
        "record".to_owned(),
        "pixel".to_owned(),
        "sha".to_owned(),
    );
    assert_eq!(
        validate_decoded_png(&missing_artifact),
        Err(ConsumerArtifactPlanError::InvalidPng(
            missing_artifact.png_path().to_path_buf()
        ))
    );

    let mismatched_artifact = crate::egui::FullRootArtifact::from_test_parts(
        "mismatched".to_owned(),
        output.join("mismatched.png"),
        output.join("mismatched.consumer-artifact.json"),
        1,
        1,
        "record".to_owned(),
        "pixel".to_owned(),
        "sha".to_owned(),
    );
    image::RgbaImage::new(2, 2)
        .save(mismatched_artifact.png_path())
        .expect("write mismatched PNG");
    assert_eq!(
        validate_decoded_png(&mismatched_artifact),
        Err(ConsumerArtifactPlanError::InvalidPng(
            mismatched_artifact.png_path().to_path_buf()
        ))
    );

    assert!(preflight_output(output.as_path(), "consumer-stage-0000").is_ok());
    let manifest_path = output.join("consumer-stage-0000.consumer-artifact.json");
    std::fs::write(&manifest_path, b"marker").expect("write marker");
    assert!(matches!(
        preflight_output(output.as_path(), "consumer-stage-0000"),
        Err(ConsumerArtifactPlanError::ExistingMedia(path)) if path == manifest_path
    ));

    let leaf = ConsumerArtifactLeafId::new("leaf-a").expect("leaf");
    let receipt = ConsumerArtifactForwardingReceipt {
        leaf: leaf.clone(),
        stage_id: "consumer-stage-0000".to_owned(),
        root_revision: 77,
        root_identity_fingerprint: sha256(b"root-a"),
        consumed: false,
        fingerprint: sha256(b"fingerprint"),
    };
    assert_eq!(
        receipt.consume_once(
            &sha256(b"root-a"),
            &ConsumerArtifactLeafId::new("leaf-b").expect("leaf"),
            "consumer-stage-0000",
            77,
        ),
        Err(ConsumerArtifactPlanError::ReceiptCrossBind)
    );
    let accepted_receipt = ConsumerArtifactForwardingReceipt {
        leaf: ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
        stage_id: "consumer-stage-0000".to_owned(),
        root_revision: 77,
        root_identity_fingerprint: sha256(b"root-a"),
        consumed: false,
        fingerprint: sha256(b"accepted-fingerprint"),
    };
    assert_eq!(
        accepted_receipt.consume_once(
            &sha256(b"root-a"),
            &ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
            "consumer-stage-0000",
            77,
        ),
        Ok(())
    );
    let reused_receipt = ConsumerArtifactForwardingReceipt {
        leaf: ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
        stage_id: "consumer-stage-0000".to_owned(),
        root_revision: 77,
        root_identity_fingerprint: sha256(b"root-a"),
        consumed: true,
        fingerprint: sha256(b"consumed-fingerprint"),
    };
    assert_eq!(
        reused_receipt.consume_once(
            &sha256(b"root-a"),
            &ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
            "consumer-stage-0000",
            77,
        ),
        Err(ConsumerArtifactPlanError::ReceiptReuse)
    );
    let evidence = ConsumerArtifactEvidence {
        stage_id: "consumer-stage-0001".to_owned(),
        leaf,
        root_revision: 77,
        png_sha256: "png-sha".to_owned(),
        pixel_hash: "pixel".to_owned(),
        root_record_hash: "root-record".to_owned(),
        accesskit_snapshot_hash: "accesskit".to_owned(),
        unicode_evidence_hash: "unicode".to_owned(),
        unicode_evidence_json: r#"{"glyph":"日本語⭐️"}"#.as_bytes().to_vec(),
        receipt: ConsumerArtifactForwardingReceipt {
            leaf: ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
            stage_id: "consumer-stage-0001".to_owned(),
            root_revision: 77,
            root_identity_fingerprint: sha256(b"root-a"),
            consumed: false,
            fingerprint: sha256(b"manifest-fingerprint"),
        },
    };
    let duplicate_evidence = ConsumerArtifactEvidence {
        stage_id: "consumer-stage-0000".to_owned(),
        leaf: ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
        root_revision: 77,
        png_sha256: "png-sha".to_owned(),
        pixel_hash: "pixel".to_owned(),
        root_record_hash: "root-record".to_owned(),
        accesskit_snapshot_hash: "accesskit".to_owned(),
        unicode_evidence_hash: "unicode".to_owned(),
        unicode_evidence_json: r#"{"glyph":"日本語⭐️"}"#.as_bytes().to_vec(),
        receipt: ConsumerArtifactForwardingReceipt {
            leaf: ConsumerArtifactLeafId::new("leaf-a").expect("leaf"),
            stage_id: "consumer-stage-0000".to_owned(),
            root_revision: 77,
            root_identity_fingerprint: sha256(b"root-a"),
            consumed: false,
            fingerprint: sha256(b"duplicate-manifest"),
        },
    };
    assert_eq!(
        write_manifest(output.as_path(), &duplicate_evidence),
        Err(ConsumerArtifactPlanError::ExistingMedia(
            manifest_path.clone()
        ))
    );
    let output_file = output.join("not-a-directory");
    std::fs::write(&output_file, b"file").expect("write output file");
    let write_error_evidence = ConsumerArtifactEvidence {
        stage_id: "consumer-stage-write-error".to_owned(),
        leaf: ConsumerArtifactLeafId::new("leaf-write-error").expect("leaf"),
        root_revision: 77,
        png_sha256: "png-sha".to_owned(),
        pixel_hash: "pixel".to_owned(),
        root_record_hash: "root-record".to_owned(),
        accesskit_snapshot_hash: "accesskit".to_owned(),
        unicode_evidence_hash: "unicode".to_owned(),
        unicode_evidence_json: r#"{"glyph":"日本語⭐️"}"#.as_bytes().to_vec(),
        receipt: ConsumerArtifactForwardingReceipt {
            leaf: ConsumerArtifactLeafId::new("leaf-write-error").expect("leaf"),
            stage_id: "consumer-stage-write-error".to_owned(),
            root_revision: 77,
            root_identity_fingerprint: sha256(b"root-a"),
            consumed: false,
            fingerprint: sha256(b"write-error-manifest"),
        },
    };
    assert!(matches!(
        write_manifest(output_file.as_path(), &write_error_evidence),
        Err(ConsumerArtifactPlanError::Artifact(_))
    ));
    write_manifest(output.as_path(), &evidence).expect("manifest should write");
    let raw = std::fs::read_to_string(output.join("consumer-stage-0001.consumer-artifact.json"))
        .expect("manifest should read");
    let manifest: serde_json::Value = serde_json::from_str(&raw).expect("manifest should parse");
    assert_eq!(
        manifest["stage_id"],
        serde_json::json!("consumer-stage-0001")
    );
    assert!(
        manifest["forwarding_receipt_fingerprint"]
            .as_str()
            .is_some_and(|fingerprint| !fingerprint.is_empty())
    );
    assert_eq!(
        manifest["unicode_evidence_file"],
        serde_json::json!("consumer-stage-0001.unicode-evidence.json")
    );
    let unicode_evidence = std::fs::read(output.join("consumer-stage-0001.unicode-evidence.json"))
        .expect("Unicode evidence should be published");
    assert_eq!(unicode_evidence, evidence.unicode_evidence_json());
    assert_eq!(
        manifest["unicode_evidence_payload_sha256"],
        serde_json::json!(sha256(&unicode_evidence))
    );
    assert!(matches!(
        write_manifest_with_forced_serialization_failure(
            temp_dir("manifest-serialization-failure").as_path(),
            &evidence,
        ),
        Err(ConsumerArtifactPlanError::Artifact(message))
            if message == "intentional serialization failure"
    ));
    assert!(matches!(
        write_manifest_with_forced_write_failure(
            temp_dir("manifest-write-failure").as_path(),
            &evidence,
        ),
        Err(ConsumerArtifactPlanError::Artifact(message))
            if message == "intentional manifest write failure"
    ));
    assert_eq!(
        write_manifest(output.as_path(), &evidence),
        Err(ConsumerArtifactPlanError::ExistingMedia(
            output.join("consumer-stage-0001.unicode-evidence.json")
        ))
    );
}
