use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootFactoryError,
};

#[derive(Debug)]
pub enum FullRootArtifactError {
    Adapter(String),
    Contract(String),
    Video(String),
    Image(image::ImageError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for FullRootArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adapter(error) => write!(formatter, "full-root adapter error: {error}"),
            Self::Contract(error) => write!(formatter, "full-root contract error: {error}"),
            Self::Video(error) => write!(formatter, "full-root video error: {error}"),
            Self::Image(error) => write!(formatter, "full-root image error: {error}"),
            Self::Io(error) => write!(formatter, "full-root I/O error: {error}"),
            Self::Json(error) => write!(formatter, "full-root JSON error: {error}"),
        }
    }
}

impl std::error::Error for FullRootArtifactError {}

impl From<image::ImageError> for FullRootArtifactError {
    fn from(error: image::ImageError) -> Self {
        Self::Image(error)
    }
}

impl From<std::io::Error> for FullRootArtifactError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for FullRootArtifactError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<EguiTextCommandSurfaceRootFactoryError> for FullRootArtifactError {
    fn from(error: EguiTextCommandSurfaceRootFactoryError) -> Self {
        Self::Adapter(error.to_string())
    }
}

impl From<EguiTextCommandSurfaceRootEventBatchForwardError<std::convert::Infallible>>
    for FullRootArtifactError
{
    fn from(
        error: EguiTextCommandSurfaceRootEventBatchForwardError<std::convert::Infallible>,
    ) -> Self {
        Self::Adapter(format!("root event forwarding failed: {error:?}"))
    }
}
