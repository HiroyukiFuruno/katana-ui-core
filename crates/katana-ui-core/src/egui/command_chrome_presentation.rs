use super::EguiCommandChromeAdapter;
use super::command_chrome_types::{
    CommandChromeRasterStyle, EguiCommandChromeActionFrame, EguiCommandChromeError, RenderedAction,
    RenderedRaster, logical_to_physical,
};
use crate::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdownTrigger,
};
use crate::render_model::{UiIconProps, UiRect, UiTextSpan, UiTextSpanStyle};
use crate::svg_raster::UiSvgRasterRequest;
use crate::text_raster::{PlatformTextMetricsRequest, PlatformTextRasterRequest};

const ACTION_PADDING_PX: u32 = 8;
const SPLIT_SECONDARY_WIDTH_PX: u32 = 20;

impl EguiCommandChromeAdapter {
    pub(super) fn render_action(
        &mut self,
        ui: &egui::Ui,
        action: &CommandChromeAction,
        display_mode: CommandChromeDisplayMode,
        style: &CommandChromeRasterStyle,
    ) -> Result<RenderedAction, EguiCommandChromeError> {
        let scale = ui.ctx().pixels_per_point();
        let icon = action
            .icon_model()
            .map(|icon| self.raster_icon(icon, style, scale))
            .transpose()?;
        let label = (!matches!(display_mode, CommandChromeDisplayMode::IconOnly))
            .then(|| self.raster_label(action.label_model(), style, scale))
            .transpose()?;
        let width = icon
            .as_ref()
            .map_or(0, |value| value.width)
            .saturating_add(label.as_ref().map_or(0, |value| value.width))
            .saturating_add(ACTION_PADDING_PX.saturating_mul(2))
            .saturating_add(split_secondary_width(action));
        let height = icon
            .as_ref()
            .map_or(0, |value| value.height)
            .max(label.as_ref().map_or(0, |value| value.height))
            .saturating_add(ACTION_PADDING_PX.saturating_mul(2));
        Ok(RenderedAction {
            bounds: UiRect::new(0, 0, width.max(1), height.max(1)),
            icon_identity: icon.as_ref().map(|raster| raster.identity.clone()),
            label_identity: label.as_ref().map(|raster| raster.identity.clone()),
            icon,
            label,
        })
    }

    pub(super) fn raster_icon(
        &mut self,
        icon: &UiIconProps,
        style: &CommandChromeRasterStyle,
        scale: f32,
    ) -> Result<RenderedRaster, EguiCommandChromeError> {
        let width = logical_to_physical(style.icon_size_px, scale);
        let raster = self.svg_rasterizer.rasterize(&UiSvgRasterRequest {
            icon: icon.clone(),
            width_px: width,
            height_px: width,
            color: style.icon_color,
        })?;
        Ok(RenderedRaster::new(
            raster.metadata.cache_key,
            raster.width_px,
            raster.height_px,
            raster.rgba_unmultiplied,
            scale,
        ))
    }

    pub(super) fn raster_label(
        &mut self,
        label: &str,
        style: &CommandChromeRasterStyle,
        scale: f32,
    ) -> Result<RenderedRaster, EguiCommandChromeError> {
        let request = PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                label,
                UiTextSpanStyle {
                    color_rgba: style.text_color_rgba,
                    ..UiTextSpanStyle::default()
                },
            ),
            font: style.font.clone(),
            fallback_color_rgba: style.text_color_rgba,
            line_height_px: style.line_height_px,
            max_width_px: None,
            scale_factor: scale,
        };
        let measured = self.metrics.borrow_mut().measure_text(
            &mut self.text_rasterizer,
            &PlatformTextMetricsRequest::from_text(label, style.font.clone(), scale),
        )?;
        let mut request = request;
        request.line_height_px = measured.line_height_px / scale.max(1.0);
        let raster = self.text_rasterizer.rasterize(&request)?;
        let pixels = raster.rgba_pixels.iter().flatten().copied().collect();
        Ok(RenderedRaster::new(
            format!("command-label:{label}:{style:?}"),
            raster.width as u32,
            raster.height as u32,
            pixels,
            scale,
        ))
    }
}

pub(super) fn toolbar_size(ui: &egui::Ui, actions: &[RenderedAction]) -> egui::Vec2 {
    let gaps = actions.len().saturating_sub(1) as f32 * ui.spacing().item_spacing.x;
    let width = actions
        .iter()
        .map(|action| action.bounds.width as f32)
        .sum::<f32>()
        + gaps;
    let height = actions
        .iter()
        .map(|action| action.bounds.height as f32)
        .fold(1.0_f32, f32::max);
    egui::vec2(width.max(1.0), height)
}

pub(super) fn split_rects(
    rect: egui::Rect,
    action: &CommandChromeAction,
) -> (egui::Rect, Option<egui::Rect>) {
    let secondary_width = split_secondary_width(action) as f32;
    if secondary_width == 0.0 {
        return (rect, None);
    }
    let primary = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.max.x - secondary_width, rect.max.y),
    );
    let secondary = egui::Rect::from_min_max(egui::pos2(primary.max.x, rect.min.y), rect.max);
    (primary, Some(secondary))
}

pub(super) fn frame_bounds(start: egui::Pos2, frames: &[EguiCommandChromeActionFrame]) -> UiRect {
    let right = frames
        .iter()
        .map(|frame| frame.bounds.x.saturating_add(frame.bounds.width as i32))
        .max()
        .unwrap_or(start.x.round() as i32);
    let bottom = frames
        .iter()
        .map(|frame| frame.bounds.y.saturating_add(frame.bounds.height as i32))
        .max()
        .unwrap_or(start.y.round() as i32);
    UiRect::new(
        start.x.round() as i32,
        start.y.round() as i32,
        right.saturating_sub(start.x.round() as i32) as u32,
        bottom.saturating_sub(start.y.round() as i32) as u32,
    )
}

fn split_secondary_width(action: &CommandChromeAction) -> u32 {
    matches!(
        action
            .dropdown_model()
            .map(|dropdown| dropdown.trigger_model()),
        Some(CommandChromeDropdownTrigger::SplitSecondary)
    )
    .then_some(SPLIT_SECONDARY_WIDTH_PX)
    .unwrap_or(0)
}
