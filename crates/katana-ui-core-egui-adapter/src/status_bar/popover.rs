use super::adapter::EguiStatusBarAdapter;
use super::types::{
    EguiStatusBarError, EguiStatusBarOutput, StatusBarPaintOperation, StatusBarPaintOperationKind,
    StatusBarPaintTexture, StatusBarRenderStyle,
};
use katana_ui_core::molecule::{StatusBar, StatusBarAction};
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiTextSpan, UiTextSpanStyle};
use katana_ui_core_text_raster::PlatformTextRasterRequest;
use sha2::{Digest, Sha256};

const POPOVER_OFFSET_PX: f32 = 12.0;
const POPOVER_TITLE_RGBA: [u8; RGBA_CHANNEL_COUNT] = [235, 235, 235, 255];
const POPOVER_BODY_RGBA: [u8; RGBA_CHANNEL_COUNT] = [205, 205, 205, 255];
const POPOVER_LINE_HEIGHT_PX: f32 = 19.0;

impl EguiStatusBarAdapter {
    pub(super) fn paint_open_popover(
        &mut self,
        ui: &egui::Ui,
        status: &StatusBar,
        out: &mut EguiStatusBarOutput,
    ) -> Result<(), EguiStatusBarError> {
        let Some(id) = status.state().open_popover().cloned() else {
            return Ok(());
        };
        let Some(segment) = status.segments().iter().find(|s| s.id() == id) else {
            return Ok(());
        };
        let Some(spec) = segment.popover_spec() else {
            return Ok(());
        };
        let area = egui::Area::new(self.id.with(("popover", id.as_str())))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(POPOVER_OFFSET_PX, POPOVER_OFFSET_PX))
            .show(ui.ctx(), |ui| -> Result<(), EguiStatusBarError> {
                egui::Frame::popup(ui.style())
                    .show(ui, |ui| -> Result<(), EguiStatusBarError> {
                        let title =
                            self.rasterize_overlay(ui, spec.title(), POPOVER_TITLE_RGBA, out)?;
                        let body =
                            self.rasterize_overlay(ui, spec.body(), POPOVER_BODY_RGBA, out)?;
                        ui.add(egui::Image::from_texture(title).fit_to_original_size(1.0));
                        ui.add(egui::Image::from_texture(body).fit_to_original_size(1.0));
                        Ok(())
                    })
                    .inner?;
                Ok(())
            });
        area.inner
    }

    fn rasterize_overlay(
        &mut self,
        ui: &egui::Ui,
        text: &str,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
        _out: &mut EguiStatusBarOutput,
    ) -> Result<egui::load::SizedTexture, EguiStatusBarError> {
        let scale = ui.ctx().pixels_per_point();
        let style = StatusBarRenderStyle::standard();
        let raster = self.text_rasterizer.rasterize(&PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                text,
                UiTextSpanStyle {
                    color_rgba,
                    ..UiTextSpanStyle::default()
                },
            ),
            font: style.font,
            fallback_color_rgba: color_rgba,
            line_height_px: POPOVER_LINE_HEIGHT_PX,
            max_width_px: None,
            scale_factor: scale,
        })?;
        let pixels: Vec<u8> = raster.rgba_pixels.iter().flatten().copied().collect();
        let identity = format!("status-bar-overlay:{:x}", Sha256::digest(text.as_bytes()));
        if let Some(plan) = self.last_paint_plan.as_mut() {
            let bounds = katana_ui_core::render_model::UiRect::new(
                POPOVER_OFFSET_PX as i32,
                POPOVER_OFFSET_PX as i32,
                raster.width as u32,
                raster.height as u32,
            );
            plan.operations.push(StatusBarPaintOperation {
                clip_bounds: bounds,
                kind: StatusBarPaintOperationKind::Texture {
                    bounds,
                    texture: StatusBarPaintTexture {
                        identity: identity.clone(),
                        width: raster.width as u32,
                        height: raster.height as u32,
                        rgba_pixels: pixels.clone(),
                    },
                },
            });
        }
        let handle = self.textures.texture_for_rgba(
            ui.ctx(),
            &identity,
            raster.width,
            raster.height,
            &pixels,
        );
        Ok(egui::load::SizedTexture::from_handle(&handle))
    }

    pub(super) fn close_popover(
        &mut self,
        ui: &egui::Ui,
        status: &mut StatusBar,
        id: &str,
        out: &mut EguiStatusBarOutput,
    ) {
        out.events
            .extend(status.apply_action(&StatusBarAction::ClosePopover { id: id.into() }));
        ui.memory_mut(|memory| memory.request_focus(self.id.with(id)));
    }
}
