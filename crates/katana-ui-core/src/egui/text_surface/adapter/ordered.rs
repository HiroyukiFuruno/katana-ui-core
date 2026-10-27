use super::super::artifact_model::{EguiTextSurfaceError, EguiTextSurfaceOutput};
use super::super::events::OrderedKeyboardInput;
use super::super::model::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use super::super::paint::paint_surface;
use crate::text_surface::TextSurface;

impl EguiTextSurfaceAdapter {
    pub fn show_with_input_policy(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
        input_policy: &EguiTextSurfaceInputPolicy,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        let output = self.show_with_input_policy_unpainted(
            ui,
            surface,
            raster_style,
            paint_style,
            input_policy,
        )?;
        paint_surface(ui, &mut self.textures, &output.artifact.paint_plan);
        Ok(output)
    }

    pub(crate) fn show_with_input_policy_unpainted(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
        input_policy: &EguiTextSurfaceInputPolicy,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        self.show_with_ordered_input_unpainted(
            ui,
            surface,
            raster_style,
            paint_style,
            input_policy,
            None,
        )
    }

    pub(crate) fn show_with_ordered_input(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
        input_policy: &EguiTextSurfaceInputPolicy,
        ordered: &mut dyn OrderedKeyboardInput,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        let output = self.show_with_ordered_input_unpainted(
            ui,
            surface,
            raster_style,
            paint_style,
            input_policy,
            Some(ordered),
        )?;
        paint_surface(ui, &mut self.textures, &output.artifact.paint_plan);
        Ok(output)
    }
}
