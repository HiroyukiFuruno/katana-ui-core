use super::canvas::Canvas;
use super::presentation_frame_scale::{
    try_present_exact_integer_scale, try_present_exact_integer_scale_into,
    try_present_exact_integer_scale_region_into,
};

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const BLUE_SHIFT: u32 = 0;
const CHANNEL_MASK: u32 = 0xff;
const SAMPLE_CENTER_OFFSET: f32 = 0.5;

#[must_use]
pub(super) fn present_frame_for_window(
    source: &Canvas,
    width: usize,
    height: usize,
    fill: u32,
) -> Canvas {
    present_frame(source, width, height, fill)
}

pub(super) fn present_frame_for_window_into(
    source: &Canvas,
    target: &mut Canvas,
    width: usize,
    height: usize,
    fill: u32,
) {
    if width == 0 || height == 0 {
        *target = Canvas::new(1, 1, fill);
        return;
    }
    if source.width() == 0 || source.height() == 0 {
        *target = Canvas::new(width, height, fill);
        return;
    }
    let rect = PresentationRect::fit(source.width(), source.height(), width, height);
    if rect.x == 0
        && rect.y == 0
        && rect.width == width
        && rect.height == height
        && try_present_exact_integer_scale_into(source, target, width, height)
    {
        return;
    }
    *target = present_frame(source, width, height, fill);
}

pub(super) fn present_frame_region_for_window_into(
    source: &Canvas,
    target: &mut Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> bool {
    if width == 0 || height == 0 {
        return true;
    }
    if source.width() == 0 || source.height() == 0 || target.width() == 0 || target.height() == 0 {
        return false;
    }
    let rect = PresentationRect::fit(
        source.width(),
        source.height(),
        target.width(),
        target.height(),
    );
    rect.x == 0
        && rect.y == 0
        && rect.width == target.width()
        && rect.height == target.height()
        && try_present_exact_integer_scale_region_into(source, target, x, y, width, height)
}

#[must_use]
fn present_frame(source: &Canvas, width: usize, height: usize, fill: u32) -> Canvas {
    if width == 0 || height == 0 {
        return Canvas::new(1, 1, fill);
    }
    if source.width() == width && source.height() == height {
        return source.clone();
    }
    if source.width() == 0 || source.height() == 0 {
        return Canvas::new(width, height, fill);
    }
    let rect = PresentationRect::fit(source.width(), source.height(), width, height);
    if rect.x == 0
        && rect.y == 0
        && rect.width == width
        && rect.height == height
        && let Some(presented) = try_present_exact_integer_scale(source, width, height)
    {
        return presented;
    }
    let mut target = Canvas::new(width, height, fill);
    for y in rect.y..rect.bottom().min(height) {
        for x in rect.x..rect.right().min(width) {
            let sample = SourceSample::from_target(x, y, source, rect);
            target.set_physical(x, y, sample_bilinear(source, sample));
        }
    }
    target
}

#[derive(Clone, Copy)]
struct PresentationRect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl PresentationRect {
    fn fit(
        source_width: usize,
        source_height: usize,
        target_width: usize,
        target_height: usize,
    ) -> Self {
        let source_ratio = source_width as f32 / source_height as f32;
        let target_ratio = target_width as f32 / target_height as f32;
        if source_ratio > target_ratio {
            let height = (target_width as f32 / source_ratio).round() as usize;
            return Self {
                x: 0,
                y: (target_height.saturating_sub(height)) / 2,
                width: target_width,
                height,
            };
        }
        let width = (target_height as f32 * source_ratio).round() as usize;
        Self {
            x: (target_width.saturating_sub(width)) / 2,
            y: 0,
            width,
            height: target_height,
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
struct SourceSample {
    x: f32,
    y: f32,
}

impl SourceSample {
    fn from_target(x: usize, y: usize, source: &Canvas, rect: PresentationRect) -> Self {
        let local_x = x.saturating_sub(rect.x) as f32 + SAMPLE_CENTER_OFFSET;
        let local_y = y.saturating_sub(rect.y) as f32 + SAMPLE_CENTER_OFFSET;
        let source_x = local_x * source.width() as f32 / rect.width as f32 - SAMPLE_CENTER_OFFSET;
        let source_y = local_y * source.height() as f32 / rect.height as f32 - SAMPLE_CENTER_OFFSET;
        Self {
            x: source_x.clamp(0.0, (source.width() - 1) as f32),
            y: source_y.clamp(0.0, (source.height() - 1) as f32),
        }
    }
}

fn sample_bilinear(source: &Canvas, sample: SourceSample) -> u32 {
    let left = sample.x.floor() as usize;
    let top = sample.y.floor() as usize;
    let right = (left + 1).min(source.width() - 1);
    let bottom = (top + 1).min(source.height() - 1);
    let tx = sample.x - left as f32;
    let ty = sample.y - top as f32;
    let top_color = mix_float(pixel(source, left, top), pixel(source, right, top), tx);
    let bottom_color = mix_float(
        pixel(source, left, bottom),
        pixel(source, right, bottom),
        tx,
    );
    mix_float(top_color, bottom_color, ty)
}

fn pixel(source: &Canvas, x: usize, y: usize) -> u32 {
    source.pixels()[y * source.width() + x]
}

fn mix_float(left: u32, right: u32, ratio: f32) -> u32 {
    let red = mix_channel(left, right, ratio, RED_SHIFT);
    let green = mix_channel(left, right, ratio, GREEN_SHIFT);
    let blue = mix_channel(left, right, ratio, BLUE_SHIFT);
    (red << RED_SHIFT) | (green << GREEN_SHIFT) | blue
}

fn mix_channel(left: u32, right: u32, ratio: f32, shift: u32) -> u32 {
    let left_channel = ((left >> shift) & CHANNEL_MASK) as f32;
    let right_channel = ((right >> shift) & CHANNEL_MASK) as f32;
    (left_channel + (right_channel - left_channel) * ratio).round() as u32
}

#[cfg(test)]
mod tests {
    use super::{present_frame, present_frame_for_window};
    use crate::visual::canvas::Canvas;

    const BACKGROUND: u32 = 0x111111;
    const BLACK: u32 = 0x000000;
    const WHITE: u32 = 0xffffff;
    const DARK_MID: u32 = 0x404040;
    const LIGHT_MID: u32 = 0xbfbfbf;

    #[test]
    fn presented_frame_matches_window_size() {
        let source = Canvas::new(4, 2, WHITE);
        let presented = present_frame(&source, 12, 8, BACKGROUND);

        assert_eq!(12, presented.width());
        assert_eq!(8, presented.height());
    }

    #[test]
    fn presented_frame_for_window_matches_window_size_when_hidpi_logical_size_matches() {
        let source = Canvas::new_scaled(1440, 920, 2.0, BACKGROUND);

        let presented = present_frame_for_window(&source, 1440, 920, BACKGROUND);

        assert_eq!(1440, presented.width());
        assert_eq!(920, presented.height());
        assert_eq!(1440, presented.logical_width());
        assert_eq!(920, presented.logical_height());
        assert_eq!(1.0, presented.scale_factor());
    }

    #[test]
    fn presented_frame_for_window_fits_when_logical_size_differs() {
        let source = Canvas::new_scaled(1440, 920, 2.0, BACKGROUND);

        let presented = present_frame_for_window(&source, 1280, 900, BACKGROUND);

        assert_eq!(1280, presented.width());
        assert_eq!(900, presented.height());
        assert_eq!(1280, presented.logical_width());
        assert_eq!(900, presented.logical_height());
    }

    #[test]
    fn presented_frame_uses_interpolation_when_scale_is_fractional() {
        let mut source = Canvas::new(2, 1, BLACK);
        source.set(1, 0, WHITE);

        let presented = present_frame(&source, 5, 2, BACKGROUND);
        let interpolation_x = 1;
        let second_interpolation_x = 2;
        let interpolated_y = 1;
        let dark_index = presented.width() * interpolated_y + interpolation_x;
        let light_index = presented.width() * interpolated_y + second_interpolation_x;

        assert_eq!(DARK_MID, presented.pixels()[dark_index]);
        assert_eq!(LIGHT_MID, presented.pixels()[light_index]);
    }

    #[test]
    fn presented_frame_uses_nearest_neighbor_when_integer_upscale_matches() {
        let mut source = Canvas::new(2, 1, BLACK);
        source.set(1, 0, WHITE);

        let presented = present_frame(&source, 4, 2, BACKGROUND);

        assert_eq!(BLACK, presented.pixels()[0]);
        assert_eq!(BLACK, presented.pixels()[1]);
        assert_eq!(WHITE, presented.pixels()[2]);
        assert_eq!(WHITE, presented.pixels()[3]);
    }

    #[test]
    fn presented_frame_downsamples_integer_scale_with_interpolation() {
        let mut source = Canvas::new(2, 1, BLACK);
        source.set(1, 0, WHITE);

        let presented = present_frame(&source, 1, 1, BACKGROUND);

        assert_eq!(0x808080, presented.pixels()[0]);
    }

    #[test]
    fn presented_frame_downsamples_integer_scale_without_dropping_subpixels() {
        let mut source = Canvas::new(2, 2, BLACK);
        source.set(1, 0, WHITE);
        source.set(0, 1, WHITE);
        source.set(1, 1, WHITE);

        let presented = present_frame(&source, 1, 1, BACKGROUND);

        assert_eq!(0xbfbfbf, presented.pixels()[0]);
    }
}
