use super::canvas::Canvas;
use super::presentation_frame_scale_average::{
    scale_down_average, scale_down_average_2x_region_into, scale_down_average_into,
    scale_down_average_region_into,
};

pub(super) fn try_present_exact_integer_scale(
    source: &Canvas,
    width: usize,
    height: usize,
) -> Option<Canvas> {
    let scale = exact_integer_scale(source.width(), source.height(), width, height)?;
    if source.width() > width || source.height() > height {
        return Some(scale_down_average(source, width, height, scale));
    }
    Some(scale_nearest(source, width, height))
}

pub(super) fn try_present_exact_integer_scale_into(
    source: &Canvas,
    target: &mut Canvas,
    width: usize,
    height: usize,
) -> bool {
    let Some(scale) = exact_integer_scale(source.width(), source.height(), width, height) else {
        return false;
    };
    if source.width() > width || source.height() > height {
        scale_down_average_into(source, target, width, height, scale);
        return true;
    }
    *target = scale_nearest(source, width, height);
    true
}

pub(super) fn try_present_exact_integer_scale_region_into(
    source: &Canvas,
    target: &mut Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> bool {
    if target.width == 0 || target.height == 0 {
        return false;
    }
    let Some(scale) =
        exact_integer_scale(source.width(), source.height(), target.width, target.height)
    else {
        return false;
    };
    if source.width() < target.width || source.height() < target.height {
        return false;
    }
    let x_end = x.saturating_add(width).min(target.width);
    let y_end = y.saturating_add(height).min(target.height);
    if x >= x_end || y >= y_end {
        return true;
    }
    if scale == 2 {
        scale_down_average_2x_region_into(source, target, x, y, x_end, y_end);
        return true;
    }
    scale_down_average_region_into(source, target, x, y, x_end, y_end, scale);
    true
}

fn exact_integer_scale(
    source_width: usize,
    source_height: usize,
    target_width: usize,
    target_height: usize,
) -> Option<usize> {
    if source_width == 0 || source_height == 0 || target_width == 0 || target_height == 0 {
        return None;
    }
    let width_scale = if source_width >= target_width {
        source_width
            .is_multiple_of(target_width)
            .then_some(source_width / target_width)?
    } else {
        target_width
            .is_multiple_of(source_width)
            .then_some(target_width / source_width)?
    };
    let height_scale = if source_height >= target_height {
        source_height
            .is_multiple_of(target_height)
            .then_some(source_height / target_height)?
    } else {
        target_height
            .is_multiple_of(source_height)
            .then_some(target_height / source_height)?
    };
    (width_scale == height_scale).then_some(width_scale)
}

fn scale_nearest(source: &Canvas, width: usize, height: usize) -> Canvas {
    if source.width() == width && source.height() == height {
        return source.clone();
    }
    let mut target = Canvas::new(width, height, source.pixels()[0]);
    if width >= source.width() {
        let scale_x = width / source.width();
        let scale_y = height / source.height();
        for y in 0..source.height() {
            for x in 0..source.width() {
                let color = source.pixels()[y * source.width() + x];
                for offset_y in 0..scale_y {
                    let target_y = y * scale_y + offset_y;
                    let row_start = target_y * target.width() + x * scale_x;
                    target
                        .pixels_mut_for_presentation()
                        .iter_mut()
                        .skip(row_start)
                        .take(scale_x)
                        .for_each(|it| *it = color);
                }
            }
        }
    }
    target
}

impl Canvas {
    fn pixels_mut_for_presentation(&mut self) -> &mut [u32] {
        self.pixels.as_mut_slice()
    }
}
