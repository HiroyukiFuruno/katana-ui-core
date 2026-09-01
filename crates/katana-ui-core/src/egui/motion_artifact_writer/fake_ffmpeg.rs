use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use super::constants::{DEFAULT_ENCODER, DEFAULT_MUXER, STAGE_DIMENSIONS_PREFIX};
use crate::egui::system::ProcessService;

const FIRST_HASH: &str = "0, 0, 0, 1, 6, 0123456789abcdef0123456789abcdef";
const SECOND_HASH: &str = "0, 1, 1, 1, 6, fedcba9876543210fedcba9876543210";
#[cfg(unix)]
const TEST_EXECUTABLE_PERMISSIONS: u32 = 0o755;

pub(super) struct FakeFfmpegSpec {
    pub version: Option<String>,
    pub version_status: i32,
    pub encoder: Option<String>,
    pub encoder_status: i32,
    pub muxer: Option<String>,
    pub muxer_status: i32,
    pub dimensions: Option<String>,
    pub source_hashes: Vec<String>,
    pub decoded_hashes: Vec<String>,
    pub encode_status: i32,
    pub frame_status: i32,
}

impl Default for FakeFfmpegSpec {
    fn default() -> Self {
        Self {
            version: Some("ffmpeg version 1.0".into()),
            version_status: 0,
            encoder: Some(DEFAULT_ENCODER.into()),
            encoder_status: 0,
            muxer: Some(DEFAULT_MUXER.into()),
            muxer_status: 0,
            dimensions: Some(format!("{STAGE_DIMENSIONS_PREFIX}2x1")),
            source_hashes: vec![FIRST_HASH.into(), SECOND_HASH.into()],
            decoded_hashes: vec![FIRST_HASH.into(), SECOND_HASH.into()],
            encode_status: 0,
            frame_status: 0,
        }
    }
}

pub(super) fn install(root: &Path, spec: &FakeFfmpegSpec) -> PathBuf {
    std::fs::create_dir_all(root).expect("fake ffmpeg root should create");
    let path = root.join(format!("ffmpeg{}", std::env::consts::EXE_SUFFIX));
    std::fs::hard_link(compiled_fixture(), &path).expect("fake ffmpeg executable should link");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            &path,
            std::fs::Permissions::from_mode(TEST_EXECUTABLE_PERMISSIONS),
        )
        .expect("fake ffmpeg should be executable");
    }
    std::fs::write(path.with_extension("fixture"), encode_spec(spec))
        .expect("fake ffmpeg fixture should write");
    path
}

fn encode_spec(spec: &FakeFfmpegSpec) -> String {
    [
        ("version", spec.version.as_deref().unwrap_or_default()),
        ("version_status", &spec.version_status.to_string()),
        ("encoder", spec.encoder.as_deref().unwrap_or_default()),
        ("encoder_status", &spec.encoder_status.to_string()),
        ("muxer", spec.muxer.as_deref().unwrap_or_default()),
        ("muxer_status", &spec.muxer_status.to_string()),
        ("dimensions", spec.dimensions.as_deref().unwrap_or_default()),
        ("source_hashes", &spec.source_hashes.join("|")),
        ("decoded_hashes", &spec.decoded_hashes.join("|")),
        ("encode_status", &spec.encode_status.to_string()),
        ("frame_status", &spec.frame_status.to_string()),
    ]
    .into_iter()
    .map(|(key, value)| format!("{key}={value}\n"))
    .collect()
}

fn compiled_fixture() -> &'static PathBuf {
    static FIXTURE: OnceLock<PathBuf> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let root =
            std::env::temp_dir().join(format!("kuc-native-fake-ffmpeg-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("native fixture root should create");
        let source = root.join("main.rs");
        let executable = root.join(format!("fixture{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(&source, FIXTURE_SOURCE).expect("native fixture source should write");
        let mut command = ProcessService::create_command(
            std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()),
        );
        let output = command
            .args(["--edition=2024", "-O"])
            .arg(&source)
            .arg("-o")
            .arg(&executable)
            .output()
            .expect("native fixture should compile");
        assert!(
            output.status.success(),
            "native fixture compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        executable
    })
}

const FIXTURE_SOURCE: &str = r#"
use std::collections::HashMap;

fn main() {
    let executable = std::env::current_exe().expect("fixture executable path");
    let fixture = std::fs::read_to_string(executable.with_extension("fixture"))
        .expect("fixture specification");
    let values = fixture
        .lines()
        .filter_map(|line| line.split_once('='))
        .collect::<HashMap<_, _>>();
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.iter().any(|arg| arg == "-version") {
        emit(values["version"]);
        finish(values["version_status"]);
    }
    if args.iter().any(|arg| arg == "-encoders") {
        if !values["encoder"].is_empty() {
            println!(" V....  {}", values["encoder"]);
        }
        finish(values["encoder_status"]);
    }
    if args.iter().any(|arg| arg == "-formats") {
        if !values["muxer"].is_empty() {
            println!(" E....  {}", values["muxer"]);
        }
        finish(values["muxer_status"]);
    }
    if args.iter().any(|arg| arg == "framemd5") {
        emit(values["dimensions"]);
        let hashes = if args.iter().any(|arg| arg == "-start_number") {
            values["source_hashes"]
        } else {
            values["decoded_hashes"]
        };
        for hash in hashes.split('|').filter(|hash| !hash.is_empty()) {
            println!("{hash}");
        }
        finish(values["frame_status"]);
    }

    if values["encode_status"] == "0" {
        let output = args.last().expect("encode output path");
        std::fs::write(output, b"motion").expect("encode output should write");
    } else {
        eprintln!("process-failed");
    }
    finish(values["encode_status"]);
}

fn emit(value: &str) {
    if !value.is_empty() {
        println!("{value}");
    }
}

fn finish(status: &str) -> ! {
    std::process::exit(status.parse().expect("numeric fixture status"));
}
"#;
