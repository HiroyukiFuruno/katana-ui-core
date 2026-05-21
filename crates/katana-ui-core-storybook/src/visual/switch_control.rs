use super::canvas::Canvas;
use super::palette::VisualPalette;

const BORDER_INSET: usize = 1;
const KNOB_INSET: usize = 3;
const EDGE_SAMPLE_COUNT: usize = 4;
const EDGE_SAMPLE_TOTAL: usize = EDGE_SAMPLE_COUNT * EDGE_SAMPLE_COUNT;
const OPAQUE_ALPHA: u8 = 255;
const COLOR_ALPHA_MAX: u32 = 255;
const SAMPLE_CENTER_OFFSET: f32 = 0.5;
const OFF_TRACK_TEXT_MIX_ALPHA: u32 = 48;
const SWITCH_THUMB_DARK_THEME: u32 = 0xf2f2f2;
const SWITCH_THUMB_LIGHT_THEME: u32 = 0xffffff;
const LUMINANCE_DARK_THRESHOLD: u32 = 128;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const BLUE_SHIFT: u32 = 0;
const CHANNEL_MASK: u32 = 0xff;
const LUMINANCE_RED_WEIGHT: u32 = 299;
const LUMINANCE_GREEN_WEIGHT: u32 = 587;
const LUMINANCE_BLUE_WEIGHT: u32 = 114;
const LUMINANCE_SCALE: u32 = 1000;

pub(super) fn draw_switch(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    enabled: bool,
) {
    let fill = switch_track_fill(palette, enabled);
    let radius = height / 2;
    draw_smooth_round_rect(canvas, x, y, width, height, radius, palette.border);
    draw_track_fill(canvas, x, y, width, height, radius, fill);
    let knob = height.saturating_sub(KNOB_INSET * 2);
    let knob_x = if enabled {
        x + width.saturating_sub(knob + KNOB_INSET)
    } else {
        x + KNOB_INSET
    };
    draw_smooth_round_rect(
        canvas,
        knob_x,
        y + KNOB_INSET,
        knob,
        knob,
        knob / 2,
        switch_thumb_fill(palette),
    );
}

fn draw_track_fill(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    radius: usize,
    fill: u32,
) {
    let inner_width = width.saturating_sub(BORDER_INSET * 2);
    let inner_height = height.saturating_sub(BORDER_INSET * 2);
    if inner_width == 0 || inner_height == 0 {
        return;
    }
    draw_smooth_round_rect(
        canvas,
        x + BORDER_INSET,
        y + BORDER_INSET,
        inner_width,
        inner_height,
        radius.saturating_sub(BORDER_INSET),
        fill,
    );
}

fn draw_smooth_round_rect(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    radius: usize,
    color: u32,
) {
    if width == 0 || height == 0 {
        return;
    }
    let rect = SmoothRoundRect::new(x, y, width, height, radius);
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            let alpha = round_rect_alpha(current_x, current_y, rect);
            if alpha == 0 {
                continue;
            }
            canvas.blend(current_x, current_y, color, alpha);
        }
    }
}

fn round_rect_alpha(pixel_x: usize, pixel_y: usize, rect: SmoothRoundRect) -> u8 {
    let mut inside = 0;
    for sample_y in 0..EDGE_SAMPLE_COUNT {
        for sample_x in 0..EDGE_SAMPLE_COUNT {
            if sample_inside_round_rect(
                pixel_x,
                pixel_y,
                rect,
                SmoothSample::new(sample_x, sample_y),
            ) {
                inside += 1;
            }
        }
    }
    ((inside * usize::from(OPAQUE_ALPHA)) / EDGE_SAMPLE_TOTAL) as u8
}

fn sample_inside_round_rect(
    pixel_x: usize,
    pixel_y: usize,
    rect: SmoothRoundRect,
    sample: SmoothSample,
) -> bool {
    let local_x = pixel_x.saturating_sub(rect.x) as f32 + sample.offset_x();
    let local_y = pixel_y.saturating_sub(rect.y) as f32 + sample.offset_y();
    let width = rect.width as f32;
    let height = rect.height as f32;
    let radius = (rect.radius as f32).min(width / 2.0).min(height / 2.0);
    let clamp_x = local_x.clamp(radius, width - radius);
    let clamp_y = local_y.clamp(radius, height - radius);
    let delta_x = local_x - clamp_x;
    let delta_y = local_y - clamp_y;
    delta_x * delta_x + delta_y * delta_y <= radius * radius
}

fn sample_offset(index: usize) -> f32 {
    (index as f32 + SAMPLE_CENTER_OFFSET) / EDGE_SAMPLE_COUNT as f32
}

#[derive(Clone, Copy)]
struct SmoothRoundRect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    radius: usize,
}

impl SmoothRoundRect {
    const fn new(x: usize, y: usize, width: usize, height: usize, radius: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            radius,
        }
    }

    const fn right(self) -> usize {
        self.x + self.width
    }

    const fn bottom(self) -> usize {
        self.y + self.height
    }
}

#[derive(Clone, Copy)]
struct SmoothSample {
    x: usize,
    y: usize,
}

impl SmoothSample {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn offset_x(self) -> f32 {
        sample_offset(self.x)
    }

    fn offset_y(self) -> f32 {
        sample_offset(self.y)
    }
}

fn switch_track_fill(palette: &VisualPalette, enabled: bool) -> u32 {
    if enabled {
        return palette.accent;
    }
    mix_color(palette.surface, palette.text, OFF_TRACK_TEXT_MIX_ALPHA)
}

fn switch_thumb_fill(palette: &VisualPalette) -> u32 {
    if luminance(palette.background) < LUMINANCE_DARK_THRESHOLD {
        return SWITCH_THUMB_DARK_THEME;
    }
    SWITCH_THUMB_LIGHT_THEME
}

fn mix_color(base: u32, overlay: u32, alpha: u32) -> u32 {
    let inverse = COLOR_ALPHA_MAX - alpha;
    let red = mix_channel(base, overlay, alpha, inverse, RED_SHIFT);
    let green = mix_channel(base, overlay, alpha, inverse, GREEN_SHIFT);
    let blue = mix_channel(base, overlay, alpha, inverse, BLUE_SHIFT);
    (red << RED_SHIFT) | (green << GREEN_SHIFT) | blue
}

fn mix_channel(base: u32, overlay: u32, alpha: u32, inverse: u32, shift: u32) -> u32 {
    let base_channel = (base >> shift) & CHANNEL_MASK;
    let overlay_channel = (overlay >> shift) & CHANNEL_MASK;
    (overlay_channel * alpha + base_channel * inverse) / COLOR_ALPHA_MAX
}

fn luminance(color: u32) -> u32 {
    let red = (color >> RED_SHIFT) & CHANNEL_MASK;
    let green = (color >> GREEN_SHIFT) & CHANNEL_MASK;
    let blue = color & CHANNEL_MASK;
    (red * LUMINANCE_RED_WEIGHT + green * LUMINANCE_GREEN_WEIGHT + blue * LUMINANCE_BLUE_WEIGHT)
        / LUMINANCE_SCALE
}

#[cfg(test)]
mod tests {
    use super::{draw_switch, switch_thumb_fill};
    use crate::visual::canvas::Canvas;
    use crate::visual::palette::VisualPalette;
    use katana_ui_core::theme::ThemeSnapshot;
    use std::collections::HashSet;

    const SWITCH_X: usize = 8;
    const SWITCH_Y: usize = 8;
    const SWITCH_WIDTH: usize = 48;
    const SWITCH_HEIGHT: usize = 22;
    const MIN_SMOOTH_COLOR_COUNT: usize = 6;

    #[test]
    fn switch_surface_has_smooth_edge_pixels() {
        let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());
        let canvas = rendered_switch(&palette, true);
        let colors = switch_colors(&canvas);

        assert!(colors.len() >= MIN_SMOOTH_COLOR_COUNT);
    }

    #[test]
    fn switch_thumb_is_filled_instead_of_cut_out_from_background() {
        let palette = VisualPalette::from_theme(&ThemeSnapshot::dark());
        let canvas = rendered_switch(&palette, true);
        let thumb_center_x = SWITCH_X + SWITCH_WIDTH - 11;
        let thumb_center_y = SWITCH_Y + SWITCH_HEIGHT / 2;
        let color = canvas.pixels()[thumb_center_y * canvas.width() + thumb_center_x];

        assert_eq!(switch_thumb_fill(&palette), color);
        assert_ne!(palette.background, color);
    }

    fn rendered_switch(palette: &VisualPalette, enabled: bool) -> Canvas {
        let mut canvas = Canvas::new(80, 48, palette.background);
        draw_switch(
            &mut canvas,
            palette,
            SWITCH_X,
            SWITCH_Y,
            SWITCH_WIDTH,
            SWITCH_HEIGHT,
            enabled,
        );
        canvas
    }

    fn switch_colors(canvas: &Canvas) -> HashSet<u32> {
        let mut colors = HashSet::new();
        for y in SWITCH_Y..SWITCH_Y + SWITCH_HEIGHT {
            for x in SWITCH_X..SWITCH_X + SWITCH_WIDTH {
                colors.insert(canvas.pixels()[y * canvas.width() + x]);
            }
        }
        colors
    }
}
