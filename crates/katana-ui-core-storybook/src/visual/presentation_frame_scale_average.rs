use super::canvas::Canvas;

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const BLUE_SHIFT: u32 = 0;
const CHANNEL_MASK: u32 = 0xff;
const RED_BLUE_MASK: u32 = 0x00ff00ff;
const GREEN_MASK: u32 = 0x0000ff00;
const RED_BLUE_AVERAGE_BIAS_2X: u32 = 0x00020002;
const GREEN_AVERAGE_BIAS_2X: u32 = 0x00000200;

pub(super) fn scale_down_average(
    source: &Canvas,
    width: usize,
    height: usize,
    scale: usize,
) -> Canvas {
    if scale == 2 {
        return scale_down_average_2x(source, width, height);
    }
    scale_down_average_generic(source, width, height, scale)
}

pub(super) fn scale_down_average_into(
    source: &Canvas,
    target: &mut Canvas,
    width: usize,
    height: usize,
    scale: usize,
) {
    if target.width != width || target.height != height || target.scale_factor != 1.0 {
        *target = Canvas::new(width, height, source.pixels()[0]);
    }
    if scale == 2 {
        scale_down_average_2x_into(source, target, width, height);
        return;
    }
    *target = scale_down_average_generic(source, width, height, scale);
}

pub(super) fn scale_down_average_2x_region_into(
    source: &Canvas,
    target: &mut Canvas,
    x: usize,
    y: usize,
    x_end: usize,
    y_end: usize,
) {
    let source_width = source.width();
    let source_pixels = source.pixels();
    let target_width = target.width;
    let target_pixels = target.pixels.as_mut_slice();
    for target_y in y..y_end {
        let top_start = target_y * 2 * source_width;
        let bottom_start = top_start + source_width;
        let target_start = target_y * target_width;
        for target_x in x..x_end {
            let source_x = target_x * 2;
            target_pixels[target_start + target_x] = average_four_pixels(
                source_pixels[top_start + source_x],
                source_pixels[top_start + source_x + 1],
                source_pixels[bottom_start + source_x],
                source_pixels[bottom_start + source_x + 1],
            );
        }
    }
}

pub(super) fn scale_down_average_region_into(
    source: &Canvas,
    target: &mut Canvas,
    x: usize,
    y: usize,
    x_end: usize,
    y_end: usize,
    scale: usize,
) {
    let target_width = target.width;
    let target_pixels = target.pixels.as_mut_slice();
    for target_y in y..y_end {
        let source_y = target_y * scale;
        let target_start = target_y * target_width;
        for target_x in x..x_end {
            let source_x = target_x * scale;
            target_pixels[target_start + target_x] =
                average_source_block(source, source_x, source_y, scale);
        }
    }
}

fn scale_down_average_2x(source: &Canvas, width: usize, height: usize) -> Canvas {
    let mut pixels = vec![0; width * height];
    let source_width = source.width();
    let source_pixels = source.pixels();
    for target_y in 0..height {
        let top_start = target_y * 2 * source_width;
        let bottom_start = top_start + source_width;
        let target_start = target_y * width;
        for target_x in 0..width {
            let source_x = target_x * 2;
            pixels[target_start + target_x] = average_four_pixels(
                source_pixels[top_start + source_x],
                source_pixels[top_start + source_x + 1],
                source_pixels[bottom_start + source_x],
                source_pixels[bottom_start + source_x + 1],
            );
        }
    }
    canvas_from_pixels(width, height, pixels)
}

fn scale_down_average_2x_into(source: &Canvas, target: &mut Canvas, width: usize, height: usize) {
    let source_width = source.width();
    let source_pixels = source.pixels();
    let target_pixels = target.pixels.as_mut_slice();
    for target_y in 0..height {
        let top_start = target_y * 2 * source_width;
        let bottom_start = top_start + source_width;
        let target_start = target_y * width;
        for target_x in 0..width {
            let source_x = target_x * 2;
            target_pixels[target_start + target_x] = average_four_pixels(
                source_pixels[top_start + source_x],
                source_pixels[top_start + source_x + 1],
                source_pixels[bottom_start + source_x],
                source_pixels[bottom_start + source_x + 1],
            );
        }
    }
}

fn scale_down_average_generic(
    source: &Canvas,
    width: usize,
    height: usize,
    scale: usize,
) -> Canvas {
    let mut pixels = Vec::with_capacity(width * height);
    for target_y in 0..height {
        let source_y = target_y * scale;
        for target_x in 0..width {
            let source_x = target_x * scale;
            pixels.push(average_source_block(source, source_x, source_y, scale));
        }
    }
    canvas_from_pixels(width, height, pixels)
}

fn canvas_from_pixels(width: usize, height: usize, pixels: Vec<u32>) -> Canvas {
    Canvas {
        width,
        height,
        logical_width: width,
        logical_height: height,
        scale_factor: 1.0,
        raster_scale_factor: 1.0,
        image_surface_extent_mode:
            super::canvas_model::CanvasImageSurfaceExtentMode::LogicalDisplay,
        pixels,
        clip: None,
        text_runs: Vec::new(),
    }
}

fn average_four_pixels(first: u32, second: u32, third: u32, fourth: u32) -> u32 {
    let red_blue = (((first & RED_BLUE_MASK)
        + (second & RED_BLUE_MASK)
        + (third & RED_BLUE_MASK)
        + (fourth & RED_BLUE_MASK)
        + RED_BLUE_AVERAGE_BIAS_2X)
        >> 2)
        & RED_BLUE_MASK;
    let green = (((first & GREEN_MASK)
        + (second & GREEN_MASK)
        + (third & GREEN_MASK)
        + (fourth & GREEN_MASK)
        + GREEN_AVERAGE_BIAS_2X)
        >> 2)
        & GREEN_MASK;
    red_blue | green
}

fn average_source_block(source: &Canvas, x: usize, y: usize, scale: usize) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for offset_y in 0..scale {
        let row_start = (y + offset_y) * source.width();
        for offset_x in 0..scale {
            let color = source.pixels()[row_start + x + offset_x];
            red += (color >> RED_SHIFT) & CHANNEL_MASK;
            green += (color >> GREEN_SHIFT) & CHANNEL_MASK;
            blue += (color >> BLUE_SHIFT) & CHANNEL_MASK;
        }
    }
    let count = (scale * scale) as u32;
    let half = count / 2;
    (((red + half) / count) << RED_SHIFT)
        | (((green + half) / count) << GREEN_SHIFT)
        | (((blue + half) / count) << BLUE_SHIFT)
}
