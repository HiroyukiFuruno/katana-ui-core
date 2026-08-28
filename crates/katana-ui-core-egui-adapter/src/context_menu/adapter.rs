use super::accessibility::ContextMenuAccessibility;
use super::artifact::artifact_frame;
use super::interaction::{is_outside_click, keyboard_actions};
use super::paint::{measure_and_build_plan, paint_plan, translate_plan};
use super::presentation::{core_items, visible_items};
use super::surface::{MenuAreaOutput, egui_rect, menu_bounds, menu_items};
use super::types::{
    ContextMenuAdapterError, ContextMenuPaintStyle, ContextMenuPresentation,
    ContextMenuRasterStyle, EguiContextMenuAdapter, EguiContextMenuFrameRecord,
    EguiContextMenuOutput,
};
use crate::text_surface::TextSurfaceContextTargetAnchor;
use crate::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use katana_ui_core::molecule::selection::{
    ContextMenu, ContextMenuAction, ContextMenuCloseReason, ContextMenuTypeAheadBuffer,
};
use katana_ui_core_svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
use katana_ui_core_text_raster::{PlatformTextRasterConfig, PlatformTextRasterizer};
use std::sync::Arc;

const TYPE_AHEAD_TIMEOUT_MS: u64 = 1_000;

impl EguiContextMenuAdapter {
    #[cfg(test)]
    pub(crate) fn catalog(&self) -> Arc<katana_ui_core_text_raster::PlatformFontCatalog> {
        self.text_rasterizer.catalog()
    }

    pub(crate) fn with_catalog(
        catalog: Arc<katana_ui_core_text_raster::PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
    ) -> Self {
        Self {
            menu: ContextMenu::new("kuc-context-menu"),
            presentation: ContextMenuPresentation::default(),
            anchor: None,
            submenu_path: Vec::new(),
            scroll_path: Vec::new(),
            vertical_scroll_offset: 0.0,
            focus_return: None,
            type_ahead: ContextMenuTypeAheadBuffer::new(TYPE_AHEAD_TIMEOUT_MS),
            text_rasterizer: PlatformTextRasterizer::with_catalog_cache_capacity(
                catalog,
                config.cache_capacity,
            ),
            svg_rasterizer: UiSvgRasterizer::new(UiSvgRasterConfig::default()),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
        }
    }

    #[must_use]
    pub fn new(config: PlatformTextRasterConfig) -> Self {
        Self {
            menu: ContextMenu::new("kuc-context-menu"),
            presentation: ContextMenuPresentation::default(),
            anchor: None,
            submenu_path: Vec::new(),
            scroll_path: Vec::new(),
            vertical_scroll_offset: 0.0,
            focus_return: None,
            type_ahead: ContextMenuTypeAheadBuffer::new(TYPE_AHEAD_TIMEOUT_MS),
            text_rasterizer: PlatformTextRasterizer::new(config),
            svg_rasterizer: UiSvgRasterizer::new(UiSvgRasterConfig::default()),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
        }
    }

    /// Synchronizes only opaque controlled presentation, preserving interaction state.
    pub fn synchronize_presentation(&mut self, presentation: ContextMenuPresentation) {
        if self.presentation != presentation {
            self.reset_scroll();
        }
        self.menu.synchronize_items(core_items(&presentation.items));
        self.presentation = presentation;
    }

    /// Returns whether this retained adapter currently owns an open menu surface.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    /// Receives an adapter-owned TextSurface fact; pixel coordinates are not public input.
    pub fn request_open(&mut self, anchor: TextSurfaceContextTargetAnchor) {
        self.anchor = Some(anchor);
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        raster_style: &ContextMenuRasterStyle,
        paint_style: &ContextMenuPaintStyle,
    ) -> Result<EguiContextMenuOutput, ContextMenuAdapterError> {
        if !self.presentation.visible {
            return Ok(self.close(ui, ContextMenuCloseReason::FocusReturn));
        }
        let Some(anchor) = self.anchor.clone() else {
            return Ok(EguiContextMenuOutput {
                record: None,
                events: Vec::new(),
                artifact: None,
            });
        };
        self.reset_scroll_for_current_path();
        let items = visible_items(&self.presentation.items, &self.submenu_path).to_vec();
        let measured = measure_and_build_plan(
            &mut self.text_rasterizer,
            &mut self.svg_rasterizer,
            &items,
            raster_style,
            paint_style,
            ui.ctx().pixels_per_point(),
        )?;
        /* WHY: ContextMenuPlacementResolver remains the KUC-owned placement
         * contract; surface::menu_bounds delegates geometry resolution to it. */
        let bounds = menu_bounds(&anchor, measured.width, measured.height);
        self.apply_wheel_scroll(ui, bounds, measured.height);
        let opening = !self.menu.is_open();
        let mut events = self.open_if_needed(ui, &anchor, measured.width, measured.height);
        let keyboard = ui.input(|input| {
            keyboard_actions(
                input,
                &items,
                &mut self.submenu_path,
                self.menu.current_highlighted_path(),
                &mut self.type_ahead,
            )
        });
        let keyboard_highlight_changed = keyboard
            .iter()
            .any(|action| matches!(action, ContextMenuAction::Highlight { .. }));
        events.extend(self.apply_actions(keyboard));
        if keyboard_highlight_changed {
            self.reveal_keyboard_highlight(bounds, measured.height, items.len());
        }
        let area_id = ui.id().with("kuc-context-menu-surface");
        let area = egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .constrain(false)
            .fixed_pos(egui::pos2(bounds.x as f32, bounds.y as f32))
            .show(ui.ctx(), |menu_ui| {
                menu_ui.set_min_size(egui::vec2(bounds.width as f32, bounds.height as f32));
                menu_ui.set_clip_rect(egui_rect(bounds));
                ContextMenuAccessibility::publish_menu(menu_ui, area_id, bounds);
                egui::ScrollArea::vertical()
                    .id_salt(area_id.with(("overflow-clip", &self.submenu_path)))
                    .scroll_source(egui::scroll_area::ScrollSource::NONE)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .auto_shrink([false, false])
                    .max_width(bounds.width as f32)
                    .max_height(bounds.height as f32)
                    .content_margin(egui::Margin::ZERO)
                    .vertical_scroll_offset(0.0)
                    .show(menu_ui, |scroll_ui| {
                        scroll_ui.set_clip_rect(egui_rect(bounds));
                        menu_items(
                            scroll_ui,
                            area_id,
                            bounds,
                            &items,
                            &self.submenu_path,
                            self.vertical_scroll_offset,
                        )
                    })
                    .inner
            });
        area.response.request_focus();
        let MenuAreaOutput {
            actions: pointer_actions,
            item_frames,
        } = area.inner;
        events.extend(self.apply_actions(pointer_actions));
        /* WHY: The opener belongs to the target surface, not to the already-open menu. A menu
        placed below the pointer would otherwise close on the same frame it is opened. */
        if !opening && ui.input(|input| is_outside_click(input, bounds)) {
            events.extend(self.apply_actions(vec![ContextMenuAction::Close {
                reason: ContextMenuCloseReason::OutsideClick,
            }]));
        }
        if !self.menu.is_open() {
            return Ok(self.finish_closed(ui, events));
        }
        let plan = translate_plan(&measured, bounds, self.vertical_scroll_offset);
        paint_plan(ui, &mut self.textures, &plan);
        let record = EguiContextMenuFrameRecord {
            bounds,
            viewport_bounds: anchor.viewport_bounds(),
            highlighted_path: self.menu.current_highlighted_path().to_vec(),
            focused: area.response.has_focus(),
            items: item_frames,
        };
        let artifact = artifact_frame(record.clone(), plan, events.clone())?;
        Ok(EguiContextMenuOutput {
            record: Some(record),
            events,
            artifact: Some(artifact),
        })
    }
}

impl Default for EguiContextMenuAdapter {
    fn default() -> Self {
        Self::new(PlatformTextRasterConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::ContextMenuPresentationItem;
    use super::*;
    use crate::text_surface::TextSurfaceContextTargetAnchor;
    use katana_ui_core::molecule::selection::ContextMenuEvent;
    use katana_ui_core::render_model::UiRect;
    use katana_ui_core::text_selection::UiTextSelectionRange;
    use katana_ui_core::theme::{FontFamily, FontToken};

    #[test]
    fn normal_open_from_generic_presentation_emits_the_complete_artifact_frame() {
        let context = egui::Context::default();
        let mut adapter = EguiContextMenuAdapter::default();
        adapter.synchronize_presentation(ContextMenuPresentation {
            visible: true,
            items: vec![ContextMenuPresentationItem::action("copy", "コピー ⭐️")],
        });
        adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
            24,
            32,
            UiTextSelectionRange::caret(0),
            UiRect::new(0, 0, 640, 360),
        ));

        let mut result = None;
        crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
            result = Some(adapter.show(ui, &raster_style(), &paint_style()));
        });
        let output = result
            .expect("the Egui frame must invoke the adapter")
            .expect("the normal generic context-menu path must succeed");
        let record = output
            .record
            .expect("open menu must produce a frame record");
        let artifact = output.artifact.expect("open menu must produce an artifact");

        assert_eq!(artifact.record, record);
        assert_eq!(artifact.events, output.events);
        assert!(!artifact.paint_plan.operations.is_empty());
        assert!(!artifact.frame_record_hash.is_empty());
        assert!(!artifact.paint_plan_hash.is_empty());
        assert_eq!(record.items.len(), 1);
        assert_eq!(record.items[0].id, "copy");
        assert!(
            output
                .events
                .iter()
                .any(|event| { matches!(event, ContextMenuEvent::Opened { .. }) })
        );
    }

    fn raster_style() -> ContextMenuRasterStyle {
        ContextMenuRasterStyle {
            font: FontToken {
                name: "context-menu-adapter-test".into(),
                family: FontFamily::Proportional,
                size: 15.0,
                weight: 400,
            },
            text_color_rgba: [230, 230, 230, 255],
            icon_color_rgba: [230, 230, 230, 255],
            line_height_px: 22.0,
        }
    }

    const fn paint_style() -> ContextMenuPaintStyle {
        ContextMenuPaintStyle {
            background_rgba: [30, 30, 32, 255],
            highlighted_rgba: [60, 80, 112, 255],
            disabled_rgba: [45, 45, 48, 255],
        }
    }
}
