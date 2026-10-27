use super::super::accesskit_evidence::AccessKitTargetClass;
use super::global_actions::{KucGlobalActionEvent, KucGlobalActionsProjection};
use crate::egui::command_chrome::{CommandChromePaintStyle, CommandChromeRasterStyle};
use crate::egui::raster_extent::LogicalRasterExtent;
use crate::egui::tab_strip_paint::{TabStripPaintPlan, TabStripPaintTexture};
use crate::egui::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use crate::render_model::{UiTextSpan, UiTextSpanStyle};
use crate::text_raster::{
    PlatformFontCatalog, PlatformTextRasterConfig, PlatformTextRasterError,
    PlatformTextRasterRequest, PlatformTextRasterizer,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

const ROW_HEIGHT: f32 = 28.0;
const PADDING: f32 = 10.0;

#[path = "global_action_render/paint.rs"]
mod paint;
pub(super) use paint::ui_rect;
use paint::{GlobalActionRow, paint_row};

pub(crate) struct KucGlobalActionsRenderer {
    rasterizer: PlatformTextRasterizer,
    textures: RgbaTextureCache,
    open: bool,
}

impl KucGlobalActionsRenderer {
    pub(super) fn new(
        catalog: Arc<PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
    ) -> Result<Self, PlatformTextRasterError> {
        Ok(Self {
            rasterizer: PlatformTextRasterizer::with_catalog(catalog, config)?,
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            open: false,
        })
    }

    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
        root: &str,
        bounds: egui::Rect,
        canvas: egui::Rect,
        projection: &KucGlobalActionsProjection,
        enabled: bool,
        raster_style: &CommandChromeRasterStyle,
        paint_style: &CommandChromePaintStyle,
    ) -> Result<(Vec<KucGlobalActionEvent>, TabStripPaintPlan), PlatformTextRasterError> {
        let enabled = enabled && ui.is_enabled();
        let clip = ui.clip_rect();
        if !enabled && self.open {
            self.open = false;
            ui.ctx().request_repaint();
        }
        let mut operations = Vec::new();
        let mut events = Vec::new();
        let trigger = self.raster(
            &projection.menu_label,
            ui.ctx().pixels_per_point(),
            raster_style,
        )?;
        let scale = ui.ctx().pixels_per_point();
        let trigger_width = LogicalRasterExtent::from_physical(trigger.width as usize, scale);
        let trigger_height = LogicalRasterExtent::from_physical(trigger.height as usize, scale);
        let width = (trigger_width as f32 + PADDING * 2.0).max(ROW_HEIGHT);
        let trigger_bounds = egui::Rect::from_min_size(bounds.min, egui::vec2(width, ROW_HEIGHT));
        let response = ui.interact(
            trigger_bounds,
            ui.id().with(root).with("global-menu-trigger"),
            egui::Sense::click(),
        );
        if activated(ui, &response, enabled) {
            self.open = !self.open;
            ui.ctx().request_repaint();
        }
        paint_row(
            ui,
            &mut self.textures,
            &mut operations,
            canvas,
            GlobalActionRow {
                rect: trigger_bounds,
                texture: trigger,
                logical_texture_size: egui::vec2(trigger_width as f32, trigger_height as f32),
                enabled,
                response_id: response.id,
                label: &projection.menu_label,
                identity: "kuc.global-menu",
                class: AccessKitTargetClass::GlobalActionMenu,
            },
            paint_style,
        );
        if self.open && ui.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.open = false;
            ui.ctx().request_repaint();
        }
        if enabled {
            let mut entries = Vec::with_capacity(projection.actions.len());
            let mut entry_width = width;
            for action in &projection.actions {
                let texture =
                    self.raster(&action.label, ui.ctx().pixels_per_point(), raster_style)?;
                let logical_width =
                    LogicalRasterExtent::from_physical(texture.width as usize, scale);
                let logical_height =
                    LogicalRasterExtent::from_physical(texture.height as usize, scale);
                entry_width = entry_width.max(logical_width as f32 + PADDING * 2.0);
                entries.push((
                    texture,
                    egui::vec2(logical_width as f32, logical_height as f32),
                ));
            }
            let menu_bounds = egui::Rect::from_min_size(
                egui::pos2(bounds.min.x, bounds.max.y),
                egui::vec2(entry_width, ROW_HEIGHT * entries.len() as f32),
            );
            let outside = ui.input(|input| input.events.iter().any(|event| matches!(event, egui::Event::PointerButton { pos, pressed: true, .. } if !trigger_bounds.contains(*pos) && !menu_bounds.contains(*pos))));
            if self.open && outside {
                self.open = false;
                ui.ctx().request_repaint();
            }
            {
                egui::Area::new(ui.id().with(root).with("global-menu-entries"))
                    .fixed_pos(menu_bounds.min)
                    .interactable(self.open)
                    .order(egui::Order::Foreground)
                    .show(ui.ctx(), |child| {
                        child.set_clip_rect(child.clip_rect().intersect(clip));
                        child.set_min_size(menu_bounds.size());
                        if !self.open {
                            return;
                        }
                        if !child.is_enabled() {
                            child.ctx().request_repaint();
                            return;
                        }
                        for (index, (action, (texture, logical_texture_size))) in
                            projection.actions.iter().zip(entries).enumerate()
                        {
                            let rect = egui::Rect::from_min_size(
                                egui::pos2(
                                    menu_bounds.min.x,
                                    menu_bounds.min.y + index as f32 * ROW_HEIGHT,
                                ),
                                egui::vec2(entry_width, ROW_HEIGHT),
                            );
                            let identity = action.target.identity();
                            let response = child.interact(
                                rect,
                                child.id().with(&identity),
                                egui::Sense::click(),
                            );
                            child.allocate_rect(rect, egui::Sense::hover());
                            if activated(child, &response, action.enabled) {
                                events.push(KucGlobalActionEvent {
                                    target: action.target.clone(),
                                });
                                self.open = false;
                                child.ctx().request_repaint();
                            }
                            paint_row(
                                child,
                                &mut self.textures,
                                &mut operations,
                                canvas,
                                GlobalActionRow {
                                    rect,
                                    texture,
                                    logical_texture_size,
                                    enabled: action.enabled,
                                    response_id: response.id,
                                    label: &action.label,
                                    identity: &identity,
                                    class: AccessKitTargetClass::GlobalAction,
                                },
                                paint_style,
                            );
                        }
                    });
            }
        }
        Ok((
            events,
            TabStripPaintPlan {
                surface_bounds: ui_rect(canvas),
                operations,
            },
        ))
    }

    pub(super) fn is_open(&self) -> bool {
        self.open
    }

    fn raster(
        &mut self,
        value: &str,
        scale: f32,
        style: &CommandChromeRasterStyle,
    ) -> Result<TabStripPaintTexture, PlatformTextRasterError> {
        let raster = self.rasterizer.rasterize(&PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                value,
                UiTextSpanStyle {
                    color_rgba: style.text_color_rgba,
                    ..Default::default()
                },
            ),
            font: style.font.clone(),
            fallback_color_rgba: style.text_color_rgba,
            line_height_px: style.line_height_px,
            max_width_px: None,
            scale_factor: scale,
        })?;
        let pixels: Vec<u8> = raster.rgba_pixels.iter().flatten().copied().collect();
        let identity = format!("kuc-global:{}", hex::encode(Sha256::digest(&pixels)));
        Ok(TabStripPaintTexture {
            identity,
            width: u32::try_from(raster.width).unwrap_or(u32::MAX),
            height: u32::try_from(raster.height).unwrap_or(u32::MAX),
            rgba_pixels: pixels,
        })
    }
}

fn activated(ui: &egui::Ui, response: &egui::Response, enabled: bool) -> bool {
    enabled && ui.is_enabled() && response.clicked()
}
