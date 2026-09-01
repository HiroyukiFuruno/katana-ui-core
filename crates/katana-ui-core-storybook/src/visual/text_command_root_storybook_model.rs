pub use super::error::FullRootArtifactError;
use super::{PAGE, ROOT_IDENTITY};
use katana_ui_core::egui::{
    FullRootArtifact, text_command_surface::EguiTextCommandSurfaceHostRootFrame,
};
use serde::Serialize;
use std::path::Path;

#[derive(Debug)]
pub(super) struct FullRootStep {
    pub(super) name: &'static str,
    pub(super) input: Vec<&'static str>,
    pub(super) evidence: RootEvidence,
    pub(super) frame: EguiTextCommandSurfaceHostRootFrame,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct RootEvidence {
    pub(super) identity: String,
    pub(super) state_revision: u64,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba_sha256: String,
    pub(super) plan_sha256: String,
    pub(super) record_sha256: String,
    pub(super) accesskit_snapshot_sha256: String,
    pub(super) event_receipt: EventReceiptEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct EventReceiptEvidence {
    pub(super) root_identity: String,
    pub(super) state_revision: u64,
    pub(super) correlation_fingerprint: String,
    pub(super) event_batch_fingerprint: String,
    pub(super) consumed_once: bool,
    pub(super) event_cardinality: usize,
    pub(super) forwarder_calls: usize,
}

#[derive(Debug, Serialize)]
pub(super) struct FullRootManifest {
    pub(super) schema: &'static str,
    pub(super) schema_version: u32,
    pub(super) page: &'static str,
    pub(super) root_identity: &'static str,
    pub(super) execution: PlatformExecutionEvidence,
    pub(super) proof_scope: ProofScope,
    pub(super) frames: Vec<FullRootManifestFrame>,
    pub(super) gif_path: String,
    pub(super) gif_sha256: String,
    pub(super) mp4: Mp4ArtifactEvidence,
}

#[derive(Debug, Serialize)]
pub(super) struct Mp4ArtifactEvidence {
    pub(super) path: String,
    pub(super) sha256: String,
    pub(super) frame_sequence_sha256: String,
    pub(super) frame_count: usize,
    pub(super) fps: FrameRate,
    pub(super) container: &'static str,
    pub(super) codec: &'static str,
    pub(super) pixel_format: &'static str,
    pub(super) ffmpeg_path: String,
    pub(super) ffmpeg_version: String,
    pub(super) required_encoder: &'static str,
    pub(super) encoder_capability_verified: bool,
    pub(super) required_muxer: &'static str,
    pub(super) muxer_capability_verified: bool,
    pub(super) decoder: DecoderEvidence,
}

#[derive(Debug, Serialize)]
pub(super) struct FrameRate {
    pub(super) numerator: u32,
    pub(super) denominator: u32,
    pub(super) frames_per_second: f64,
}

#[derive(Debug, Serialize)]
pub(super) struct DecoderEvidence {
    pub(super) tool: String,
    pub(super) verified: bool,
    pub(super) decoded_frame_count: usize,
    pub(super) source_frame_hashes: Vec<String>,
    pub(super) decoded_frame_hashes: Vec<String>,
}

#[derive(Debug)]
pub(super) struct Mp4Artifact {
    pub(super) path: String,
    pub(super) sha256: String,
    pub(super) frame_sequence_sha256: String,
    pub(super) frame_count: usize,
    pub(super) fps: FrameRate,
    pub(super) container: &'static str,
    pub(super) codec: &'static str,
    pub(super) pixel_format: &'static str,
    pub(super) ffmpeg_path: String,
    pub(super) ffmpeg_version: String,
    pub(super) required_encoder: &'static str,
    pub(super) encoder_capability_verified: bool,
    pub(super) required_muxer: &'static str,
    pub(super) muxer_capability_verified: bool,
    pub(super) decoder: DecoderEvidence,
    pub(super) gif_path: String,
    pub(super) gif_sha256: String,
}

#[derive(Debug, Serialize)]
pub(super) struct PlatformExecutionEvidence {
    pub(super) current_os: &'static str,
    pub(super) executed_profiles: Vec<String>,
    pub(super) unavailable_profiles: Vec<UnavailableProfile>,
}

#[derive(Debug, Serialize)]
pub(super) struct UnavailableProfile {
    pub(super) profile: &'static str,
    pub(super) status: &'static str,
    pub(super) reason: &'static str,
}

#[derive(Debug, Serialize)]
pub(super) struct ProofScope {
    pub(super) proves: Vec<&'static str>,
    pub(super) does_not_prove: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
pub(super) struct FullRootManifestFrame {
    pub(super) index: usize,
    pub(super) name: &'static str,
    pub(super) input: Vec<&'static str>,
    pub(super) png_path: String,
    pub(super) evidence: RootEvidence,
}

pub(super) struct FullRootSequence {
    pub(super) steps: Vec<FullRootStep>,
}

impl FullRootManifest {
    pub(super) fn from_sequence(
        sequence: &FullRootSequence,
        receipts: &[FullRootArtifact],
        video: &Mp4Artifact,
    ) -> Self {
        let current = std::env::consts::OS;
        let profiles = ["macos", "windows", "linux"];
        let executed_profiles = profiles
            .iter()
            .filter(|profile| **profile == current_profile(current))
            .map(|profile| (*profile).to_string())
            .collect();
        let unavailable_profiles = profiles
            .iter()
            .filter(|profile| **profile != current_profile(current))
            .map(|profile| UnavailableProfile {
                profile,
                status: "typed unavailable",
                reason: "this artifact was executed on one OS only; no cross-OS result is inferred",
            })
            .collect();
        Self {
            schema: "kuc.text-command-root-storybook",
            schema_version: 2,
            page: PAGE,
            root_identity: ROOT_IDENTITY,
            execution: PlatformExecutionEvidence {
                current_os: current,
                executed_profiles,
                unavailable_profiles,
            },
            proof_scope: ProofScope {
                proves: vec![
                    "one retained KUC host root facade is reused for every RawInput step",
                    "final root RGBA, paint-plan, record, and AccessKit hashes are recorded per step",
                    "each root event batch is forwarded once and receipt fingerprints are recorded",
                    "the same root path renders Japanese, exact ⭐️ VS16, ZWJ, gutter, annotation, toolbar, context menu, and search controls",
                    "the MP4 is encoded from the same root-owned RGBA frame sequence and decoded back with the required frame count",
                ],
                does_not_prove: vec![
                    "KLE or KatanA host semantic actions, parser behavior, or document mutation",
                    "execution on an OS listed as typed unavailable",
                    "actual KatanA host E2E, even though the closed KUC root event receipts are recorded",
                ],
            },
            frames: sequence
                .steps
                .iter()
                .zip(receipts)
                .enumerate()
                .map(|(index, (step, receipt))| FullRootManifestFrame {
                    index,
                    name: step.name,
                    input: step.input.clone(),
                    png_path: absolute_path(receipt.png_path()),
                    evidence: step.evidence.clone(),
                })
                .collect(),
            gif_path: video.gif_path.clone(),
            gif_sha256: video.gif_sha256.clone(),
            mp4: Mp4ArtifactEvidence {
                path: video.path.clone(),
                sha256: video.sha256.clone(),
                frame_sequence_sha256: video.frame_sequence_sha256.clone(),
                frame_count: video.frame_count,
                fps: FrameRate {
                    numerator: video.fps.numerator,
                    denominator: video.fps.denominator,
                    frames_per_second: video.fps.frames_per_second,
                },
                container: video.container,
                codec: video.codec,
                pixel_format: video.pixel_format,
                ffmpeg_path: video.ffmpeg_path.clone(),
                ffmpeg_version: video.ffmpeg_version.clone(),
                required_encoder: video.required_encoder,
                encoder_capability_verified: video.encoder_capability_verified,
                required_muxer: video.required_muxer,
                muxer_capability_verified: video.muxer_capability_verified,
                decoder: DecoderEvidence {
                    tool: video.decoder.tool.clone(),
                    verified: video.decoder.verified,
                    decoded_frame_count: video.decoder.decoded_frame_count,
                    source_frame_hashes: video.decoder.source_frame_hashes.clone(),
                    decoded_frame_hashes: video.decoder.decoded_frame_hashes.clone(),
                },
            },
        }
    }
}

pub(super) fn current_profile(current_os: &str) -> &'static str {
    match current_os {
        "macos" => "macos",
        "windows" => "windows",
        "linux" => "linux",
        _ => "unknown",
    }
}

pub(super) fn absolute_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::egui::text_command_surface::{
        EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootFactoryError,
    };

    #[test]
    fn full_root_error_and_platform_helpers_cover_each_closed_variant() {
        for error in [
            FullRootArtifactError::Adapter("adapter".to_string()),
            FullRootArtifactError::Contract("contract".to_string()),
            FullRootArtifactError::Video("video".to_string()),
        ] {
            assert!(error.to_string().contains("full-root"));
        }

        let image =
            FullRootArtifactError::from(image::ImageError::IoError(std::io::Error::other("image")));
        assert!(image.to_string().contains("image error"));
        let io = FullRootArtifactError::from(std::io::Error::other("io"));
        assert!(io.to_string().contains("I/O error"));
        let json = serde_json::from_str::<serde_json::Value>("{")
            .err()
            .map(FullRootArtifactError::from)
            .map(|error| error.to_string());
        assert!(json.is_some_and(|error| error.contains("JSON error")));
        let root = FullRootArtifactError::from(
            EguiTextCommandSurfaceRootFactoryError::InvalidToken("opaque"),
        );
        assert!(root.to_string().contains("adapter error"));
        let forwarding = FullRootArtifactError::from(
            EguiTextCommandSurfaceRootEventBatchForwardError::<std::convert::Infallible>::AlreadyConsumed,
        );
        assert!(forwarding.to_string().contains("forwarding failed"));

        assert_eq!(current_profile("macos"), "macos");
        assert_eq!(current_profile("windows"), "windows");
        assert_eq!(current_profile("linux"), "linux");
        assert_eq!(current_profile("other"), "unknown");
        assert_eq!(absolute_path(Path::new("opaque/path")), "opaque/path");
    }
}
