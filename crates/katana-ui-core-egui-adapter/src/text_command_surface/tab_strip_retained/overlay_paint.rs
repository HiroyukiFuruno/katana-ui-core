use super::support::ui_rect;
use super::{
    DRAG_GHOST_OFFSET_PX, TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintTexture,
    TabStripRetainedError, TabStripRetainedState,
};

impl TabStripRetainedState {
    pub(super) fn paint_overlay_label(
        &mut self,
        ui: &egui::Ui,
        operations: &mut Vec<TabStripPaintOperation>,
        clip_bounds: egui::Rect,
        text: &super::tab_strip_projection_lease::TabStripText,
        row: egui::Rect,
        prefix: &str,
        index: usize,
    ) -> Result<(), TabStripRetainedError> {
        let raster = self
            .rasterizer
            .rasterize(text, ui.ctx().pixels_per_point())
            .map_err(TabStripRetainedError::Raster)?;
        let texture = TabStripPaintTexture {
            identity: format!("tab-strip-overlay:{prefix}:{index}"),
            width: raster.width,
            height: raster.height,
            rgba_pixels: raster.rgba_pixels,
        };
        let text_bounds = egui::Rect::from_min_size(
            egui::pos2(
                row.min.x + DRAG_GHOST_OFFSET_PX,
                row.center().y - texture.height as f32 / 2.0,
            ),
            egui::vec2(texture.width as f32, texture.height as f32),
        );
        self.paint_overlay_texture(ui, operations, clip_bounds, &texture, text_bounds);
        Ok(())
    }

    pub(super) fn paint_overlay_texture(
        &mut self,
        ui: &egui::Ui,
        operations: &mut Vec<TabStripPaintOperation>,
        clip_bounds: egui::Rect,
        texture: &TabStripPaintTexture,
        bounds: egui::Rect,
    ) {
        let handle = self.textures.texture_for_rgba(
            ui.ctx(),
            &texture.identity,
            texture.width as usize,
            texture.height as usize,
            &texture.rgba_pixels,
        );
        ui.painter().image(
            handle.id(),
            bounds,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(clip_bounds),
            kind: TabStripPaintOperationKind::Texture {
                bounds: ui_rect(bounds),
                texture: texture.clone(),
            },
        });
    }
}

#[cfg(test)]
#[path = "overlay_paint_tests.rs"]
mod tests;
