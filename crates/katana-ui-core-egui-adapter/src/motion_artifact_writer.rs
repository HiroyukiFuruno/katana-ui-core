use crate::{FullRootArtifact, OpaqueMotionReceiptSequence};
use image::{GenericImageView, ImageDecoder};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Cursor;
use std::path::Path;

mod ffmpeg;
mod support;
mod types;

use ffmpeg::{DEFAULT_ENCODER, DEFAULT_MUXER, Ffmpeg};
use support::MotionSupport;
pub use types::{
    MotionArtifact, MotionArtifactError, MotionArtifactManifest, MotionArtifactSettings,
    MotionArtifactWriter, MotionSourceArtifact,
};

impl MotionArtifactWriter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn write(
        &self,
        receipts: &[FullRootArtifact],
        output_dir: &Path,
        settings: MotionArtifactSettings,
    ) -> Result<MotionArtifact, MotionArtifactError> {
        self.write_full_root(receipts, output_dir, settings)
    }

    pub fn write_opaque(
        &self,
        sequence: &OpaqueMotionReceiptSequence,
        output_dir: &Path,
    ) -> Result<MotionArtifact, MotionArtifactError> {
        let receipts = sequence
            .receipts()
            .iter()
            .map(|receipt| receipt.artifact().clone())
            .collect::<Vec<_>>();
        if receipts.is_empty() {
            return Err(MotionArtifactError::EmptySequence);
        }
        let (width, height) = receipts.iter().fold((0, 0), |(width, height), receipt| {
            (width.max(receipt.width()), height.max(receipt.height()))
        });
        let settings = MotionArtifactSettings::new(receipts.len(), width, height);
        self.write_full_root(&receipts, output_dir, settings)
    }

    pub(crate) fn write_full_root(
        &self,
        receipts: &[FullRootArtifact],
        output_dir: &Path,
        settings: MotionArtifactSettings,
    ) -> Result<MotionArtifact, MotionArtifactError> {
        if receipts.is_empty() {
            return Err(MotionArtifactError::EmptySequence);
        }
        MotionSupport::validate_settings(settings)?;
        if receipts.len() != settings.expected_frame_count {
            return Err(MotionArtifactError::FrameCount {
                expected: settings.expected_frame_count,
                actual: receipts.len(),
            });
        }
        fs::create_dir_all(output_dir).map_err(MotionSupport::io_error)?;
        let mut stages = std::collections::BTreeSet::new();
        let staging_dir = output_dir.join(".kuc-motion-frames");
        fs::create_dir_all(&staging_dir).map_err(MotionSupport::io_error)?;
        let mut frame_sequence = Sha256::new();
        frame_sequence.update((receipts.len() as u64).to_le_bytes());
        let mut root_hashes = Vec::with_capacity(receipts.len());
        let mut source_artifacts = Vec::with_capacity(receipts.len());
        for (index, receipt) in receipts.iter().enumerate() {
            if !stages.insert(receipt.stage_id()) {
                return Err(MotionArtifactError::DuplicateStage(
                    receipt.stage_id().to_owned(),
                ));
            }
            let expected = format!("frame-{index:03}");
            if receipt.stage_id() != expected {
                return Err(MotionArtifactError::StaleStage {
                    expected,
                    actual: receipt.stage_id().to_owned(),
                });
            }
            let png = receipt.png_path();
            if !png.is_file() {
                return Err(MotionArtifactError::MissingPng(png.to_path_buf()));
            }
            if !receipt.manifest_path().is_file() {
                return Err(MotionArtifactError::MissingProvenance(
                    receipt.manifest_path().to_path_buf(),
                ));
            }
            let bytes = fs::read(png).map_err(MotionSupport::io_error)?;
            if MotionSupport::sha256(&bytes) != receipt.png_sha256() {
                return Err(MotionArtifactError::BadPngSha {
                    path: png.to_path_buf(),
                });
            }
            MotionSupport::validate_provenance(receipt)?;
            let decoder =
                image::codecs::png::PngDecoder::new(Cursor::new(&bytes)).map_err(|error| {
                    MotionArtifactError::InvalidPng {
                        path: png.to_path_buf(),
                        reason: error.to_string(),
                    }
                })?;
            if decoder.color_type() != image::ColorType::Rgba8 {
                return Err(MotionArtifactError::InvalidPng {
                    path: png.to_path_buf(),
                    reason: "PNG color type is not RGBA8".into(),
                });
            }
            let image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)
                .map_err(|error| MotionArtifactError::InvalidPng {
                    path: png.to_path_buf(),
                    reason: error.to_string(),
                })?;
            if image.dimensions() != (receipt.width(), receipt.height()) {
                return Err(MotionArtifactError::WrongDimensions {
                    path: png.to_path_buf(),
                    expected: (receipt.width(), receipt.height()),
                    actual: image.dimensions(),
                });
            }
            if receipt.width() > settings.width || receipt.height() > settings.height {
                return Err(MotionArtifactError::SourceCanvasExceedsCanonical {
                    path: png.to_path_buf(),
                    source: image.dimensions(),
                    canonical: (settings.width, settings.height),
                });
            }
            let rgba = image.to_rgba8();
            if rgba.pixels().all(|pixel| pixel.0 == [0, 0, 0, 0]) {
                return Err(MotionArtifactError::EmptyPixels(png.to_path_buf()));
            }
            let mut canonical = image::RgbaImage::from_pixel(
                settings.width,
                settings.height,
                image::Rgba([0, 0, 0, 0]),
            );
            image::imageops::overlay(&mut canonical, &rgba, 0, 0);
            let canonical_path = staging_dir.join(format!("frame-{index:03}.png"));
            let canonical_bytes = MotionSupport::encode_rgba_png(&canonical)?;
            fs::write(&canonical_path, &canonical_bytes).map_err(MotionSupport::io_error)?;
            frame_sequence.update((canonical_bytes.len() as u64).to_le_bytes());
            frame_sequence.update(&canonical_bytes);
            root_hashes.push(receipt.root_record_hash().to_owned());
            let provenance_bytes =
                fs::read(receipt.manifest_path()).map_err(MotionSupport::io_error)?;
            source_artifacts.push(MotionSourceArtifact {
                stage_id: receipt.stage_id().to_owned(),
                png_path: png.display().to_string(),
                provenance_path: receipt.manifest_path().display().to_string(),
                provenance_sha256: MotionSupport::sha256(&provenance_bytes),
                width: receipt.width(),
                height: receipt.height(),
                root_record_hash: receipt.root_record_hash().to_owned(),
                pixel_hash: receipt.pixel_hash().to_owned(),
                png_sha256: receipt.png_sha256().to_owned(),
            });
        }
        let gif_path = output_dir.join("motion.gif");
        let ffmpeg = Ffmpeg::discover()?;
        ffmpeg.encode_gif(&gif_path, &staging_dir, receipts.len(), settings)?;
        let (gif_frame_count, gif_width, gif_height) = ffmpeg.decode(&gif_path)?;
        if gif_frame_count != receipts.len()
            || gif_width != settings.width
            || gif_height != settings.height
        {
            return Err(MotionArtifactError::Encoder(format!(
                "GIF decoded evidence mismatch: frames={gif_frame_count}, dimensions={gif_width}x{gif_height}"
            )));
        }
        let mp4_path = output_dir.join("motion.mp4");
        ffmpeg.encode(&mp4_path, &staging_dir, receipts.len(), settings)?;
        let (decoded_frame_count, decoded_width, decoded_height) = ffmpeg.decode(&mp4_path)?;
        if decoded_frame_count != receipts.len()
            || decoded_width != settings.width
            || decoded_height != settings.height
        {
            return Err(MotionArtifactError::Encoder(format!(
                "decoded evidence mismatch: frames={decoded_frame_count}, dimensions={decoded_width}x{decoded_height}"
            )));
        }
        let manifest_path = output_dir.join("motion-manifest.json");
        let mut manifest = MotionArtifactManifest {
            schema: "kuc.retained-root-motion.v1",
            frame_count: receipts.len(),
            width: settings.width,
            height: settings.height,
            frame_sequence_sha256: format!("{:x}", frame_sequence.finalize()),
            gif_path: gif_path.display().to_string(),
            gif_sha256: MotionSupport::sha256(
                &fs::read(&gif_path).map_err(MotionSupport::io_error)?,
            ),
            mp4_path: mp4_path.display().to_string(),
            mp4_sha256: MotionSupport::sha256(
                &fs::read(&mp4_path).map_err(MotionSupport::io_error)?,
            ),
            root_record_hashes: root_hashes,
            ffmpeg_path: ffmpeg.path.display().to_string(),
            ffmpeg_version: ffmpeg.version,
            encoder: DEFAULT_ENCODER,
            muxer: DEFAULT_MUXER,
            source_artifacts,
            decoded_frame_count,
            decoded_width,
            decoded_height,
            canonical_sha256: String::new(),
        };
        let canonical = serde_json::to_vec(&manifest)
            .map_err(|error| MotionArtifactError::Json(error.to_string()))?;
        manifest.canonical_sha256 = MotionSupport::sha256(&canonical);
        let bytes = serde_json::to_vec_pretty(&manifest)
            .map_err(|error| MotionArtifactError::Json(error.to_string()))?;
        fs::write(&manifest_path, bytes).map_err(MotionSupport::io_error)?;
        Ok(MotionArtifact {
            manifest,
            manifest_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FullRootArtifact;
    use image::codecs::png::PngEncoder;
    use image::{ColorType, ImageEncoder};
    use std::path::PathBuf;
    fn sha256(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    fn receipt(root: &Path, stage: &str, width: u32, height: u32, rgba: &[u8]) -> FullRootArtifact {
        let mut png = Vec::new();
        PngEncoder::new(&mut png)
            .write_image(rgba, width, height, ColorType::Rgba8.into())
            .expect("test PNG should encode");
        let png_path = root.join(format!("{stage}.png"));
        let manifest_path = root.join(format!("{stage}.manifest.json"));
        fs::write(&png_path, &png).expect("test PNG should write");
        let root_record_hash = format!("record-{stage}");
        let provenance = serde_json::json!({
            "width": width,
            "height": height,
            "root_record_hash": root_record_hash,
            "pixel_hash": sha256(rgba),
            "png_sha256": sha256(&png),
        });
        fs::write(
            &manifest_path,
            serde_json::to_vec(&provenance).expect("test provenance should encode"),
        )
        .expect("test provenance should write");
        FullRootArtifact::from_test_parts(
            stage.to_owned(),
            png_path,
            manifest_path,
            width,
            height,
            format!("record-{stage}"),
            sha256(rgba),
            sha256(&png),
        )
    }

    fn one_receipt(root: &Path) -> FullRootArtifact {
        receipt(root, "frame-000", 2, 1, &[255, 0, 0, 255, 0, 0, 0, 255])
    }

    fn writer_error(receipts: &[FullRootArtifact], root: &Path) -> MotionArtifactError {
        MotionArtifactWriter::new()
            .write(
                receipts,
                root,
                MotionArtifactSettings::new(receipts.len(), 2, 1),
            )
            .expect_err("invalid receipt must be rejected")
    }

    #[test]
    fn rejects_empty_sequence() {
        let root = tempfile_dir("empty");
        let error = writer_error(&[], &root);
        assert_eq!(error, MotionArtifactError::EmptySequence);
    }

    #[test]
    fn rejects_duplicate_stage() {
        let root = tempfile_dir("duplicate");
        let first = one_receipt(&root);
        let second_root = root.join("second");
        fs::create_dir_all(&second_root).expect("second test directory should create");
        let second = receipt(
            &second_root,
            "frame-000",
            2,
            1,
            &[0, 255, 0, 255, 0, 0, 0, 255],
        );
        let error = writer_error(&[first, second], &root);
        assert!(
            matches!(error, MotionArtifactError::DuplicateStage(_)),
            "{error:?}"
        );
    }

    #[test]
    fn rejects_stale_stage() {
        let root = tempfile_dir("stale");
        let stale = receipt(&root, "frame-001", 2, 1, &[255, 0, 0, 255, 0, 0, 0, 255]);
        assert!(matches!(
            writer_error(&[stale], &root),
            MotionArtifactError::StaleStage { .. }
        ));
    }

    #[test]
    fn rejects_missing_png_and_provenance() {
        let root = tempfile_dir("missing");
        let missing_png = one_receipt(&root);
        fs::remove_file(missing_png.png_path()).expect("test PNG should remove");
        assert!(matches!(
            writer_error(&[missing_png], &root),
            MotionArtifactError::MissingPng(_)
        ));
        let missing_provenance = one_receipt(&root);
        fs::remove_file(missing_provenance.manifest_path()).expect("test provenance should remove");
        assert!(matches!(
            writer_error(&[missing_provenance], &root),
            MotionArtifactError::MissingProvenance(_)
        ));
    }

    #[test]
    fn rejects_bad_sha_wrong_dimension_and_empty_pixels() {
        let root = tempfile_dir("content");
        let mut bad_sha = one_receipt(&root);
        bad_sha = FullRootArtifact::from_test_parts(
            bad_sha.stage_id().to_owned(),
            bad_sha.png_path().to_owned(),
            bad_sha.manifest_path().to_owned(),
            2,
            1,
            bad_sha.root_record_hash().to_owned(),
            bad_sha.pixel_hash().to_owned(),
            "bad".into(),
        );
        assert!(matches!(
            writer_error(&[bad_sha], &root),
            MotionArtifactError::BadPngSha { .. }
        ));
        let wrong = receipt(&root, "frame-000", 1, 2, &[255, 0, 0, 255, 0, 0, 0, 255]);
        assert!(matches!(
            writer_error(&[wrong], &root),
            MotionArtifactError::SourceCanvasExceedsCanonical { .. }
        ));
        let empty = receipt(&root, "frame-000", 2, 1, &[0; 8]);
        assert!(matches!(
            writer_error(&[empty], &root),
            MotionArtifactError::EmptyPixels(_)
        ));
    }

    #[test]
    fn writes_gif_mp4_and_canonical_manifest_with_decoded_evidence() {
        let root = tempfile_dir("success");
        let first = receipt(&root, "frame-000", 2, 1, &[255, 0, 0, 255, 0, 0, 0, 255]);
        let second = receipt(&root, "frame-001", 2, 1, &[0, 255, 0, 255, 0, 0, 0, 255]);
        let artifact = MotionArtifactWriter::new()
            .write(
                &[first, second],
                &root,
                MotionArtifactSettings::new(2, 2, 1),
            )
            .expect("real ffmpeg should produce the retained motion artifact");
        let manifest = artifact.manifest();
        assert_eq!(manifest.frame_count, 2);
        assert_eq!(manifest.decoded_frame_count, 2);
        assert_eq!((manifest.decoded_width, manifest.decoded_height), (2, 1));
        assert_eq!(manifest.source_artifacts.len(), 2);
        assert!(Path::new(&manifest.gif_path).is_file());
        assert!(Path::new(&manifest.mp4_path).is_file());
        let mut canonical = manifest.clone();
        canonical.canonical_sha256.clear();
        let canonical_bytes =
            serde_json::to_vec(&canonical).expect("manifest should have canonical JSON");
        assert_eq!(manifest.canonical_sha256, sha256(&canonical_bytes));
    }

    #[test]
    fn normalizes_mixed_source_canvases_without_overwriting_inputs() {
        let root = tempfile_dir("mixed-canvases");
        let first = receipt(&root, "frame-000", 2, 1, &[255, 0, 0, 255, 0, 0, 0, 255]);
        let second_root = root.join("small");
        fs::create_dir_all(&second_root).expect("small source directory should create");
        let second = receipt(&second_root, "frame-001", 1, 1, &[0, 255, 0, 255]);
        let first_bytes = fs::read(first.png_path()).expect("first source should read");
        let second_bytes = fs::read(second.png_path()).expect("second source should read");

        let mut sequence = OpaqueMotionReceiptSequence::new();
        sequence
            .push("frame-000", first.clone().into())
            .expect("first receipt should be ordered");
        sequence
            .push("frame-001", second.clone().into())
            .expect("second receipt should be ordered");
        let artifact = MotionArtifactWriter::new()
            .write_opaque(&sequence, &root)
            .expect("mixed source canvases should be padded by the KUC writer");
        let manifest = artifact.manifest();
        assert_eq!((manifest.width, manifest.height), (2, 1));
        assert_eq!((manifest.decoded_width, manifest.decoded_height), (2, 1));
        assert_eq!(manifest.source_artifacts[1].width, 1);
        assert_eq!(manifest.source_artifacts[1].height, 1);
        assert_eq!(
            fs::read(first.png_path()).expect("first source should remain"),
            first_bytes
        );
        assert_eq!(
            fs::read(second.png_path()).expect("second source should remain"),
            second_bytes
        );

        let padded = image::open(root.join(".kuc-motion-frames/frame-001.png"))
            .expect("canonical padded frame should exist")
            .to_rgba8();
        assert_eq!(padded.dimensions(), (2, 1));
        assert_eq!(padded.get_pixel(0, 0).0, [0, 255, 0, 255]);
        assert_eq!(padded.get_pixel(1, 0).0, [0, 0, 0, 0]);
    }

    #[cfg(unix)]
    #[test]
    fn gif_encode_uses_ffmpeg_frame_sequence_contract() {
        let root = tempfile_dir("gif-encode-contract");
        let script = root.join("ffmpeg");
        let log = root.join("args.log");
        let script_body = format!(
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nprintf x > '{}'\n",
            log.display(),
            root.join("motion.gif").display()
        );
        fs::write(&script, script_body).expect("fake ffmpeg should write");
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
            .expect("fake ffmpeg should be executable");

        let output = root.join("motion.gif");
        Ffmpeg {
            path: script,
            version: "test".into(),
        }
        .encode_gif(
            &output,
            &root,
            36,
            MotionArtifactSettings::new(36, 1280, 720),
        )
        .expect("GIF encode command should succeed");

        let args = fs::read_to_string(log).expect("fake ffmpeg should log arguments");
        assert!(args.lines().any(|line| line == "-c:v"));
        assert!(args.lines().any(|line| line == "gif"));
        assert!(args.lines().any(|line| line == "-gifflags"));
        assert!(args.lines().any(|line| line == "-transdiff"));
        assert!(args.lines().any(|line| line == "-f"));
        assert!(args.lines().any(|line| line == "-frames:v"));
        assert!(args.lines().any(|line| line == "36"));
        assert!(args.lines().any(|line| line == "1000/180"));
        assert!(args.lines().any(|line| line.ends_with("frame-%03d.png")));
        assert!(output.is_file());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_missing_encoder_and_decode_failure() {
        let missing = Ffmpeg::discover_at(Path::new("/definitely-missing/ffmpeg"))
            .expect_err("missing ffmpeg must fail");
        assert!(matches!(missing, MotionArtifactError::Encoder(_)));
        let root = tempfile_dir("decode");
        let script = root.join("ffmpeg");
        fs::write(&script, "#!/bin/sh\nexit 17\n").expect("fake ffmpeg should write");
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
            .expect("fake ffmpeg should be executable");
        let decode = (Ffmpeg {
            path: script,
            version: "test".into(),
        })
        .decode(&root.join("missing.mp4"));
        assert!(matches!(decode, Err(MotionArtifactError::Encoder(_))));
    }

    fn tempfile_dir(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("kuc-motion-{label}-{}", std::process::id()));
        fs::create_dir_all(&path).expect("test directory should create");
        path
    }
}
