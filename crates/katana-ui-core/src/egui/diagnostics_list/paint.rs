//! Pure paint-plan helpers for the diagnostics surface.

use super::adapter::EguiDiagnosticsListAdapter;
use super::types::{DiagnosticsListPaintOperation, DiagnosticsListPaintOperationKind};
use super::types::{
    DiagnosticsListPaintPlan, DiagnosticsListPaintTexture, DiagnosticsListRasterEvidence,
    DiagnosticsListStyle, EguiDiagnosticsListError,
};
use crate::molecule::CodeDiffLineKind;
use crate::render_model::UiRect;
use crate::render_model::{UiTextSpan, UiTextSpanStyle};
use crate::text_raster::PlatformTextRasterRequest;
use sha2::{Digest, Sha256};

pub(crate) const DIAGNOSTICS_LEFT_INSET: f32 = 8.0;
pub(crate) const DIAGNOSTICS_SMALL_INSET: f32 = 4.0;
pub(crate) const DIAGNOSTICS_DOUBLE_INSET: f32 = 4.0;
pub(crate) const DIAGNOSTICS_SCOPE_LABEL_PADDING: f32 = 20.0;
pub(crate) const DIAGNOSTICS_FILTER_PADDING: f32 = 16.0;
pub(crate) const DIAGNOSTICS_DISCLOSURE_WIDTH: f32 = 24.0;
pub(crate) const DIAGNOSTICS_DISCLOSURE_HEIGHT_INSET: f32 = 6.0;
pub(crate) const DIAGNOSTICS_DISCLOSURE_TOP_INSET: f32 = 3.0;
pub(crate) const DIAGNOSTICS_PREVIEW_LEFT_INSET: f32 = 28.0;
pub(crate) const DIAGNOSTICS_PREVIEW_RIGHT_INSET: f32 = 32.0;
pub(crate) const DIAGNOSTICS_QUICKFIX_RIGHT_INSET: f32 = 116.0;
pub(crate) const DIAGNOSTICS_QUICKFIX_WIDTH: f32 = 108.0;
pub(crate) const DIAGNOSTICS_TEXT_LINE_HEIGHT_SCALE: f32 = 1.45;
pub(crate) const DIAGNOSTICS_DEFAULT_ROW_HEIGHT: f32 = 34.0;
const COLOR_CHANNEL_COUNT: usize = 4;
const ALPHA_CHANNEL_INDEX: usize = 3;

pub(crate) struct DiagnosticsPaint;

impl DiagnosticsPaint {
    pub(crate) fn fill(
        clip_bounds: UiRect,
        bounds: UiRect,
        color_rgba: [u8; COLOR_CHANNEL_COUNT],
    ) -> DiagnosticsListPaintOperation {
        DiagnosticsListPaintOperation {
            clip_bounds,
            kind: DiagnosticsListPaintOperationKind::Fill { bounds, color_rgba },
        }
    }

    #[must_use]
    pub(crate) fn egui_rect(rect: UiRect) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(rect.x as f32, rect.y as f32),
            egui::vec2(rect.width as f32, rect.height as f32),
        )
    }

    #[must_use]
    pub(crate) fn ui_rect(rect: egui::Rect) -> UiRect {
        UiRect::new(
            rect.min.x.round() as i32,
            rect.min.y.round() as i32,
            rect.width().round().max(0.0) as u32,
            rect.height().round().max(0.0) as u32,
        )
    }

    #[must_use]
    pub(crate) fn color(rgba: [u8; COLOR_CHANNEL_COUNT]) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[ALPHA_CHANNEL_INDEX])
    }
}

impl EguiDiagnosticsListAdapter {
    pub(super) fn paint_text(
        &mut self,
        plan: &mut DiagnosticsListPaintPlan,
        bounds: egui::Rect,
        text: &str,
        style: &DiagnosticsListStyle,
        scale: f32,
    ) -> Result<(), EguiDiagnosticsListError> {
        let raster = self.text_rasterizer.rasterize(&PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                text,
                UiTextSpanStyle {
                    color_rgba: style.text,
                    ..UiTextSpanStyle::default()
                },
            ),
            font: style.font.clone(),
            fallback_color_rgba: style.text,
            line_height_px: style.font.size * DIAGNOSTICS_TEXT_LINE_HEIGHT_SCALE,
            max_width_px: Some(bounds.width() * scale),
            scale_factor: scale,
        })?;
        let pixels: Vec<u8> = raster.rgba_pixels.iter().flatten().copied().collect();
        let pixel_hash = hex::encode(Sha256::digest(&pixels));
        self.raster_evidence.push(DiagnosticsListRasterEvidence {
            text: text.to_string(),
            width: raster.width as u32,
            height: raster.height as u32,
            chromatic_pixel_count: raster.chromatic_pixel_count(),
            sha256: pixel_hash.clone(),
        });
        let image = egui::Rect::from_min_size(
            bounds.left_top() + egui::vec2(DIAGNOSTICS_SMALL_INSET, DIAGNOSTICS_SMALL_INSET),
            egui::vec2(raster.width as f32 / scale, raster.height as f32 / scale),
        );
        plan.operations.push(DiagnosticsListPaintOperation {
            clip_bounds: DiagnosticsPaint::ui_rect(bounds),
            kind: DiagnosticsListPaintOperationKind::Texture {
                bounds: DiagnosticsPaint::ui_rect(image),
                texture: DiagnosticsListPaintTexture {
                    identity: format!(concat!("diagnostics-text:", "{}"), pixel_hash),
                    width: raster.width as u32,
                    height: raster.height as u32,
                    rgba_pixels: pixels,
                },
            },
        });
        Ok(())
    }

    pub(super) fn text_width(
        &mut self,
        text: &str,
        style: &DiagnosticsListStyle,
        scale: f32,
    ) -> Result<f32, EguiDiagnosticsListError> {
        Ok(self
            .text_rasterizer
            .rasterize(&PlatformTextRasterRequest::from_text(
                text,
                style.font.clone(),
                style.text,
            ))?
            .width as f32
            / scale)
    }

    pub(super) fn paint_plan(&mut self, ui: &egui::Ui, plan: &DiagnosticsListPaintPlan) {
        for operation in &plan.operations {
            let painter = ui
                .painter()
                .with_clip_rect(DiagnosticsPaint::egui_rect(operation.clip_bounds));
            match &operation.kind {
                DiagnosticsListPaintOperationKind::Fill { bounds, color_rgba } => {
                    painter.rect_filled(
                        DiagnosticsPaint::egui_rect(*bounds),
                        0.0,
                        DiagnosticsPaint::color(*color_rgba),
                    );
                }
                DiagnosticsListPaintOperationKind::Texture { bounds, texture } => {
                    let handle = self.textures.texture_for_rgba(
                        ui.ctx(),
                        &texture.identity,
                        texture.width as usize,
                        texture.height as usize,
                        &texture.rgba_pixels,
                    );
                    painter.image(
                        handle.id(),
                        DiagnosticsPaint::egui_rect(*bounds),
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }

    pub(super) fn row_height(
        &self,
        item: &crate::molecule::DiagnosticItem,
        expanded_ids: &std::collections::BTreeSet<crate::molecule::DiagnosticId>,
        style: &DiagnosticsListStyle,
    ) -> f32 {
        style.row_height
            + if expanded_ids.contains(&item.id) {
                Self::item_preview_height(item, style)
            } else {
                0.0
            }
    }

    pub(super) fn item_preview_height(
        item: &crate::molecule::DiagnosticItem,
        style: &DiagnosticsListStyle,
    ) -> f32 {
        let line_count = item
            .fix_preview
            .as_ref()
            .map_or(1, |preview| preview.diff.lines().len().max(1));
        style.preview_padding + line_count as f32 * style.preview_line_height
    }

    pub(super) fn paint_preview(
        &mut self,
        plan: &mut DiagnosticsListPaintPlan,
        viewport: egui::Rect,
        bounds: egui::Rect,
        preview: &crate::molecule::DiagnosticFixPreview,
        style: &DiagnosticsListStyle,
        scale: f32,
    ) -> Result<(), EguiDiagnosticsListError> {
        let lines = preview.diff.lines();
        if lines.is_empty() {
            self.paint_text(plan, bounds, "差分なし", style, scale)?;
            return Ok(());
        }
        for (line_index, line) in lines.iter().enumerate() {
            let line_bounds = egui::Rect::from_min_size(
                egui::pos2(
                    bounds.left(),
                    bounds.top() + line_index as f32 * style.preview_line_height,
                ),
                egui::vec2(bounds.width(), style.preview_line_height),
            );
            let line_color = match line.kind {
                CodeDiffLineKind::Added => style.preview_added,
                CodeDiffLineKind::Removed => style.preview_removed,
                CodeDiffLineKind::Context | CodeDiffLineKind::Placeholder => style.preview_context,
            };
            plan.operations.push(DiagnosticsPaint::fill(
                DiagnosticsPaint::ui_rect(viewport),
                DiagnosticsPaint::ui_rect(line_bounds),
                line_color,
            ));
            let old_number = line
                .old_number
                .map_or_else(|| "    ".to_string(), |value| format!("{value:>4}"));
            let new_number = line
                .new_number
                .map_or_else(|| "    ".to_string(), |value| format!("{value:>4}"));
            let prefix = match line.kind {
                CodeDiffLineKind::Added => "+",
                CodeDiffLineKind::Removed => "-",
                CodeDiffLineKind::Context => " ",
                CodeDiffLineKind::Placeholder => "…",
            };
            let line_text = format!("{old_number} {new_number} {prefix} {}", line.text);
            self.paint_text(plan, line_bounds, &line_text, style, scale)?;
        }
        Ok(())
    }
}
