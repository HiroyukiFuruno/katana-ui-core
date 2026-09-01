use super::types::{
    ContextMenuAdapterError, ContextMenuPaintOperation, ContextMenuPaintOperationKind,
    ContextMenuPaintPlan, ContextMenuPaintStyle, ContextMenuPaintTexture,
    ContextMenuPresentationItem, ContextMenuRasterStyle, ICON_LABEL_GAP_PX, ICON_SIZE_PX,
    ITEM_LEFT_PADDING_PX, ITEM_TOP_PADDING_PX, MENU_MIN_WIDTH_PX, MENU_PADDING_PX, ROW_HEIGHT_PX,
};
use crate::egui::text_surface::SharedTextMetrics;
use crate::egui::texture_cache::RgbaTextureCache;
use crate::molecule::RgbaColor;
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect, UiTextSpan, UiTextSpanStyle};
use crate::svg_raster::{UiSvgRasterRequest, UiSvgRasterizer};
use crate::text_raster::{
    PlatformTextMetricsRequest, PlatformTextRasterRequest, PlatformTextRasterizer,
};

#[path = "paint/render.rs"]
mod render;

pub(super) fn paint_plan(ui: &egui::Ui, cache: &mut RgbaTextureCache, plan: &ContextMenuPaintPlan) {
    render::paint_plan(ui, cache, plan);
}

pub(super) struct ContextMenuMeasuredPlan {
    pub(super) plan: ContextMenuPaintPlan,
    pub(super) width: u32,
    pub(super) height: u32,
}

pub(super) fn measure_and_build_plan(
    text_rasterizer: &mut PlatformTextRasterizer,
    svg_rasterizer: &mut UiSvgRasterizer,
    metrics: &SharedTextMetrics,
    items: &[ContextMenuPresentationItem],
    style: &ContextMenuRasterStyle,
    paint_style: &ContextMenuPaintStyle,
    scale_factor: f32,
) -> Result<ContextMenuMeasuredPlan, ContextMenuAdapterError> {
    let rows = render_rows(
        text_rasterizer,
        svg_rasterizer,
        metrics,
        items,
        style,
        scale_factor,
    )?;
    let width = rows
        .iter()
        .map(|row| row.width)
        .max()
        .unwrap_or(MENU_MIN_WIDTH_PX)
        .max(MENU_MIN_WIDTH_PX);
    let height = u32::try_from(rows.len())
        .unwrap_or(u32::MAX)
        .saturating_mul(ROW_HEIGHT_PX)
        .saturating_add(MENU_PADDING_PX.saturating_mul(2));
    Ok(ContextMenuMeasuredPlan {
        plan: plan_for_rows(UiRect::new(0, 0, width, height), &rows, paint_style),
        width,
        height,
    })
}

pub(super) fn translate_plan(
    measured: &ContextMenuMeasuredPlan,
    bounds: UiRect,
    vertical_scroll_offset: f32,
) -> ContextMenuPaintPlan {
    let delta_x = bounds.x;
    let delta_y = bounds
        .y
        .saturating_sub(vertical_scroll_offset.round() as i32);
    ContextMenuPaintPlan {
        surface_bounds: bounds,
        operations: measured
            .plan
            .operations
            .iter()
            .map(|operation| translate_operation(operation, bounds, delta_x, delta_y))
            .collect(),
    }
}

struct RenderedRow {
    item: ContextMenuPresentationItem,
    icon: Option<ContextMenuPaintTexture>,
    label: ContextMenuPaintTexture,
    width: u32,
}

fn render_rows(
    text_rasterizer: &mut PlatformTextRasterizer,
    svg_rasterizer: &mut UiSvgRasterizer,
    metrics: &SharedTextMetrics,
    items: &[ContextMenuPresentationItem],
    style: &ContextMenuRasterStyle,
    scale_factor: f32,
) -> Result<Vec<RenderedRow>, ContextMenuAdapterError> {
    items
        .iter()
        .map(|item| {
            let label = raster_label(text_rasterizer, metrics, item, style, scale_factor)?;
            let icon = item
                .icon
                .as_ref()
                .map(|icon| raster_icon(svg_rasterizer, icon, style, scale_factor))
                .transpose()?;
            let width = u32::try_from(ITEM_LEFT_PADDING_PX)
                .unwrap_or_default()
                .saturating_add(label.width)
                .saturating_add(icon.as_ref().map_or(0, |texture| texture.width))
                .saturating_add(u32::from(icon.is_some()) * ICON_LABEL_GAP_PX as u32)
                .saturating_add(u32::try_from(ITEM_LEFT_PADDING_PX).unwrap_or_default());
            Ok(RenderedRow {
                item: item.clone(),
                icon,
                label,
                width,
            })
        })
        .collect()
}

fn plan_for_rows(
    bounds: UiRect,
    rows: &[RenderedRow],
    style: &ContextMenuPaintStyle,
) -> ContextMenuPaintPlan {
    let mut operations = vec![fill(bounds, bounds, style.background_rgba)];
    for (index, row) in rows.iter().enumerate() {
        let row_bounds = UiRect::new(
            bounds.x,
            bounds
                .y
                .saturating_add(MENU_PADDING_PX as i32)
                .saturating_add((index as i32).saturating_mul(ROW_HEIGHT_PX as i32)),
            bounds.width,
            ROW_HEIGHT_PX,
        );
        let color = if row.item.enabled {
            style.highlighted_rgba
        } else {
            style.disabled_rgba
        };
        operations.push(fill(bounds, row_bounds, color));
        let mut x = row_bounds.x.saturating_add(ITEM_LEFT_PADDING_PX);
        if let Some(icon) = &row.icon {
            operations.push(texture(
                bounds,
                UiRect::new(
                    x,
                    row_bounds.y.saturating_add(ITEM_TOP_PADDING_PX),
                    icon.width,
                    icon.height,
                ),
                icon.clone(),
            ));
            x = x
                .saturating_add_unsigned(icon.width)
                .saturating_add(ICON_LABEL_GAP_PX);
        }
        operations.push(texture(
            bounds,
            UiRect::new(
                x,
                row_bounds.y.saturating_add(ITEM_TOP_PADDING_PX),
                row.label.width,
                row.label.height,
            ),
            row.label.clone(),
        ));
    }
    ContextMenuPaintPlan {
        surface_bounds: bounds,
        operations,
    }
}

fn raster_label(
    rasterizer: &mut PlatformTextRasterizer,
    metrics: &SharedTextMetrics,
    item: &ContextMenuPresentationItem,
    style: &ContextMenuRasterStyle,
    scale_factor: f32,
) -> Result<ContextMenuPaintTexture, ContextMenuAdapterError> {
    let measured = metrics.borrow_mut().measure_text(
        rasterizer,
        &PlatformTextMetricsRequest::from_text(
            item.label.clone(),
            style.font.clone(),
            scale_factor,
        ),
    )?;
    let raster = rasterizer.rasterize(&PlatformTextRasterRequest {
        spans: UiTextSpan::emoji_marked_spans(
            &item.label,
            UiTextSpanStyle {
                color_rgba: style.text_color_rgba,
                ..UiTextSpanStyle::default()
            },
        ),
        font: style.font.clone(),
        fallback_color_rgba: style.text_color_rgba,
        line_height_px: measured.line_height_px / scale_factor.max(1.0),
        max_width_px: None,
        scale_factor,
    })?;
    Ok(ContextMenuPaintTexture {
        identity: format!("context-menu-label:{}:{}", item.id, item.label),
        width: raster.width as u32,
        height: raster.height as u32,
        rgba_pixels: raster.rgba_pixels.iter().flatten().copied().collect(),
    })
}

fn raster_icon(
    rasterizer: &mut UiSvgRasterizer,
    icon: &crate::render_model::UiIconProps,
    style: &ContextMenuRasterStyle,
    scale_factor: f32,
) -> Result<ContextMenuPaintTexture, ContextMenuAdapterError> {
    let size = (ICON_SIZE_PX as f32 * scale_factor).ceil() as u32;
    let [red, green, blue, alpha] = style.icon_color_rgba;
    let raster = rasterizer.rasterize(&UiSvgRasterRequest {
        icon: icon.clone(),
        width_px: size,
        height_px: size,
        color: RgbaColor::new(red, green, blue, alpha),
    })?;
    Ok(ContextMenuPaintTexture {
        identity: format!("context-menu-icon:{}", raster.metadata.cache_key),
        width: raster.width_px,
        height: raster.height_px,
        rgba_pixels: raster.rgba_unmultiplied,
    })
}

fn fill(
    clip_bounds: UiRect,
    bounds: UiRect,
    color_rgba: [u8; RGBA_CHANNEL_COUNT],
) -> ContextMenuPaintOperation {
    ContextMenuPaintOperation {
        clip_bounds,
        kind: ContextMenuPaintOperationKind::Fill { bounds, color_rgba },
    }
}

fn texture(
    clip_bounds: UiRect,
    bounds: UiRect,
    texture: ContextMenuPaintTexture,
) -> ContextMenuPaintOperation {
    ContextMenuPaintOperation {
        clip_bounds,
        kind: ContextMenuPaintOperationKind::Texture { bounds, texture },
    }
}

fn translate_operation(
    source: &ContextMenuPaintOperation,
    clip_bounds: UiRect,
    delta_x: i32,
    delta_y: i32,
) -> ContextMenuPaintOperation {
    let translate = |bounds: UiRect| {
        UiRect::new(
            bounds.x.saturating_add(delta_x),
            bounds.y.saturating_add(delta_y),
            bounds.width,
            bounds.height,
        )
    };
    let kind = match &source.kind {
        ContextMenuPaintOperationKind::Fill { bounds, color_rgba } => {
            ContextMenuPaintOperationKind::Fill {
                bounds: translate(*bounds),
                color_rgba: *color_rgba,
            }
        }
        ContextMenuPaintOperationKind::Texture { bounds, texture } => {
            ContextMenuPaintOperationKind::Texture {
                bounds: translate(*bounds),
                texture: texture.clone(),
            }
        }
    };
    ContextMenuPaintOperation { clip_bounds, kind }
}
