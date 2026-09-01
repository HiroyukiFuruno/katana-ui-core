use super::metadata::{FrameRootMetadata, FullRootArtifact, FullRootArtifactError};
use super::validation::{
    checked_rgba_len, encode_png, sha256_hex, validate_output_dir, validate_stage_id,
};
use crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame;
use std::fs;
use std::path::Path;

/// KUC-owned encoder for one already-composited root frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct FullRootArtifactWriter;

impl FullRootArtifactWriter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Writes one root PNG and a metadata-only manifest.
    pub fn write(
        &self,
        frame: &EguiTextCommandSurfaceHostRootFrame,
        output_dir: &Path,
        stage_id: &str,
    ) -> Result<FullRootArtifact, FullRootArtifactError> {
        validate_output_dir(output_dir)?;
        validate_stage_id(stage_id)?;

        let (rgba, width, height, expected_pixel_hash) = frame.artifact_rgba();
        let pixel_hash = validate_frame_pixels(rgba, width, height, expected_pixel_hash)?;

        fs::create_dir_all(output_dir).map_err(FullRootArtifactError::CreateDirectory)?;
        let png_path = output_dir.join(format!("{stage_id}.png"));
        let manifest_path = output_dir.join(format!("{stage_id}.manifest.json"));
        let png_bytes = encode_png(rgba, width, height)?;
        fs::write(&png_path, &png_bytes).map_err(|source| FullRootArtifactError::Write {
            path: png_path.clone(),
            source,
        })?;

        let png_sha256 = sha256_hex(&png_bytes);
        let metadata = FrameRootMetadata {
            stage_id,
            png_path: png_path.clone(),
            manifest_path: manifest_path.clone(),
            width,
            height,
            root_record_hash: frame.record().record_hash(),
            pixel_hash: pixel_hash.clone(),
        };
        let manifest = metadata.manifest(&png_sha256);
        let manifest_bytes =
            serde_json::to_vec_pretty(&manifest).map_err(FullRootArtifactError::ManifestEncode)?;
        fs::write(&manifest_path, manifest_bytes).map_err(|source| {
            FullRootArtifactError::Write {
                path: manifest_path.clone(),
                source,
            }
        })?;

        Ok(FullRootArtifact::from_metadata(metadata, png_sha256))
    }
}

fn validate_frame_pixels(
    rgba: &[u8],
    width: u32,
    height: u32,
    expected_pixel_hash: &str,
) -> Result<String, FullRootArtifactError> {
    let expected_len = checked_rgba_len(width, height)?;
    if rgba.len() != expected_len {
        return Err(FullRootArtifactError::RgbaLength {
            expected: expected_len,
            actual: rgba.len(),
        });
    }
    if rgba.iter().all(|byte| *byte == 0) {
        return Err(FullRootArtifactError::EmptyPixels);
    }
    let pixel_hash = sha256_hex(rgba);
    if pixel_hash != expected_pixel_hash {
        return Err(FullRootArtifactError::FrameHashMismatch);
    }
    Ok(pixel_hash)
}

#[cfg(test)]
mod tests {
    use crate::atom::TextArea;
    use crate::egui::text_command_surface::{
        EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
        EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
        EguiTextCommandSurfaceRootFactory, TextCommandSurfaceStyle,
    };
    use crate::render_model::UiStateId;
    use crate::text_surface::{TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn root_frame(stage_id: &str, width: u32, height: u32) -> EguiTextCommandSurfaceHostRootFrame {
        let mut props = TextSurfaceProps::new(
            TextArea::new("full-root-process").value(stage_id),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, width, height),
        );
        props.accessibility_label = "root".to_owned();
        let presentation = EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new(stage_id)),
            text: TextSurfacePresentation::from_props(&props),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        };
        let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
            1,
            b"full-root-target".to_vec(),
            presentation,
            TextCommandSurfaceStyle::standard().expect("standard style"),
        )
        .expect("token should encode");
        let mut root: EguiTextCommandSurfaceHostRoot = EguiTextCommandSurfaceRootFactory::new()
            .retain(token)
            .expect("host root should retain");
        let mut captured = None;
        let context = egui::Context::default();
        crate::egui::run_ui_discard(
            &context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 360.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                captured = Some(root.show(ui));
            },
        );
        captured
            .expect("root should render")
            .expect("host root should render")
    }

    #[test]
    fn write_success() {
        let writer = super::FullRootArtifactWriter::new();
        let dir = tempfile_dir("full-root-process-success");
        let frame = root_frame("frame-000", 10, 12);
        let artifact = writer
            .write(&frame, dir.as_path(), "frame-000")
            .expect("full root write should succeed");
        assert_eq!(artifact.stage_id(), "frame-000");
        assert!(artifact.png_path().is_file());
        assert!(artifact.manifest_path().is_file());

        let opaque_dir = tempfile_dir("full-root-process-opaque");
        let opaque = crate::egui::OpaqueRootArtifactReceiptWriter::new()
            .write(&frame, opaque_dir.as_path(), "frame-000")
            .expect("opaque receipt write should succeed");
        assert_eq!(opaque.stage_id(), "frame-000");
    }

    #[test]
    fn write_reports_png_and_manifest_destination_failures() {
        let writer = super::FullRootArtifactWriter::new();
        let frame = root_frame("frame-000", 10, 12);

        let png_dir = tempfile_dir("full-root-process-png-write-error");
        std::fs::create_dir(png_dir.join("frame-000.png"))
            .expect("blocking PNG directory should create");
        assert!(matches!(
            writer.write(&frame, &png_dir, "frame-000"),
            Err(super::FullRootArtifactError::Write { path, .. })
                if path == png_dir.join("frame-000.png")
        ));

        let manifest_dir = tempfile_dir("full-root-process-manifest-write-error");
        std::fs::create_dir(manifest_dir.join("frame-000.manifest.json"))
            .expect("blocking manifest directory should create");
        assert!(matches!(
            writer.write(&frame, &manifest_dir, "frame-000"),
            Err(super::FullRootArtifactError::Write { path, .. })
                if path == manifest_dir.join("frame-000.manifest.json")
        ));
    }

    #[test]
    fn frame_pixel_validation_rejects_length_empty_and_hash_mismatches() {
        assert!(matches!(
            super::validate_frame_pixels(&[1, 2, 3], 1, 1, "unused"),
            Err(super::FullRootArtifactError::RgbaLength { .. })
        ));
        assert!(matches!(
            super::validate_frame_pixels(&[0, 0, 0, 0], 1, 1, "unused"),
            Err(super::FullRootArtifactError::EmptyPixels)
        ));
        assert!(matches!(
            super::validate_frame_pixels(&[1, 2, 3, 4], 1, 1, "wrong"),
            Err(super::FullRootArtifactError::FrameHashMismatch)
        ));
    }

    fn tempfile_dir(label: &str) -> std::path::PathBuf {
        let pid = std::process::id();
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should work")
            .as_millis();
        let path =
            std::env::temp_dir().join(format!("kuc-full-root-process-{label}-{pid}-{millis}"));
        std::fs::create_dir_all(&path).expect("test directory should create");
        path
    }
}
