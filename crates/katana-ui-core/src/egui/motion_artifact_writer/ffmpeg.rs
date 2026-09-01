use std::path::{Path, PathBuf};
use std::process::Output;

use super::constants::{
    DEFAULT_ENCODER, DEFAULT_MUXER, DEFAULT_PIXEL_FORMAT, ROOT_IMAGE_PATTERN,
    STAGE_DIMENSIONS_PREFIX,
};
use super::error::MotionArtifactError;
use super::types::MotionArtifactSettings;
use crate::egui::system::ProcessService;

const FRAME_MD5_HEX_LENGTH: usize = 32;

#[derive(Debug)]
pub(super) struct Ffmpeg {
    pub path: PathBuf,
    pub version: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct FrameEvidence {
    pub frame_hashes: Vec<String>,
    pub width: u32,
    pub height: u32,
}

impl Ffmpeg {
    pub(super) fn discover() -> Result<Self, MotionArtifactError> {
        let path = find_executable("ffmpeg")
            .ok_or(MotionArtifactError::Encoder("ffmpeg was not found".into()))?;
        Self::discover_at(&path)
    }

    pub(super) fn discover_at(path: &Path) -> Result<Self, MotionArtifactError> {
        let service = ProcessService;
        let version = discover_version(path, &service)?;

        let encoders = service
            .run_output(path, ["-hide_banner", "-loglevel", "error", "-encoders"])
            .map_err(|error| MotionArtifactError::Encoder(format!("ffmpeg -encoders: {error}")))?;
        if !encoders.status.success() {
            return Err(MotionArtifactError::Encoder(process_detail(&encoders)));
        }
        if !String::from_utf8_lossy(&encoders.stdout)
            .lines()
            .any(|line| {
                let mut fields = line.split_whitespace();
                fields.next().is_some_and(|flags| flags.contains('V'))
                    && fields.next() == Some(DEFAULT_ENCODER)
            })
        {
            return Err(MotionArtifactError::Encoder(
                "required video encoder is unavailable".into(),
            ));
        }

        let formats = service
            .run_output(path, ["-hide_banner", "-loglevel", "error", "-formats"])
            .map_err(|error| MotionArtifactError::Encoder(format!("ffmpeg -formats: {error}")))?;
        if !formats.status.success() {
            return Err(MotionArtifactError::Encoder(process_detail(&formats)));
        }
        if !String::from_utf8_lossy(&formats.stdout)
            .lines()
            .any(|line| {
                let mut fields = line.split_whitespace();
                fields.next().is_some_and(|flags| flags.contains('E'))
                    && fields.next() == Some(DEFAULT_MUXER)
            })
        {
            return Err(MotionArtifactError::Encoder(
                "required video muxer is unavailable".into(),
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            version,
        })
    }

    pub(super) fn encode(
        &self,
        output: &Path,
        dir: &Path,
        count: usize,
        settings: MotionArtifactSettings,
    ) -> Result<(), MotionArtifactError> {
        let pattern = dir.join(ROOT_IMAGE_PATTERN);
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
            "-crf",
            "0",
            "-pix_fmt",
            DEFAULT_PIXEL_FORMAT,
            "-f",
            DEFAULT_MUXER,
            output.to_str().unwrap_or_default(),
        ];
        let service = ProcessService;
        let result = service
            .run_output(&self.path, args)
            .map_err(|error| MotionArtifactError::Encoder(format!("ffmpeg encode: {error}")))?;
        if !result.status.success()
            || !output.is_file()
            || !std::fs::metadata(output).is_ok_and(|metadata| metadata.len() > 0)
        {
            return Err(MotionArtifactError::Encoder(process_detail(&result)));
        }
        Ok(())
    }

    pub(super) fn source_evidence(
        &self,
        dir: &Path,
        count: usize,
        settings: MotionArtifactSettings,
    ) -> Result<FrameEvidence, MotionArtifactError> {
        let pattern = dir.join(ROOT_IMAGE_PATTERN);
        self.frame_evidence([
            "-framerate",
            &format!("{}/{}", settings.fps_numerator, settings.fps_denominator),
            "-start_number",
            "0",
            "-i",
            pattern.to_str().unwrap_or_default(),
            "-frames:v",
            &count.to_string(),
            "-map",
            "0:v:0",
            "-pix_fmt",
            DEFAULT_PIXEL_FORMAT,
            "-f",
            "framemd5",
            "-",
        ])
    }

    pub(super) fn decode(&self, path: &Path) -> Result<FrameEvidence, MotionArtifactError> {
        self.frame_evidence([
            "-i",
            path.to_str().unwrap_or_default(),
            "-map",
            "0:v:0",
            "-pix_fmt",
            DEFAULT_PIXEL_FORMAT,
            "-f",
            "framemd5",
            "-",
        ])
    }

    fn frame_evidence<const N: usize>(
        &self,
        body_args: [&str; N],
    ) -> Result<FrameEvidence, MotionArtifactError> {
        let service = ProcessService;
        let mut args = vec!["-hide_banner", "-loglevel", "error"];
        args.extend(body_args);
        let result = service
            .run_output(&self.path, args)
            .map_err(|error| MotionArtifactError::Encoder(format!("ffmpeg decode: {error}")))?;
        if !result.status.success() {
            return Err(MotionArtifactError::Encoder(process_detail(&result)));
        }
        let text = String::from_utf8_lossy(&result.stdout);
        let dimensions = parse_framemd5_dimensions(&text)?;
        let frame_hashes = text
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .map(parse_framemd5_hash)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FrameEvidence {
            frame_hashes,
            width: dimensions.0,
            height: dimensions.1,
        })
    }
}

fn discover_version(path: &Path, service: &ProcessService) -> Result<String, MotionArtifactError> {
    let output = service
        .run_output(path, ["-version"])
        .map_err(|error| MotionArtifactError::Encoder(format!("ffmpeg -version: {error}")))?;
    if !output.status.success() {
        return Err(MotionArtifactError::Encoder(process_detail(&output)));
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

fn parse_framemd5_dimensions(text: &str) -> Result<(u32, u32), MotionArtifactError> {
    let dimensions = text
        .lines()
        .find_map(|line| line.strip_prefix(STAGE_DIMENSIONS_PREFIX))
        .and_then(|value| {
            let mut parts = value.trim().split('x');
            Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
        })
        .ok_or_else(|| MotionArtifactError::Encoder("decoder omitted dimensions".into()))?;
    Ok(dimensions)
}

fn parse_framemd5_hash(line: &str) -> Result<String, MotionArtifactError> {
    let hash = line
        .rsplit(',')
        .next()
        .map(str::trim)
        .filter(|hash| {
            hash.len() == FRAME_MD5_HEX_LENGTH && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
        .ok_or_else(|| {
            MotionArtifactError::Encoder("decoder emitted an invalid frame hash".into())
        })?;
    Ok(hash.to_ascii_lowercase())
}

fn find_executable(name: &str) -> Option<PathBuf> {
    let names = executable_names(name, std::env::consts::EXE_SUFFIX);
    std::env::var_os("PATH").and_then(|path| {
        std::env::split_paths(&path)
            .flat_map(|dir| names.iter().map(move |name| dir.join(name)))
            .find(|path| path.is_file())
            .and_then(|path| std::fs::canonicalize(path).ok())
    })
}

fn executable_names(name: &str, suffix: &str) -> Vec<String> {
    let mut names = vec![name.to_owned()];
    if !suffix.is_empty() {
        names.push(format!("{name}{suffix}"));
    }
    names
}

fn process_detail(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if stderr.is_empty() {
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    } else {
        stderr
    }
}

#[cfg(test)]
mod tests {
    use super::STAGE_DIMENSIONS_PREFIX;
    use super::{Ffmpeg, MotionArtifactSettings, discover_version, parse_framemd5_dimensions};
    use crate::egui::motion_artifact_writer::error::MotionArtifactError;
    use crate::egui::motion_artifact_writer::fake_ffmpeg::{FakeFfmpegSpec, install};
    use crate::egui::system::ProcessService;
    use std::path::PathBuf;
    use std::process::Output;

    #[derive(Default)]
    struct PathEnvGuard {
        saved: Option<std::ffi::OsString>,
    }

    impl PathEnvGuard {
        fn with_root(root: &std::path::Path) -> Self {
            let saved = std::env::var_os("PATH");
            let original = saved.as_ref().expect("test process must define PATH");
            let replacement = std::env::join_paths(
                std::iter::once(root.to_path_buf()).chain(std::env::split_paths(original)),
            )
            .expect("path join should build");
            /* SAFETY: the tests isolate PATH within a single test process and restore it in Drop. */
            unsafe { std::env::set_var("PATH", replacement) };
            Self { saved }
        }
    }

    impl Drop for PathEnvGuard {
        fn drop(&mut self) {
            if let Some(path) = self.saved.take() {
                /* SAFETY: restore the captured PATH value in test cleanup. */
                unsafe { std::env::set_var("PATH", path) };
            }
        }
    }

    #[cfg(unix)]
    fn write_self_disabling_script(root: &std::path::Path, body: &str) -> PathBuf {
        let path = root.join("ffmpeg");
        let body = format!("#!/bin/sh\n{body}\n");
        std::fs::write(&path, body).expect("script should write");
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("script should be executable");
        path
    }

    #[test]
    fn rejects_empty_version_output() {
        let spec = FakeFfmpegSpec {
            version: None,
            ..FakeFfmpegSpec::default()
        };
        let script = install(&tempfile_dir("motion-ffmpeg-empty"), &spec);
        assert!(matches!(
            Ffmpeg::discover_at(&script),
            Err(MotionArtifactError::Encoder(_))
        ));
    }

    #[test]
    fn discover_parses_encoder_and_muxer_lines() {
        let dir = tempfile_dir("motion-ffmpeg-discover");
        let script = install(&dir, &FakeFfmpegSpec::default());
        let ffmpeg = Ffmpeg::discover_at(&script).expect("discover should pass");
        assert_eq!(ffmpeg.version, "ffmpeg version 1.0");
    }

    #[test]
    fn discover_rejects_encoder_and_muxer_failures() {
        let cases = [
            FakeFfmpegSpec {
                encoder_status: 1,
                ..FakeFfmpegSpec::default()
            },
            FakeFfmpegSpec {
                encoder: None,
                ..FakeFfmpegSpec::default()
            },
            FakeFfmpegSpec {
                muxer_status: 1,
                ..FakeFfmpegSpec::default()
            },
            FakeFfmpegSpec {
                muxer: None,
                ..FakeFfmpegSpec::default()
            },
        ];
        for (index, spec) in cases.iter().enumerate() {
            let script = install(
                &tempfile_dir(&format!("motion-ffmpeg-discovery-failure-{index}")),
                spec,
            );
            assert!(matches!(
                Ffmpeg::discover_at(&script),
                Err(MotionArtifactError::Encoder(_))
            ));
        }
    }

    #[cfg(unix)]
    #[test]
    fn discover_reports_process_launch_failures_after_version_and_encoder_queries() {
        let version_dir = tempfile_dir("motion-ffmpeg-version-launch");
        let version_script =
            write_self_disabling_script(&version_dir, "echo version\nchmod -x \"$0\"\nexit 0");
        assert!(matches!(
            Ffmpeg::discover_at(&version_script),
            Err(MotionArtifactError::Encoder(_))
        ));

        let encoder_dir = tempfile_dir("motion-ffmpeg-encoder-launch");
        let encoder_script = write_self_disabling_script(
            &encoder_dir,
            &format!(
                "if [ \"$1\" = \"-version\" ]; then echo version; exit 0; fi\necho ' V.... {}'\nchmod -x \"$0\"\nexit 0",
                super::DEFAULT_ENCODER
            ),
        );
        assert!(matches!(
            Ffmpeg::discover_at(&encoder_script),
            Err(MotionArtifactError::Encoder(_))
        ));
    }

    #[test]
    fn discover_uses_path_lookup() {
        let _guard = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = tempfile_dir("motion-ffmpeg-path-discover");
        let _script = install(&root, &FakeFfmpegSpec::default());
        let _path_guard = PathEnvGuard::with_root(root.as_path());
        assert!(Ffmpeg::discover().is_ok());
    }

    #[test]
    fn find_executable_discovers_from_temp_path() {
        let _guard = super::super::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let root = tempfile_dir("motion-ffmpeg-exec");
        let _script = install(&root, &FakeFfmpegSpec::default());
        let _path_guard = PathEnvGuard::with_root(root.as_path());
        let found = super::find_executable("ffmpeg").expect("ffmpeg should be discovered");
        let expected_name = format!("ffmpeg{}", std::env::consts::EXE_SUFFIX);
        assert_eq!(
            found.file_name().and_then(std::ffi::OsStr::to_str),
            Some(expected_name.as_str())
        );
        drop(_path_guard);
    }

    #[test]
    fn find_executable_returns_none_for_missing_binary() {
        let original = super::find_executable("definitely-not-a-motion-test-binary");
        assert!(original.is_none());
    }

    #[test]
    fn executable_names_include_the_platform_suffix_when_present() {
        assert_eq!(super::executable_names("ffmpeg", ""), ["ffmpeg"]);
        assert_eq!(
            super::executable_names("ffmpeg", ".exe"),
            ["ffmpeg", "ffmpeg.exe"]
        );
    }

    #[cfg(unix)]
    #[test]
    fn process_detail_prefers_stderr_if_present() {
        let status = std::process::Command::new("/bin/sh")
            .args(["-c", "exit 1"])
            .output()
            .expect("process should fail")
            .status;
        let output = Output {
            status,
            stdout: b"stdout message".to_vec(),
            stderr: b"stderr message".to_vec(),
        };
        assert_eq!(super::process_detail(&output), "stderr message");
    }

    #[cfg(unix)]
    #[test]
    fn process_detail_falls_back_to_stdout_when_stderr_is_empty() {
        let status = std::process::Command::new("/bin/sh")
            .args(["-c", "exit 1"])
            .output()
            .expect("process should fail")
            .status;
        let output = Output {
            status,
            stdout: b"stdout message".to_vec(),
            stderr: vec![],
        };
        assert_eq!(super::process_detail(&output), "stdout message");
    }

    #[cfg(unix)]
    #[test]
    fn parse_framemd5_dimensions_rejects_invalid_prefix_or_values() {
        assert!(super::parse_framemd5_dimensions("no prefix").is_err());
        assert!(
            super::parse_framemd5_dimensions(&format!("{STAGE_DIMENSIONS_PREFIX}2x"),).is_err()
        );
        assert!(
            super::parse_framemd5_dimensions(&format!("{STAGE_DIMENSIONS_PREFIX}x10"),).is_err()
        );
    }

    #[test]
    fn decode_requires_dimensions_prefix() {
        let dir = tempfile_dir("motion-ffmpeg-decode");
        let spec = FakeFfmpegSpec {
            dimensions: None,
            ..FakeFfmpegSpec::default()
        };
        let script = install(&dir, &spec);
        let ffmpeg = Ffmpeg::discover_at(&script).expect("discovery should pass");
        let err = ffmpeg
            .decode(&dir.join("missing.mp4"))
            .expect_err("decode missing dimensions should fail");
        assert!(matches!(err, MotionArtifactError::Encoder(_)));
    }

    #[test]
    fn encode_and_decode_happy_path() {
        let dir = tempfile_dir("motion-ffmpeg-encode");
        let script = install(&dir, &FakeFfmpegSpec::default());
        let ffmpeg = Ffmpeg::discover_at(&script).expect("discover should pass");
        let mp4_path = dir.join("motion.mp4");
        ffmpeg
            .encode(&mp4_path, &dir, 2, MotionArtifactSettings::new(2, 2, 1))
            .expect("fake encode should write mp4");
        let evidence = ffmpeg
            .decode(&mp4_path)
            .expect("decode should parse fake framemd5");
        assert_eq!(evidence.frame_hashes.len(), 2);
        assert_eq!(evidence.width, 2);
        assert_eq!(evidence.height, 1);
    }

    #[test]
    fn encode_and_decode_report_launch_and_process_failures() {
        let dir = tempfile_dir("motion-ffmpeg-process-failures");
        let missing = Ffmpeg {
            path: dir.join("missing-ffmpeg"),
            version: "missing".into(),
        };
        assert!(
            missing
                .encode(
                    &dir.join("missing.mp4"),
                    &dir,
                    1,
                    MotionArtifactSettings::new(1, 1, 1),
                )
                .is_err()
        );
        assert!(missing.decode(&dir.join("missing.mp4")).is_err());

        let spec = FakeFfmpegSpec {
            encode_status: 1,
            frame_status: 1,
            ..FakeFfmpegSpec::default()
        };
        let failing_script = install(&dir, &spec);
        let failing = Ffmpeg {
            path: failing_script,
            version: "failing".into(),
        };
        assert!(
            failing
                .encode(
                    &dir.join("failed.mp4"),
                    &dir,
                    1,
                    MotionArtifactSettings::new(1, 1, 1),
                )
                .is_err()
        );
        assert!(failing.decode(&dir.join("failed.mp4")).is_err());
    }

    #[test]
    fn parse_framemd5_dimensions_from_text() {
        let parsed =
            parse_framemd5_dimensions(&format!("{STAGE_DIMENSIONS_PREFIX}2x1\n")).expect("parse");
        assert_eq!(parsed, (2, 1));
        assert!(parse_framemd5_dimensions("no-dim\n").is_err());
    }

    #[test]
    fn parse_framemd5_hash_rejects_invalid_value() {
        assert!(super::parse_framemd5_hash("invalid").is_err());
        assert!(super::parse_framemd5_hash("0, 0, 0, 1, 6, 0123456789abc").is_err());
        let error = super::parse_framemd5_hash("0, 0, 0, 1, 6, zz0123456789abcdef0123456789abcdef")
            .expect_err("non-hex frame hash must fail closed");
        assert!(
            error
                .to_string()
                .contains("decoder emitted an invalid frame hash")
        );
    }

    #[test]
    fn discover_version_fails_for_non_zero_exit() {
        let dir = tempfile_dir("motion-ffmpeg-version-status");
        let spec = FakeFfmpegSpec {
            version_status: 1,
            ..FakeFfmpegSpec::default()
        };
        let executable = install(&dir, &spec);
        let service = ProcessService;
        assert!(discover_version(&executable, &service).is_err());
    }

    fn tempfile_dir(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("kuc-motion-{label}-{}", std::process::id()));
        std::fs::create_dir_all(&path).expect("dir should create");
        path
    }
}
