use super::artifact_model::{EguiTextSurfaceError, EguiTextSurfaceFrameRecord};
use super::gutter_icon::marker_texture_operation;
use super::model::SharedTextMetrics;
use super::model::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintPlan, TextSurfacePaintStyle, TextSurfacePaintTexture, TextSurfaceRasterStyle,
};
use super::raster::{RasterFrame, rasterize_gutter_label};
use crate::texture_cache::RgbaTextureCache;
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use katana_ui_core::text_surface::{TextSurfaceAnnotationStyle, TextSurfaceFrameRecord};
use katana_ui_core_svg_raster::UiSvgRasterizer;
use katana_ui_core_text_raster::{PlatformTextRaster, PlatformTextRasterizer};

mod layers;

pub(super) use layers::PaintLayers;

pub(super) fn build_paint_plan(
    rasterizer: &mut PlatformTextRasterizer,
    svg_rasterizer: &mut UiSvgRasterizer,
    raster: &RasterFrame,
    placeholder: Option<&RasterFrame>,
    record: &EguiTextSurfaceFrameRecord,
    style: &TextSurfacePaintStyle,
    raster_style: &TextSurfaceRasterStyle,
    scale_factor: f32,
    metrics: &SharedTextMetrics,
) -> Result<TextSurfacePaintPlan, EguiTextSurfaceError> {
    let frame = &record.frame;
    let gutter_bounds = gutter_bounds(frame);
    let mut operations = vec![fill(
        EguiTextSurfaceDrawLayer::Background,
        frame.surface_bounds,
        frame.surface_bounds,
        style.background_rgba,
    )];
    for gutter in &frame.gutter {
        let paint = style.gutter_paint(&gutter.visual_role);
        let has_state = gutter.active || gutter.hovered;
        let background =
            style.gutter_background_rgba(&gutter.visual_role, gutter.active, gutter.hovered);
        operations.push(fill(
            EguiTextSurfaceDrawLayer::Gutter,
            gutter_bounds,
            gutter.bounds,
            background.unwrap_or(style.gutter_background_rgba),
        ));
        let mut label_style = raster_style.clone();
        if has_state
            && style
                .gutter_foreground_rgba(&gutter.visual_role, gutter.active, gutter.hovered)
                .is_some()
        {
            label_style.fallback_color_rgba = style
                .gutter_foreground_rgba(&gutter.visual_role, gutter.active, gutter.hovered)
                .unwrap_or(label_style.fallback_color_rgba);
        } else if let Some(paint) = paint {
            label_style.fallback_color_rgba = paint.foreground_rgba;
        }
        let label_raster = rasterize_gutter_label(
            rasterizer,
            &gutter.display_label,
            &label_style,
            scale_factor,
            metrics,
        )?;
        let identity = format!(
            "gutter:{}:{}:{}:{label_style:?}",
            gutter.logical_row, gutter.visual_role, gutter.display_label,
        );
        operations.push(texture(
            EguiTextSurfaceDrawLayer::Gutter,
            gutter_bounds,
            UiRect::new(
                gutter.bounds.x,
                gutter.bounds.y,
                u32::try_from(label_raster.width).unwrap_or(gutter.bounds.width),
                u32::try_from(label_raster.height).unwrap_or(gutter.bounds.height),
            ),
            texture_from_raster(identity, &label_raster),
        ));
        if let Some(operation) = marker_texture_operation(
            svg_rasterizer,
            gutter,
            gutter_bounds,
            style
                .gutter_foreground_rgba(&gutter.visual_role, gutter.active, gutter.hovered)
                .unwrap_or(style.caret_rgba),
        )? {
            operations.push(operation);
        }
    }
    for bounds in &frame.selection.rects {
        operations.push(fill(
            EguiTextSurfaceDrawLayer::Selection,
            frame.viewport_bounds,
            *bounds,
            style.selection_rgba,
        ));
    }
    if let Some(preedit) = frame.preedit.as_ref() {
        for bounds in &preedit.rects {
            operations.push(fill(
                EguiTextSurfaceDrawLayer::Preedit,
                frame.viewport_bounds,
                bottom_stroke(*bounds),
                style.preedit_rgba,
            ));
        }
    }
    for annotation in &frame.annotations {
        let color = style.annotation_color(&annotation.visual_role);
        for bounds in &annotation.rects {
            match annotation.style {
                TextSurfaceAnnotationStyle::Underline => operations.push(fill(
                    EguiTextSurfaceDrawLayer::Annotation,
                    frame.viewport_bounds,
                    bottom_stroke(*bounds),
                    color,
                )),
                TextSurfaceAnnotationStyle::Outline => {
                    operations.push(fill(
                        EguiTextSurfaceDrawLayer::Annotation,
                        frame.viewport_bounds,
                        top_stroke(*bounds),
                        color,
                    ));
                    operations.push(fill(
                        EguiTextSurfaceDrawLayer::Annotation,
                        frame.viewport_bounds,
                        bottom_stroke(*bounds),
                        color,
                    ));
                }
                TextSurfaceAnnotationStyle::Fill => operations.push(fill(
                    EguiTextSurfaceDrawLayer::Annotation,
                    frame.viewport_bounds,
                    *bounds,
                    color,
                )),
            }
        }
    }
    if let (Some(placeholder), Some(bounds)) = (placeholder, record.placeholder_texture_bounds) {
        operations.push(texture(
            EguiTextSurfaceDrawLayer::PlaceholderTexture,
            frame.viewport_bounds,
            bounds,
            texture_from_raster(placeholder.identity.clone(), &placeholder.raster),
        ));
    }
    operations.push(texture(
        EguiTextSurfaceDrawLayer::TextTexture,
        frame.viewport_bounds,
        record.texture_bounds,
        texture_from_raster(raster.identity.clone(), &raster.raster),
    ));
    if frame.selection.caret.width > 0 && frame.selection.caret.height > 0 {
        operations.push(fill(
            EguiTextSurfaceDrawLayer::Caret,
            frame.viewport_bounds,
            frame.selection.caret,
            style.caret_rgba,
        ));
    }
    Ok(TextSurfacePaintPlan {
        surface_bounds: frame.surface_bounds,
        viewport_bounds: frame.viewport_bounds,
        operations,
    })
}

pub(super) fn paint_surface(
    ui: &egui::Ui,
    cache: &mut RgbaTextureCache,
    plan: &TextSurfacePaintPlan,
) {
    for operation in &plan.operations {
        let painter = ui
            .painter()
            .with_clip_rect(egui_rect(operation.clip_bounds));
        match &operation.kind {
            TextSurfacePaintOperationKind::Fill { bounds, color_rgba } => {
                painter.rect_filled(egui_rect(*bounds), 0.0, color(*color_rgba));
            }
            TextSurfacePaintOperationKind::Texture { bounds, texture } => {
                let texture = texture_for_plan_texture(cache, ui.ctx(), texture);
                painter.image(
                    texture.id(),
                    egui_rect(*bounds),
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

fn fill(
    layer: EguiTextSurfaceDrawLayer,
    clip_bounds: UiRect,
    bounds: UiRect,
    color_rgba: [u8; RGBA_CHANNEL_COUNT],
) -> TextSurfacePaintOperation {
    TextSurfacePaintOperation {
        layer,
        clip_bounds,
        kind: TextSurfacePaintOperationKind::Fill { bounds, color_rgba },
    }
}

fn texture(
    layer: EguiTextSurfaceDrawLayer,
    clip_bounds: UiRect,
    bounds: UiRect,
    texture: TextSurfacePaintTexture,
) -> TextSurfacePaintOperation {
    TextSurfacePaintOperation {
        layer,
        clip_bounds,
        kind: TextSurfacePaintOperationKind::Texture { bounds, texture },
    }
}

fn texture_from_raster(identity: String, raster: &PlatformTextRaster) -> TextSurfacePaintTexture {
    TextSurfacePaintTexture {
        identity,
        width: u32::try_from(raster.width).unwrap_or_default(),
        height: u32::try_from(raster.height).unwrap_or_default(),
        rgba_pixels: raster.rgba_pixels.iter().flatten().copied().collect(),
    }
}

fn gutter_bounds(frame: &TextSurfaceFrameRecord) -> UiRect {
    UiRect::new(
        frame.surface_bounds.x,
        frame.surface_bounds.y,
        u32::try_from(
            frame
                .viewport_bounds
                .x
                .saturating_sub(frame.surface_bounds.x),
        )
        .unwrap_or_default(),
        frame.surface_bounds.height,
    )
}

fn top_stroke(bounds: UiRect) -> UiRect {
    UiRect::new(bounds.x, bounds.y, bounds.width, 1)
}

fn bottom_stroke(bounds: UiRect) -> UiRect {
    UiRect::new(
        bounds.x,
        bounds
            .y
            .saturating_add_unsigned(bounds.height.saturating_sub(1)),
        bounds.width,
        1,
    )
}

fn texture_for_plan_texture(
    cache: &mut RgbaTextureCache,
    context: &egui::Context,
    texture: &TextSurfacePaintTexture,
) -> egui::TextureHandle {
    cache.texture_for_rgba(
        context,
        &texture.identity,
        usize::try_from(texture.width).unwrap_or_default(),
        usize::try_from(texture.height).unwrap_or_default(),
        &texture.rgba_pixels,
    )
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
