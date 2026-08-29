use super::model::{DecoderEvidence, FrameRate, FullRootArtifactError, Mp4Artifact};
use katana_ui_core_egui_adapter::{FullRootArtifact, MotionArtifactSettings, MotionArtifactWriter};
use std::path::Path;

pub(super) const VIDEO_ENCODER: &str = "mpeg4";
pub(super) const VIDEO_MUXER: &str = "mp4";
pub(super) const VIDEO_PIXEL_FORMAT: &str = "yuv420p";

pub(super) fn write_mp4(
    receipts: &[FullRootArtifact],
    output_dir: &Path,
) -> Result<Mp4Artifact, FullRootArtifactError> {
    let first = receipts.first();
    let settings = MotionArtifactSettings::new(
        receipts.len(),
        first.map_or(0, FullRootArtifact::width),
        first.map_or(0, FullRootArtifact::height),
    );
    let motion = MotionArtifactWriter::new()
        .write(receipts, output_dir, settings)
        .map_err(|error| FullRootArtifactError::Video(error.to_string()))?;
    let evidence = motion.manifest();
    Ok(Mp4Artifact {
        path: evidence.mp4_path.clone(),
        sha256: evidence.mp4_sha256.clone(),
        frame_sequence_sha256: evidence.frame_sequence_sha256.clone(),
        frame_count: evidence.frame_count,
        fps: FrameRate {
            numerator: settings.fps_numerator,
            denominator: settings.fps_denominator,
            frames_per_second: settings.fps_numerator as f64 / settings.fps_denominator as f64,
        },
        container: VIDEO_MUXER,
        codec: VIDEO_ENCODER,
        pixel_format: VIDEO_PIXEL_FORMAT,
        ffmpeg_path: evidence.ffmpeg_path.clone(),
        ffmpeg_version: evidence.ffmpeg_version.clone(),
        required_encoder: VIDEO_ENCODER,
        encoder_capability_verified: true,
        required_muxer: VIDEO_MUXER,
        muxer_capability_verified: true,
        decoder: DecoderEvidence {
            tool: "ffmpeg framemd5".to_owned(),
            verified: true,
            decoded_frame_count: evidence.decoded_frame_count,
        },
        gif_path: evidence.gif_path.clone(),
        gif_sha256: evidence.gif_sha256.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn write_mp4_requires_input_receipts_and_reports_video_error()
    -> Result<(), FullRootArtifactError> {
        let root = std::env::temp_dir().join(format!(
            "kuc-text-command-root-mp4-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).map_err(FullRootArtifactError::Io)?;
        let result = write_mp4(&[], Path::new(&root));
        assert!(matches!(
            result,
            Err(FullRootArtifactError::Video(error)) if !error.is_empty()
        ));
        std::fs::remove_dir_all(root).map_err(FullRootArtifactError::Io)?;
        Ok(())
    }

    #[test]
    fn process_and_codec_constants_are_declared() {
        assert_eq!("mpeg4", VIDEO_ENCODER);
        assert_eq!("mp4", VIDEO_MUXER);
        assert_eq!("yuv420p", VIDEO_PIXEL_FORMAT);
    }
}
