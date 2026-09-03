mod error;
mod frames;
mod output;
mod semantic;

pub use error::VariableViewportMotionArtifactError;

use crate::egui::OpaqueMotionReceiptSequence;
use std::path::Path;

use super::constants::{
    DEFAULT_ENCODER, DEFAULT_FPS_DENOMINATOR, DEFAULT_FPS_NUMERATOR, DEFAULT_MUXER,
    VARIABLE_VIEWPORT_GIF_FILENAME, VARIABLE_VIEWPORT_MANIFEST_FILENAME,
    VARIABLE_VIEWPORT_MP4_FILENAME, VARIABLE_VIEWPORT_SCHEMA,
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
use output::{
    claim_public_staging_directory, open_output_directory, private_scratch_directory,
    publish_public_frames, publish_scratch_file, reject_occupied_output_targets,
    reject_scratch_output_overlap, verify_public_artifact_file, verify_public_frame_files,
    verify_public_output_directories, write_new_output,
};
use semantic::semantic_evidence;

impl MotionArtifactWriter {
    /// KUC 発行の可変 viewport シーケンスから固定 canvas の成果物を 1 つ出力する。
    ///
    /// `output_dir` 自体は既存でもよいが、staging directory、GIF、MP4、manifest の各出力先は
    /// 存在してはならない。再出力には新しい出力先を使用する。
    /// export が失敗した場合、writer 自身が作成した staging と一部の出力が残ることがある。
    /// pathname の identity 検証は各検証時点の観測であり、出力 namespace をロックしない。
    /// 安定した成果物 path が必要な間は、caller が出力先への外部変更を防ぐこと。
    pub fn write_opaque_variable_viewport(
        &self,
        sequence: &OpaqueMotionReceiptSequence,
        output_dir: &Path,
    ) -> Result<VariableViewportMotionArtifact, VariableViewportMotionArtifactError> {
        if output_dir.to_str().is_none() {
            return Err(MotionArtifactError::InvalidSettings.into());
        }
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

        let output = open_output_directory(output_dir)?;
        reject_occupied_output_targets(&output, output_dir)?;
        let temporary_parent = std::env::temp_dir();
        let scratch = private_scratch_directory(&temporary_parent)?;
        let scratch_dir = scratch.path();
        reject_scratch_output_overlap(scratch_dir, output_dir)?;
        let public_staging = claim_public_staging_directory(&output, output_dir)?;
        let normalized = normalize_frames(&loaded.images, width, height);
        write_staging_frames(&normalized, scratch_dir)?;

        let gif_path = output_dir.join(VARIABLE_VIEWPORT_GIF_FILENAME);
        let scratch_gif_path = scratch_dir.join(VARIABLE_VIEWPORT_GIF_FILENAME);
        write_gif(&normalized, &scratch_gif_path, settings.fps_denominator).map_err(io_error)?;
        let mp4_path = output_dir.join(VARIABLE_VIEWPORT_MP4_FILENAME);
        let scratch_mp4_path = scratch_dir.join(VARIABLE_VIEWPORT_MP4_FILENAME);
        let ffmpeg = Ffmpeg::discover()?;
        if ffmpeg.path.to_str().is_none() {
            return Err(MotionArtifactError::InvalidSettings.into());
        }
        let source_evidence = ffmpeg.source_evidence(scratch_dir, normalized.len(), settings)?;
        ffmpeg.encode(&scratch_mp4_path, scratch_dir, normalized.len(), settings)?;
        let decoded_evidence = ffmpeg.decode(&scratch_mp4_path)?;
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
            gif_sha256: hash_sha256(&std::fs::read(&scratch_gif_path).map_err(io_error)?),
            mp4_path: mp4_path.display().to_string(),
            mp4_sha256: hash_sha256(&std::fs::read(&scratch_mp4_path).map_err(io_error)?),
            ffmpeg_path: ffmpeg.path.display().to_string(),
            ffmpeg_version: ffmpeg.version,
            encoder: DEFAULT_ENCODER,
            muxer: DEFAULT_MUXER,
            canonical_sha256: String::new(),
        };
        manifest.canonical_sha256 =
            hash_sha256(&serde_json::to_vec(&manifest).map_err(json_error)?);
        verify_public_output_directories(&output, &public_staging, output_dir)?;
        let frames =
            publish_public_frames(scratch_dir, &public_staging, output_dir, normalized.len())?;
        let gif = publish_scratch_file(
            &scratch_gif_path,
            &output,
            VARIABLE_VIEWPORT_GIF_FILENAME,
            &gif_path,
        )?;
        let mp4 = publish_scratch_file(
            &scratch_mp4_path,
            &output,
            VARIABLE_VIEWPORT_MP4_FILENAME,
            &mp4_path,
        )?;
        let manifest_file = write_new_output(
            &output,
            VARIABLE_VIEWPORT_MANIFEST_FILENAME,
            &manifest_path,
            &serde_json::to_vec_pretty(&manifest).map_err(json_error)?,
        )?;
        verify_public_output_directories(&output, &public_staging, output_dir)?;
        verify_public_artifact_file(&gif, &gif_path)?;
        verify_public_artifact_file(&mp4, &mp4_path)?;
        verify_public_artifact_file(&manifest_file, &manifest_path)?;
        verify_public_frame_files(&frames, output_dir)?;

        Ok(VariableViewportMotionArtifact::from_parts(
            manifest,
            manifest_path,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::motion_artifact_writer::constants::VARIABLE_VIEWPORT_STAGING_DIRECTORY;
    use crate::egui::motion_artifact_writer::fake_ffmpeg::{FakeFfmpegSpec, install};
    use crate::egui::motion_artifact_writer::types::VariableViewportSourceViewport;
    use crate::egui::opaque_motion_receipt::MotionFrameSemanticEvidence;
    use crate::egui::text_command_surface::STAR_TEXT;
    use crate::egui::text_command_surface::accesskit_projection::{
        AccessKitTextInputBounds, AccessKitTextInputNode, AccessKitTextInputRole,
    };
    use crate::egui::{FullRootArtifact, OpaqueRootArtifactReceipt};
    #[cfg(unix)]
    use cap_fs_ext::DirExt;
    use image::{ColorType, ImageEncoder, Rgba, RgbaImage};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    #[cfg(unix)]
    #[derive(Clone, Copy)]
    enum ReplacementKind {
        Regular,
        Hardlink,
        Symlink,
        DanglingSymlink,
    }

    #[cfg(unix)]
    impl ReplacementKind {
        const ALL: [Self; 4] = [
            Self::Regular,
            Self::Hardlink,
            Self::Symlink,
            Self::DanglingSymlink,
        ];

        const fn label(self) -> &'static str {
            match self {
                Self::Regular => "regular",
                Self::Hardlink => "hardlink",
                Self::Symlink => "symlink",
                Self::DanglingSymlink => "dangling-symlink",
            }
        }
    }

    #[cfg(unix)]
    #[derive(Clone, Copy)]
    enum OccupiedTargetKind {
        StagingSymlink,
        GifHardlink,
        Mp4Symlink,
        GifDanglingSymlink,
        ManifestHardlink,
    }

    #[cfg(unix)]
    impl OccupiedTargetKind {
        const ALL: [Self; 5] = [
            Self::StagingSymlink,
            Self::GifHardlink,
            Self::Mp4Symlink,
            Self::GifDanglingSymlink,
            Self::ManifestHardlink,
        ];

        const fn label(self) -> &'static str {
            match self {
                Self::StagingSymlink => "staging-symlink",
                Self::GifHardlink => "gif-hardlink",
                Self::Mp4Symlink => "mp4-symlink",
                Self::GifDanglingSymlink => "gif-dangling-symlink",
                Self::ManifestHardlink => "manifest-hardlink",
            }
        }

        const fn target(self) -> &'static str {
            match self {
                Self::StagingSymlink => VARIABLE_VIEWPORT_STAGING_DIRECTORY,
                Self::GifHardlink | Self::GifDanglingSymlink => VARIABLE_VIEWPORT_GIF_FILENAME,
                Self::Mp4Symlink => VARIABLE_VIEWPORT_MP4_FILENAME,
                Self::ManifestHardlink => VARIABLE_VIEWPORT_MANIFEST_FILENAME,
            }
        }
    }

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
            expected_accesskit_text_input_value: "⭐️入力".to_owned(),
            accesskit_text_input_nodes: vec![valid_accesskit_text_input_node()],
            accesskit_snapshot_hash: format!("accesskit-{root_record_hash}"),
        }
    }

    fn valid_accesskit_text_input_node() -> AccessKitTextInputNode {
        let value = "⭐️入力".to_owned();
        AccessKitTextInputNode {
            role: AccessKitTextInputRole::MultilineTextInput,
            scalar_sequence: value.chars().map(u32::from).collect(),
            value: Some(value),
            bounds: Some(AccessKitTextInputBounds {
                x0_bits: 0.0_f64.to_bits(),
                y0_bits: 0.0_f64.to_bits(),
                x1_bits: 320.0_f64.to_bits(),
                y1_bits: 240.0_f64.to_bits(),
            }),
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
    fn staging_encoder_writes_and_decodes_png_with_a_file() {
        let root = temp_dir("file-png-writer");
        let path = root.join("frame-000.png");
        let image = RgbaImage::from_pixel(1, 1, Rgba([1, 2, 3, u8::MAX]));
        let writer = std::fs::File::create(&path).expect("PNG output should create");

        frames::encode_staging_frame(&image, &path, writer)
            .expect("PNG should encode through a file");

        assert_eq!(
            image::open(&path)
                .expect("encoded PNG should decode")
                .to_rgba8()
                .get_pixel(0, 0)
                .0,
            [1, 2, 3, u8::MAX]
        );
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

    #[cfg(unix)]
    #[test]
    fn variable_viewport_writer_rejects_non_utf8_output_before_any_writes() {
        use std::os::unix::ffi::OsStringExt;

        let root = temp_dir("non-utf8-output");
        let output = root.join(std::ffi::OsString::from_vec(vec![b'o', 0xff]));
        assert_eq!(
            MotionArtifactWriter::new()
                .write_opaque_variable_viewport(&OpaqueMotionReceiptSequence::new(), &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::InvalidSettings
            ))
        );
        assert!(!output.exists(), "invalid output must not be created");
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
    fn variable_receipt_validation_bounds_encoded_png_and_provenance_reads() {
        let root = temp_dir("bounded-receipt-inputs");
        let oversized_png = receipt(&root, "frame-000", 1, 1, &[255, 0, 0, 255]);
        std::fs::OpenOptions::new()
            .write(true)
            .open(oversized_png.artifact().png_path())
            .expect("PNG fixture should open")
            .set_len(frames::MAX_VARIABLE_VIEWPORT_ENCODED_PNG_BYTES + 1)
            .expect("PNG fixture should become sparse and oversized");
        assert!(matches!(
            load_receipts(&[oversized_png]),
            Err(MotionArtifactError::InvalidSettings)
        ));

        let provenance_root = root.join("provenance");
        std::fs::create_dir(&provenance_root).expect("provenance root should create");
        let oversized_provenance = receipt(&provenance_root, "frame-000", 1, 1, &[0, 255, 0, 255]);
        std::fs::OpenOptions::new()
            .write(true)
            .open(oversized_provenance.artifact().manifest_path())
            .expect("provenance fixture should open")
            .set_len(frames::MAX_VARIABLE_VIEWPORT_PROVENANCE_BYTES + 1)
            .expect("provenance fixture should become sparse and oversized");
        assert!(matches!(
            load_receipts(&[oversized_provenance]),
            Err(MotionArtifactError::InvalidSettings)
        ));
    }

    #[test]
    fn bounded_receipt_read_enforces_boundary_growth_and_io_errors() {
        struct ReadFailure;

        impl std::io::Read for ReadFailure {
            fn read(&mut self, _buffer: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("reader failed"))
            }
        }

        let root = temp_dir("bounded-receipt-read");
        let path = root.join("bytes");
        std::fs::write(&path, b"abc").expect("bounded fixture should write");
        assert_eq!(
            frames::read_bounded_file(&path, 3).expect("exactly bounded file should read"),
            b"abc"
        );
        let initial_length = std::fs::metadata(&path)
            .expect("bounded fixture metadata should read")
            .len();
        let reader = std::fs::File::open(&path).expect("bounded fixture should open");
        let mut appender = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("bounded fixture should reopen for append");
        std::io::Write::write_all(&mut appender, b"d")
            .expect("bounded fixture should grow after opening reader");
        assert_eq!(
            frames::read_bounded(reader, initial_length, 3),
            Err(MotionArtifactError::InvalidSettings)
        );
        assert!(matches!(
            frames::read_bounded(std::io::Cursor::new(b"abcd"), 0, 3),
            Err(MotionArtifactError::InvalidSettings)
        ));
        assert!(matches!(
            frames::read_bounded(ReadFailure, 0, 3),
            Err(MotionArtifactError::Io(_))
        ));
        assert!(matches!(
            frames::read_bounded(std::io::Cursor::new([]), u64::MAX, 3),
            Err(MotionArtifactError::InvalidSettings)
        ));
        assert!(matches!(
            frames::read_bounded(std::io::Cursor::new([]), 0, u64::MAX),
            Err(MotionArtifactError::InvalidSettings)
        ));
        assert!(matches!(
            frames::read_bounded_file(&root.join("missing"), 3),
            Err(MotionArtifactError::Io(_))
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
        let staging = output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY);
        for index in 0..manifest.source_frame_count {
            assert!(
                staging
                    .join(super::super::validation::expected_stage_name(index))
                    .with_extension("png")
                    .is_file(),
                "public staging must contain normalized frame {index}"
            );
        }
        assert_eq!(
            hash_sha256(
                &std::fs::read(output.join(VARIABLE_VIEWPORT_GIF_FILENAME))
                    .expect("published GIF should read"),
            ),
            manifest.gif_sha256
        );
        assert_eq!(
            hash_sha256(
                &std::fs::read(output.join(VARIABLE_VIEWPORT_MP4_FILENAME))
                    .expect("published MP4 should read"),
            ),
            manifest.mp4_sha256
        );

        let normalized = image::open(staging.join("frame-000.png"))
            .expect("normalized frame should decode")
            .to_rgba8();
        assert_eq!(normalized.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(normalized.get_pixel(1, 1).0, [0, 0, 0, 255]);
    }

    #[test]
    fn public_staging_claim_serializes_exports() {
        let output = temp_dir("concurrent-staging-claim");
        let first_output = open_output_directory(&output).expect("output directory should open");
        let second_output = first_output
            .try_clone()
            .expect("output directory should clone");
        let staging = output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY);
        let barrier = std::sync::Barrier::new(2);
        let results = std::thread::scope(|scope| {
            let first = scope.spawn(|| {
                barrier.wait();
                claim_public_staging_directory(&first_output, &output)
            });
            let second = scope.spawn(|| {
                barrier.wait();
                claim_public_staging_directory(&second_output, &output)
            });
            [
                first.join().expect("first claim should finish"),
                second.join().expect("second claim should finish"),
            ]
        });
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(
            results.iter().filter(|result| {
                matches!(result, Err(VariableViewportMotionArtifactError::OccupiedOutputTarget { path }) if path == &staging)
            }).count(),
            1
        );
        assert!(staging.is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn pinned_directories_preserve_receipts_after_output_and_staging_paths_swap() {
        use std::os::unix::fs::symlink;

        let root = temp_dir("pinned-directory-path-swap");
        let source = root.join("source");
        let output = root.join("output");
        std::fs::create_dir_all(&source).expect("source directory should create");
        let sequence = variable_sequence(&source);
        let input = sequence
            .receipts()
            .iter()
            .map(|receipt| {
                (
                    receipt.artifact().png_path().to_path_buf(),
                    std::fs::read(receipt.artifact().png_path()).expect("source PNG should read"),
                    receipt.artifact().manifest_path().to_path_buf(),
                    std::fs::read(receipt.artifact().manifest_path())
                        .expect("source provenance should read"),
                )
            })
            .collect::<Vec<_>>();
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let staging_capability = claim_public_staging_directory(&output_capability, &output)
            .expect("staging claim should succeed");
        let displaced_staging = root.join("displaced-staging");
        std::fs::rename(
            output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY),
            &displaced_staging,
        )
        .expect("staging directory should rename");
        symlink(&source, output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY))
            .expect("replacement staging symlink should create");
        let displaced_output = root.join("displaced-output");
        std::fs::rename(&output, &displaced_output).expect("output directory should rename");
        symlink(&source, &output).expect("replacement output symlink should create");

        write_new_output(
            &output_capability,
            VARIABLE_VIEWPORT_GIF_FILENAME,
            &output.join(VARIABLE_VIEWPORT_GIF_FILENAME),
            b"safe GIF",
        )
        .expect("pinned output should write outside replacement symlink");
        write_new_output(
            &staging_capability,
            "frame-000.png",
            &output
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join("frame-000.png"),
            b"safe frame",
        )
        .expect("pinned staging should write outside replacement symlink");
        assert!(matches!(
            verify_public_output_directories(&output_capability, &staging_capability, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));

        assert_eq!(
            std::fs::read(displaced_output.join(VARIABLE_VIEWPORT_GIF_FILENAME))
                .expect("pinned GIF should remain in displaced output"),
            b"safe GIF"
        );
        assert_eq!(
            std::fs::read(displaced_staging.join("frame-000.png"),)
                .expect("pinned frame should remain in independently displaced staging"),
            b"safe frame"
        );
        for (png_path, png, manifest_path, manifest) in input {
            assert_eq!(
                std::fs::read(png_path).expect("source PNG should remain readable"),
                png
            );
            assert_eq!(
                std::fs::read(manifest_path).expect("source provenance should remain readable"),
                manifest
            );
        }
        assert!(load_receipts(sequence.receipts()).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn public_directory_identity_accepts_a_symlink_to_the_pinned_output() {
        use std::os::unix::fs::symlink;

        let root = temp_dir("pinned-directory-original-symlink");
        let output = root.join("output");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let staging_capability = claim_public_staging_directory(&output_capability, &output)
            .expect("staging claim should succeed");
        let displaced_output = root.join("displaced-output");
        std::fs::rename(&output, &displaced_output).expect("output directory should rename");
        symlink(&displaced_output, &output).expect("original output symlink should create");

        verify_public_output_directories(&output_capability, &staging_capability, &output)
            .expect("symlink to the pinned output should remain publishable");
    }

    #[cfg(unix)]
    #[test]
    fn public_staging_identity_rejects_a_replacement_after_capability_writes() {
        use std::os::unix::fs::symlink;

        let root = temp_dir("pinned-staging-identity-replacement");
        let output = root.join("output");
        let replacement = root.join("replacement");
        std::fs::create_dir(&replacement).expect("replacement directory should create");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let staging_capability = claim_public_staging_directory(&output_capability, &output)
            .expect("staging claim should succeed");
        write_new_output(
            &staging_capability,
            "frame-000.png",
            &output
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join("frame-000.png"),
            b"safe frame",
        )
        .expect("pinned staging should write before replacement");
        let displaced_staging = root.join("displaced-staging");
        std::fs::rename(
            output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY),
            &displaced_staging,
        )
        .expect("staging directory should rename");
        symlink(
            &replacement,
            output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY),
        )
        .expect("replacement staging symlink should create");

        assert!(matches!(
            verify_public_output_directories(&output_capability, &staging_capability, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
        assert_eq!(
            std::fs::read(displaced_staging.join("frame-000.png"))
                .expect("pinned frame should remain in displaced staging"),
            b"safe frame"
        );
    }

    #[cfg(unix)]
    #[test]
    fn public_directory_identity_preserves_missing_path_errors() {
        let root = temp_dir("pinned-directory-identity-missing-path");
        let output = root.join("output");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let staging_capability = claim_public_staging_directory(&output_capability, &output)
            .expect("staging claim should succeed");
        std::fs::remove_dir(output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY))
            .expect("staging directory should remove");

        assert!(matches!(
            verify_public_output_directories(&output_capability, &staging_capability, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn public_artifact_identity_rejects_replaced_entries() {
        use std::os::unix::fs::symlink;

        for filename in [
            VARIABLE_VIEWPORT_GIF_FILENAME,
            VARIABLE_VIEWPORT_MP4_FILENAME,
            VARIABLE_VIEWPORT_MANIFEST_FILENAME,
        ] {
            let root = temp_dir(&format!("pinned-artifact-identity-{filename}"));
            let output = root.join("output");
            let output_capability =
                open_output_directory(&output).expect("output directory should open");
            let artifact_path = output.join(filename);
            let artifact = write_new_output(
                &output_capability,
                filename,
                &artifact_path,
                b"published artifact",
            )
            .expect("pinned artifact should write");
            verify_public_artifact_file(&artifact, &artifact_path)
                .expect("unchanged artifact should verify");

            let same_file_alias = output.join(format!("{filename}.same-file"));
            std::fs::hard_link(&artifact_path, &same_file_alias)
                .expect("same-file alias should create");
            std::fs::remove_file(&artifact_path).expect("published artifact should remove");
            std::fs::hard_link(&same_file_alias, &artifact_path)
                .expect("same-file replacement should create");
            verify_public_artifact_file(&artifact, &artifact_path)
                .expect("same file alias should remain publishable");

            for replacement in ReplacementKind::ALL {
                std::fs::remove_file(&artifact_path)
                    .expect("same-file artifact should remove before replacement");
                let replacement_path = output.join(format!("{filename}.{}", replacement.label()));
                match replacement {
                    ReplacementKind::Regular => {
                        std::fs::write(&artifact_path, b"unrelated artifact")
                            .expect("regular replacement should write");
                    }
                    ReplacementKind::Hardlink => {
                        std::fs::write(&replacement_path, b"unrelated artifact")
                            .expect("hard-link source should write");
                        std::fs::hard_link(&replacement_path, &artifact_path)
                            .expect("hard-link replacement should create");
                    }
                    ReplacementKind::Symlink => {
                        std::fs::write(&replacement_path, b"unrelated artifact")
                            .expect("symlink target should write");
                        symlink(&replacement_path, &artifact_path)
                            .expect("symlink replacement should create");
                    }
                    ReplacementKind::DanglingSymlink => {
                        symlink(&replacement_path, &artifact_path)
                            .expect("dangling symlink replacement should create");
                    }
                }
                assert!(matches!(
                    verify_public_artifact_file(&artifact, &artifact_path),
                    Err(VariableViewportMotionArtifactError::Motion(
                        MotionArtifactError::Io(_)
                    ))
                ));
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn public_frame_identity_rejects_replaced_entries_in_an_unchanged_staging_directory() {
        use std::os::unix::fs::symlink;

        for replacement in ReplacementKind::ALL {
            let root = temp_dir(&format!("pinned-frame-identity-{}", replacement.label()));
            let output = root.join("output");
            let output_capability =
                open_output_directory(&output).expect("output directory should open");
            let staging_capability = claim_public_staging_directory(&output_capability, &output)
                .expect("staging claim should succeed");
            let frame_path = output
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join("frame-000.png");
            let frame = write_new_output(
                &staging_capability,
                "frame-000.png",
                &frame_path,
                b"normalized frame",
            )
            .expect("pinned frame should write");
            verify_public_output_directories(&output_capability, &staging_capability, &output)
                .expect("unchanged public directories should verify");
            verify_public_frame_files(std::slice::from_ref(&frame), &output)
                .expect("unchanged public frame should verify");

            std::fs::remove_file(&frame_path)
                .expect("public frame should remove before replacement");
            let replacement_path = output.join(format!("frame-000.{}", replacement.label()));
            match replacement {
                ReplacementKind::Regular => {
                    std::fs::write(&frame_path, b"unrelated frame")
                        .expect("regular replacement should write");
                }
                ReplacementKind::Hardlink => {
                    std::fs::write(&replacement_path, b"unrelated frame")
                        .expect("hard-link source should write");
                    std::fs::hard_link(&replacement_path, &frame_path)
                        .expect("hard-link replacement should create");
                }
                ReplacementKind::Symlink => {
                    std::fs::write(&replacement_path, b"unrelated frame")
                        .expect("symlink target should write");
                    symlink(&replacement_path, &frame_path)
                        .expect("symlink replacement should create");
                }
                ReplacementKind::DanglingSymlink => {
                    symlink(&replacement_path, &frame_path)
                        .expect("dangling symlink replacement should create");
                }
            }

            verify_public_output_directories(&output_capability, &staging_capability, &output)
                .expect("staging directory identity should remain unchanged");
            assert!(matches!(
                verify_public_frame_files(std::slice::from_ref(&frame), &output),
                Err(VariableViewportMotionArtifactError::Motion(
                    MotionArtifactError::Io(_)
                ))
            ));
        }
    }

    #[test]
    fn scratch_directory_rejects_a_caller_output_overlap() {
        let root = temp_dir("scratch-output-overlap");
        let output = root.join("output");
        let scratch = output.join("scratch");
        std::fs::create_dir_all(&scratch).expect("overlapping scratch should create");
        assert_eq!(
            reject_scratch_output_overlap(&scratch, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::InvalidSettings
            ))
        );
        assert!(matches!(
            reject_scratch_output_overlap(&root.join("missing"), &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn private_scratch_directory_rejects_a_non_utf8_parent_before_creation() {
        use std::os::unix::ffi::OsStringExt;

        let root = temp_dir("non-utf8-scratch-parent");
        let temporary_parent = root.join(std::ffi::OsString::from_vec(vec![b't', 0xff]));
        assert!(matches!(
            private_scratch_directory(&temporary_parent),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::InvalidSettings
            ))
        ));
        assert!(
            !temporary_parent.exists(),
            "rejected temporary parent must not be created"
        );
    }

    #[cfg(unix)]
    #[test]
    fn pinned_directories_reject_late_output_entries_without_changing_receipts() {
        use std::os::unix::fs::symlink;

        for target_name in [
            VARIABLE_VIEWPORT_GIF_FILENAME,
            VARIABLE_VIEWPORT_MP4_FILENAME,
            VARIABLE_VIEWPORT_MANIFEST_FILENAME,
        ] {
            for entry_kind in ReplacementKind::ALL {
                let root = temp_dir(&format!("late-{target_name}-{}", entry_kind.label()));
                let source = root.join("source");
                let output = root.join("output");
                std::fs::create_dir_all(&source).expect("source directory should create");
                std::fs::create_dir_all(&output).expect("output directory should create");
                let sequence = variable_sequence(&source);
                let input = sequence
                    .receipts()
                    .iter()
                    .map(|receipt| {
                        (
                            receipt.artifact().png_path().to_path_buf(),
                            std::fs::read(receipt.artifact().png_path())
                                .expect("source PNG should read"),
                            receipt.artifact().manifest_path().to_path_buf(),
                            std::fs::read(receipt.artifact().manifest_path())
                                .expect("source provenance should read"),
                        )
                    })
                    .collect::<Vec<_>>();
                let output_capability =
                    open_output_directory(&output).expect("output directory should open");
                claim_public_staging_directory(&output_capability, &output)
                    .expect("staging claim should succeed");
                let scratch = root.join("scratch");
                std::fs::create_dir_all(&scratch).expect("scratch directory should create");
                let staged = scratch.join("staged-output");
                std::fs::write(&staged, b"staged output").expect("staged output should write");
                let target = output.join(target_name);

                match entry_kind {
                    ReplacementKind::Regular => {
                        std::fs::write(&target, b"existing output")
                            .expect("late regular output should write");
                    }
                    ReplacementKind::Hardlink => {
                        std::fs::hard_link(&input[0].0, &target)
                            .expect("late hard link should create");
                    }
                    ReplacementKind::Symlink => {
                        symlink(&input[0].0, &target).expect("late symlink should create");
                    }
                    ReplacementKind::DanglingSymlink => {
                        symlink(source.join("missing-output"), &target)
                            .expect("late dangling symlink should create");
                    }
                }

                let original_target_bytes = std::fs::read(&target).ok();
                let original_link = std::fs::read_link(&target).ok();
                assert_eq!(
                    publish_scratch_file(&staged, &output_capability, target_name, &target),
                    Err(VariableViewportMotionArtifactError::OccupiedOutputTarget {
                        path: target.clone(),
                    })
                );
                assert_eq!(std::fs::read(&target).ok(), original_target_bytes);
                assert_eq!(std::fs::read_link(&target).ok(), original_link);
                assert_eq!(
                    std::fs::read(&staged).expect("staged output should remain readable"),
                    b"staged output"
                );
                for (png_path, png, manifest_path, manifest) in input {
                    assert_eq!(
                        std::fs::read(png_path).expect("source PNG should remain readable"),
                        png
                    );
                    assert_eq!(
                        std::fs::read(manifest_path)
                            .expect("source provenance should remain readable"),
                        manifest
                    );
                }
                assert!(load_receipts(sequence.receipts()).is_ok());
            }
        }
    }

    #[test]
    fn pinned_output_allows_exactly_one_concurrent_final_output() {
        let output = temp_dir("concurrent-final-publish");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let first_capability = output_capability
            .try_clone()
            .expect("output directory should clone");
        let second_capability = output_capability
            .try_clone()
            .expect("output directory should clone");
        let scratch = temp_dir("concurrent-final-publish-scratch");
        let first = scratch.join("first-output");
        let second = scratch.join("second-output");
        std::fs::write(&first, b"first").expect("first staged output should write");
        std::fs::write(&second, b"second").expect("second staged output should write");
        let target = output.join(VARIABLE_VIEWPORT_GIF_FILENAME);
        let barrier = std::sync::Barrier::new(2);
        let results = std::thread::scope(|scope| {
            let first_worker = scope.spawn(|| {
                barrier.wait();
                publish_scratch_file(
                    &first,
                    &first_capability,
                    VARIABLE_VIEWPORT_GIF_FILENAME,
                    &target,
                )
            });
            let second_worker = scope.spawn(|| {
                barrier.wait();
                publish_scratch_file(
                    &second,
                    &second_capability,
                    VARIABLE_VIEWPORT_GIF_FILENAME,
                    &target,
                )
            });
            [
                first_worker.join().expect("first publisher should finish"),
                second_worker
                    .join()
                    .expect("second publisher should finish"),
            ]
        });
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter(|result| {
                    matches!(result, Err(VariableViewportMotionArtifactError::OccupiedOutputTarget { path }) if path == &target)
                })
                .count(),
            1
        );
        let published = std::fs::read(&target).expect("one output should publish");
        assert!(published == b"first" || published == b"second");
    }

    #[test]
    fn pinned_output_preserves_io_errors() {
        let root = temp_dir("pinned-output-io");
        let output = open_output_directory(&root).expect("output directory should open");
        let staged = root.join("staged-output");
        std::fs::write(&staged, b"staged output").expect("staged output should write");

        assert!(matches!(
            publish_scratch_file(
                &staged,
                &output,
                Path::new("missing-parent").join("output.gif"),
                &root.join("missing-parent").join("output.gif"),
            ),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
        assert!(matches!(
            publish_scratch_file(
                &staged,
                &output,
                "missing-parent/output.gif",
                &root.join("missing-parent").join("output.gif"),
            ),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
    }

    #[test]
    fn staging_and_public_frame_writes_preserve_io_errors() {
        let root = temp_dir("staging-public-frame-io");
        let staging_file = root.join("staging-file");
        std::fs::write(&staging_file, b"not a directory")
            .expect("staging file blocker should write");
        let image = RgbaImage::from_pixel(1, 1, Rgba([1, 2, 3, u8::MAX]));

        assert!(matches!(
            write_staging_frames(&[image], &staging_file),
            Err(MotionArtifactError::Io(_))
        ));

        let scratch = root.join("scratch");
        std::fs::create_dir(&scratch).expect("scratch directory should create");
        let output = root.join("output");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let public_staging = claim_public_staging_directory(&output_capability, &output)
            .expect("public staging directory should claim");

        assert!(matches!(
            publish_public_frames(&scratch, &public_staging, &output, 1),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
        assert!(
            !output
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join("frame-000.png")
                .exists()
        );
    }

    #[test]
    fn public_frame_publisher_preserves_occupied_frame() {
        let root = temp_dir("public-frame-occupied");
        let scratch = root.join("scratch");
        std::fs::create_dir(&scratch).expect("scratch directory should create");
        let scratch_frame = scratch.join("frame-000.png");
        std::fs::write(&scratch_frame, b"normalized frame").expect("scratch frame should write");
        let output = root.join("output");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        let public_staging = claim_public_staging_directory(&output_capability, &output)
            .expect("public staging directory should claim");
        let occupied = output
            .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
            .join("frame-000.png");
        std::fs::write(&occupied, b"existing frame").expect("occupied frame should write");

        assert_eq!(
            publish_public_frames(&scratch, &public_staging, &output, 1),
            Err(VariableViewportMotionArtifactError::OccupiedOutputTarget {
                path: occupied.clone(),
            })
        );
        assert_eq!(
            std::fs::read(&occupied).expect("occupied frame should remain readable"),
            b"existing frame"
        );
        assert_eq!(
            std::fs::read(&scratch_frame).expect("scratch frame should remain readable"),
            b"normalized frame"
        );
    }

    #[test]
    fn scratch_publisher_preserves_missing_source_io_error() {
        let root = temp_dir("scratch-publisher-source-io");
        let output = open_output_directory(&root).expect("output directory should open");
        let missing = root.join("missing-scratch-file");
        let published = root.join("output.gif");

        assert!(matches!(
            publish_scratch_file(&missing, &output, "output.gif", &published),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
        assert!(!published.exists());
    }

    #[test]
    fn variable_viewport_writer_rejects_late_public_artifact_collisions() {
        let _lock = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        for target_name in [
            VARIABLE_VIEWPORT_GIF_FILENAME,
            VARIABLE_VIEWPORT_MP4_FILENAME,
            VARIABLE_VIEWPORT_MANIFEST_FILENAME,
        ] {
            let root = temp_dir(&format!("late-public-{target_name}"));
            let source = root.join("source");
            let output = root.join("output");
            let ffmpeg = root.join("bin");
            std::fs::create_dir_all(&source).expect("source directory should create");
            let target = output.join(target_name);
            install(
                &ffmpeg,
                &FakeFfmpegSpec {
                    dimensions: Some("#dimensions 0:2x2".to_owned()),
                    late_output: Some(target.display().to_string()),
                    ..FakeFfmpegSpec::default()
                },
            );
            let _path = PathEnvGuard::prepend(&ffmpeg);
            let sequence = variable_sequence(&source);

            assert_eq!(
                MotionArtifactWriter::new().write_opaque_variable_viewport(&sequence, &output),
                Err(VariableViewportMotionArtifactError::OccupiedOutputTarget {
                    path: target.clone(),
                })
            );
            assert_eq!(
                std::fs::read(&target).expect("late target should remain readable"),
                b"late public output"
            );
            assert!(load_receipts(sequence.receipts()).is_ok());
        }
    }

    #[cfg(unix)]
    #[test]
    fn public_staging_open_rejects_a_symlink_after_claim() {
        let output = temp_dir("late-staging-symlink");
        let source = temp_dir("late-staging-symlink-source");
        let sequence = variable_sequence(&source);
        let staging = output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY);
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        output_capability
            .create_dir(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
            .expect("staging claim should succeed");
        std::fs::remove_dir(&staging).expect("claimed staging should remove");
        std::os::unix::fs::symlink(&source, &staging).expect("late symlink should create");
        assert!(
            output_capability
                .open_dir_nofollow(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .is_err()
        );
        assert!(load_receipts(sequence.receipts()).is_ok());
    }

    #[test]
    fn output_directory_open_preserves_io_errors() {
        let root = temp_dir("staging-claim-io");
        let output = root.join("not-a-directory");
        std::fs::write(&output, b"file").expect("output blocker should write");
        assert!(matches!(
            open_output_directory(&output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn occupied_output_preflight_preserves_metadata_io_errors() {
        let root = temp_dir("occupied-output-metadata-io");
        let output = root.join("not-a-directory");
        std::fs::write(&output, b"file").expect("output blocker should write");
        let malformed_output = cap_std::fs::Dir::from_std_file(
            std::fs::File::open(&output).expect("output blocker should open"),
        );

        assert!(matches!(
            reject_occupied_output_targets(&malformed_output, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn public_staging_claim_preserves_io_errors_after_pinned_output_removal() {
        let root = temp_dir("removed-pinned-output");
        let output = root.join("output");
        let output_capability =
            open_output_directory(&output).expect("output directory should open");
        std::fs::remove_dir(&output).expect("empty output directory should remove");

        assert!(matches!(
            claim_public_staging_directory(&output_capability, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::Io(_)
            ))
        ));
    }

    #[test]
    fn variable_viewport_writer_preserves_receipts_in_overlapping_staging_directory() {
        let root = temp_dir("overlapping-staging-directory");
        let output = root.join("output");
        let staging = output.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY);
        std::fs::create_dir_all(&staging).expect("staging directory should create");
        let sequence = variable_sequence(&staging);
        let input = sequence
            .receipts()
            .iter()
            .map(|receipt| {
                (
                    receipt.artifact().png_path().to_path_buf(),
                    std::fs::read(receipt.artifact().png_path()).expect("source PNG should read"),
                    receipt.artifact().manifest_path().to_path_buf(),
                    std::fs::read(receipt.artifact().manifest_path())
                        .expect("source provenance should read"),
                )
            })
            .collect::<Vec<_>>();

        let error = MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&sequence, &output)
            .expect_err("overlapping staging directory must reject before writing");
        assert_eq!(
            error,
            VariableViewportMotionArtifactError::OccupiedOutputTarget { path: staging }
        );
        for (png_path, png, manifest_path, manifest) in input {
            assert_eq!(
                std::fs::read(png_path).expect("source PNG should remain readable"),
                png
            );
            assert_eq!(
                std::fs::read(manifest_path).expect("source provenance should remain readable"),
                manifest
            );
        }
        assert!(load_receipts(sequence.receipts()).is_ok());
        for target in [
            VARIABLE_VIEWPORT_GIF_FILENAME,
            VARIABLE_VIEWPORT_MP4_FILENAME,
            VARIABLE_VIEWPORT_MANIFEST_FILENAME,
        ] {
            assert!(
                std::fs::symlink_metadata(output.join(target)).is_err(),
                "rejected export must not create {target}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn variable_viewport_writer_preserves_receipts_when_output_targets_alias_inputs() {
        use std::os::unix::fs::symlink;

        for occupied_target in OccupiedTargetKind::ALL {
            let root = temp_dir(occupied_target.label());
            let source = root.join("source");
            let output = root.join("output");
            std::fs::create_dir_all(&source).expect("source directory should create");
            std::fs::create_dir_all(&output).expect("output directory should create");
            let sequence = variable_sequence(&source);
            let input = sequence
                .receipts()
                .iter()
                .map(|receipt| {
                    (
                        receipt.artifact().png_path().to_path_buf(),
                        std::fs::read(receipt.artifact().png_path())
                            .expect("source PNG should read"),
                        receipt.artifact().manifest_path().to_path_buf(),
                        std::fs::read(receipt.artifact().manifest_path())
                            .expect("source provenance should read"),
                    )
                })
                .collect::<Vec<_>>();
            let occupied = output.join(occupied_target.target());

            match occupied_target {
                OccupiedTargetKind::StagingSymlink => {
                    symlink(&source, &occupied).expect("staging symlink should create");
                }
                OccupiedTargetKind::GifHardlink => {
                    std::fs::hard_link(&input[0].0, &occupied)
                        .expect("GIF hard link should create");
                }
                OccupiedTargetKind::Mp4Symlink => {
                    symlink(&input[0].0, &occupied).expect("MP4 symlink should create");
                }
                OccupiedTargetKind::GifDanglingSymlink => {
                    symlink(source.join("missing.png"), &occupied)
                        .expect("dangling GIF symlink should create");
                }
                OccupiedTargetKind::ManifestHardlink => {
                    std::fs::hard_link(&input[0].2, &occupied)
                        .expect("manifest hard link should create");
                }
            }

            let error = MotionArtifactWriter::new()
                .write_opaque_variable_viewport(&sequence, &output)
                .expect_err("occupied output target must reject before writing");
            assert_eq!(
                error,
                VariableViewportMotionArtifactError::OccupiedOutputTarget {
                    path: occupied.clone(),
                }
            );
            for (png_path, png, manifest_path, manifest) in input {
                assert_eq!(
                    std::fs::read(png_path).expect("source PNG should remain readable"),
                    png
                );
                assert_eq!(
                    std::fs::read(manifest_path).expect("source provenance should remain readable"),
                    manifest
                );
            }
            assert!(
                load_receipts(sequence.receipts()).is_ok(),
                "all receipts must remain valid after the rejected export"
            );
        }
    }

    #[test]
    fn variable_viewport_writer_reports_output_metadata_errors_before_writing() {
        let root = temp_dir("output-metadata-error");
        let source = root.join("source");
        let output = root.join("not-a-directory");
        std::fs::create_dir_all(&source).expect("source directory should create");
        std::fs::write(&output, b"file").expect("output blocker should write");
        let sequence = variable_sequence(&source);

        let error = MotionArtifactWriter::new()
            .write_opaque_variable_viewport(&sequence, &output)
            .expect_err("metadata errors must stop export before writing");
        assert!(matches!(
            error,
            VariableViewportMotionArtifactError::Motion(MotionArtifactError::Io(_))
        ));
        assert!(load_receipts(sequence.receipts()).is_ok());
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
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: Vec::new(),
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    value: None,
                    ..valid_accesskit_text_input_node()
                }],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    bounds: None,
                    ..valid_accesskit_text_input_node()
                }],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    role: AccessKitTextInputRole::Other,
                    ..valid_accesskit_text_input_node()
                }],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    scalar_sequence: vec![0x2b50],
                    ..valid_accesskit_text_input_node()
                }],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    bounds: Some(AccessKitTextInputBounds {
                        x1_bits: 0.0_f64.to_bits(),
                        ..valid_accesskit_text_input_node()
                            .bounds
                            .expect("valid node should define bounds")
                    }),
                    ..valid_accesskit_text_input_node()
                }],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![
                    valid_accesskit_text_input_node(),
                    valid_accesskit_text_input_node(),
                ],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    value: Some("別の入力".to_owned()),
                    scalar_sequence: "別の入力".chars().map(u32::from).collect(),
                    ..valid_accesskit_text_input_node()
                }],
                ..unrelated.clone()
            },
            MotionFrameSemanticEvidence {
                accesskit_text_input_nodes: vec![AccessKitTextInputNode {
                    bounds: Some(AccessKitTextInputBounds {
                        x0_bits: f64::NAN.to_bits(),
                        ..valid_accesskit_text_input_node()
                            .bounds
                            .expect("valid node should define bounds")
                    }),
                    ..valid_accesskit_text_input_node()
                }],
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

    #[cfg(target_os = "linux")]
    #[test]
    fn variable_viewport_writer_rejects_a_non_utf8_ffmpeg_executable_path() {
        use std::os::unix::ffi::OsStringExt;

        let _lock = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = temp_dir("non-utf8-ffmpeg-path");
        let source = root.join("source");
        let output = root.join("output");
        let ffmpeg = root.join(std::ffi::OsString::from_vec(vec![b'b', b'i', b'n', 0xff]));
        std::fs::create_dir_all(&source).expect("source directory should create");
        install(&ffmpeg, &FakeFfmpegSpec::default());
        let _path = PathEnvGuard::prepend(&ffmpeg);
        let sequence = variable_sequence(&source);

        assert!(matches!(
            MotionArtifactWriter::new().write_opaque_variable_viewport(&sequence, &output),
            Err(VariableViewportMotionArtifactError::Motion(
                MotionArtifactError::InvalidSettings
            ))
        ));
        assert!(load_receipts(sequence.receipts()).is_ok());
        for target in [
            VARIABLE_VIEWPORT_GIF_FILENAME,
            VARIABLE_VIEWPORT_MP4_FILENAME,
            VARIABLE_VIEWPORT_MANIFEST_FILENAME,
        ] {
            assert!(
                std::fs::symlink_metadata(output.join(target)).is_err(),
                "non-UTF-8 ffmpeg must reject before public {target} publication"
            );
        }
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
