use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct RunnerOptions {
    pub run_id: String,
    pub revision: String,
    pub challenge: String,
    pub producer_id: String,
    pub runner_id: String,
    pub output: PathBuf,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NativeTranscript {
    pub schema_version: String,
    pub revision: String,
    pub challenge: String,
    pub run_id: String,
    pub producer_id: String,
    pub runner_id: String,
    pub platform: String,
    pub input_method: String,
    pub preedit: String,
    pub commit: String,
    pub preedit_sequence: u64,
    pub commit_sequence: u64,
    pub egui_frames: u64,
    /// True only when the transcript contains an OS-delivered winit IME event.
    pub native_event_observed: bool,
}

#[derive(Debug)]
pub enum RunnerError {
    EventLoop(winit::error::EventLoopError),
    InputMethod(io::Error),
    Window(winit::error::OsError),
    Serialize(serde_json::Error),
    Write(io::Error),
    Core(String),
    MissingNativeEvent,
    Timeout,
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EventLoop(e) => write!(f, "event loop: {e}"),
            Self::InputMethod(e) => write!(f, "input method: {e}"),
            Self::Window(e) => write!(f, "window: {e}"),
            Self::Serialize(e) => write!(f, "serialize transcript: {e}"),
            Self::Write(e) => write!(f, "write transcript: {e}"),
            Self::Core(e) => write!(f, "KUC native root: {e}"),
            Self::MissingNativeEvent => write!(f, "no OS-native winit IME event was observed"),
            Self::Timeout => write!(f, "native IME evidence timed out before commit"),
        }
    }
}
impl std::error::Error for RunnerError {}
