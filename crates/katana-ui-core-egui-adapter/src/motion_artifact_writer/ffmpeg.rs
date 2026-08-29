use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use super::support::MotionSupport;
use super::{MotionArtifactError, MotionArtifactSettings};

pub(crate) const DEFAULT_ENCODER: &str = "mpeg4";
pub(crate) const DEFAULT_MUXER: &str = "mp4";
const GIF_ENCODER: &str = "gif";
const GIF_MUXER: &str = "gif";
const DEFAULT_PIXEL_FORMAT: &str = "yuv420p";

#[derive(Debug)]
pub(crate) struct Ffmpeg {
    pub(crate) path: PathBuf,
    pub(crate) version: String,
}

impl Ffmpeg {
    pub(crate) fn discover() -> Result<Self, MotionArtifactError> {
        let path = MotionSupport::find_executable("ffmpeg")
            .ok_or_else(|| MotionArtifactError::Encoder("ffmpeg was not found".into()))?;
        Self::discover_at(&path)
    }

    pub(crate) fn discover_at(path: &Path) -> Result<Self, MotionArtifactError> {
        let version = run(path, ["-version"], "ffmpeg -version")?;
        let encoders = MotionSupport::run_output(
            path,
            ["-hide_banner", "-loglevel", "error", "-encoders"],
            "ffmpeg -encoders",
        )?;
        if !has_encoder(&encoders, DEFAULT_ENCODER) || !has_encoder(&encoders, GIF_ENCODER) {
            return Err(MotionArtifactError::Encoder(
                "required video encoder is unavailable".into(),
            ));
        }
        let formats = MotionSupport::run_output(
            path,
            ["-hide_banner", "-loglevel", "error", "-formats"],
            "ffmpeg -formats",
        )?;
        if !has_muxer(&formats, DEFAULT_MUXER) || !has_muxer(&formats, GIF_MUXER) {
            return Err(MotionArtifactError::Encoder(
                "required video muxer is unavailable".into(),
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            version,
        })
    }

    pub(crate) fn encode(
        &self,
        output: &Path,
        dir: &Path,
        count: usize,
        settings: MotionArtifactSettings,
    ) -> Result<(), MotionArtifactError> {
        let pattern = dir.join("frame-%03d.png");
        let args = [
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-framerate",
            &format!("{}/{}", settings.fps_numerator, settings.fps_denominator),
            "-start_number",
            "0",
            "-i",
            pattern.to_str().unwrap_or_default(),
            "-frames:v",
            &count.to_string(),
            "-an",
            "-c:v",
            DEFAULT_ENCODER,
            "-pix_fmt",
            DEFAULT_PIXEL_FORMAT,
            "-f",
            DEFAULT_MUXER,
            output.to_str().unwrap_or_default(),
        ];
        self.encode_command(output, &args, "ffmpeg encode")
    }

    pub(crate) fn encode_gif(
        &self,
        output: &Path,
        dir: &Path,
        count: usize,
        settings: MotionArtifactSettings,
    ) -> Result<(), MotionArtifactError> {
        let pattern = dir.join("frame-%03d.png");
        let args = [
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-framerate",
            &format!("{}/{}", settings.fps_numerator, settings.fps_denominator),
            "-start_number",
            "0",
            "-i",
            pattern.to_str().unwrap_or_default(),
            "-frames:v",
            &count.to_string(),
            "-an",
            "-c:v",
            GIF_ENCODER,
            "-gifflags",
            "-transdiff",
            "-f",
            GIF_MUXER,
            output.to_str().unwrap_or_default(),
        ];
        self.encode_command(output, &args, "ffmpeg GIF encode")
    }

    fn encode_command(
        &self,
        output: &Path,
        args: &[&str],
        operation: &str,
    ) -> Result<(), MotionArtifactError> {
        let result = MotionSupport::run_output(&self.path, args, operation)?;
        if !result.status.success()
            || !output.is_file()
            || fs::metadata(output).map_err(MotionSupport::io_error)?.len() == 0
        {
            return Err(MotionArtifactError::Encoder(MotionSupport::process_detail(
                &result,
            )));
        }
        Ok(())
    }

    pub(crate) fn decode(&self, path: &Path) -> Result<(usize, u32, u32), MotionArtifactError> {
        let result = MotionSupport::run_output(
            &self.path,
            [
                "-hide_banner",
                "-loglevel",
                "error",
                "-i",
                path.to_str().unwrap_or_default(),
                "-map",
                "0:v:0",
                "-f",
                "framemd5",
                "-",
            ],
            "ffmpeg decode",
        )?;
        if !result.status.success() {
            return Err(MotionArtifactError::Encoder(MotionSupport::process_detail(
                &result,
            )));
        }
        let text = String::from_utf8_lossy(&result.stdout);
        let dimensions = text
            .lines()
            .find_map(|line| line.strip_prefix("#dimensions 0:"))
            .and_then(|value| {
                let mut parts = value.trim().split('x');
                Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
            })
            .ok_or_else(|| MotionArtifactError::Encoder("decoder omitted dimensions".into()))?;
        let count = text
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .count();
        Ok((count, dimensions.0, dimensions.1))
    }
}

fn has_encoder(output: &Output, name: &str) -> bool {
    String::from_utf8_lossy(&output.stdout).lines().any(|line| {
        let mut fields = line.split_whitespace();
        fields.next().is_some_and(|flags| flags.contains('V')) && fields.next() == Some(name)
    })
}

fn has_muxer(output: &Output, name: &str) -> bool {
    String::from_utf8_lossy(&output.stdout).lines().any(|line| {
        let mut fields = line.split_whitespace();
        fields.next().is_some_and(|flags| flags.contains('E')) && fields.next() == Some(name)
    })
}

fn run(
    path: &Path,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    operation: &str,
) -> Result<String, MotionArtifactError> {
    let output = MotionSupport::run_output(path, args, operation)?;
    if !output.status.success() {
        return Err(MotionArtifactError::Encoder(MotionSupport::process_detail(
            &output,
        )));
    }
    let version = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or_default()
        .to_owned();
    if version.is_empty() {
        Err(MotionArtifactError::Encoder(
            "ffmpeg version is empty".into(),
        ))
    } else {
        Ok(version)
    }
}
