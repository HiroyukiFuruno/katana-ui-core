use super::support::ui_rect;
use super::{
    CORNER_RADIUS_PX, DRAG_GHOST_OFFSET_PX, DRAG_GHOST_RGBA, PREEDIT_RGBA, RGBA_ALPHA_INDEX,
    TAB_GAP_PX, TAB_PADDING_PX, TabStripPaintOperation, TabStripPaintOperationKind,
    TabStripPaintTexture, TabStripRetainedError, TabStripRetainedState, TabStripText,
};
use sha2::{Digest, Sha256};

pub(super) struct TabStripDragFeedback<'a> {
    pub(super) label: &'a TabStripText,
    pub(super) pointer: egui::Pos2,
    pub(super) indicator: Option<egui::Rect>,
    pub(super) ghost_width: Option<f32>,
}

#[derive(Clone, Copy)]
pub(super) enum TabStripDropIndicator {
    Before,
    After,
    EndOfStrip,
}

pub(super) fn drop_indicator(
    kind: TabStripDropIndicator,
    target_bounds: egui::Rect,
    strip_bounds: egui::Rect,
    pointer_x: f32,
) -> egui::Rect {
    match kind {
        TabStripDropIndicator::Before => egui::Rect::from_min_size(
            egui::pos2(target_bounds.min.x - 1.0, target_bounds.min.y),
            egui::vec2(2.0, target_bounds.height()),
        ),
        TabStripDropIndicator::After => egui::Rect::from_min_size(
            egui::pos2(target_bounds.max.x - 1.0, target_bounds.min.y),
            egui::vec2(2.0, target_bounds.height()),
        ),
        TabStripDropIndicator::EndOfStrip => egui::Rect::from_min_size(
            egui::pos2(pointer_x - 1.0, strip_bounds.min.y + TAB_GAP_PX),
            egui::vec2(2.0, (strip_bounds.height() - TAB_GAP_PX * 2.0).max(1.0)),
        ),
    }
}

impl TabStripRetainedState {
    pub(super) fn paint_drag_feedback(
        &mut self,
        ui: &egui::Ui,
        feedback: TabStripDragFeedback<'_>,
        strip_bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let raster = self
            .rasterizer
            .rasterize(feedback.label, ui.ctx().pixels_per_point())
            .map_err(TabStripRetainedError::Raster)?;
        let fingerprint = hex::encode(Sha256::digest(&raster.rgba_pixels));
        let texture = TabStripPaintTexture {
            identity: format!("tab-strip-drag-ghost:{fingerprint}"),
            width: raster.width,
            height: raster.height,
            rgba_pixels: raster.rgba_pixels,
        };
        let ghost = egui::Rect::from_min_size(
            feedback.pointer + egui::vec2(DRAG_GHOST_OFFSET_PX, DRAG_GHOST_OFFSET_PX),
            egui::vec2(
                feedback
                    .ghost_width
                    .unwrap_or(texture.width as f32 + TAB_PADDING_PX * 2.0),
                (strip_bounds.height() - TAB_GAP_PX * 2.0).max(1.0),
            ),
        );
        ui.painter().rect_filled(
            ghost,
            CORNER_RADIUS_PX,
            egui::Color32::from_rgba_unmultiplied(
                DRAG_GHOST_RGBA[0],
                DRAG_GHOST_RGBA[1],
                DRAG_GHOST_RGBA[2],
                DRAG_GHOST_RGBA[RGBA_ALPHA_INDEX],
            ),
        );
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(strip_bounds),
            kind: TabStripPaintOperationKind::Fill {
                bounds: ui_rect(ghost),
                color_rgba: DRAG_GHOST_RGBA,
            },
        });
        let text_bounds = egui::Rect::from_min_size(
            egui::pos2(
                ghost.min.x + TAB_PADDING_PX,
                ghost.center().y - texture.height as f32 / 2.0,
            ),
            egui::vec2(texture.width as f32, texture.height as f32),
        );
        self.paint_overlay_texture(ui, operations, strip_bounds, &texture, text_bounds);
        if let Some(indicator) = feedback.indicator {
            ui.painter().rect_filled(
                indicator,
                1.0,
                egui::Color32::from_rgb(PREEDIT_RGBA[0], PREEDIT_RGBA[1], PREEDIT_RGBA[2]),
            );
            operations.push(TabStripPaintOperation {
                clip_bounds: ui_rect(strip_bounds),
                kind: TabStripPaintOperationKind::Fill {
                    bounds: ui_rect(indicator),
                    color_rgba: PREEDIT_RGBA,
                },
            });
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "drag_feedback_tests.rs"]
mod tests;
