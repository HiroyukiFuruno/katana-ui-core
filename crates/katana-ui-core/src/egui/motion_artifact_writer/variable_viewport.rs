mod error;
mod frames;
mod semantic;

pub use error::VariableViewportMotionArtifactError;

use crate::egui::OpaqueMotionReceiptSequence;
use std::path::Path;

use super::constants::{
    DEFAULT_ENCODER, DEFAULT_FPS_DENOMINATOR, DEFAULT_FPS_NUMERATOR, DEFAULT_MUXER,
    VARIABLE_VIEWPORT_GIF_FILENAME, VARIABLE_VIEWPORT_MANIFEST_FILENAME,
    VARIABLE_VIEWPORT_MP4_FILENAME, VARIABLE_VIEWPORT_SCHEMA, VARIABLE_VIEWPORT_STAGING_DIRECTORY,
};
use super::error::MotionArtifactError;
use super::ffmpeg::Ffmpeg;
use super::process::{json_error, write_gif};
use super::types::{
    MotionArtifactSettings, MotionArtifactWriter, VariableViewportMotionArtifact,
    VariableViewportMotionArtifactManifest,
};
use super::validation::{hash_sha256, io_error, validate_settings};
use frames::{load_receipts, normalize_frames, write_staging_frames};
use semantic::semantic_evidence;

impl MotionArtifactWriter {
    /// Writes one fixed-canvas artifact from a KUC-issued variable viewport sequence.
    pub fn write_opaque_variable_viewport(
        &self,
        sequence: &OpaqueMotionReceiptSequence,
        output_dir: &Path,
    ) -> Result<VariableViewportMotionArtifact, VariableViewportMotionArtifactError> {
        if sequence.is_empty() {
            return Err(MotionArtifactError::EmptySequence.into());
        }

        let loaded = load_receipts(sequence.receipts())?;
        let semantic_evidence = semantic_evidence(sequence.receipts())?;

        let width = loaded
            .source_viewports
            .iter()
            .map(|viewport| viewport.width)
            .max()
            .ok_or(MotionArtifactError::EmptySequence)?;
        let height = loaded
            .source_viewports
            .iter()
            .map(|viewport| viewport.height)
            .max()
            .ok_or(MotionArtifactError::EmptySequence)?;
        let settings = MotionArtifactSettings {
            expected_frame_count: loaded.images.len(),
            width,
            height,
            fps_numerator: DEFAULT_FPS_NUMERATOR,
            fps_denominator: DEFAULT_FPS_DENOMINATOR,
        };
        validate_settings(settings)?;

        std::fs::create_dir_all(output_dir).map_err(io_error)?;
        let staging_dir = output_dir.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY);
        std::fs::create_dir_all(&staging_dir).map_err(io_error)?;
        let normalized = normalize_frames(&loaded.images, width, height);
        write_staging_frames(&normalized, &staging_dir)?;

        let gif_path = output_dir.join(VARIABLE_VIEWPORT_GIF_FILENAME);
        write_gif(&normalized, &gif_path, settings.fps_denominator).map_err(io_error)?;
        let mp4_path = output_dir.join(VARIABLE_VIEWPORT_MP4_FILENAME);
        let ffmpeg = Ffmpeg::discover()?;
        let source_evidence = ffmpeg.source_evidence(&staging_dir, normalized.len(), settings)?;
        ffmpeg.encode(&mp4_path, &staging_dir, normalized.len(), settings)?;
        let decoded_evidence = ffmpeg.decode(&mp4_path)?;
        if decoded_evidence.frame_hashes.len() != normalized.len()
            || decoded_evidence.width != width
            || decoded_evidence.height != height
        {
            return Err(MotionArtifactError::Encoder(format!(
                "decoded variable viewport evidence mismatch: frames={}, dimensions={}x{}",
                decoded_evidence.frame_hashes.len(),
                decoded_evidence.width,
                decoded_evidence.height
            ))
            .into());
        }
        if source_evidence.frame_hashes != decoded_evidence.frame_hashes {
            return Err(MotionArtifactError::Encoder(
                "decoded frame hashes do not match the normalized PNG frame sequence".into(),
            )
            .into());
        }

        let manifest_path = output_dir.join(VARIABLE_VIEWPORT_MANIFEST_FILENAME);
        let mut manifest = VariableViewportMotionArtifactManifest {
            schema: VARIABLE_VIEWPORT_SCHEMA,
            source_frame_count: normalized.len(),
            decoded_frame_count: decoded_evidence.frame_hashes.len(),
            width,
            height,
            source_viewports: loaded.source_viewports,
            source_png_sha256: loaded.source_png_sha256,
            source_frame_hashes: source_evidence.frame_hashes,
            decoded_frame_hashes: decoded_evidence.frame_hashes,
            root_record_hashes: loaded.root_record_hashes,
            semantic_evidence,
            frame_sequence_sha256: loaded.frame_sequence_sha256,
            gif_path: gif_path.display().to_string(),
            gif_sha256: hash_sha256(&std::fs::read(&gif_path).map_err(io_error)?),
            mp4_path: mp4_path.display().to_string(),
            mp4_sha256: hash_sha256(&std::fs::read(&mp4_path).map_err(io_error)?),
            ffmpeg_path: ffmpeg.path.display().to_string(),
            ffmpeg_version: ffmpeg.version,
            encoder: DEFAULT_ENCODER,
            muxer: DEFAULT_MUXER,
            canonical_sha256: String::new(),
        };
        manifest.canonical_sha256 =
            hash_sha256(&serde_json::to_vec(&manifest).map_err(json_error)?);
        std::fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).map_err(json_error)?,
        )
        .map_err(io_error)?;

        Ok(VariableViewportMotionArtifact::from_parts(
            manifest,
            manifest_path,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::motion_artifact_writer::fake_ffmpeg::{FakeFfmpegSpec, install};
    use crate::egui::motion_artifact_writer::types::VariableViewportSourceViewport;
    use crate::egui::opaque_motion_receipt::MotionFrameSemanticEvidence;
    use crate::egui::text_command_surface::STAR_TEXT;
    use crate::egui::{FullRootArtifact, OpaqueRootArtifactReceipt};
    use image::{ColorType, ImageEncoder, Rgba, RgbaImage};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct PathEnvGuard(Option<std::ffi::OsString>);

    impl PathEnvGuard {
        fn prepend(root: &Path) -> Self {
            let saved = std::env::var_os("PATH");
            let existing = saved.as_ref().expect("test process defines PATH");
            let replacement = std::env::join_paths(
                std::iter::once(root.to_path_buf()).chain(std::env::split_paths(existing)),
            )
            .expect("PATH should join");
            /* SAFETY: the global test lock serializes PATH mutation. */
            unsafe { std::env::set_var("PATH", replacement) };
            Self(saved)
        }
    }

    impl Drop for PathEnvGuard {
        fn drop(&mut self) {
            if let Some(saved) = self.0.take() {
                /* SAFETY: restore PATH before releasing the global test lock. */
                unsafe { std::env::set_var("PATH", saved) };
            }
        }
    }

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kuc-variable-motion-{label}-{}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("test directory should create");
        path
    }

    fn receipt(
        root: &Path,
        stage: &str,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> OpaqueRootArtifactReceipt {
        let mut png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut png)
            .write_image(rgba, width, height, ColorType::Rgba8.into())
            .expect("test PNG should encode");
        let png_path = root.join(format!("{stage}.png"));
        let manifest_path = root.join(format!("{stage}.manifest.json"));
        std::fs::write(&png_path, &png).expect("test PNG should write");
        let root_record_hash = format!("record-{stage}");
        let pixel_hash = hash_sha256(rgba);
        let png_sha256 = hash_sha256(&png);
        std::fs::write(
            &manifest_path,
            serde_json::to_vec(&serde_json::json!({
                "width": width,
                "height": height,
                "root_record_hash": root_record_hash,
                "pixel_hash": pixel_hash,
                "png_sha256": png_sha256,
            }))
            .expect("test provenance should encode"),
        )
        .expect("test provenance should write");
        let artifact = FullRootArtifact::from_test_parts(
            stage.to_owned(),
            png_path,
            manifest_path,
            width,
            height,
            root_record_hash.clone(),
            pixel_hash,
            png_sha256,
        );
        let semantics = valid_motion_semantics(
            &root_record_hash,
            stage == "frame-000",
            stage == "frame-001",
        );
        OpaqueRootArtifactReceipt::from_test_parts(artifact, Some(semantics))
    }

    fn receipt_from_png(
        root: &Path,
        stage: &str,
        width: u32,
        height: u32,
        png: &[u8],
    ) -> OpaqueRootArtifactReceipt {
        let png_path = root.join(format!("{stage}.png"));
        let manifest_path = root.join(format!("{stage}.manifest.json"));
        std::fs::write(&png_path, png).expect("test PNG should write");
        let root_record_hash = format!("record-{stage}");
        let pixel_hash = "opaque-pixel-hash".to_owned();
        let png_sha256 = hash_sha256(png);
        std::fs::write(
            &manifest_path,
            serde_json::to_vec(&serde_json::json!({
                "width": width,
                "height": height,
                "root_record_hash": root_record_hash,
                "pixel_hash": pixel_hash,
                "png_sha256": png_sha256,
            }))
            .expect("test provenance should encode"),
        )
        .expect("test provenance should write");
        let artifact = FullRootArtifact::from_test_parts(
            stage.to_owned(),
            png_path,
            manifest_path,
            width,
            height,
            root_record_hash.clone(),
            pixel_hash,
            png_sha256,
        );
        OpaqueRootArtifactReceipt::from_test_parts(
            artifact,
            Some(valid_motion_semantics(&root_record_hash, true, true)),
        )
    }

    fn valid_motion_semantics(
        root_record_hash: &str,
        ime_preedit_event_seen: bool,
        ime_commit_event_seen: bool,
    ) -> MotionFrameSemanticEvidence {
        MotionFrameSemanticEvidence {
            root_record_hash: root_record_hash.to_owned(),
            star_scalar_sequence: STAR_TEXT.chars().map(u32::from).collect(),
            star_chromatic_pixel_count: 1,
            control_star_chromatic_pixel_count: 0,
            star_hit_test_seen: true,
            ime_preedit_event_seen,
            ime_commit_event_seen,
            accesskit_snapshot_hash: format!("accesskit-{root_record_hash}"),
        }
    }

    fn metadata_receipt(stage: &str, width: u32, height: u32) -> OpaqueRootArtifactReceipt {
        metadata_receipt_with_semantics(stage, width, height, None)
    }

    fn metadata_receipt_with_semantics(
        stage: &str,
        width: u32,
        height: u32,
        motion_semantics: Option<MotionFrameSemanticEvidence>,
    ) -> OpaqueRootArtifactReceipt {
        let root_record_hash = format!("record-{stage}");
        let artifact = FullRootArtifact::from_test_parts(
            stage.to_owned(),
            Path::new("missing.png").to_path_buf(),
            Path::new("missing.manifest.json").to_path_buf(),
            width,
            height,
            root_record_hash,
            "pixel".into(),
            "png".into(),
        );
        OpaqueRootArtifactReceipt::from_test_parts(artifact, motion_semantics)
    }

    fn variable_sequence(root: &Path) -> OpaqueMotionReceiptSequence {
        let mut sequence = OpaqueMotionReceiptSequence::new();
        sequence
            .push(
                "frame-000",
                receipt(root, "frame-000", 1, 1, &[255, 0, 0, 255]),
            )
            .expect("first receipt should append");
        sequence
            .push(
                "frame-001",
                receipt(root, "frame-001", 2, 2, &[0, 255, 0, 255].repeat(4)),
            )
            .expect("second receipt should append");
        sequence
    }

    #[test]
    fn normalization_uses_max_canvas_and_top_left_black_padding() {
        let small = RgbaImage::from_pixel(1, 1, Rgba([255, 0, 0, 255]));
        let large = RgbaImage::from_pixel(2, 2, Rgba([0, 255, 0, 255]));
        let normalized = normalize_frames(&[small, large], 2, 2);
        assert_eq!(normalized[0].get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(normalized[0].get_pixel(1, 1).0, [0, 0, 0, 255]);
        assert_eq!(normalized[1].get_pixel(1, 1).0, [0, 255, 0, 255]);
    }

    #[test]
    fn normalized_working_set_rejects_extreme_cross_axis_canvas_before_decode() {
        let receipts = [
            metadata_receipt("frame-000", 1_000_000, 1),
            metadata_receipt("frame-001", 1, 1_000_000),
        ];
        assert!(matches!(
            load_receipts(&receipts),
            Err(MotionArtifactError::InvalidSettings)
        ));
        let normal = [
            metadata_receipt("frame-000", 1280, 720),
            metadata_receipt("frame-001", 900, 520),
        ];
        assert!(frames::validate_normalized_working_set(&normal).is_ok());
        let overflow = [metadata_receipt("frame-000", u32::MAX, u32::MAX)];
        assert_eq!(
            frames::validate_normalized_working_set(&overflow),
            Err(MotionArtifactError::InvalidSettings)
        );
    }

    #[test]
    fn staging_encoder_maps_writer_failure_to_invalid_png() {
        let root = temp_dir("readonly-png-writer");
        let readonly_path = root.join("readonly.bin");
        std::fs::write(&readonly_path, []).expect("read-only fixture should write");
        let readonly = std::fs::File::open(readonly_path).expect("fixture should open read-only");
        let image = RgbaImage::from_pixel(1, 1, Rgba([255, 0, 0, 255]));
        assert!(matches!(
            frames::encode_staging_frame(&image, Path::new("frame-000.png"), readonly),
            Err(MotionArtifactError::InvalidPng { .. })
        ));
    }

    #[test]
    fn variable_viewport_writer_rejects_empty_sequence() {
        let root = temp_dir("empty");
        assert_eq!(
            MotionArtifactWriter::new()
                .write_opaque_variable_viewport(&OpaqueMotionReceiptSequence::new(), &root,),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::EmptySequence
            ))
        );
    }

    #[test]
    fn variable_receipt_validation_rejects_stage_and_missing_files() {
        let root = temp_dir("receipt-structure");
        let first = receipt(&root, "frame-000", 1, 1, &[255, 0, 0, 255]);
        let second_root = root.join("second");
        std::fs::create_dir_all(&second_root).expect("second root should create");
        let duplicate = receipt(&second_root, "frame-000", 1, 1, &[0, 255, 0, 255]);
        assert!(matches!(
            load_receipts(&[first, duplicate]),
            Err(MotionArtifactError::DuplicateStage(_))
        ));

        let stale = receipt(&root, "frame-001", 1, 1, &[255, 0, 0, 255]);
        assert!(matches!(
            load_receipts(&[stale]),
            Err(MotionArtifactError::StaleStage { .. })
        ));

        let missing_png = receipt(&root, "frame-000", 1, 1, &[255, 0, 0, 255]);
        std::fs::remove_file(missing_png.artifact().png_path()).expect("test PNG should remove");
        assert!(matches!(
            load_receipts(&[missing_png]),
            Err(MotionArtifactError::MissingPng(_))
        ));

        let missing_manifest = receipt(&root, "frame-000", 1, 1, &[255, 0, 0, 255]);
        std::fs::remove_file(missing_manifest.artifact().manifest_path())
            .expect("test provenance should remove");
        assert!(matches!(
            load_receipts(&[missing_manifest]),
            Err(MotionArtifactError::MissingProvenance(_))
        ));
    }

    #[test]
    fn variable_receipt_validation_rejects_invalid_image_payloads() {
        let root = temp_dir("receipt-content");
        let bad_sha = receipt(&root, "frame-000", 1, 1, &[255, 0, 0, 255]);
        std::fs::write(bad_sha.artifact().png_path(), b"tampered").expect("test PNG should mutate");
        assert!(matches!(
            load_receipts(&[bad_sha]),
            Err(MotionArtifactError::BadPngSha { .. })
        ));

        let invalid_root = root.join("invalid");
        std::fs::create_dir_all(&invalid_root).expect("invalid root should create");
        let invalid = receipt_from_png(&invalid_root, "frame-000", 1, 1, b"not-a-png");
        assert!(matches!(
            load_receipts(&[invalid]),
            Err(MotionArtifactError::InvalidPng { .. })
        ));

        let mut rgb_png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut rgb_png)
            .write_image(&[255, 0, 0], 1, 1, ColorType::Rgb8.into())
            .expect("RGB test PNG should encode");
        let rgb_root = root.join("rgb");
        std::fs::create_dir_all(&rgb_root).expect("RGB root should create");
        let rgb = receipt_from_png(&rgb_root, "frame-000", 1, 1, &rgb_png);
        assert!(matches!(
            load_receipts(&[rgb]),
            Err(MotionArtifactError::InvalidPng { .. })
        ));

        let mut corrupt_png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut corrupt_png)
            .write_image(&[255, 0, 0, 255], 1, 1, ColorType::Rgba8.into())
            .expect("RGBA test PNG should encode");
        let idat = corrupt_png
            .windows(4)
            .position(|window| window == b"IDAT")
            .expect("encoded PNG should contain IDAT");
        corrupt_png[idat + 4] ^= u8::MAX;
        let corrupt_root = root.join("corrupt");
        std::fs::create_dir_all(&corrupt_root).expect("corrupt root should create");
        let corrupt = receipt_from_png(&corrupt_root, "frame-000", 1, 1, &corrupt_png);
        assert!(matches!(
            load_receipts(&[corrupt]),
            Err(MotionArtifactError::InvalidPng { .. })
        ));
    }

    #[test]
    fn variable_receipt_validation_rejects_dimensions_and_empty_pixels() {
        let root = temp_dir("receipt-pixels");
        let mut one_pixel_png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut one_pixel_png)
            .write_image(&[255, 0, 0, 255], 1, 1, ColorType::Rgba8.into())
            .expect("RGBA test PNG should encode");
        let wrong = receipt_from_png(&root, "frame-000", 2, 1, &one_pixel_png);
        assert!(matches!(
            load_receipts(&[wrong]),
            Err(MotionArtifactError::WrongDimensions { .. })
        ));

        let empty_root = root.join("empty");
        std::fs::create_dir_all(&empty_root).expect("empty root should create");
        let empty = receipt(&empty_root, "frame-000", 1, 1, &[0, 0, 0, 0]);
        assert!(matches!(
            load_receipts(&[empty]),
            Err(MotionArtifactError::EmptyPixels(_))
        ));
    }

    #[test]
    fn variable_viewport_writer_preserves_source_and_normalized_evidence() {
        let _lock = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = temp_dir("success");
        let source = root.join("source");
        let output = root.join("output");
        let ffmpeg = root.join("bin");
        std::fs::create_dir_all(&source).expect("source directory should create");
        let spec = FakeFfmpegSpec {
            dimensions: Some("#dimensions 0:2x2".to_owned()),
            ..FakeFfmpegSpec::default()
        };
        install(&ffmpeg, &spec);
        let _path = PathEnvGuard::prepend(&ffmpeg);
        let sequence = variable_sequence(&source);

        let artifact = MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&sequence, &output)
            .expect("variable viewport artifact should write");
        let manifest = artifact.manifest();
        assert_eq!(manifest.schema, VARIABLE_VIEWPORT_SCHEMA);
        assert_eq!(manifest.source_frame_count, 2);
        assert_eq!(manifest.decoded_frame_count, 2);
        assert_eq!((manifest.width, manifest.height), (2, 2));
        assert_eq!(
            manifest.source_viewports,
            vec![
                VariableViewportSourceViewport {
                    width: 1,
                    height: 1,
                },
                VariableViewportSourceViewport {
                    width: 2,
                    height: 2,
                },
            ]
        );
        assert_eq!(manifest.source_png_sha256.len(), 2);
        assert_eq!(manifest.source_frame_hashes, manifest.decoded_frame_hashes);
        assert_eq!(
            manifest.root_record_hashes,
            ["record-frame-000", "record-frame-001"]
        );
        assert_eq!(
            manifest.semantic_evidence.star_scalar_sequence,
            vec![0x2b50, 0xfe0f]
        );
        assert!(manifest.semantic_evidence.ime_preedit_event_seen);
        assert!(manifest.semantic_evidence.ime_commit_event_seen);
        assert_eq!(manifest.semantic_evidence.hit_test_count, 1);
        assert_eq!(
            manifest.semantic_evidence.root_record_hashes,
            ["record-frame-000", "record-frame-001"]
        );
        assert!(!manifest.canonical_sha256.is_empty());
        assert!(artifact.manifest_path().is_file());

        let normalized = image::open(
            output
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join("frame-000.png"),
        )
        .expect("normalized frame should decode")
        .to_rgba8();
        assert_eq!(normalized.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(normalized.get_pixel(1, 1).0, [0, 0, 0, 255]);
    }

    #[test]
    fn semantic_evidence_must_come_from_related_complete_motion_receipts() {
        let root = temp_dir("semantic");
        let sequence = variable_sequence(&root);
        let summary =
            semantic_evidence(sequence.receipts()).expect("complete sequence should summarize");
        assert_eq!(summary.root_record_hash, "record-frame-001");
        assert_eq!(
            summary.root_record_hashes,
            ["record-frame-000", "record-frame-001"]
        );

        let missing = metadata_receipt("frame-000", 1, 1);
        assert!(matches!(
            semantic_evidence(&[missing]),
            Err(VariableViewportMotionArtifactError::InvalidSemanticEvidence(_))
        ));

        let mut unrelated = valid_motion_semantics("record-unrelated", true, true);
        let receipt = metadata_receipt_with_semantics("frame-000", 1, 1, Some(unrelated.clone()));
        assert!(matches!(
            semantic_evidence(&[receipt]),
            Err(VariableViewportMotionArtifactError::UnrelatedSemanticEvidence { .. })
        ));

        unrelated.root_record_hash = "record-frame-000".into();
        for invalid in [
            MotionFrameSemanticEvidence {
                star_scalar_sequence: vec![0x2b50],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                star_chromatic_pixel_count: 0,
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                star_hit_test_seen: false,
                ..unrelated.clone()
            },
        ] {
            let receipt = metadata_receipt_with_semantics("frame-000", 1, 1, Some(invalid));
            assert!(matches!(
                semantic_evidence(&[receipt]),
                Err(VariableViewportMotionArtifactError::InvalidSemanticEvidence(_))
            ));
        }

        for invalid in [
            MotionFrameSemanticEvidence {
                ime_preedit_event_seen: false,
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                ime_commit_event_seen: false,
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_snapshot_hash: String::new(),
                ..unrelated.clone()
            },
        ] {
            let receipt = metadata_receipt_with_semantics("frame-000", 1, 1, Some(invalid));
            assert!(matches!(
                semantic_evidence(&[receipt]),
                Err(VariableViewportMotionArtifactError::InvalidSemanticEvidence(_))
            ));
        }

        let complete = metadata_receipt_with_semantics("frame-000", 1, 1, Some(unrelated));
        let single_root =
            semantic_evidence(&[complete]).expect("one frame may carry all semantic facts");
        assert_eq!(single_root.root_record_hashes, ["record-frame-000"]);

        let serializer_error = serde_json::to_vec(&std::collections::BTreeMap::from([(
            vec![1_u8, 2],
            "value",
        )]))
        .expect_err("byte-array JSON keys should be rejected");
        assert!(matches!(
            semantic::semantic_serialization_error(serializer_error),
            VariableViewportMotionArtifactError::InvalidSemanticEvidence(_)
        ));
    }

    #[test]
    fn variable_viewport_writer_rejects_decoded_evidence_mismatch() {
        let _lock = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = temp_dir("decoded-mismatch");
        let source = root.join("source");
        let ffmpeg = root.join("bin");
        std::fs::create_dir_all(&source).expect("source directory should create");
        install(&ffmpeg, &FakeFfmpegSpec::default());
        let _path = PathEnvGuard::prepend(&ffmpeg);
        let error = MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&variable_sequence(&source), &root.join("output"))
            .expect_err("decoded dimensions should mismatch the normalized canvas");
        assert!(matches!(
            error,
            VariableViewportMotionArtifactError::Motion(MotionArtifactError::Encoder(message))
                if message.contains("decoded variable viewport evidence mismatch")
        ));
    }

    #[test]
    fn variable_viewport_writer_rejects_decoded_hash_mismatch() {
        let _lock = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = temp_dir("decoded-hash-mismatch");
        let source = root.join("source");
        let ffmpeg = root.join("bin");
        std::fs::create_dir_all(&source).expect("source directory should create");
        let spec = FakeFfmpegSpec {
            dimensions: Some("#dimensions 0:2x2".to_owned()),
            decoded_hashes: vec![
                "0, 0, 0, 1, 6, 00112233445566778899aabbccddeeff".into(),
                "0, 1, 1, 1, 6, fedcba9876543210fedcba9876543210".into(),
            ],
            ..FakeFfmpegSpec::default()
        };
        install(&ffmpeg, &spec);
        let _path = PathEnvGuard::prepend(&ffmpeg);
        let error = MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&variable_sequence(&source), &root.join("output"))
            .expect_err("decoded hashes should mismatch the normalized frame sequence");
        assert!(matches!(
            error,
            VariableViewportMotionArtifactError::Motion(MotionArtifactError::Encoder(message))
                if message.contains("decoded frame hashes do not match")
        ));
    }

    #[test]
    fn fixed_dimension_writer_still_rejects_variable_viewports() {
        let root = temp_dir("fixed-contract");
        let sequence = variable_sequence(&root);
        assert!(matches!(
            MotionArtifactWriter::new().write_opaque(&sequence, &root.join("output")),
            Err(MotionArtifactError::WrongDimensions { .. })
        ));
    }

    #[test]
    fn full_motion_contract_links_resize_ime_unicode_hit_test_and_accesskit() {
        use crate::egui::OpaqueRootArtifactReceiptWriter;
        use crate::egui::text_command_surface::{
            EguiTextCommandSurfaceRootFactory, FullTextCommandSurfaceMotionPlan,
            FullTextCommandSurfaceScenarioFactory, FullTextCommandSurfaceScenarioId,
        };

        let plan = FullTextCommandSurfaceMotionPlan::issue(
            FullTextCommandSurfaceMotionPlan::minimum_frame_count(),
        )
        .expect("complete KUC motion plan should issue");
        let scenario = FullTextCommandSurfaceScenarioFactory::new()
            .issue(FullTextCommandSurfaceScenarioId::ResizeScrollIme)
            .expect("resize scenario should issue");
        let mut root = EguiTextCommandSurfaceRootFactory::new()
            .retain_with_lease(
                scenario
                    .into_lease()
                    .expect("resize scenario lease should be available"),
            )
            .expect("resize scenario root should retain");
        let context = egui::Context::default();
        let artifact_root = temp_dir("full-motion-semantic-receipts");
        let mut sequence = OpaqueMotionReceiptSequence::new();
        let mut continuation = None;
        for (index, motion_frame) in plan
            .frames()
            .iter()
            .filter(|frame| {
                frame.scenario_id() == FullTextCommandSurfaceScenarioId::ResizeScrollIme
            })
            .enumerate()
        {
            let mut input = egui::RawInput::default();
            motion_frame
                .apply_to(&mut input, &mut continuation)
                .expect("resize/IME stage should apply without an opaque continuation");
            let mut captured = None;
            crate::egui::run_ui_discard(&context, input, |ui| {
                captured = Some(root.show(ui));
            });
            let frame = captured
                .expect("egui should invoke the retained root")
                .expect("resize root should render");
            motion_frame
                .capture_continuation(frame.interaction_locator(), &mut continuation)
                .expect("resize stages should not require a continuation");
            let stage = format!("frame-{index:03}");
            let receipt = OpaqueRootArtifactReceiptWriter::new()
                .write(&frame, &artifact_root, &stage)
                .expect("same-frame receipt and semantic evidence should write");
            sequence
                .push(&stage, receipt)
                .expect("motion receipt should append in order");
        }

        let viewports = sequence
            .receipts()
            .iter()
            .map(|receipt| (receipt.artifact().width(), receipt.artifact().height()))
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            viewports.len() > 1,
            "resize scenario must vary viewport size"
        );
        let summary =
            semantic_evidence(sequence.receipts()).expect("real motion evidence should summarize");
        assert_eq!(summary.star_scalar_sequence, vec![0x2b50, 0xfe0f]);
        assert_eq!(summary.hit_test_count, 1);
        assert!(!summary.accesskit_snapshot_hash.is_empty());
        assert!(summary.ime_preedit_event_seen && summary.ime_commit_event_seen);
        assert!(
            summary.root_record_hashes.len() >= 3,
            "star, preedit, and commit must bind their actual sequence frames"
        );
        assert!(summary.root_record_hashes.iter().all(|root| {
            sequence
                .receipts()
                .iter()
                .any(|receipt| receipt.artifact().root_record_hash() == root)
        }));
    }
}
