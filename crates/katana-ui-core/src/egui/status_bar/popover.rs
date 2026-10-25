use super::adapter::EguiStatusBarAdapter;
use super::paint::StatusBarPaint;
use super::types::{
    EguiStatusBarError, EguiStatusBarOutput, StatusBarPaintOperation, StatusBarPaintOperationKind,
    StatusBarPaintPlan, StatusBarPaintTexture, StatusBarRenderStyle,
};
use crate::egui::texture_cache::RgbaTextureCache;
use crate::egui::texture_tiles::rgba_texture_tiles;
use crate::interaction::placement::{
    AnchorKind, Placement, PlacementConsumer, PlacementEngine, PlacementRequest, Rect, Size,
};
use crate::molecule::{StatusBar, StatusBarAction};
use crate::render_model::{RGBA_CHANNEL_COUNT, UiNodeId, UiTextSpan, UiTextSpanStyle};
use crate::text_raster::PlatformTextRasterRequest;
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
        let (title_size, title_texture) =
            self.rasterize_overlay(ui, spec.title(), POPOVER_TITLE_RGBA)?;
        let (body_size, body_texture) =
            self.rasterize_overlay(ui, spec.body(), POPOVER_BODY_RGBA)?;
        let frame = egui::Frame::popup(ui.style());
        let frame_margin = frame.total_margin().sum();
        let panel_size = egui::vec2(
            title_size.x.max(body_size.x) + frame_margin.x,
            title_size.y + ui.style().spacing.item_spacing.y + body_size.y + frame_margin.y,
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
        let texture_cache = &mut self.textures;
        let parent_enabled = ui.is_enabled();
        egui::Area::new(self.id.with(("popover", id.as_str())))
            .order(egui::Order::Foreground)
            .default_size(panel_size)
            .constrain_to(viewport)
            .fixed_pos(egui::pos2(
                placement.position.x as f32,
                placement.position.y as f32,
            ))
            .enabled(parent_enabled)
            .show(ui.ctx(), |ui| {
                frame
                    .show(ui, |ui| {
                        let title = ui.allocate_exact_size(title_size, egui::Sense::hover()).0;
                        let body = ui.allocate_exact_size(body_size, egui::Sense::hover()).0;
                        let max_texture_side =
                            ui.ctx().input(|input| input.max_texture_side).max(1);
                        paint_overlay_tiles(
                            ui,
                            texture_cache,
                            title,
                            &title_texture,
                            max_texture_side,
                            paint_plan,
                        );
                        paint_overlay_tiles(
                            ui,
                            texture_cache,
                            body,
                            &body_texture,
                            max_texture_side,
                            paint_plan,
                        );
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
    ) -> Result<(egui::Vec2, StatusBarPaintTexture), EguiStatusBarError> {
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
            identity,
            width: raster.width as u32,
            height: raster.height as u32,
            rgba_pixels: pixels,
        };
        Ok((
            egui::vec2(raster.width as f32, raster.height as f32),
            texture,
        ))
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

fn paint_overlay_tiles(
    ui: &egui::Ui,
    textures: &mut RgbaTextureCache,
    bounds: egui::Rect,
    texture: &StatusBarPaintTexture,
    max_texture_side: usize,
    paint_plan: &mut StatusBarPaintPlan,
) {
    let bounds = StatusBarPaint::ui_rect(bounds);
    for (tile_bounds, tile) in rgba_texture_tiles(
        bounds,
        &texture.identity,
        texture.width,
        texture.height,
        &texture.rgba_pixels,
        max_texture_side,
    ) {
        let handle = textures.texture_for_rgba(
            ui.ctx(),
            &tile.identity,
            tile.width as usize,
            tile.height as usize,
            &tile.rgba_pixels,
        );
        let rect = egui::Rect::from_min_size(
            egui::pos2(tile_bounds.x as f32, tile_bounds.y as f32),
            egui::vec2(tile_bounds.width as f32, tile_bounds.height as f32),
        );
        ui.painter().image(
            handle.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        paint_plan.operations.push(StatusBarPaintOperation {
            clip_bounds: bounds,
            kind: StatusBarPaintOperationKind::Texture {
                bounds: tile_bounds,
                texture: StatusBarPaintTexture {
                    identity: tile.identity,
                    width: tile.width,
                    height: tile.height,
                    rgba_pixels: tile.rgba_pixels,
                },
            },
        });
    }
}

fn placement_rect(rect: egui::Rect) -> Rect {
    let rect = StatusBarPaint::ui_rect(rect);
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}
