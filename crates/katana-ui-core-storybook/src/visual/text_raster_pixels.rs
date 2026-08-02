use crate::visual::canvas::Canvas;
use crate::visual::text_raster_color::{
    OPAQUE_TEXT_ALPHA, RGB_MASK, blue, green, pack_rgb, packed_color, red, text_color,
};
use cosmic_text::{Buffer, SwashCache};

const TEXT_SUPERSAMPLE_SCALE: f32 = 2.0;
const TEXT_SUPERSAMPLE_SAMPLES: u32 = 4;
const VERTICAL_SCALE_ROW_GAIN: f32 = 6.0;

pub(super) struct CachedTextRaster {
    pixels: Vec<CachedTextPixel>,
}

impl CachedTextRaster {
    pub(super) const fn new(pixels: Vec<CachedTextPixel>) -> Self {
        Self { pixels }
    }

    pub(super) fn draw(
        &self,
        canvas: &mut Canvas,
        origin_x: i32,
        origin_y: i32,
        color: u32,
        vertical_scale: f32,
    ) {
        let vertical_scale = normalized_vertical_scale(vertical_scale);
        for pixel in &self.pixels {
            let x = origin_x + pixel.x;
            if x < 0 {
                continue;
            }
            let extra_rows = extra_vertical_coverage_rows(vertical_scale);
            for y in pixel.y..=pixel.y.saturating_add(extra_rows) {
                let y = origin_y + y;
                if y < 0 {
                    continue;
                }
                canvas.blend_physical(
                    x as usize,
                    y as usize,
                    pixel.color_override.unwrap_or(color),
                    pixel.alpha,
                );
            }
        }
    }

    pub(super) fn width(&self) -> usize {
        self.pixels
            .iter()
            .filter_map(|pixel| usize::try_from(pixel.x + 1).ok())
            .max()
            .unwrap_or(0)
    }
}

fn normalized_vertical_scale(scale: f32) -> f32 {
    if scale.is_finite() && scale > 1.0 {
        scale
    } else {
        1.0
    }
}

fn extra_vertical_coverage_rows(scale: f32) -> i32 {
    ((scale - 1.0) * VERTICAL_SCALE_ROW_GAIN).ceil().max(0.0) as i32
}

pub(super) struct CachedTextPixel {
    x: i32,
    y: i32,
    alpha: u8,
    color_override: Option<u32>,
}

#[derive(Clone, Copy)]
struct SuperSample {
    x: i32,
    y: i32,
    alpha: u8,
    color_override: Option<u32>,
}

pub(super) fn raster_pixels(
    buffer: &mut cosmic_text::BorrowedWithFontSystem<'_, Buffer>,
    swash_cache: &mut SwashCache,
    color: u32,
) -> Vec<CachedTextPixel> {
    let mut samples = Vec::new();
    let requested_color = color & RGB_MASK;
    buffer.draw(
        swash_cache,
        text_color(color),
        |left, top, width, height, color| {
            if color.a() == 0 {
                return;
            }
            let sample_color = packed_color(color);
            push_glyph_samples(
                &mut samples,
                left,
                top,
                width,
                height,
                color.a(),
                (sample_color != requested_color).then_some(sample_color),
            );
        },
    );
    samples.sort_unstable_by_key(|sample| (sample.y, sample.x));
    combine_samples(&samples)
}

fn push_glyph_samples(
    samples: &mut Vec<SuperSample>,
    left: i32,
    top: i32,
    width: u32,
    height: u32,
    alpha: u8,
    color_override: Option<u32>,
) {
    for dy in 0..height {
        for dx in 0..width {
            samples.push(SuperSample {
                x: logical_sample_position(left + dx as i32),
                y: logical_sample_position(top + dy as i32),
                alpha,
                color_override,
            });
        }
    }
}

fn logical_sample_position(value: i32) -> i32 {
    (value as f32 / TEXT_SUPERSAMPLE_SCALE).floor() as i32
}

fn combine_samples(samples: &[SuperSample]) -> Vec<CachedTextPixel> {
    let mut pixels = Vec::with_capacity(samples.len());
    let mut index = 0;
    while index < samples.len() {
        let current = samples[index];
        let group_start = index;
        let mut alpha_sum = 0u32;
        while index < samples.len()
            && samples[index].x == current.x
            && samples[index].y == current.y
        {
            alpha_sum += u32::from(samples[index].alpha);
            index += 1;
        }
        let alpha = ((alpha_sum as f32 / TEXT_SUPERSAMPLE_SAMPLES as f32).round() as u32)
            .min(u32::from(OPAQUE_TEXT_ALPHA));
        if alpha != 0 {
            pixels.push(CachedTextPixel {
                x: current.x,
                y: current.y,
                alpha: alpha as u8,
                color_override: combined_color_override(&samples[group_start..index]),
            });
        }
    }
    pixels
}

fn combined_color_override(samples: &[SuperSample]) -> Option<u32> {
    if !samples.iter().any(|sample| sample.color_override.is_some()) {
        return None;
    }
    let mut red_sum = 0u32;
    let mut green_sum = 0u32;
    let mut blue_sum = 0u32;
    let mut alpha_sum = 0u32;
    for sample in samples {
        let Some(color) = sample.color_override else {
            continue;
        };
        let alpha = u32::from(sample.alpha);
        red_sum += u32::from(red(color)) * alpha;
        green_sum += u32::from(green(color)) * alpha;
        blue_sum += u32::from(blue(color)) * alpha;
        alpha_sum += alpha;
    }
    if alpha_sum == 0 {
        return None;
    }
    Some(pack_rgb(
        (red_sum / alpha_sum) as u8,
        (green_sum / alpha_sum) as u8,
        (blue_sum / alpha_sum) as u8,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        CachedTextPixel, CachedTextRaster, SuperSample, combined_color_override, push_glyph_samples,
    };
    use crate::visual::canvas::Canvas;

    #[test]
    fn glyph_callback_rect_expands_all_supersample_pixels() {
        let mut samples = Vec::new();

        push_glyph_samples(&mut samples, 4, 6, 3, 2, 128, Some(0x00ff00));

        assert_eq!(6, samples.len());
        assert_eq!((2, 3), position(samples[0]));
        assert_eq!((3, 3), position(samples[2]));
        assert_eq!((2, 3), position(samples[3]));
        assert!(samples.iter().all(|sample| sample.alpha == 128));
        assert!(
            samples
                .iter()
                .all(|sample| sample.color_override == Some(0x00ff00))
        );
    }

    #[test]
    fn negative_raster_rows_and_zero_alpha_color_samples_are_ignored() {
        let raster = CachedTextRaster::new(vec![CachedTextPixel {
            x: 0,
            y: -1,
            alpha: 255,
            color_override: None,
        }]);
        let mut canvas = Canvas::new(1, 1, 7);
        raster.draw(&mut canvas, 0, 0, 9, 1.0);
        assert_eq!(&[7], canvas.pixels());

        let samples = [SuperSample {
            x: 0,
            y: 0,
            alpha: 0,
            color_override: Some(0x00ff00),
        }];
        assert_eq!(None, combined_color_override(&samples));

        let mixed_samples = [
            SuperSample {
                x: 0,
                y: 0,
                alpha: 255,
                color_override: None,
            },
            SuperSample {
                x: 0,
                y: 0,
                alpha: 255,
                color_override: Some(0x123456),
            },
        ];
        assert_eq!(Some(0x123456), combined_color_override(&mixed_samples));
    }

    fn position(sample: SuperSample) -> (i32, i32) {
        (sample.x, sample.y)
    }
}
