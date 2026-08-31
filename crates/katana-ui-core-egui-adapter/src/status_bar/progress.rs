use super::adapter::EguiStatusBarAdapter;
use super::paint::StatusBarPaint;
use super::types::{
    StatusBarPaintOperation, StatusBarPaintOperationKind, StatusBarPaintPlan,
    StatusBarPaintTexture, StatusBarRenderStyle,
};
use katana_ui_core::molecule::ProgressMeterShape;
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiTone};

const PROGRESS_BOTTOM_OFFSET_PX: f32 = 4.0;
const PROGRESS_HEIGHT_PX: f32 = 3.0;
const PROGRESS_DIAMETER_PX: f32 = 14.0;
const FULL_PERCENT: f32 = 100.0;
const PIXEL_CENTER_OFFSET: f32 = 0.5;
const DIAMETER_TO_RADIUS_DIVISOR: f32 = 2.0;
const RING_STROKE_WIDTH_PX: f32 = 2.0;
const RGBA_COMPONENT_COUNT: usize = RGBA_CHANNEL_COUNT;
const PROGRESS_BACKGROUND_RGBA: [u8; RGBA_CHANNEL_COUNT] = [80, 80, 80, 255];

struct ProgressPaint<'a> {
    bounds: egui::Rect,
    percent: u8,
    tone: UiTone,
    style: &'a StatusBarRenderStyle,
    paint_plan: &'a mut StatusBarPaintPlan,
}

impl EguiStatusBarAdapter {
    pub(super) fn paint_progress(
        &mut self,
        ui: &egui::Ui,
        bounds: egui::Rect,
        shape: ProgressMeterShape,
        percent: u8,
        tone: UiTone,
        style: &StatusBarRenderStyle,
        paint_plan: &mut StatusBarPaintPlan,
    ) {
        let paint = ProgressPaint {
            bounds,
            percent,
            tone,
            style,
            paint_plan,
        };
        match shape {
            ProgressMeterShape::Linear => self.paint_linear_progress(ui, paint),
            ProgressMeterShape::Ring => self.paint_radial_progress(paint, "ring", false),
            ProgressMeterShape::Pie => self.paint_radial_progress(paint, "pie", true),
        }
    }

    fn paint_linear_progress(&mut self, ui: &egui::Ui, paint: ProgressPaint<'_>) {
        let ProgressPaint {
            bounds,
            percent,
            tone,
            style,
            paint_plan,
        } = paint;
        let bar = egui::Rect::from_min_size(
            egui::pos2(bounds.left(), bounds.bottom() - PROGRESS_BOTTOM_OFFSET_PX),
            egui::vec2(bounds.width(), PROGRESS_HEIGHT_PX),
        );
        let fill = egui::Rect::from_min_size(
            bar.min,
            egui::vec2(
                bar.width() * f32::from(percent) / FULL_PERCENT,
                bar.height(),
            ),
        );
        let foreground = StatusBarPaint::tone_color(tone, style.neutral_text_rgba);
        paint_plan.operations.extend([
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
        ui.painter()
            .rect_filled(bar, 1.0, StatusBarPaint::color(PROGRESS_BACKGROUND_RGBA));
        ui.painter()
            .rect_filled(fill, 1.0, StatusBarPaint::color(foreground));
    }

    fn paint_radial_progress(&mut self, paint: ProgressPaint<'_>, shape: &str, filled: bool) {
        let ProgressPaint {
            bounds,
            percent,
            tone,
            style,
            paint_plan,
        } = paint;
        let meter = progress_meter_bounds(bounds);
        let foreground = StatusBarPaint::tone_color(tone, style.neutral_text_rgba);
        paint_plan.operations.push(StatusBarPaintOperation {
            clip_bounds: StatusBarPaint::ui_rect(bounds),
            kind: StatusBarPaintOperationKind::Texture {
                bounds: StatusBarPaint::ui_rect(meter),
                texture: progress_texture(
                    shape,
                    meter,
                    percent,
                    PROGRESS_BACKGROUND_RGBA,
                    foreground,
                    filled,
                ),
            },
        });
    }
}

fn progress_meter_bounds(bounds: egui::Rect) -> egui::Rect {
    egui::Rect::from_center_size(
        egui::pos2(
            bounds.right()
                - PROGRESS_DIAMETER_PX / DIAMETER_TO_RADIUS_DIVISOR
                - PROGRESS_BOTTOM_OFFSET_PX,
            bounds.center().y,
        ),
        egui::vec2(PROGRESS_DIAMETER_PX, PROGRESS_DIAMETER_PX),
    )
    .intersect(bounds)
}

fn progress_texture(
    shape: &str,
    bounds: egui::Rect,
    percent: u8,
    background_rgba: [u8; RGBA_CHANNEL_COUNT],
    foreground_rgba: [u8; RGBA_CHANNEL_COUNT],
    filled: bool,
) -> StatusBarPaintTexture {
    let width = bounds.width().round().max(1.0) as u32;
    let height = bounds.height().round().max(1.0) as u32;
    let radius = width.min(height) as f32 / DIAMETER_TO_RADIUS_DIVISOR;
    let center_x = width as f32 / DIAMETER_TO_RADIUS_DIVISOR;
    let center_y = height as f32 / DIAMETER_TO_RADIUS_DIVISOR;
    let progress_angle = std::f32::consts::TAU * f32::from(percent) / FULL_PERCENT;
    let mut rgba_pixels =
        Vec::with_capacity(width as usize * height as usize * RGBA_COMPONENT_COUNT);
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 + PIXEL_CENTER_OFFSET - center_x;
            let dy = y as f32 + PIXEL_CENTER_OFFSET - center_y;
            let distance = dx.hypot(dy);
            let angle =
                (dy.atan2(dx) + std::f32::consts::FRAC_PI_2).rem_euclid(std::f32::consts::TAU);
            let in_shape = if filled {
                distance <= radius
            } else {
                (radius - RING_STROKE_WIDTH_PX..=radius).contains(&distance)
            };
            let color = if in_shape && angle <= progress_angle {
                foreground_rgba
            } else if in_shape {
                background_rgba
            } else {
                [0, 0, 0, 0]
            };
            rgba_pixels.extend(color);
        }
    }
    StatusBarPaintTexture {
        identity: format!(
            "status-bar-progress:{shape}:{percent}:{width}x{height}:{}-{}-{}-{}:{}-{}-{}-{}",
            background_rgba[0],
            background_rgba[1],
            background_rgba[2],
            background_rgba[3],
            foreground_rgba[0],
            foreground_rgba[1],
            foreground_rgba[2],
            foreground_rgba[3],
        ),
        width,
        height,
        rgba_pixels,
    }
}
