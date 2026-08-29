use super::support::ui_rect;
use super::{
    ACTIVE_TAB_RGBA, CORNER_RADIUS_PX, ICON_SIZE_PX, INACTIVE_TAB_RGBA, PRIMARY_TEXT_RGBA,
    RGBA_ALPHA_INDEX, RgbaColor, TAB_GAP_PX, TAB_PADDING_PX, TRAILING_CONTROL_WIDTH_PX,
    TabStripIcon, TabStripLabelRender, TabStripLabelRenderRequest, TabStripPaintOperation,
    TabStripPaintOperationKind, TabStripPaintTexture, TabStripRetainedError, TabStripRetainedState,
    TabStripTrailingControl, UiSvgRasterRequest,
};

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
            ui.id().with(path.clone()),
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
        }
        let texture = TabStripPaintTexture {
            identity: format!("tab-strip:{path}"),
            width: raster.width,
            height: raster.height,
            rgba_pixels: raster.rgba_pixels,
        };
        let texture_handle = self.textures.texture_for_rgba(
            ui.ctx(),
            &texture.identity,
            texture.width as usize,
            texture.height as usize,
            &texture.rgba_pixels,
        );
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
        ui.painter().image(
            texture_handle.id(),
            label_rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
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
        let response = ui.interact(rect, ui.id().with(&control.path), egui::Sense::click());
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
        let handle = self.textures.texture_for_rgba(
            ui.ctx(),
            &texture.identity,
            texture.width as usize,
            texture.height as usize,
            &texture.rgba_pixels,
        );
        ui.painter().image(
            handle.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
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
