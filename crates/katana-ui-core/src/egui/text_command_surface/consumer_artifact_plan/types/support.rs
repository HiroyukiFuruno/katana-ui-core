use super::{
    ConsumerArtifactEvidence, ConsumerArtifactPlanError, GenericInteractionClass, SCHEMA_VERSION,
};
use crate::egui::FullRootArtifact;
use crate::egui::text_command_surface::EguiTextCommandSurfaceRootFactoryError;
use image::GenericImageView;
use sha2::{Digest, Sha256};
use std::path::Path;

const STAGE_WIDTH: f32 = 1280.0;
const STAGE_HEIGHT: f32 = 720.0;
const RESIZED_STAGE_WIDTH: f32 = 900.0;
const RESIZED_STAGE_HEIGHT: f32 = 520.0;

#[derive(serde::Serialize)]
struct ConsumerArtifactManifest<'a> {
    schema: &'static str,
    schema_version: u16,
    stage_id: &'a str,
    leaf_id: &'a str,
    root_revision: u64,
    png_sha256: &'a str,
    pixel_hash: &'a str,
    root_record_hash: &'a str,
    accesskit_snapshot_hash: &'a str,
    unicode_evidence_hash: &'a str,
    unicode_evidence_payload_sha256: String,
    unicode_evidence_file: &'a str,
    forwarding_receipt_fingerprint: &'a str,
}

type ManifestSerializer =
    for<'a> fn(&'a ConsumerArtifactManifest<'a>) -> Result<Vec<u8>, serde_json::Error>;
type ManifestWriter = fn(&Path, &[u8]) -> std::io::Result<()>;

fn serialize_manifest<'a>(
    manifest: &'a ConsumerArtifactManifest<'a>,
) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(manifest)
}

fn write_manifest_bytes(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, bytes)
}

pub(super) fn map_root_error(
    error: EguiTextCommandSurfaceRootFactoryError,
) -> ConsumerArtifactPlanError {
    match error {
        EguiTextCommandSurfaceRootFactoryError::IdentityChanged
        | EguiTextCommandSurfaceRootFactoryError::StaleRevision { .. }
        | EguiTextCommandSurfaceRootFactoryError::RevisionConflict { .. } => {
            ConsumerArtifactPlanError::TokenRootMismatch
        }
        other => ConsumerArtifactPlanError::Root(other.to_string()),
    }
}

pub(super) fn raw_input(class: GenericInteractionClass) -> egui::RawInput {
    let events = match class {
        GenericInteractionClass::TextInput => vec![egui::Event::Text("日本語⭐️".to_owned())],
        GenericInteractionClass::ImeCommit => vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語⭐️".to_owned(),
        ))],
        /* WHY: KUC resolves physical interaction coordinates from the retained root's locator. */
        GenericInteractionClass::Selection
        | GenericInteractionClass::ToolbarActivation
        | GenericInteractionClass::FloatingToolbar
        | GenericInteractionClass::ContextMenu => Vec::new(),
        GenericInteractionClass::Scroll => vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 24.0),
            modifiers: egui::Modifiers::NONE,
            phase: egui::TouchPhase::Move,
        }],
        GenericInteractionClass::Search | GenericInteractionClass::AccessibilityActivation => {
            Vec::new()
        }
        GenericInteractionClass::ViewportResize => Vec::new(),
    };
    let size = if matches!(class, GenericInteractionClass::ViewportResize) {
        egui::vec2(RESIZED_STAGE_WIDTH, RESIZED_STAGE_HEIGHT)
    } else {
        egui::vec2(STAGE_WIDTH, STAGE_HEIGHT)
    };
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        events,
        ..egui::RawInput::default()
    }
}

pub(super) fn preflight_output(
    output_dir: &Path,
    stage_id: &str,
) -> Result<(), ConsumerArtifactPlanError> {
    for suffix in [
        ".png",
        ".manifest.json",
        ".consumer-artifact.json",
        ".unicode-evidence.json",
    ] {
        let path = output_dir.join(format!("{stage_id}{suffix}"));
        if path.exists() {
            return Err(ConsumerArtifactPlanError::ExistingMedia(path));
        }
    }
    Ok(())
}

pub(super) fn cleanup_stage_output(
    output_dir: &Path,
    stage_id: &str,
) -> Result<(), ConsumerArtifactPlanError> {
    for suffix in [
        ".png",
        ".manifest.json",
        ".consumer-artifact.json",
        ".unicode-evidence.json",
    ] {
        let path = output_dir.join(format!("{stage_id}{suffix}"));
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|error| ConsumerArtifactPlanError::Artifact(error.to_string()))?;
        }
    }
    Ok(())
}

pub(super) fn write_manifest(
    output_dir: &Path,
    evidence: &ConsumerArtifactEvidence,
) -> Result<(), ConsumerArtifactPlanError> {
    write_manifest_with_dependencies(
        output_dir,
        evidence,
        serialize_manifest,
        write_manifest_bytes,
    )
}

fn write_manifest_with_dependencies(
    output_dir: &Path,
    evidence: &ConsumerArtifactEvidence,
    serializer: ManifestSerializer,
    writer: ManifestWriter,
) -> Result<(), ConsumerArtifactPlanError> {
    let unicode_evidence_file = format!("{}.unicode-evidence.json", evidence.stage_id);
    let unicode_evidence_path = output_dir.join(&unicode_evidence_file);
    if unicode_evidence_path.exists() {
        return Err(ConsumerArtifactPlanError::ExistingMedia(
            unicode_evidence_path,
        ));
    }
    std::fs::write(&unicode_evidence_path, evidence.unicode_evidence_json())
        .map_err(|error| ConsumerArtifactPlanError::Artifact(error.to_string()))?;
    let manifest = ConsumerArtifactManifest {
        schema: "kuc.consumer-full-editor-artifact.v1",
        schema_version: SCHEMA_VERSION,
        stage_id: &evidence.stage_id,
        leaf_id: evidence.leaf.as_str(),
        root_revision: evidence.root_revision,
        png_sha256: &evidence.png_sha256,
        pixel_hash: &evidence.pixel_hash,
        root_record_hash: &evidence.root_record_hash,
        accesskit_snapshot_hash: &evidence.accesskit_snapshot_hash,
        unicode_evidence_hash: &evidence.unicode_evidence_hash,
        unicode_evidence_payload_sha256: sha256(evidence.unicode_evidence_json()),
        unicode_evidence_file: &unicode_evidence_file,
        forwarding_receipt_fingerprint: &evidence.receipt.fingerprint,
    };
    let bytes = serializer(&manifest)
        .map_err(|error| ConsumerArtifactPlanError::Artifact(error.to_string()))?;
    let path = output_dir.join(format!("{}.consumer-artifact.json", evidence.stage_id));
    if path.exists() {
        return Err(ConsumerArtifactPlanError::ExistingMedia(path));
    }
    writer(&path, &bytes).map_err(|error| ConsumerArtifactPlanError::Artifact(error.to_string()))
}

#[cfg(test)]
pub(super) fn write_manifest_with_forced_serialization_failure(
    output_dir: &Path,
    evidence: &ConsumerArtifactEvidence,
) -> Result<(), ConsumerArtifactPlanError> {
    write_manifest_with_dependencies(
        output_dir,
        evidence,
        |_| {
            Err(serde_json::Error::io(std::io::Error::other(
                "intentional serialization failure",
            )))
        },
        write_manifest_bytes,
    )
}

#[cfg(test)]
pub(super) fn write_manifest_with_forced_write_failure(
    output_dir: &Path,
    evidence: &ConsumerArtifactEvidence,
) -> Result<(), ConsumerArtifactPlanError> {
    write_manifest_with_dependencies(output_dir, evidence, serialize_manifest, |_, _| {
        Err(std::io::Error::other("intentional manifest write failure"))
    })
}

pub(super) fn json_bytes<T: serde::Serialize>(
    value: &T,
) -> Result<Vec<u8>, ConsumerArtifactPlanError> {
    serde_json::to_vec(value)
        .map_err(|error| ConsumerArtifactPlanError::Artifact(error.to_string()))
}

pub(super) fn validate_decoded_png(
    artifact: &FullRootArtifact,
) -> Result<(), ConsumerArtifactPlanError> {
    let bytes = std::fs::read(artifact.png_path())
        .map_err(|_| ConsumerArtifactPlanError::InvalidPng(artifact.png_path().to_path_buf()))?;
    let image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)
        .map_err(|_| ConsumerArtifactPlanError::InvalidPng(artifact.png_path().to_path_buf()))?;
    if image.dimensions() != (artifact.width(), artifact.height()) {
        return Err(ConsumerArtifactPlanError::InvalidPng(
            artifact.png_path().to_path_buf(),
        ));
    }
    Ok(())
}

pub(super) fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
