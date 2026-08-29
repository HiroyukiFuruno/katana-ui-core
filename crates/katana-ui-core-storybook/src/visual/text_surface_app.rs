use super::text_surface_artifact::{TextSurfacePlanPixels, render_artifact_frame};
use super::text_surface_fixture::{paint_style, raster_style, text_surface_fixture};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::TextSurface;
use katana_ui_core_egui_adapter::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceError, EguiTextSurfaceFrameRecord,
    TextSurfaceArtifactFrame, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};

pub(super) struct TextSurfaceStorybookApp {
    adapter: EguiTextSurfaceAdapter,
    surface: TextSurface,
    raster_style: TextSurfaceRasterStyle,
    paint_style: TextSurfacePaintStyle,
    frames_remaining: Option<usize>,
    pub(super) last_record: Option<EguiTextSurfaceFrameRecord>,
    pub(super) last_artifact: Option<TextSurfaceArtifactFrame>,
    pub(super) last_pixels: Option<TextSurfacePlanPixels>,
    pub(super) last_error: Option<EguiTextSurfaceError>,
}

impl TextSurfaceStorybookApp {
    pub(super) fn new(frames: usize) -> Self {
        Self {
            adapter: EguiTextSurfaceAdapter::default(),
            surface: text_surface_fixture(),
            raster_style: raster_style(),
            paint_style: paint_style(),
            frames_remaining: (frames > 0).then_some(frames),
            last_record: None,
            last_artifact: None,
            last_pixels: None,
            last_error: None,
        }
    }

    pub(super) fn show(&mut self, ui: &mut egui::Ui) {
        match self
            .adapter
            .show(ui, &mut self.surface, &self.raster_style, &self.paint_style)
        {
            Ok(output) => {
                let canvas = actual_root_canvas(ui);
                let pixels = match render_artifact_frame(&output.artifact, canvas) {
                    Ok(pixels) => pixels,
                    Err(error) => {
                        self.last_error = Some(EguiTextSurfaceError::ArtifactSerialization(error));
                        return;
                    }
                };
                self.last_record = Some(output.record);
                self.last_artifact = Some(output.artifact);
                self.last_pixels = Some(pixels);
                self.last_error = None;
            }
            Err(error) => self.last_error = Some(error),
        }
    }
}

fn actual_root_canvas(ui: &egui::Ui) -> UiRect {
    let bounds = ui.ctx().input(|input| input.content_rect());
    UiRect::new(
        bounds.min.x.round() as i32,
        bounds.min.y.round() as i32,
        bounds.width().round() as u32,
        bounds.height().round() as u32,
    )
}

impl eframe::App for TextSurfaceStorybookApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
        let Some(remaining) = self.frames_remaining.as_mut() else {
            return;
        };
        *remaining = remaining.saturating_sub(1);
        if *remaining == 0 {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
