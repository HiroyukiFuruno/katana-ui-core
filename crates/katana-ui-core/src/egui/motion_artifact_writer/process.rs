use crate::egui::{FullRootArtifact, OpaqueMotionReceiptSequence};
use image::codecs::gif::GifEncoder;
use image::{Delay, Frame, GenericImageView, ImageDecoder, RgbaImage};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

use super::constants::{
    DEFAULT_ENCODER, DEFAULT_GIF_FILENAME, DEFAULT_MANIFEST_FILENAME, DEFAULT_MP4_FILENAME,
    DEFAULT_MUXER, GIF_DELAY_DENOMINATOR_MS, MOTION_SCHEMA,
};
use super::error::MotionArtifactError;
use super::ffmpeg::Ffmpeg;
use super::types::{
    MotionArtifact, MotionArtifactManifest, MotionArtifactSettings, MotionArtifactWriter,
};
use super::validation::{
    expected_stage_name, hash_sha256, io_error, validate_provenance, validate_settings,
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
        let Some(first) = receipts.first() else {
            return Err(MotionArtifactError::EmptySequence);
        };
        let settings = MotionArtifactSettings::new(receipts.len(), first.width(), first.height());
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
        validate_settings(settings)?;
        if receipts.len() != settings.expected_frame_count {
            return Err(MotionArtifactError::FrameCount {
                expected: settings.expected_frame_count,
                actual: receipts.len(),
            });
        }

        std::fs::create_dir_all(output_dir).map_err(io_error)?;
        let mut stages = BTreeSet::new();
        let mut images = Vec::with_capacity(receipts.len());
        let mut frame_sequence = Sha256::new();
        frame_sequence.update((receipts.len() as u64).to_le_bytes());
        let mut root_hashes = Vec::with_capacity(receipts.len());

        for (index, receipt) in receipts.iter().enumerate() {
            if !stages.insert(receipt.stage_id()) {
                return Err(MotionArtifactError::DuplicateStage(
                    receipt.stage_id().to_owned(),
                ));
            }
            let expected_stage = expected_stage_name(index);
            if receipt.stage_id() != expected_stage {
                return Err(MotionArtifactError::StaleStage {
                    expected: expected_stage,
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

            let bytes = std::fs::read(png).map_err(io_error)?;
            if hash_sha256(&bytes) != receipt.png_sha256() {
                return Err(MotionArtifactError::BadPngSha {
                    path: png.to_path_buf(),
                });
            }

            validate_provenance(receipt)?;
            let decoder = image::codecs::png::PngDecoder::new(std::io::Cursor::new(&bytes))
                .map_err(|error| MotionArtifactError::InvalidPng {
                    path: png.to_path_buf(),
                    reason: error.to_string(),
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
            if image.dimensions() != (settings.width, settings.height) {
                return Err(MotionArtifactError::WrongDimensions {
                    path: png.to_path_buf(),
                    expected: (settings.width, settings.height),
                    actual: image.dimensions(),
                });
            }
            let rgba = image.to_rgba8();
            if rgba.pixels().all(|pixel| pixel.0 == [0, 0, 0, 0]) {
                return Err(MotionArtifactError::EmptyPixels(png.to_path_buf()));
            }
            frame_sequence.update((bytes.len() as u64).to_le_bytes());
            frame_sequence.update(&bytes);
            root_hashes.push(receipt.root_record_hash().to_owned());
            images.push(rgba);
        }

        let gif_path = output_dir.join(DEFAULT_GIF_FILENAME);
        write_gif(&images, &gif_path, settings.fps_denominator).map_err(io_error)?;
        let mp4_path = output_dir.join(DEFAULT_MP4_FILENAME);
        let ffmpeg = Ffmpeg::discover()?;
        let source_evidence = ffmpeg.source_evidence(output_dir, receipts.len(), settings)?;
        ffmpeg.encode(&mp4_path, output_dir, receipts.len(), settings)?;
        let decoded_evidence = ffmpeg.decode(&mp4_path)?;
        let decoded_frame_count = decoded_evidence.frame_hashes.len();
        let decoded_width = decoded_evidence.width;
        let decoded_height = decoded_evidence.height;
        if decoded_frame_count != receipts.len()
            || decoded_width != settings.width
            || decoded_height != settings.height
        {
            return Err(MotionArtifactError::Encoder(format!(
                "decoded evidence mismatch: frames={decoded_frame_count}, dimensions={decoded_width}x{decoded_height}"
            )));
        }
        if decoded_evidence.frame_hashes != source_evidence.frame_hashes {
            return Err(MotionArtifactError::Encoder(
                "decoded frame hashes do not match the PNG frame sequence".into(),
            ));
        }

        let manifest_path = output_dir.join(DEFAULT_MANIFEST_FILENAME);
        let mut manifest = MotionArtifactManifest {
            schema: MOTION_SCHEMA,
            frame_count: receipts.len(),
            width: settings.width,
            height: settings.height,
            frame_sequence_sha256: hex::encode(frame_sequence.finalize()),
            gif_path: gif_path.display().to_string(),
            gif_sha256: hash_sha256(&std::fs::read(&gif_path).map_err(io_error)?),
            mp4_path: mp4_path.display().to_string(),
            mp4_sha256: hash_sha256(&std::fs::read(&mp4_path).map_err(io_error)?),
            root_record_hashes: root_hashes,
            ffmpeg_path: ffmpeg.path.display().to_string(),
            ffmpeg_version: ffmpeg.version,
            encoder: DEFAULT_ENCODER,
            muxer: DEFAULT_MUXER,
            decoded_frame_count,
            decoded_width,
            decoded_height,
            source_frame_hashes: source_evidence.frame_hashes,
            decoded_frame_hashes: decoded_evidence.frame_hashes,
            canonical_sha256: String::new(),
        };

        let canonical = serde_json::to_vec(&manifest).map_err(json_error)?;
        manifest.canonical_sha256 = hash_sha256(&canonical);
        let bytes = serde_json::to_vec_pretty(&manifest).map_err(json_error)?;
        std::fs::write(&manifest_path, bytes).map_err(io_error)?;

        Ok(MotionArtifact::from_parts(manifest, manifest_path))
    }
}

fn json_error(error: serde_json::Error) -> MotionArtifactError {
    MotionArtifactError::Json(error.to_string())
}

fn write_gif(images: &[RgbaImage], path: &Path, delay_ms: u32) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut encoder = GifEncoder::new(file);
    let frames = images
        .iter()
        .cloned()
        .map(|image| {
            Frame::from_parts(
                image,
                0,
                0,
                Delay::from_numer_denom_ms(delay_ms, GIF_DELAY_DENOMINATOR_MS),
            )
        })
        .collect::<Vec<_>>();
    encoder.encode_frames(frames).map_err(std::io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::motion_artifact_writer::MotionArtifactError;
    use crate::egui::{FullRootArtifact, OpaqueMotionReceiptSequence, OpaqueRootArtifactReceipt};
    use image::ColorType;
    use image::ImageEncoder;
    use sha2::{Digest, Sha256};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_DIR_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn json_serialization_errors_map_to_the_typed_motion_error() {
        let result = serde_json::from_slice::<serde_json::Value>(b"").map_err(json_error);
        assert!(matches!(result, Err(MotionArtifactError::Json(_))));
    }

    #[derive(Default)]
    struct PathEnvGuard {
        saved: Option<std::ffi::OsString>,
    }

    impl PathEnvGuard {
        fn with_root(root: &std::path::Path) -> Self {
            let saved = std::env::var_os("PATH");
            let existing = saved.as_ref().expect("test process must define PATH");
            let replacement = std::env::join_paths(
                std::iter::once(root.to_path_buf()).chain(std::env::split_paths(existing)),
            )
            .expect("path join should build");
            /* SAFETY: test-local PATH mutation is isolated and restored on Drop. */
            unsafe { std::env::set_var("PATH", replacement) };
            Self { saved }
        }
    }

    impl Drop for PathEnvGuard {
        fn drop(&mut self) {
            if let Some(path) = self.saved.take() {
                /* SAFETY: restore PATH when guard exits to avoid leaking test state. */
                unsafe { std::env::set_var("PATH", path) };
            }
        }
    }

    fn sha256(bytes: &[u8]) -> String {
        hex::encode(Sha256::digest(bytes))
    }

    fn receipt(stage: &str, width: u32, height: u32, rgba: &[u8]) -> FullRootArtifact {
        let mut png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut png)
            .write_image(rgba, width, height, ColorType::Rgba8.into())
            .expect("test png encode");
        let dir = tempfile_dir("motion-process");
        let png_path = dir.join(format!("{stage}.png"));
        let manifest_path = dir.join(format!("{stage}.manifest.json"));
        std::fs::write(&png_path, &png).expect("test png write");
        let record_hash = format!("record-{stage}");
        let pixel_hash = sha256(rgba);
        let manifest = serde_json::json!({
            "width": width,
            "height": height,
            "root_record_hash": record_hash,
            "pixel_hash": pixel_hash,
            "png_sha256": sha256(&png),
        });
        std::fs::write(
            &manifest_path,
            serde_json::to_vec(&manifest).expect("manifest encode"),
        )
        .expect("manifest write");
        FullRootArtifact::from_test_parts(
            stage.to_owned(),
            png_path,
            manifest_path,
            width,
            height,
            record_hash,
            pixel_hash,
            sha256(&png),
        )
    }

    fn receipt_from_png(stage: &str, width: u32, height: u32, png: &[u8]) -> FullRootArtifact {
        let dir = tempfile_dir(stage);
        let png_path = dir.join(format!("{stage}.png"));
        let manifest_path = dir.join(format!("{stage}.manifest.json"));
        std::fs::write(&png_path, png).expect("test png write");
        let record_hash = format!("record-{stage}");
        let pixel_hash = "opaque-pixel-hash".to_owned();
        let png_sha = sha256(png);
        let manifest = serde_json::json!({
            "width": width,
            "height": height,
            "root_record_hash": record_hash,
            "pixel_hash": pixel_hash,
            "png_sha256": png_sha,
        });
        std::fs::write(
            &manifest_path,
            serde_json::to_vec(&manifest).expect("manifest encode"),
        )
        .expect("manifest write");
        FullRootArtifact::from_test_parts(
            stage.to_owned(),
            png_path,
            manifest_path,
            width,
            height,
            record_hash,
            pixel_hash,
            png_sha,
        )
    }

    fn fake_ffmpeg_script(root: &std::path::Path, decode_dimensions: &str) -> PathBuf {
        let path = root.join("ffmpeg");
        let body = format!(
            r#"if [ "$1" = "-version" ]; then
  echo "ffmpeg version 1.0"
elif [ "$1" = "-hide_banner" ] && [ "$3" = "error" ] && [ "$4" = "-encoders" ]; then
  echo " V....  libx264rgb"
elif [ "$1" = "-hide_banner" ] && [ "$3" = "error" ] && [ "$4" = "-formats" ]; then
  echo " E....  mp4"
elif echo "$@" | grep -q framemd5; then
  echo "{decode_dimensions}"
  echo "0, 0, 0, 1, 6, 0123456789abcdef0123456789abcdef"
  echo "0, 1, 1, 1, 6, fedcba9876543210fedcba9876543210"
else
  arg=""
  for token in "$@"; do
    arg="$token"
  done
  printf "motion" > "$arg"
fi
exit 0
"#
        );
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("ffmpeg script should write");
        #[cfg(unix)]
        use std::os::unix::fs::PermissionsExt;
        #[cfg(unix)]
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("ffmpeg script should execute");
        path
    }

    fn with_fake_ffmpeg<R>(decode_dimensions: &str, test: impl FnOnce(&std::path::Path) -> R) -> R {
        let _guard = super::super::TEST_ENV_LOCK.lock().unwrap();
        let root = tempfile_dir("motion-process-ffmpeg");
        let _script = fake_ffmpeg_script(&root, decode_dimensions);
        let _path = PathEnvGuard::with_root(&root);
        test(&root)
    }

    fn with_fake_ffmpeg_hash_series(
        source_hashes: &[&str],
        decoded_hashes: &[&str],
        test: impl FnOnce(&std::path::Path),
    ) {
        let _guard = super::super::TEST_ENV_LOCK.lock().unwrap();
        let root = tempfile_dir("motion-process-ffmpeg-hash-series");
        let path = root.join("ffmpeg");
        let mut source_body = String::new();
        for hash in source_hashes {
            source_body.push_str(&format!("    echo \"{hash}\"\n"));
        }
        let mut decoded_body = String::new();
        for hash in decoded_hashes {
            decoded_body.push_str(&format!("    echo \"{hash}\"\n"));
        }

        let body = [
            "if [ \"$1\" = \"-version\" ]; then\n",
            "  echo \"ffmpeg version 1.0\"\n",
            "elif [ \"$1\" = \"-hide_banner\" ] && [ \"$3\" = \"error\" ] && [ \"$4\" = \"-encoders\" ]; then\n",
            "  echo \" V....  libx264rgb\"\n",
            "elif [ \"$1\" = \"-hide_banner\" ] && [ \"$3\" = \"error\" ] && [ \"$4\" = \"-formats\" ]; then\n",
            "  echo \" E....  mp4\"\n",
            "elif echo \"$@\" | grep -q framemd5; then\n",
            "  if echo \"$@\" | grep -q -- \"-start_number\"; then\n",
            "    echo \"#dimensions 0:2x1\"\n",
            "{source_body}",
            "  else\n",
            "    echo \"#dimensions 0:2x1\"\n",
            "{decoded_body}",
            "  fi\n",
            "else\n",
            "  arg=\"\"\n",
            "  for token in \"$@\"; do\n",
            "    arg=\"$token\"\n",
            "  done\n",
            "  printf \"motion\" > \"$arg\"\n",
            "fi\n",
            "exit 0\n",
        ]
        .concat();
        let body = body
            .replace("{source_body}", &source_body)
            .replace("{decoded_body}", &decoded_body);
        std::fs::write(&path, format!("#!/bin/sh\n{body}")).expect("ffmpeg script should write");
        #[cfg(unix)]
        use std::os::unix::fs::PermissionsExt;
        #[cfg(unix)]
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("ffmpeg script should execute");
        let _path = PathEnvGuard::with_root(&root);
        test(&root)
    }

    #[test]
    fn write_rejects_mismatched_decoded_frame_hashes() {
        let first = receipt("frame-000", 2, 1, &[255, 0, 0, 255, 0, 255, 0, 255]);
        let second = receipt("frame-001", 2, 1, &[0, 255, 0, 255, 255, 0, 0, 255]);
        with_fake_ffmpeg_hash_series(
            &[
                "0, 0, 0, 1, 6, 0123456789abcdef0123456789abcdef",
                "0, 0, 1, 1, 6, fedcba9876543210fedcba9876543210",
            ],
            &[
                "0, 0, 0, 1, 6, 0123456789abcdef0123456789abcdef",
                "0, 1, 1, 1, 6, 00112233445566778899aabbccddeeff",
            ],
            |output| {
                let error = MotionArtifactWriter::new()
                    .write(
                        &[first, second],
                        output,
                        MotionArtifactSettings::new(2, 2, 1),
                    )
                    .expect_err("frame hashes should mismatch between source and decode");
                assert!(
                    matches!(
                        &error,
                        MotionArtifactError::Encoder(message)
                            if message.contains("decoded frame hashes do not match the PNG frame sequence")
                    ),
                    "unexpected motion artifact error: {error:?}"
                );
            },
        );
    }

    #[test]
    fn write_success_generates_motion_artifact() {
        let first = receipt("frame-000", 2, 1, &[255, 0, 0, 255, 0, 255, 0, 255]);
        let second = receipt("frame-001", 2, 1, &[0, 255, 0, 255, 255, 0, 0, 255]);
        with_fake_ffmpeg("#dimensions 0:2x1", |output| {
            let artifact = MotionArtifactWriter::new()
                .write(
                    &[first.clone(), second.clone()],
                    output,
                    MotionArtifactSettings::new(2, 2, 1),
                )
                .expect("motion write should pass");
            assert_eq!(artifact.manifest().frame_count, 2);
            assert_eq!(artifact.manifest().decoded_frame_count, 2);
            assert_eq!(artifact.manifest().decoded_width, 2);
            assert_eq!(artifact.manifest().decoded_height, 1);
            assert!(artifact.manifest_path().is_file());
        });
    }

    #[test]
    fn write_opaque_success_and_frame_count_guard() {
        let first = receipt("frame-000", 2, 1, &[255, 0, 0, 255, 0, 255, 0, 255]);
        let second = receipt("frame-001", 2, 1, &[0, 255, 0, 255, 255, 0, 0, 255]);
        let mut sequence = OpaqueMotionReceiptSequence::new();
        assert!(
            sequence
                .push("frame-000", OpaqueRootArtifactReceipt::from(first))
                .is_ok()
        );
        assert!(
            sequence
                .push("frame-001", OpaqueRootArtifactReceipt::from(second))
                .is_ok()
        );
        with_fake_ffmpeg("#dimensions 0:2x1", |output| {
            let artifact = MotionArtifactWriter::new()
                .write_opaque(&sequence, output)
                .expect("opaque motion should pass");
            assert_eq!(artifact.manifest().frame_count, 2);
        });

        let empty = OpaqueMotionReceiptSequence::new();
        assert!(matches!(
            MotionArtifactWriter::new().write_opaque(&empty, std::env::temp_dir().as_path()),
            Err(MotionArtifactError::EmptySequence)
        ));

        let one = receipt("frame-000", 2, 1, &[255, 0, 0, 255, 0, 255, 0, 255]);
        assert!(matches!(
            MotionArtifactWriter::new().write_full_root(
                std::slice::from_ref(&one),
                std::env::temp_dir().as_path(),
                MotionArtifactSettings::new(2, 2, 1),
            ),
            Err(MotionArtifactError::FrameCount {
                expected: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn write_rejects_invalid_decode_evidence() {
        let frame = receipt("frame-000", 2, 1, &[255, 0, 0, 255, 0, 255, 0, 255]);
        with_fake_ffmpeg("#dimensions 0:1x1", |output| {
            let error = MotionArtifactWriter::new()
                .write(
                    std::slice::from_ref(&frame),
                    output,
                    MotionArtifactSettings::new(1, 2, 1),
                )
                .expect_err("dimension mismatch should reject");
            assert!(matches!(error, MotionArtifactError::Encoder(_)));
        });
    }

    #[test]
    fn write_rejects_invalid_and_non_rgba_png_payloads() {
        let invalid = receipt_from_png("frame-000", 1, 1, b"not-a-png");
        let output = tempfile_dir("motion-invalid-png-output");
        assert!(matches!(
            MotionArtifactWriter::new().write_full_root(
                std::slice::from_ref(&invalid),
                &output,
                MotionArtifactSettings::new(1, 1, 1),
            ),
            Err(MotionArtifactError::InvalidPng { .. })
        ));

        let mut rgb_png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut rgb_png)
            .write_image(&[255, 0, 0], 1, 1, ColorType::Rgb8.into())
            .expect("RGB test png encode");
        let rgb = receipt_from_png("frame-000", 1, 1, &rgb_png);
        assert!(matches!(
            MotionArtifactWriter::new().write_full_root(
                std::slice::from_ref(&rgb),
                &output,
                MotionArtifactSettings::new(1, 1, 1),
            ),
            Err(MotionArtifactError::InvalidPng { .. })
        ));

        let mut corrupt_rgba_png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut corrupt_rgba_png)
            .write_image(&[255, 0, 0, 255], 1, 1, ColorType::Rgba8.into())
            .expect("RGBA test png encode");
        let idat = corrupt_rgba_png
            .windows(4)
            .position(|window| window == b"IDAT")
            .expect("encoded PNG contains IDAT");
        corrupt_rgba_png[idat + 4] ^= 0xff;
        let corrupt = receipt_from_png("frame-000", 1, 1, &corrupt_rgba_png);
        assert!(matches!(
            MotionArtifactWriter::new().write_full_root(
                std::slice::from_ref(&corrupt),
                &output,
                MotionArtifactSettings::new(1, 1, 1),
            ),
            Err(MotionArtifactError::InvalidPng { .. })
        ));
    }

    fn tempfile_dir(label: &str) -> PathBuf {
        let sequence = TEMP_DIR_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kuc-motion-process-{}-{}-{}",
            label,
            std::process::id(),
            sequence
        ));
        std::fs::create_dir_all(&path).expect("temp directory should create");
        path
    }
}
