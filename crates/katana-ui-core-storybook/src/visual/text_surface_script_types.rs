use super::text_surface_artifact::TextSurfacePlanPixels;
use katana_ui_core::text_surface::{TextSurfaceAccessibilityTree, TextSurfaceEvent};
use katana_ui_core_egui_adapter::text_surface::{
    EguiTextSurfaceError, EguiTextSurfaceOutput, TextSurfaceArtifactFrame,
    TextSurfacePaintOperationKind,
};
use serde::Serialize;

const RGBA_CHANNELS: usize = 4;
const ALPHA_CHANNEL: usize = 3;

#[derive(Debug)]
pub enum TextSurfaceArtifactError {
    Adapter(EguiTextSurfaceError),
    Contract(String),
    Image(image::ImageError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<EguiTextSurfaceError> for TextSurfaceArtifactError {
    fn from(value: EguiTextSurfaceError) -> Self {
        Self::Adapter(value)
    }
}

impl std::fmt::Display for TextSurfaceArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adapter(error) => write!(formatter, "TextSurface adapter error: {error}"),
            Self::Contract(error) => {
                write!(formatter, "TextSurface artifact contract failed: {error}")
            }
            Self::Image(error) => write!(formatter, "TextSurface artifact image error: {error}"),
            Self::Io(error) => write!(formatter, "TextSurface artifact I/O error: {error}"),
            Self::Json(error) => write!(formatter, "TextSurface artifact JSON error: {error}"),
        }
    }
}

impl std::error::Error for TextSurfaceArtifactError {}

#[derive(Debug, Clone)]
pub(super) struct TextSurfaceScriptStep {
    pub(super) name: &'static str,
    pub(super) events: Vec<egui::Event>,
}

#[derive(Debug, Clone)]
pub(super) struct TextSurfaceScriptResult {
    pub(super) steps: Vec<TextSurfaceArtifactStep>,
}

#[derive(Debug, Clone)]
pub(super) struct TextSurfaceArtifactStep {
    pub(super) index: usize,
    pub(super) name: &'static str,
    pub(super) artifact: TextSurfaceArtifactFrame,
    pub(super) pixels: TextSurfacePlanPixels,
    pub(super) events: Vec<TextSurfaceEvent>,
    pub(super) surface_focused: bool,
    pub(super) raw_events: Vec<egui::Event>,
}

#[derive(Debug)]
pub(super) struct ScriptedEguiFrame {
    pub(super) output: EguiTextSurfaceOutput,
    pub(super) raw_events: Vec<egui::Event>,
}

#[derive(Debug, Serialize)]
pub(super) struct StorybookTextSurfaceManifest {
    schema: &'static str,
    input_origin: &'static str,
    artifact_encoder: &'static str,
    frames: Vec<StorybookTextSurfaceManifestFrame>,
}

#[derive(Debug, Serialize)]
struct StorybookTextSurfaceManifestFrame {
    index: usize,
    name: &'static str,
    png: String,
    frame_record_hash: String,
    paint_plan_hash: String,
    pixel_hash: String,
    surface_bounds: katana_ui_core::render_model::UiRect,
    viewport_bounds: katana_ui_core::render_model::UiRect,
    scroll_x: i32,
    scroll_y: i32,
    raster_identity: String,
    text_texture_identities: Vec<String>,
    star_variation_selector_present: bool,
    color_emoji_texture_present: bool,
    typed_events: Vec<TextSurfaceEvent>,
    surface_focused: bool,
    accessibility: TextSurfaceAccessibilityTree,
}

impl StorybookTextSurfaceManifest {
    pub(super) fn from_sequence(sequence: &TextSurfaceScriptResult) -> Self {
        Self {
            schema: "kuc.text-surface-storybook.v1",
            input_origin: "actual-egui-raw-input",
            artifact_encoder: "adapter-paint-plan-only",
            frames: sequence
                .steps
                .iter()
                .map(|step| StorybookTextSurfaceManifestFrame {
                    index: step.index,
                    name: step.name,
                    png: frame_png_name(step.index, step.name),
                    frame_record_hash: step.artifact.frame_record_hash.clone(),
                    paint_plan_hash: step.artifact.paint_plan_hash.clone(),
                    pixel_hash: step.pixels.pixel_hash.clone(),
                    surface_bounds: step.artifact.record.frame.surface_bounds,
                    viewport_bounds: step.artifact.record.frame.viewport_bounds,
                    scroll_x: step.artifact.record.frame.viewport.scroll_x,
                    scroll_y: step.artifact.record.frame.viewport.scroll_y,
                    raster_identity: step.artifact.record.raster_identity.clone(),
                    text_texture_identities: texture_identities(&step.artifact),
                    star_variation_selector_present: step
                        .artifact
                        .record
                        .raster_identity
                        .contains("⭐️"),
                    color_emoji_texture_present: has_colored_star_texture(&step.artifact),
                    typed_events: step.events.clone(),
                    surface_focused: step.surface_focused,
                    accessibility: step.artifact.record.frame.accessibility.clone(),
                })
                .collect(),
        }
    }
}

pub(super) fn frame_png_name(index: usize, name: &str) -> String {
    format!("{index:02}-{name}.png")
}

fn texture_identities(frame: &TextSurfaceArtifactFrame) -> Vec<String> {
    frame
        .paint_plan
        .operations
        .iter()
        .filter_map(|operation| match &operation.kind {
            TextSurfacePaintOperationKind::Texture { texture, .. } => {
                Some(texture.identity.clone())
            }
            TextSurfacePaintOperationKind::Fill { .. } => None,
        })
        .collect()
}

pub(super) fn has_colored_star_texture(frame: &TextSurfaceArtifactFrame) -> bool {
    frame.paint_plan.operations.iter().any(|operation| {
        let TextSurfacePaintOperationKind::Texture { texture, .. } = &operation.kind else {
            return false;
        };
        texture.identity.contains("⭐️")
            && texture
                .rgba_pixels
                .as_chunks::<RGBA_CHANNELS>()
                .0
                .iter()
                .any(|rgba| rgba[ALPHA_CHANNEL] > 0 && (rgba[0] != rgba[1] || rgba[1] != rgba[2]))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_error_display_covers_each_typed_source() {
        let adapter = TextSurfaceArtifactError::from(EguiTextSurfaceError::FrameNotProduced);
        assert!(adapter.to_string().contains("adapter error"));

        let image = TextSurfaceArtifactError::Image(image::ImageError::IoError(
            std::io::Error::other("image"),
        ));
        assert!(image.to_string().contains("artifact image error"));

        let io = TextSurfaceArtifactError::Io(std::io::Error::other("io"));
        assert!(io.to_string().contains("artifact I/O error"));

        let json = serde_json::from_str::<serde_json::Value>("{")
            .err()
            .map(TextSurfaceArtifactError::Json)
            .map(|error| error.to_string());
        assert!(json.is_some_and(|error| error.contains("artifact JSON error")));

        let contract = TextSurfaceArtifactError::Contract("contract".to_string());
        assert!(contract.to_string().contains("artifact contract failed"));
    }
}
