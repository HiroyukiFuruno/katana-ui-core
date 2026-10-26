use super::support::{rect_from_ui_rect, ui_rect};
use super::{
    CORNER_RADIUS_PX, DISABLED_TAB_RGBA, ICON_DISABLED_RGB, ICON_ENABLED_RGB, INACTIVE_TAB_RGBA,
    NAVIGATION_CONTROL_WIDTH_PX, PRIMARY_TEXT_RGBA, RGBA_ALPHA_INDEX, RgbaColor, TAB_GAP_PX,
    TabStripIconControl, TabStripPaintOperation, TabStripPaintOperationKind, TabStripRetainedError,
    TabStripRetainedState,
};
use crate::egui::texture_cache::RgbaTextureCache;
use crate::egui::texture_tiles::rgba_texture_tiles;
use crate::render_model::UiRect;

pub(super) fn paint_texture_tiled(
    painter: &egui::Painter,
    context: &egui::Context,
    cache: &mut RgbaTextureCache,
    bounds: UiRect,
    texture: &super::TabStripPaintTexture,
) {
    let max_texture_side = context.input(|input| input.max_texture_side).max(1);
    for (tile_bounds, tile) in rgba_texture_tiles(
        bounds,
        &texture.identity,
        texture.width,
        texture.height,
        &texture.rgba_pixels,
        max_texture_side,
    ) {
        let handle = cache.texture_for_rgba(
            context,
            &tile.identity,
            tile.width as usize,
            tile.height as usize,
            &tile.rgba_pixels,
        );
        painter.image(
            handle.id(),
            rect_from_ui_rect(tile_bounds),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

impl TabStripRetainedState {
    pub(super) fn render_icon_control(
        &mut self,
        ui: &mut egui::Ui,
        control: TabStripIconControl<'_>,
        x: &mut f32,
        bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let rect = egui::Rect::from_min_size(
            egui::pos2(*x, bounds.min.y + TAB_GAP_PX),
            egui::vec2(
                NAVIGATION_CONTROL_WIDTH_PX,
                (bounds.height() - TAB_GAP_PX * 2.0).max(1.0),
            ),
        );
        let response = ui.interact(
            rect,
            self.routes.interaction_id(ui.id(), control.path),
            if control.enabled {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            },
        );
        let response_id = response.id;
        self.routes.register_response(
            control.path,
            response_id,
            ui_rect(rect),
            &control.presentation.accessibility_label.value,
            !control.enabled,
        );
        self.publish_response_accesskit(ui, response_id);
        self.forward_response_activation(ui, &response)?;
        response.on_hover_text(&control.presentation.tooltip.value);
        let texture = self.raster_icon(
            control.icon,
            if control.enabled {
                RgbaColor::new(
                    ICON_ENABLED_RGB[0],
                    ICON_ENABLED_RGB[1],
                    ICON_ENABLED_RGB[2],
                    PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
                )
            } else {
                RgbaColor::new(
                    ICON_DISABLED_RGB[0],
                    ICON_DISABLED_RGB[1],
                    ICON_DISABLED_RGB[2],
                    PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
                )
            },
        )?;
        let background = if control.enabled {
            INACTIVE_TAB_RGBA
        } else {
            DISABLED_TAB_RGBA
        };
        ui.painter().rect_filled(
            rect,
            CORNER_RADIUS_PX,
            egui::Color32::from_rgba_unmultiplied(
                background[0],
                background[1],
                background[2],
                background[RGBA_ALPHA_INDEX],
            ),
        );
        let icon_rect = egui::Rect::from_center_size(
            rect.center(),
            egui::vec2(texture.width as f32, texture.height as f32),
        );
        paint_texture_tiled(
            ui.painter(),
            ui.ctx(),
            &mut self.textures,
            ui_rect(icon_rect),
            &texture,
        );
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(bounds),
            kind: TabStripPaintOperationKind::Fill {
                bounds: ui_rect(rect),
                color_rgba: background,
            },
        });
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(bounds),
            kind: TabStripPaintOperationKind::Texture {
                bounds: ui_rect(icon_rect),
                texture,
            },
        });
        *x += NAVIGATION_CONTROL_WIDTH_PX + TAB_GAP_PX;
        Ok(())
    }
}
