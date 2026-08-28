use super::model::{EguiTextSurfaceDrawLayer, TextSurfacePaintPlan, TextSurfaceRasterStyle};
use katana_ui_core::render_model::{UiContextMenuAnchor, UiContextMenuRect, UiRect};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceEvent, TextSurfaceFocusRequestResult, TextSurfaceFrameRecord,
    TextSurfaceScrollRequestResult,
};
use katana_ui_core_svg_raster::UiSvgRasterError;
use katana_ui_core_text_raster::PlatformTextRaster;
use katana_ui_core_text_raster::PlatformTextRasterError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextSurfaceFrameRecord {
    pub frame: TextSurfaceFrameRecord,
    pub raster_identity: String,
    pub texture_bounds: UiRect,
    pub placeholder_raster_identity: Option<String>,
    pub placeholder_texture_bounds: Option<UiRect>,
    pub hit_target: String,
    pub layers: Vec<EguiTextSurfaceDrawLayer>,
    pub scroll_request: Option<TextSurfaceScrollRequestResult>,
    pub focus_request: Option<TextSurfaceFocusRequestResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceArtifactFrame {
    pub record: EguiTextSurfaceFrameRecord,
    pub paint_plan: TextSurfacePaintPlan,
    pub events: Vec<TextSurfaceEvent>,
    pub frame_record_hash: String,
    pub paint_plan_hash: String,
}

impl TextSurfaceArtifactFrame {
    pub(super) fn new(
        record: EguiTextSurfaceFrameRecord,
        paint_plan: TextSurfacePaintPlan,
        events: Vec<TextSurfaceEvent>,
    ) -> Result<Self, EguiTextSurfaceError> {
        Ok(Self {
            frame_record_hash: artifact_hash(&record)?,
            paint_plan_hash: artifact_hash(&paint_plan)?,
            record,
            paint_plan,
            events,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EguiTextSurfaceOutput {
    pub record: EguiTextSurfaceFrameRecord,
    pub events: Vec<TextSurfaceEvent>,
    pub artifact: TextSurfaceArtifactFrame,
    /// Adapter-owned invocation fact. Consumers can pass it to the shared menu,
    /// but cannot provide or alter the underlying coordinates.
    pub context_target: Option<TextSurfaceContextTargetAnchor>,
    pub(crate) raster: PlatformTextRaster,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceContextTargetAnchor {
    pub(crate) anchor: UiContextMenuAnchor,
    pub selection: UiTextSelectionRange,
    pub viewport_bounds: UiRect,
}

impl TextSurfaceContextTargetAnchor {
    #[must_use]
    pub fn selection(&self) -> UiTextSelectionRange {
        self.selection
    }

    #[must_use]
    pub fn viewport_bounds(&self) -> UiRect {
        self.viewport_bounds
    }

    #[must_use]
    pub(crate) fn pointer(
        x: i32,
        y: i32,
        selection: UiTextSelectionRange,
        viewport_bounds: UiRect,
    ) -> Self {
        Self {
            anchor: UiContextMenuAnchor::Pointer { x, y },
            selection,
            viewport_bounds,
        }
    }

    #[must_use]
    pub(crate) fn selection_or_caret(
        selection: UiTextSelectionRange,
        bounds: UiRect,
        viewport_bounds: UiRect,
    ) -> Self {
        Self {
            anchor: UiContextMenuAnchor::VirtualRect(UiContextMenuRect::new(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            )),
            selection,
            viewport_bounds,
        }
    }

    #[must_use]
    pub(crate) fn anchor(&self) -> &UiContextMenuAnchor {
        &self.anchor
    }
}

impl EguiTextSurfaceOutput {
    /// Returns the KUC-resolved selection or caret anchor for keyboard and AccessKit invocation.
    #[must_use]
    pub fn keyboard_context_target(&self) -> TextSurfaceContextTargetAnchor {
        let selection = &self.record.frame.selection;
        let bounds = selection.rects.first().copied().unwrap_or(selection.caret);
        TextSurfaceContextTargetAnchor::selection_or_caret(
            selection.range,
            bounds,
            self.record.frame.viewport_bounds,
        )
    }
}

pub(super) fn raster_identity(
    surface: &TextSurface,
    raster_style: &TextSurfaceRasterStyle,
) -> String {
    format!(
        "{}:{}:{:?}:{:?}:{raster_style:?}",
        surface.state().text_area.state_id.as_str(),
        surface.state().text_area.value,
        surface.state().text_area.selection,
        surface.state().text_area.composition,
    )
}

pub(super) fn publish_ime_output(
    ui: &egui::Ui,
    surface: &TextSurface,
    record: &EguiTextSurfaceFrameRecord,
) {
    let state = &surface.state().text_area;
    if !state.focused || state.readonly || state.disabled {
        return;
    }
    ui.output_mut(|output| {
        output.mutable_text_under_cursor = true;
        output.ime = Some(egui::output::IMEOutput {
            purpose: egui::IMEPurpose::Normal,
            rect: egui_rect(record.frame.content_bounds),
            cursor_rect: egui_rect(record.frame.selection.caret),
            should_interrupt_composition: false,
        });
    });
}

pub(super) fn ui_rect(bounds: egui::Rect) -> UiRect {
    UiRect::new(
        bounds.min.x.round() as i32,
        bounds.min.y.round() as i32,
        bounds.width().round().max(0.0) as u32,
        bounds.height().round().max(0.0) as u32,
    )
}

pub(super) fn context_target_from_actual_input(
    ui: &egui::Ui,
    response: &egui::Response,
    record: &EguiTextSurfaceFrameRecord,
) -> (Option<TextSurfaceContextTargetAnchor>, bool) {
    let pointer_target = ui.input(|input| {
        let pointer = &input.pointer;
        let position = pointer.interact_pos().or_else(|| pointer.latest_pos())?;
        ((pointer.secondary_clicked()
            || pointer.secondary_pressed()
            || pointer.secondary_released())
            && response.rect.contains(position))
        .then(|| {
            TextSurfaceContextTargetAnchor::pointer(
                position.x.round() as i32,
                position.y.round() as i32,
                record.frame.selection.range,
                record.frame.viewport_bounds,
            )
        })
    });
    let pointer_invoked = pointer_target.is_some();
    let keyboard_or_accesskit = ui.input(|input| {
        input.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::F10,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers.shift
            )
        }) || input
            .has_accesskit_action_request(response.id, egui::accesskit::Action::ShowContextMenu)
    });
    let target = pointer_target.or_else(|| {
        keyboard_or_accesskit.then(|| {
            let selection = &record.frame.selection;
            TextSurfaceContextTargetAnchor::selection_or_caret(
                selection.range,
                selection.rects.first().copied().unwrap_or(selection.caret),
                record.frame.viewport_bounds,
            )
        })
    });
    (target, pointer_invoked)
}

fn egui_rect(bounds: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EguiTextSurfaceError {
    FrameNotProduced,
    Raster(PlatformTextRasterError),
    Svg(UiSvgRasterError),
    ArtifactSerialization(String),
}

impl From<PlatformTextRasterError> for EguiTextSurfaceError {
    fn from(value: PlatformTextRasterError) -> Self {
        Self::Raster(value)
    }
}

impl From<UiSvgRasterError> for EguiTextSurfaceError {
    fn from(value: UiSvgRasterError) -> Self {
        Self::Svg(value)
    }
}

impl std::fmt::Display for EguiTextSurfaceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrameNotProduced => {
                formatter.write_str("egui did not produce a text surface frame")
            }
            Self::Raster(error) => write!(formatter, "text surface raster failed: {error}"),
            Self::Svg(error) => write!(formatter, "gutter svg raster failed: {error:?}"),
            Self::ArtifactSerialization(error) => {
                write!(
                    formatter,
                    "text surface artifact serialization failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for EguiTextSurfaceError {}

fn artifact_hash(value: &impl Serialize) -> Result<String, EguiTextSurfaceError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| EguiTextSurfaceError::ArtifactSerialization(error.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SerializeFail;

    impl Serialize for SerializeFail {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("closed serialization failure"))
        }
    }

    #[test]
    fn artifact_hash_maps_serialization_failures_to_the_typed_surface_error() {
        assert!(matches!(
            artifact_hash(&SerializeFail),
            Err(EguiTextSurfaceError::ArtifactSerialization(_))
        ));
    }

    #[test]
    fn context_targets_and_error_conversions_cover_the_closed_model_surface() {
        let selection = UiTextSelectionRange::new(2, 4);
        let viewport = UiRect::new(1, 2, 300, 180);
        let pointer = TextSurfaceContextTargetAnchor::pointer(10, 20, selection, viewport);
        assert_eq!(pointer.selection(), selection);
        assert_eq!(pointer.viewport_bounds(), viewport);
        assert!(matches!(
            pointer.anchor(),
            UiContextMenuAnchor::Pointer { x: 10, y: 20 }
        ));

        let virtual_target = TextSurfaceContextTargetAnchor::selection_or_caret(
            selection,
            UiRect::new(5, 6, 7, 8),
            viewport,
        );
        assert!(matches!(
            virtual_target.anchor(),
            UiContextMenuAnchor::VirtualRect(_)
        ));

        let raster = EguiTextSurfaceError::from(PlatformTextRasterError::EmptyText);
        assert!(raster.to_string().contains("text surface raster failed"));
        let svg = EguiTextSurfaceError::from(UiSvgRasterError::EmptySource);
        assert!(svg.to_string().contains("gutter svg raster failed"));
        assert_eq!(
            EguiTextSurfaceError::FrameNotProduced.to_string(),
            "egui did not produce a text surface frame"
        );
        assert!(
            EguiTextSurfaceError::ArtifactSerialization("opaque".to_string())
                .to_string()
                .contains("artifact serialization failed")
        );
    }
}
