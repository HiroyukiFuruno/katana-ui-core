use super::adapter::EguiStatusBarAdapter;
use super::paint::StatusBarPaint;
use super::types::{
    EguiStatusBarError, EguiStatusBarOutput, StatusBarPaintOperation, StatusBarPaintOperationKind,
    StatusBarPaintPlan, StatusBarPaintTexture, StatusBarRenderStyle,
};
use katana_ui_core::interaction::placement::{
    AnchorKind, Placement, PlacementConsumer, PlacementEngine, PlacementRequest, Rect, Size,
};
use katana_ui_core::molecule::{StatusBar, StatusBarAction};
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiNodeId, UiTextSpan, UiTextSpanStyle};
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
        paint_plan: &mut StatusBarPaintPlan,
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
        let Some(anchor) = self.segment_bounds.get(id.as_str()).copied() else {
            return Ok(());
        };
        let (title, title_texture) =
            self.rasterize_overlay(ui, spec.title(), POPOVER_TITLE_RGBA)?;
        let (body, body_texture) = self.rasterize_overlay(ui, spec.body(), POPOVER_BODY_RGBA)?;
        let frame = egui::Frame::popup(ui.style());
        let frame_margin = frame.total_margin().sum();
        let panel_size = egui::vec2(
            title.size.x.max(body.size.x) + frame_margin.x,
            title.size.y + ui.style().spacing.item_spacing.y + body.size.y + frame_margin.y,
        );
        let viewport = ui.clip_rect();
        let request = PlacementRequest::new(
            AnchorKind::node_rect(UiNodeId::new(id.clone()), placement_rect(anchor)),
            Placement::BottomStart,
            Size::new(
                panel_size.x.ceil().max(1.0) as u32,
                panel_size.y.ceil().max(1.0) as u32,
            ),
            placement_rect(viewport),
        )
        .offset(POPOVER_OFFSET_PX.round() as i32);
        let placement = PlacementEngine::resolve_for(PlacementConsumer::Popover, &request);
        egui::Area::new(self.id.with(("popover", id.as_str())))
            .order(egui::Order::Foreground)
            .default_size(panel_size)
            .constrain_to(viewport)
            .fixed_pos(egui::pos2(
                placement.position.x as f32,
                placement.position.y as f32,
            ))
            .show(ui.ctx(), |ui| {
                frame
                    .show(ui, |ui| {
                        let title =
                            ui.add(egui::Image::from_texture(title).fit_to_original_size(1.0));
                        let body =
                            ui.add(egui::Image::from_texture(body).fit_to_original_size(1.0));
                        paint_plan.operations.push(StatusBarPaintOperation {
                            clip_bounds: StatusBarPaint::ui_rect(title.rect),
                            kind: StatusBarPaintOperationKind::Texture {
                                bounds: StatusBarPaint::ui_rect(title.rect),
                                texture: title_texture,
                            },
                        });
                        paint_plan.operations.push(StatusBarPaintOperation {
                            clip_bounds: StatusBarPaint::ui_rect(body.rect),
                            kind: StatusBarPaintOperationKind::Texture {
                                bounds: StatusBarPaint::ui_rect(body.rect),
                                texture: body_texture,
                            },
                        });
                    })
                    .inner
            });
        Ok(())
    }

    fn rasterize_overlay(
        &mut self,
        ui: &egui::Ui,
        text: &str,
        color_rgba: [u8; RGBA_CHANNEL_COUNT],
    ) -> Result<(egui::load::SizedTexture, StatusBarPaintTexture), EguiStatusBarError> {
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
        let identity = format!(
            concat!("status-bar-overlay:", "{}"),
            hex::encode(Sha256::digest(&pixels))
        );
        let texture = StatusBarPaintTexture {
            identity: identity.clone(),
            width: raster.width as u32,
            height: raster.height as u32,
            rgba_pixels: pixels.clone(),
        };
        let handle = self.textures.texture_for_rgba(
            ui.ctx(),
            &identity,
            raster.width,
            raster.height,
            &pixels,
        );
        Ok((egui::load::SizedTexture::from_handle(&handle), texture))
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

fn placement_rect(rect: egui::Rect) -> Rect {
    let rect = StatusBarPaint::ui_rect(rect);
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}
