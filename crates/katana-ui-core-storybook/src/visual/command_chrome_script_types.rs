#[path = "command_chrome_artifact_manifest.rs"]
mod command_chrome_artifact_manifest;
#[path = "command_chrome_artifact_types.rs"]
mod command_chrome_artifact_types;

use crate::visual::command_chrome_script::CommandChromeScriptError;
#[cfg(test)]
pub(super) use command_chrome_artifact_types::StorybookCommandChromeTypedEvent;
pub(super) use command_chrome_artifact_types::{
    CommandChromeArtifactFrame, CommandChromeArtifactSequence, StorybookCommandChromeManifest,
};
use image::ImageError;
use std::io;

#[derive(Debug)]
pub enum CommandChromeArtifactError {
    Adapter(String),
    Contract(String),
    Image(ImageError),
    Io(io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for CommandChromeArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adapter(error) => write!(formatter, "CommandChrome adapter error: {error}"),
            Self::Contract(error) => write!(formatter, "CommandChrome artifact contract: {error}"),
            Self::Image(error) => write!(formatter, "CommandChrome artifact image error: {error}"),
            Self::Io(error) => write!(formatter, "CommandChrome artifact I/O error: {error}"),
            Self::Json(error) => write!(formatter, "CommandChrome artifact JSON error: {error}"),
        }
    }
}

impl std::error::Error for CommandChromeArtifactError {}

impl From<crate::visual::command_chrome_script::CommandChromeScriptError>
    for CommandChromeArtifactError
{
    fn from(value: CommandChromeScriptError) -> Self {
        Self::Adapter(value.to_string())
    }
}

impl From<ImageError> for CommandChromeArtifactError {
    fn from(error: ImageError) -> Self {
        Self::Image(error)
    }
}

impl From<io::Error> for CommandChromeArtifactError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for CommandChromeArtifactError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
