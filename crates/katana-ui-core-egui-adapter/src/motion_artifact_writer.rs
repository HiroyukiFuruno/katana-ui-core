mod constants;
mod error;
mod ffmpeg;
mod process;
#[cfg(test)]
mod tests;
mod types;
mod validation;

#[cfg(test)]
pub(crate) static TEST_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub use error::MotionArtifactError;
pub use types::{
    MotionArtifact, MotionArtifactManifest, MotionArtifactSettings, MotionArtifactWriter,
};
