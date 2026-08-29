#[path = "command_chrome_artifact_manifest.rs"]
mod command_chrome_artifact_manifest;
#[path = "command_chrome_artifact_types.rs"]
mod command_chrome_artifact_types;

use crate::visual::command_chrome_script::CommandChromeScriptError;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_error_display_and_conversions_cover_every_variant() {
        let script = CommandChromeScriptError::message("adapter");
        assert_eq!(
            CommandChromeArtifactError::from(script).to_string(),
            "CommandChrome adapter error: adapter"
        );
        assert_eq!(
            CommandChromeArtifactError::Contract("contract".into()).to_string(),
            "CommandChrome artifact contract: contract"
        );

        let image = ImageError::IoError(io::Error::other("image"));
        assert!(
            CommandChromeArtifactError::from(image)
                .to_string()
                .contains("CommandChrome artifact image error: image")
        );
        let io = io::Error::other("io");
        assert_eq!(
            CommandChromeArtifactError::from(io).to_string(),
            "CommandChrome artifact I/O error: io"
        );
        let json = serde_json::from_str::<serde_json::Value>("{")
            .err()
            .map(CommandChromeArtifactError::from)
            .map(|error| error.to_string());
        assert!(json.is_some_and(|error| error.starts_with("CommandChrome artifact JSON error:")));
    }

    #[test]
    fn artifact_error_implements_error() {
        let error: &dyn std::error::Error =
            &CommandChromeArtifactError::Contract("contract".into());
        assert_eq!(
            error.to_string(),
            "CommandChrome artifact contract: contract"
        );
    }
}
