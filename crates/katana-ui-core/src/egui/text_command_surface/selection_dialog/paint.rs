use super::super::TextCommandSurfaceStyle;
use super::super::accesskit_evidence::{AccessKitTargetClass, publish_labeled_button_accesskit};
use crate::egui::text_surface::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintTexture,
};
use crate::egui::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use crate::egui::texture_tiles::texture_tiles;
use crate::render_model::UiRect;
use crate::text_raster::{
    PlatformTextRasterRequest, PlatformTextRasterResources, PlatformTextRasterizer,
};
use sha2::{Digest, Sha256};

const HORIZONTAL_PADDING: f32 = 8.0;
const VERTICAL_PADDING: f32 = 6.0;
const MINIMUM_HIT_HEIGHT: f32 = 28.0;
const ALPHA_CHANNEL: usize = crate::render_model::RGBA_CHANNEL_COUNT - 1;

mod text;
use text::{DialogText, styled_spans};

pub(super) struct DialogPainter {
    rasterizer: PlatformTextRasterizer,
    textures: RgbaTextureCache,
}

impl DialogPainter {
    pub(super) fn new(resources: &PlatformTextRasterResources) -> Self {
        Self {
            rasterizer: resources.rasterizer(),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
        }
    }

    pub(super) fn label(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        key: &str,
        enabled: bool,
        interactive: bool,
        selected: bool,
        style: &TextCommandSurfaceStyle,
        operations: &mut Vec<TextSurfacePaintOperation>,
    ) -> Result<egui::Response, String> {
        self.draw_label(
            ui,
            DialogText {
                value: text,
                highlights: &[],
            },
            key,
            enabled,
            interactive,
            selected,
            style,
            operations,
        )
    }

    pub(super) fn highlighted_button(
        &mut self,
        ui: &mut egui::Ui,
        text: (&str, &[(usize, usize)]),
        key: &str,
        enabled: bool,
        selected: bool,
        style: &TextCommandSurfaceStyle,
        operations: &mut Vec<TextSurfacePaintOperation>,
    ) -> Result<egui::Response, String> {
        self.draw_label(
            ui,
            DialogText {
                value: text.0,
                highlights: text.1,
            },
            key,
            enabled,
            true,
            selected,
            style,
            operations,
        )
    }

    pub(super) fn snippet(
        &mut self,
        ui: &mut egui::Ui,
        text: (&str, &[(usize, usize)]),
        key: &str,
        style: &TextCommandSurfaceStyle,
        operations: &mut Vec<TextSurfacePaintOperation>,
    ) -> Result<egui::Response, String> {
        self.draw_label(
            ui,
            DialogText {
                value: text.0,
                highlights: text.1,
            },
            key,
            true,
            false,
            false,
            style,
            operations,
        )
    }

    fn draw_label(
        &mut self,
        ui: &mut egui::Ui,
        text: DialogText<'_>,
        key: &str,
        enabled: bool,
        interactive: bool,
        selected: bool,
        style: &TextCommandSurfaceStyle,
        operations: &mut Vec<TextSurfacePaintOperation>,
    ) -> Result<egui::Response, String> {
        let spans = styled_spans(text, style);
        let text = text.value;
        let scale = ui.ctx().pixels_per_point();
        let width = ui.available_width().max(1.0);
        let raster = self
            .rasterizer
            .rasterize(&PlatformTextRasterRequest {
                spans,
                font: style.text_raster.font.clone(),
                fallback_color_rgba: style.text_raster.fallback_color_rgba,
                line_height_px: style.text_raster.line_height_px,
                max_width_px: Some((width - HORIZONTAL_PADDING * 2.0).max(1.0)),
                scale_factor: scale,
            })
            .map_err(|error| error.to_string())?;
        let size = egui::vec2(
            width,
            (raster.height as f32 / scale + VERTICAL_PADDING * 2.0).max(MINIMUM_HIT_HEIGHT),
        );
        let (bounds, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        let response = ui.interact(
            bounds,
            ui.id().with(key),
            if interactive && enabled {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            },
        );
        let background = if selected || (interactive && response.hovered()) {
            style.text_paint.selection_rgba
        } else {
            style.text_paint.background_rgba
        };
        ui.painter().rect_filled(
            bounds,
            2.0,
            egui::Color32::from_rgba_unmultiplied(
                background[0],
                background[1],
                background[2],
                background[ALPHA_CHANNEL],
            ),
        );
        operations.push(fill(bounds, ui.clip_rect(), background));
        let pixels: Vec<u8> = raster.rgba_pixels.iter().flatten().copied().collect();
        let texture = TextSurfacePaintTexture {
            identity: format!("kuc.dialog:{}", hex::encode(Sha256::digest(&pixels))),
            width: raster.width as u32,
            height: raster.height as u32,
            rgba_pixels: pixels,
        };
        let text_bounds = egui::Rect::from_min_size(
            bounds.min + egui::vec2(HORIZONTAL_PADDING, VERTICAL_PADDING),
            egui::vec2(texture.width as f32 / scale, texture.height as f32 / scale),
        );
        let max_texture_side = ui.ctx().input(|input| input.max_texture_side).max(1);
        for (tile_bounds, tile) in texture_tiles(rect(text_bounds), &texture, max_texture_side) {
            let handle = self.textures.texture_for_rgba(
                ui.ctx(),
                &tile.identity,
                tile.width as usize,
                tile.height as usize,
                &tile.rgba_pixels,
            );
            ui.painter().image(
                handle.id(),
                egui::Rect::from_min_size(
                    egui::pos2(tile_bounds.x as f32, tile_bounds.y as f32),
                    egui::vec2(tile_bounds.width as f32, tile_bounds.height as f32),
                ),
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
            operations.push(TextSurfacePaintOperation {
                layer: EguiTextSurfaceDrawLayer::TextTexture,
                clip_bounds: rect(ui.clip_rect()),
                kind: TextSurfacePaintOperationKind::Texture {
                    bounds: tile_bounds,
                    texture: tile,
                },
            });
        }
        if interactive {
            publish_labeled_button_accesskit(
                ui,
                response.id,
                text,
                !enabled,
                rect(bounds),
                key,
                AccessKitTargetClass::SearchControl,
            );
        } else {
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Label);
                node.set_label(text);
                node.set_bounds(egui::accesskit::Rect {
                    x0: bounds.min.x.into(),
                    y0: bounds.min.y.into(),
                    x1: bounds.max.x.into(),
                    y1: bounds.max.y.into(),
                });
            });
        }
        Ok(response)
    }
}

pub(super) fn activated(ui: &egui::Ui, response: &egui::Response) -> bool {
    response.clicked()
        || (response.has_focus()
            && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)))
        || ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(event,
            egui::Event::AccessKitActionRequest(request)
                if request.action == egui::accesskit::Action::Click
                    && request.target_node == egui::accesskit::NodeId(response.id.value()))
            })
        })
}

pub(super) fn rect(value: egui::Rect) -> UiRect {
    UiRect::new(
        value.min.x.round() as i32,
        value.min.y.round() as i32,
        value.width().max(0.0).round() as u32,
        value.height().max(0.0).round() as u32,
    )
}

pub(super) fn fill(
    bounds: egui::Rect,
    clip: egui::Rect,
    color_rgba: [u8; crate::render_model::RGBA_CHANNEL_COUNT],
) -> TextSurfacePaintOperation {
    TextSurfacePaintOperation {
        layer: EguiTextSurfaceDrawLayer::Background,
        clip_bounds: rect(clip),
        kind: TextSurfacePaintOperationKind::Fill {
            bounds: rect(bounds),
            color_rgba,
        },
    }
}
