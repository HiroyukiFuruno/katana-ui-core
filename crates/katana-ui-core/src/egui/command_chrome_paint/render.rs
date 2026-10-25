use super::super::command_chrome_artifact::{
    CommandChromePaintOperationKind, CommandChromePaintPlan,
};
use crate::egui::texture_cache::RgbaTextureCache;
use crate::egui::texture_tiles::rgba_texture_tiles;
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};

pub(super) fn paint_command_chrome(
    ui: &egui::Ui,
    cache: &mut RgbaTextureCache,
    plan: &CommandChromePaintPlan,
) {
    for operation in &plan.operations {
        let painter = ui
            .painter()
            .with_clip_rect(egui_rect(operation.clip_bounds));
        match &operation.kind {
            CommandChromePaintOperationKind::Fill { bounds, color_rgba } => {
                painter.rect_filled(egui_rect(*bounds), 0.0, color(*color_rgba));
            }
            CommandChromePaintOperationKind::RoundedFill {
                bounds,
                color_rgba,
                radius_px,
            } => {
                painter.rect_filled(egui_rect(*bounds), *radius_px as f32, color(*color_rgba));
            }
            CommandChromePaintOperationKind::Texture { bounds, texture } => {
                let max_texture_side = ui.ctx().input(|input| input.max_texture_side).max(1);
                for (tile_bounds, tile) in rgba_texture_tiles(
                    *bounds,
                    &texture.identity,
                    texture.width,
                    texture.height,
                    &texture.rgba_pixels,
                    max_texture_side,
                ) {
                    let texture = cache.texture_for_rgba(
                        ui.ctx(),
                        &tile.identity,
                        tile.width as usize,
                        tile.height as usize,
                        &tile.rgba_pixels,
                    );
                    painter.image(
                        texture.id(),
                        egui_rect(tile_bounds),
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }
}

fn egui_rect(bounds: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    )
}

fn color([red, green, blue, alpha]: [u8; RGBA_CHANNEL_COUNT]) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}
