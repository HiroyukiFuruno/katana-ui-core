use super::paint_geometry::{color, egui_rect};
use super::types::{
    ContextMenuAdapterError, ContextMenuPaintOperation, ContextMenuPaintOperationKind,
    ContextMenuPaintPlan, ContextMenuPaintStyle, ContextMenuPaintTexture,
    ContextMenuPresentationItem, ContextMenuRasterStyle, ICON_LABEL_GAP_PX, ICON_SIZE_PX,
    ITEM_LEFT_PADDING_PX, ITEM_TOP_PADDING_PX, MENU_MIN_WIDTH_PX, MENU_PADDING_PX, ROW_HEIGHT_PX,
};
use crate::texture_cache::RgbaTextureCache;
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect, UiTextSpan, UiTextSpanStyle};
use katana_ui_core_svg_raster::{UiSvgRasterRequest, UiSvgRasterizer};
use katana_ui_core_text_raster::{PlatformTextRasterRequest, PlatformTextRasterizer};

pub(super) struct ContextMenuMeasuredPlan {
    pub(super) plan: ContextMenuPaintPlan,
    pub(super) width: u32,
    pub(super) height: u32,
}

pub(super) fn measure_and_build_plan(
    text_rasterizer: &mut PlatformTextRasterizer,
    svg_rasterizer: &mut UiSvgRasterizer,
    items: &[ContextMenuPresentationItem],
    style: &ContextMenuRasterStyle,
    paint_style: &ContextMenuPaintStyle,
    scale_factor: f32,
) -> Result<ContextMenuMeasuredPlan, ContextMenuAdapterError> {
    let rows = render_rows(text_rasterizer, svg_rasterizer, items, style, scale_factor)?;
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

pub(super) fn paint_plan(ui: &egui::Ui, cache: &mut RgbaTextureCache, plan: &ContextMenuPaintPlan) {
    for operation in &plan.operations {
        let painter = ui
            .painter()
            .with_clip_rect(egui_rect(operation.clip_bounds));
        match &operation.kind {
            ContextMenuPaintOperationKind::Fill { bounds, color_rgba } => {
                painter.rect_filled(egui_rect(*bounds), 0.0, color(*color_rgba));
            }
            ContextMenuPaintOperationKind::Texture { bounds, texture } => {
                let handle = cache.texture_for_rgba(
                    ui.ctx(),
                    &texture.identity,
                    texture.width as usize,
                    texture.height as usize,
                    &texture.rgba_pixels,
                );
                painter.image(
                    handle.id(),
                    egui_rect(*bounds),
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
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
    items: &[ContextMenuPresentationItem],
    style: &ContextMenuRasterStyle,
    scale_factor: f32,
) -> Result<Vec<RenderedRow>, ContextMenuAdapterError> {
    items
        .iter()
        .map(|item| {
            let label = raster_label(text_rasterizer, item, style, scale_factor)?;
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
    item: &ContextMenuPresentationItem,
    style: &ContextMenuRasterStyle,
    scale_factor: f32,
) -> Result<ContextMenuPaintTexture, ContextMenuAdapterError> {
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
        line_height_px: style.line_height_px,
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
    icon: &katana_ui_core::render_model::UiIconProps,
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

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::UiIconProps;
    use katana_ui_core::theme::{FontFamily, FontToken};
    use katana_ui_core_svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
    use katana_ui_core_text_raster::{PlatformTextRasterConfig, PlatformTextRasterizer};

    fn texture_fixture(identity: &str) -> ContextMenuPaintTexture {
        ContextMenuPaintTexture {
            identity: identity.to_owned(),
            width: 1,
            height: 1,
            rgba_pixels: vec![255, 0, 0, 255],
        }
    }

    #[test]
    fn row_plan_translation_and_egui_paint_cover_fill_icon_and_label_layers() {
        let rows = [
            RenderedRow {
                item: ContextMenuPresentationItem::action("enabled", "Enabled"),
                icon: Some(texture_fixture("icon")),
                label: texture_fixture("label-enabled"),
                width: 80,
            },
            RenderedRow {
                item: ContextMenuPresentationItem {
                    enabled: false,
                    ..ContextMenuPresentationItem::action("disabled", "Disabled")
                },
                icon: None,
                label: texture_fixture("label-disabled"),
                width: 70,
            },
        ];
        let style = ContextMenuPaintStyle {
            background_rgba: [1, 2, 3, 255],
            highlighted_rgba: [4, 5, 6, 255],
            disabled_rgba: [7, 8, 9, 255],
        };
        let local = plan_for_rows(UiRect::new(0, 0, 180, 72), &rows, &style);
        let measured = ContextMenuMeasuredPlan {
            plan: local,
            width: 180,
            height: 72,
        };
        let translated = translate_plan(&measured, UiRect::new(10, 20, 180, 72), 3.0);
        assert_eq!(translated.surface_bounds, UiRect::new(10, 20, 180, 72));
        assert!(
            translated.operations.iter().any(|operation| matches!(
                operation.kind,
                ContextMenuPaintOperationKind::Fill { .. }
            ))
        );
        assert!(translated.operations.iter().any(|operation| matches!(
            operation.kind,
            ContextMenuPaintOperationKind::Texture { .. }
        )));

        let context = egui::Context::default();
        let mut cache = RgbaTextureCache::new(4);
        crate::run_ui_discard(&context, Default::default(), |ui| {
            paint_plan(ui, &mut cache, &translated);
        });
        assert_eq!(
            color([1, 2, 3, 4]),
            egui::Color32::from_rgba_unmultiplied(1, 2, 3, 4)
        );
        assert_eq!(egui_rect(UiRect::new(1, 2, 3, 4)).min, egui::pos2(1.0, 2.0));
    }

    #[test]
    fn measured_plan_rasterizes_presented_svg_icons() {
        let item = ContextMenuPresentationItem {
            icon: Some(UiIconProps::new(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"><rect width="1" height="1"/></svg>"#,
            )),
            ..ContextMenuPresentationItem::action("icon", "Icon")
        };
        let raster_style = ContextMenuRasterStyle {
            font: FontToken {
                name: "context-menu-icon-test".into(),
                family: FontFamily::Proportional,
                size: 12.0,
                weight: 400,
            },
            text_color_rgba: [255; 4],
            icon_color_rgba: [255; 4],
            line_height_px: 16.0,
        };
        let paint_style = ContextMenuPaintStyle {
            background_rgba: [0; 4],
            highlighted_rgba: [1; 4],
            disabled_rgba: [2; 4],
        };
        let mut text = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
        let mut svg = UiSvgRasterizer::new(UiSvgRasterConfig::default());
        let measured = measure_and_build_plan(
            &mut text,
            &mut svg,
            &[item],
            &raster_style,
            &paint_style,
            1.0,
        )
        .expect("the valid icon must rasterize");
        assert!(measured.plan.operations.iter().any(|operation| matches!(
            operation.kind,
            ContextMenuPaintOperationKind::Texture { ref texture, .. }
                if texture.identity.starts_with("context-menu-icon:")
        )));
    }
}
