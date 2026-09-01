use super::support::ui_rect;
use super::{
    Arc, DEFAULT_TEXTURE_CACHE_CAPACITY, EguiTextSurfaceAdapter, NAVIGATION_AREA_WIDTH_PX,
    PlatformFontCatalog, PlatformTextMetricsFrame, PlatformTextRasterConfig, Rc, RefCell,
    RgbaTextureCache, TAB_GAP_PX, TAB_STRIP_HEIGHT_PX, TabStripIcon, TabStripIconControl,
    TabStripOverlayState, TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintPlan,
    TabStripProjection, TabStripProjectionLease, TabStripRetainedError, TabStripRetainedState,
    TabStripRootOutput, TabStripRouteTable, TabStripTextRasterizer, UiSvgRasterConfig,
    UiSvgRasterizer,
};

impl TabStripRetainedState {
    pub(crate) fn from_lease(
        lease: TabStripProjectionLease,
        catalog: Arc<PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
    ) -> Result<Self, TabStripRetainedError> {
        let (projection, port) = lease.into_parts();
        let routes = TabStripRouteTable::from_projection(&projection);
        let active_reveal_pending = projection.scroll_presentation.request_active_reveal;
        Ok(Self {
            projection,
            port,
            rasterizer: TabStripTextRasterizer::with_catalog(Arc::clone(&catalog), config.clone())
                .map_err(TabStripRetainedError::Raster)?,
            svg_rasterizer: UiSvgRasterizer::new(UiSvgRasterConfig::default()),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            routes,
            active_reveal_pending,
            horizontal_scroll_offset: 0.0,
            next_nonce: 0,
            overlay: TabStripOverlayState::Closed,
            overlay_primary_press: None,
            rename_adapter: EguiTextSurfaceAdapter::with_catalog_and_metrics(
                catalog,
                config,
                Rc::new(RefCell::new(PlatformTextMetricsFrame::new())),
            )
            .map_err(TabStripRetainedError::Raster)?,
            drag: None,
            drag_release_pending: false,
            drag_candidates: Vec::new(),
        })
    }

    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<TabStripRootOutput, TabStripRetainedError> {
        self.routes.begin_frame();
        self.drag_candidates.clear();
        let revision = self.projection.revision;
        let correlation = self.projection.correlation.copy_for_route();
        let projection = std::mem::replace(
            &mut self.projection,
            TabStripProjection {
                revision,
                correlation: correlation.copy_for_route(),
                tabs: Vec::new(),
                groups: Vec::new(),
                capabilities: Default::default(),
                navigation: None,
                scroll_presentation: Default::default(),
            },
        );
        let mut active_reveal_pending = self.active_reveal_pending;
        let result = self.show_projection(
            ui,
            &projection,
            &mut active_reveal_pending,
            self.horizontal_scroll_offset,
        );
        if result.is_ok() {
            self.active_reveal_pending = active_reveal_pending;
            self.horizontal_scroll_offset = result
                .as_ref()
                .map(|output| output.horizontal_scroll_offset)
                .unwrap_or(self.horizontal_scroll_offset);
        }
        self.projection = projection;
        result
    }

    pub(super) fn show_projection(
        &mut self,
        ui: &mut egui::Ui,
        projection: &TabStripProjection,
        active_reveal_pending: &mut bool,
        horizontal_scroll_offset: f32,
    ) -> Result<TabStripRootOutput, TabStripRetainedError> {
        let available = ui.available_rect_before_wrap();
        let bounds = egui::Rect::from_min_size(
            available.min,
            egui::vec2(
                available.width(),
                TAB_STRIP_HEIGHT_PX.min(available.height()),
            ),
        );
        let mut operations = vec![TabStripPaintOperation {
            clip_bounds: ui_rect(bounds),
            kind: TabStripPaintOperationKind::Fill {
                bounds: ui_rect(bounds),
                color_rgba: [36, 36, 36, 255],
            },
        }];
        let navigation_width = projection
            .navigation
            .as_ref()
            .map_or(0.0, |_| NAVIGATION_AREA_WIDTH_PX);
        let tab_bounds = egui::Rect::from_min_size(
            bounds.min,
            egui::vec2(
                (bounds.width() - navigation_width).max(1.0),
                bounds.height(),
            ),
        );
        let mut item_result = Ok(());
        let mut next_horizontal_scroll_offset = horizontal_scroll_offset;
        ui.scope_builder(egui::UiBuilder::new().max_rect(tab_bounds), |tabs_ui| {
            let scroll_output = egui::ScrollArea::horizontal()
                .id_salt("root-tab-strip-scroll")
                .horizontal_scroll_offset(horizontal_scroll_offset)
                .max_width(tab_bounds.width())
                .max_height(tab_bounds.height())
                .auto_shrink([false, true])
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(tabs_ui, |tabs_ui| {
                    let content_bounds = egui::Rect::from_min_size(
                        egui::pos2(
                            tab_bounds.min.x - horizontal_scroll_offset,
                            tab_bounds.min.y,
                        ),
                        tab_bounds.size(),
                    );
                    let mut x = content_bounds.min.x + TAB_GAP_PX;
                    for (index, tab) in projection.tabs.iter().enumerate() {
                        self.render_tab(
                            tabs_ui,
                            tab,
                            format!("root-tab-{index}"),
                            &mut x,
                            tab_bounds,
                            &mut operations,
                            active_reveal_pending,
                        )?;
                    }
                    for (index, group) in projection.groups.iter().enumerate() {
                        self.render_group(
                            tabs_ui,
                            group,
                            format!("root-group-{index}"),
                            &mut x,
                            tab_bounds,
                            &mut operations,
                            active_reveal_pending,
                        )?;
                    }
                    tabs_ui.allocate_rect(
                        egui::Rect::from_min_max(
                            content_bounds.min,
                            egui::pos2(x.max(content_bounds.max.x), content_bounds.max.y),
                        ),
                        egui::Sense::hover(),
                    );
                    Ok(())
                });
            next_horizontal_scroll_offset = scroll_output.state.offset.x;
            item_result = scroll_output.inner;
        });
        item_result?;
        self.resolve_tab_drag(
            ui,
            tab_bounds,
            projection.capabilities.tab_drop_at_end_available,
            &mut operations,
        )?;
        if let Some(navigation) = projection.navigation.as_ref() {
            let mut navigation_x = tab_bounds.max.x + TAB_GAP_PX;
            self.render_icon_control(
                ui,
                TabStripIconControl {
                    icon: TabStripIcon::Previous,
                    presentation: &navigation.previous,
                    enabled: projection.capabilities.previous_available,
                    path: "tab-strip-previous",
                },
                &mut navigation_x,
                bounds,
                &mut operations,
            )?;
            self.render_icon_control(
                ui,
                TabStripIconControl {
                    icon: TabStripIcon::Next,
                    presentation: &navigation.next,
                    enabled: projection.capabilities.next_available,
                    path: "tab-strip-next",
                },
                &mut navigation_x,
                bounds,
                &mut operations,
            )?;
        }
        ui.allocate_rect(bounds, egui::Sense::hover());
        let overlay_paint_plan = self.render_overlay(ui, projection)?;
        Ok(TabStripRootOutput {
            paint_plan: TabStripPaintPlan {
                surface_bounds: ui_rect(bounds),
                operations,
            },
            overlay_paint_plan,
            horizontal_scroll_offset: next_horizontal_scroll_offset,
        })
    }
}

#[cfg(test)]
#[path = "retained_state_tests.rs"]
mod tests;
