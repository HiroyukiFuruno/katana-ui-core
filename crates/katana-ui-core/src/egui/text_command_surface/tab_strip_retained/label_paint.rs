use super::support::ui_rect;
use super::tab_paint::paint_texture_tiled;
use super::{
    ACTIVE_TAB_RGBA, CORNER_RADIUS_PX, ICON_SIZE_PX, INACTIVE_TAB_RGBA, PRIMARY_TEXT_RGBA,
    RGBA_ALPHA_INDEX, RgbaColor, TAB_GAP_PX, TAB_PADDING_PX, TRAILING_CONTROL_WIDTH_PX,
    TabStripIcon, TabStripLabelRender, TabStripLabelRenderRequest, TabStripPaintOperation,
    TabStripPaintOperationKind, TabStripPaintTexture, TabStripRetainedError, TabStripRetainedState,
    TabStripTrailingControl, UiSvgRasterRequest,
};
use sha2::{Digest, Sha256};

impl TabStripRetainedState {
    pub(super) fn render_label(
        &mut self,
        ui: &mut egui::Ui,
        operations: &mut Vec<TabStripPaintOperation>,
        request: TabStripLabelRenderRequest<'_>,
    ) -> Result<TabStripLabelRender, TabStripRetainedError> {
        let TabStripLabelRenderRequest {
            text,
            path,
            x,
            bounds,
            active,
            active_reveal_pending,
            interaction,
            draggable,
        } = request;
        let raster = self
            .rasterizer
            .rasterize(text, ui.ctx().pixels_per_point())
            .map_err(TabStripRetainedError::Raster)?;
        let width = raster.width as f32 + TAB_PADDING_PX * 2.0;
        let rect = egui::Rect::from_min_size(
            egui::pos2(x, bounds.min.y + TAB_GAP_PX),
            egui::vec2(width, (bounds.height() - TAB_GAP_PX * 2.0).max(1.0)),
        );
        let response = ui.interact(
            rect,
            self.routes.interaction_id(ui.id(), &path),
            if draggable {
                egui::Sense::click_and_drag()
            } else {
                egui::Sense::click()
            },
        );
        let drag_started = draggable && response.drag_started();
        let drag_stopped = draggable && response.drag_stopped();
        if let Some(route_path) = interaction.route_path.as_deref() {
            self.routes.register_response(
                route_path,
                response.id,
                ui_rect(rect),
                &text.value,
                false,
            );
            self.publish_response_accesskit(ui, response.id);
            if !drag_started && !response.dragged() && !drag_stopped {
                self.forward_response_activation(ui, &response)?;
            }
        } else {
            ui.ctx().accesskit_node_builder(response.id, |node| {
                /* WHY: 二次ボタンmenu用のsenseがあっても、非操作labelへAX操作を残さない。 */
                node.clear_actions();
                node.set_role(egui::accesskit::Role::Label);
                node.set_label(text.value.as_str());
                node.set_bounds(egui::accesskit::Rect {
                    x0: rect.min.x.into(),
                    y0: rect.min.y.into(),
                    x1: rect.max.x.into(),
                    y1: rect.max.y.into(),
                });
            });
            crate::egui::text_command_surface::accesskit_evidence::record_custom(
                ui.ctx(), response.id, ui_rect(rect), &text.value, true, &path,
                crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::TabStripControl,
            );
        }
        let raster_fingerprint = hex::encode(Sha256::digest(&raster.rgba_pixels));
        let texture = TabStripPaintTexture {
            identity: format!("tab-strip:{path}:{raster_fingerprint}"),
            width: raster.width,
            height: raster.height,
            rgba_pixels: raster.rgba_pixels,
        };
        let background = if active {
            ACTIVE_TAB_RGBA
        } else {
            INACTIVE_TAB_RGBA
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
        let label_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.min.x + TAB_PADDING_PX,
                rect.center().y - texture.height as f32 / 2.0,
            ),
            egui::vec2(texture.width as f32, texture.height as f32),
        );
        paint_texture_tiled(
            ui.painter(),
            ui.ctx(),
            &mut self.textures,
            ui_rect(label_rect),
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
                bounds: ui_rect(label_rect),
                texture,
            },
        });
        if active && *active_reveal_pending {
            response.scroll_to_me(Some(egui::Align::Center));
            *active_reveal_pending = false;
        }
        Ok(TabStripLabelRender {
            advance: width + TAB_GAP_PX,
            secondary_clicked: response.secondary_clicked(),
            drag_started,
            drag_stopped,
            bounds: rect,
        })
    }

    pub(super) fn render_tab_trailing_control(
        &mut self,
        ui: &mut egui::Ui,
        control: TabStripTrailingControl<'_>,
        x: &mut f32,
        bounds: egui::Rect,
        operations: &mut Vec<TabStripPaintOperation>,
    ) -> Result<(), TabStripRetainedError> {
        let rect = egui::Rect::from_min_size(
            egui::pos2(*x, bounds.min.y + TAB_GAP_PX),
            egui::vec2(
                TRAILING_CONTROL_WIDTH_PX,
                (bounds.height() - TAB_GAP_PX * 2.0).max(1.0),
            ),
        );
        let response = ui.interact(
            rect,
            self.routes.interaction_id(ui.id(), &control.path),
            egui::Sense::click(),
        );
        self.routes.register_response(
            &control.path,
            response.id,
            ui_rect(rect),
            &control.presentation.accessibility_label.value,
            false,
        );
        self.publish_response_accesskit(ui, response.id);
        self.forward_response_activation(ui, &response)?;
        response.on_hover_text(&control.presentation.tooltip.value);
        let texture = self.raster_icon(
            if control.tab.capabilities.pinned {
                TabStripIcon::Pin
            } else {
                TabStripIcon::Close
            },
            RgbaColor::new(
                PRIMARY_TEXT_RGBA[0],
                PRIMARY_TEXT_RGBA[1],
                PRIMARY_TEXT_RGBA[2],
                PRIMARY_TEXT_RGBA[RGBA_ALPHA_INDEX],
            ),
        )?;
        paint_texture_tiled(
            ui.painter(),
            ui.ctx(),
            &mut self.textures,
            ui_rect(rect),
            &texture,
        );
        operations.push(TabStripPaintOperation {
            clip_bounds: ui_rect(bounds),
            kind: TabStripPaintOperationKind::Texture {
                bounds: ui_rect(rect),
                texture,
            },
        });
        *x += TRAILING_CONTROL_WIDTH_PX + TAB_GAP_PX;
        Ok(())
    }

    pub(super) fn raster_icon(
        &mut self,
        icon: TabStripIcon,
        color: RgbaColor,
    ) -> Result<TabStripPaintTexture, TabStripRetainedError> {
        let raster = self
            .svg_rasterizer
            .rasterize(&UiSvgRasterRequest {
                icon: icon.icon_props(),
                width_px: ICON_SIZE_PX,
                height_px: ICON_SIZE_PX,
                color,
            })
            .map_err(TabStripRetainedError::Svg)?;
        Ok(TabStripPaintTexture {
            identity: format!("tab-strip-icon:{}", raster.metadata.cache_key),
            width: raster.width_px,
            height: raster.height_px,
            rgba_pixels: raster.rgba_unmultiplied,
        })
    }
}

#[cfg(test)]
#[path = "label_paint_tests.rs"]
mod tests;
