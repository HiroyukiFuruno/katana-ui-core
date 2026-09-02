mod constants;
mod error;
#[cfg(test)]
mod fake_ffmpeg;
mod ffmpeg;
mod process;
#[cfg(test)]
mod tests;
mod types;
mod validation;
mod variable_viewport;

#[cfg(test)]
pub(crate) static TEST_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub use error::MotionArtifactError;
pub use types::{
    MotionArtifact, MotionArtifactManifest, MotionArtifactSettings, MotionArtifactWriter,
    VariableViewportMotionArtifact, VariableViewportMotionArtifactManifest,
    VariableViewportSemanticEvidence, VariableViewportSourceViewport,
};
pub use variable_viewport::VariableViewportMotionArtifactError;
