use super::adapter::EguiSourceAddressStripAdapter;
use super::types::{
    EguiSourceAddressStripError, SourceAddressLabelRasterEvidence, SourceAddressPaintOperation,
    SourceAddressPaintOperationKind, SourceAddressPaintPlan, SourceAddressPaintTexture,
    SourceAddressRenderStyle,
};
use crate::egui::text_surface::TextSurfacePaintOperationKind;
use crate::render_model::{UiTextSpan, UiTextSpanStyle};
use crate::text_raster::{PlatformTextMetricsRequest, PlatformTextRasterRequest};
use sha2::{Digest, Sha256};

const LABEL_LINE_HEIGHT_MULTIPLIER: f32 = 1.45;

pub(crate) struct Raster;

impl Raster {
    pub(crate) fn raster_button(
        adapter: &mut EguiSourceAddressStripAdapter,
        paint_plan: &mut SourceAddressPaintPlan,
        ui: &mut egui::Ui,
        label: &str,
        tooltip: &str,
        enabled: bool,
        style: &SourceAddressRenderStyle,
    ) -> Result<egui::Response, EguiSourceAddressStripError> {
        let scale = ui.ctx().pixels_per_point();
        let request = PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                label,
                UiTextSpanStyle {
                    color_rgba: style.label_color_rgba,
                    ..UiTextSpanStyle::default()
                },
            ),
            font: style.label_font.clone(),
            fallback_color_rgba: style.label_color_rgba,
            line_height_px: style.label_font.size * LABEL_LINE_HEIGHT_MULTIPLIER,
            max_width_px: None,
            scale_factor: scale,
        };
        let measured = adapter.metrics.borrow_mut().measure_text(
            &mut adapter.text_rasterizer,
            &PlatformTextMetricsRequest::from_text(label, style.label_font.clone(), scale),
        )?;
        let mut request = request;
        request.line_height_px = measured.line_height_px / scale.max(1.0);
        let raster = adapter.text_rasterizer.rasterize(&request)?;
        let width = (raster.width as f32 / scale).ceil() + style.button_padding_px as f32 * 2.0;
        let height = (raster.height as f32 / scale)
            .ceil()
            .max(style.input_height_px as f32)
            + style.button_padding_px as f32;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
        let fill = if enabled {
            style.button_background_rgba
        } else {
            style.button_disabled_rgba
        };
        ui.painter()
            .rect_filled(rect, 0.0, super::paint::Paint::color(fill));
        let pixels: Vec<u8> = raster.rgba_pixels.iter().flatten().copied().collect();
        let label_fingerprint = Self::source_address_label_fingerprint(label);
        let raster_fingerprint = hex::encode(Sha256::digest(&pixels));
        let identity = format!(
            "source-address-label:{label_fingerprint}:{:?}:{scale}:{raster_fingerprint}",
            style.label_font,
        );
        adapter
            .last_label_rasters
            .push(SourceAddressLabelRasterEvidence {
                label_fingerprint,
                width: raster.width as u32,
                height: raster.height as u32,
                chromatic_pixel_count: raster.chromatic_pixel_count(),
                sha256: raster_fingerprint,
            });
        let image_size = egui::vec2(raster.width as f32 / scale, raster.height as f32 / scale);
        let image_rect = egui::Rect::from_center_size(rect.center(), image_size);
        paint_plan.operations.push(SourceAddressPaintOperation {
            clip_bounds: super::paint::Paint::ui_rect(rect),
            kind: SourceAddressPaintOperationKind::Fill {
                bounds: super::paint::Paint::ui_rect(rect),
                color_rgba: fill,
            },
        });
        paint_plan.operations.push(SourceAddressPaintOperation {
            clip_bounds: super::paint::Paint::ui_rect(rect),
            kind: SourceAddressPaintOperationKind::Texture {
                bounds: super::paint::Paint::ui_rect(image_rect),
                texture: SourceAddressPaintTexture {
                    identity,
                    width: raster.width as u32,
                    height: raster.height as u32,
                    rgba_pixels: pixels,
                },
            },
        });
        Ok(response.on_hover_text(tooltip))
    }

    fn source_address_label_fingerprint(label: &str) -> String {
        hex::encode(Sha256::digest(label.as_bytes()))
    }

    pub(crate) fn sanitize_input_kind(
        kind: TextSurfacePaintOperationKind,
    ) -> TextSurfacePaintOperationKind {
        match kind {
            TextSurfacePaintOperationKind::Fill { bounds, color_rgba } => {
                TextSurfacePaintOperationKind::Fill { bounds, color_rgba }
            }
            TextSurfacePaintOperationKind::Texture {
                bounds,
                mut texture,
            } => {
                texture.identity = format!(
                    "source-address-input:{}",
                    hex::encode(Sha256::digest(texture.identity))
                );
                TextSurfacePaintOperationKind::Texture { bounds, texture }
            }
        }
    }
}
