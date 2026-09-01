use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use sha2::{Digest, Sha256};

use super::{MotionArtifactError, MotionArtifactSettings};
use crate::egui::FullRootArtifact;

macro_rules! create_process_command {
    ($program:expr) => {
        std::process::Command::new($program)
    };
}

pub(crate) struct MotionSupport;

impl MotionSupport {
    pub(crate) fn run_output(
        path: &Path,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
        operation: &str,
    ) -> Result<Output, MotionArtifactError> {
        let mut command = ProcessService::create_command(path);
        command
            .args(args)
            .output()
            .map_err(|error| MotionArtifactError::Encoder(format!("{operation}: {error}")))
    }

    pub(crate) fn process_detail(output: &Output) -> String {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if stderr.is_empty() {
            String::from_utf8_lossy(&output.stdout).trim().to_owned()
        } else {
            stderr
        }
    }

    pub(crate) fn io_error(error: std::io::Error) -> MotionArtifactError {
        MotionArtifactError::Io(error.to_string())
    }

    pub(crate) fn sha256(bytes: &[u8]) -> String {
        hex::encode(Sha256::digest(bytes))
    }

    pub(crate) fn encode_rgba_png(
        image: &image::RgbaImage,
    ) -> Result<Vec<u8>, MotionArtifactError> {
        let mut bytes = Vec::new();
        PngEncoder::new(&mut bytes)
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                ColorType::Rgba8.into(),
            )
            .map_err(|error| MotionArtifactError::Encoder(error.to_string()))?;
        Ok(bytes)
    }

    pub(crate) fn find_executable(name: &str) -> Option<PathBuf> {
        std::env::var_os("PATH").and_then(|path| {
            std::env::split_paths(&path)
                .map(|dir| dir.join(name))
                .find(|path| path.is_file())
                .and_then(|path| fs::canonicalize(path).ok())
        })
    }

    pub(crate) fn validate_provenance(
        receipt: &FullRootArtifact,
    ) -> Result<(), MotionArtifactError> {
        let value: serde_json::Value =
            serde_json::from_slice(&fs::read(receipt.manifest_path()).map_err(Self::io_error)?)
                .map_err(|error| MotionArtifactError::Json(error.to_string()))?;
        let object = value
            .as_object()
            .ok_or_else(|| MotionArtifactError::Json("root provenance is not an object".into()))?;
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
}

struct ProcessService;

impl ProcessService {
    pub(crate) fn create_command<S>(program: S) -> std::process::Command
    where
        S: AsRef<OsStr>,
    {
        let mut command = create_process_command!(program);
        apply_silent_policy(&mut command);
        command
    }
}

#[cfg(windows)]
fn apply_silent_policy(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn apply_silent_policy(_command: &mut Command) {}

impl MotionSupport {
    pub(crate) fn validate_settings(
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
}
