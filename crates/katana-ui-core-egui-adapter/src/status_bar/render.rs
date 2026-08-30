use super::accessibility::publish_accessibility;
use super::adapter::EguiStatusBarAdapter;
use super::paint::StatusBarPaint;
use super::types::{
    EguiStatusBarError, EguiStatusBarOutput, StatusBarLabelRasterEvidence, StatusBarPaintOperation,
    StatusBarPaintOperationKind, StatusBarPaintTexture, StatusBarRenderStyle,
};
use katana_ui_core::molecule::{
    StatusBar, StatusBarAction, StatusBarSegment, StatusBarSegmentAlignment,
};
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle, UiTone};
use katana_ui_core_text_raster::PlatformTextRasterRequest;
use sha2::{Digest, Sha256};

const LINE_HEIGHT_MULTIPLIER: f32 = 1.45;
const PROGRESS_BOTTOM_OFFSET_PX: f32 = 4.0;
const PROGRESS_HEIGHT_PX: f32 = 3.0;
const PROGRESS_BACKGROUND_RGBA: [u8; katana_ui_core::render_model::RGBA_CHANNEL_COUNT] =
    [80, 80, 80, 255];
pub(super) const STATUS_ALIGNMENTS: [StatusBarSegmentAlignment; 3] = [
    StatusBarSegmentAlignment::Leading,
    StatusBarSegmentAlignment::Center,
    StatusBarSegmentAlignment::Trailing,
];

pub(super) struct SegmentSnapshot {
    id: String,
    label: String,
    accessibility: String,
    tone: UiTone,
    interactive: bool,
    icon: Option<String>,
    tooltip: Option<String>,
    progress: Option<ProgressSnapshot>,
}
struct ProgressSnapshot {
    percent: u8,
    tone: UiTone,
}
impl SegmentSnapshot {
    pub(super) fn single_message(message: String, accessibility: &str) -> Self {
        Self {
            id: "single-message".into(),
            label: message,
            accessibility: accessibility.into(),
            tone: UiTone::Neutral,
            interactive: false,
            icon: None,
            tooltip: None,
            progress: None,
        }
    }
    fn display_label(&self) -> String {
        match &self.icon {
            Some(icon) => format!("{icon} {}", self.label),
            None => self.label.clone(),
        }
    }
}
impl From<&StatusBarSegment> for SegmentSnapshot {
    fn from(segment: &StatusBarSegment) -> Self {
        Self {
            id: segment.id().into(),
            label: segment.label().into(),
            accessibility: segment.accessibility_label_text().into(),
            tone: segment.tone_value(),
            interactive: segment.is_interactive(),
            icon: segment.icon_name().map(str::to_owned),
            tooltip: segment.tooltip_text().map(str::to_owned),
            progress: segment.progress_spec().map(|progress| ProgressSnapshot {
                percent: progress.percent(),
                tone: progress.tone_value(),
            }),
        }
    }
}

impl EguiStatusBarAdapter {
    pub(super) fn paint_alignment(
        &mut self,
        ui: &mut egui::Ui,
        root: egui::Rect,
        status: &mut StatusBar,
        alignment: StatusBarSegmentAlignment,
        style: &StatusBarRenderStyle,
        out: &mut EguiStatusBarOutput,
    ) -> Result<(), EguiStatusBarError> {
        let segments: Vec<_> = status
            .segments_for(alignment)
            .into_iter()
            .map(SegmentSnapshot::from)
            .collect();
        if segments.is_empty() {
            return Ok(());
        }
        let widths = segments
            .iter()
            .map(|segment| self.raster_width(&segment.display_label(), style))
            .collect::<Result<Vec<_>, _>>()?;
        let gap = style.segment_gap_px as f32;
        let total = widths.iter().sum::<f32>() + gap * widths.len().saturating_sub(1) as f32;
        let mut x = match alignment {
            StatusBarSegmentAlignment::Leading => root.left(),
            StatusBarSegmentAlignment::Center => root.center().x - total / 2.0,
            StatusBarSegmentAlignment::Trailing => root.right() - total,
        };
        for (segment, width) in segments.iter().zip(widths) {
            let bounds = egui::Rect::from_min_size(
                egui::pos2(x, root.top()),
                egui::vec2(width, root.height()),
            );
            self.paint_segment(ui, bounds, segment, style, out, status)?;
            x += width + gap;
        }
        Ok(())
    }

    pub(super) fn paint_segment(
        &mut self,
        ui: &mut egui::Ui,
        bounds: egui::Rect,
        segment: &SegmentSnapshot,
        style: &StatusBarRenderStyle,
        out: &mut EguiStatusBarOutput,
        status: &mut StatusBar,
    ) -> Result<(), EguiStatusBarError> {
        let label = segment.display_label();
        let response = ui.interact(
            bounds,
            self.id.with(&segment.id),
            if segment.interactive {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            },
        );
        if segment.interactive {
            publish_accessibility(ui, response.id, bounds, &segment.accessibility, &segment.id);
            let response_has_focus = response.has_focus();
            if response.clicked()
                || ui.input(|input| {
                    input.has_accesskit_action_request(response.id, egui::accesskit::Action::Click)
                        || (response_has_focus
                            && (input.key_pressed(egui::Key::Enter)
                                || input.key_pressed(egui::Key::Space)))
                })
            {
                out.events
                    .extend(status.apply_action(&StatusBarAction::PressSegment {
                        id: segment.id.clone(),
                    }));
            }
        }
        if let Some(tooltip) = segment.tooltip.as_deref() {
            response.clone().on_hover_text(tooltip);
            if response.hovered() && self.last_tooltip_segment.as_deref() != Some(&segment.id) {
                out.events
                    .extend(status.apply_action(&StatusBarAction::ShowTooltip {
                        id: segment.id.clone(),
                    }));
                self.last_tooltip_segment = Some(segment.id.clone());
            }
        }
        if let Some(progress) = segment.progress.as_ref() {
            self.paint_progress(ui, bounds, progress.percent, progress.tone, style);
        }
        let scale = ui.ctx().pixels_per_point();
        let tone = StatusBarPaint::tone_color(segment.tone, style.neutral_text_rgba);
        let raster = self.text_rasterizer.rasterize(&PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                &label,
                UiTextSpanStyle {
                    color_rgba: tone,
                    ..UiTextSpanStyle::default()
                },
            ),
            font: style.font.clone(),
            fallback_color_rgba: tone,
            line_height_px: style.font.size * LINE_HEIGHT_MULTIPLIER,
            max_width_px: None,
            scale_factor: scale,
        })?;
        let pixels: Vec<u8> = raster.rgba_pixels.iter().flatten().copied().collect();
        let fingerprint = hex::encode(Sha256::digest(label.as_bytes()));
        let identity = format!("status-bar-label:{fingerprint}:{:?}:{scale}", style.font);
        self.last_label_rasters.push(StatusBarLabelRasterEvidence {
            label_fingerprint: fingerprint,
            width: raster.width as u32,
            height: raster.height as u32,
            chromatic_pixel_count: raster.chromatic_pixel_count(),
            sha256: hex::encode(Sha256::digest(&pixels)),
        });
        let image = egui::Rect::from_center_size(
            bounds.center(),
            egui::vec2(raster.width as f32 / scale, raster.height as f32 / scale),
        );
        if let Some(plan) = self.last_paint_plan.as_mut() {
            plan.operations.push(StatusBarPaintOperation {
                clip_bounds: StatusBarPaint::ui_rect(bounds),
                kind: StatusBarPaintOperationKind::Texture {
                    bounds: StatusBarPaint::ui_rect(image),
                    texture: StatusBarPaintTexture {
                        identity,
                        width: raster.width as u32,
                        height: raster.height as u32,
                        rgba_pixels: pixels,
                    },
                },
            });
        }
        Ok(())
    }

    fn paint_progress(
        &mut self,
        ui: &egui::Ui,
        bounds: egui::Rect,
        percent: u8,
        tone: UiTone,
        style: &StatusBarRenderStyle,
    ) {
        let bar = egui::Rect::from_min_size(
            egui::pos2(bounds.left(), bounds.bottom() - PROGRESS_BOTTOM_OFFSET_PX),
            egui::vec2(bounds.width(), PROGRESS_HEIGHT_PX),
        );
        let fill = egui::Rect::from_min_size(
            bar.min,
            egui::vec2(bar.width() * f32::from(percent) / 100.0, bar.height()),
        );
        let foreground = StatusBarPaint::tone_color(tone, style.neutral_text_rgba);
        if let Some(plan) = self.last_paint_plan.as_mut() {
            plan.operations.extend([
                StatusBarPaintOperation {
                    clip_bounds: StatusBarPaint::ui_rect(bar),
                    kind: StatusBarPaintOperationKind::Fill {
                        bounds: StatusBarPaint::ui_rect(bar),
                        color_rgba: PROGRESS_BACKGROUND_RGBA,
                    },
                },
                StatusBarPaintOperation {
                    clip_bounds: StatusBarPaint::ui_rect(fill),
                    kind: StatusBarPaintOperationKind::Fill {
                        bounds: StatusBarPaint::ui_rect(fill),
                        color_rgba: foreground,
                    },
                },
            ]);
        }
        ui.painter()
            .rect_filled(bar, 1.0, StatusBarPaint::color(PROGRESS_BACKGROUND_RGBA));
        ui.painter()
            .rect_filled(fill, 1.0, StatusBarPaint::color(foreground));
    }

    fn raster_width(
        &mut self,
        label: &str,
        style: &StatusBarRenderStyle,
    ) -> Result<f32, EguiStatusBarError> {
        let raster = self.text_rasterizer.rasterize(&PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(label, UiTextSpanStyle::default()),
            font: style.font.clone(),
            fallback_color_rgba: style.neutral_text_rgba,
            line_height_px: style.font.size * LINE_HEIGHT_MULTIPLIER,
            max_width_px: None,
            scale_factor: 1.0,
        })?;
        Ok(raster.width as f32 + style.segment_padding_px as f32 * 2.0)
    }
}
