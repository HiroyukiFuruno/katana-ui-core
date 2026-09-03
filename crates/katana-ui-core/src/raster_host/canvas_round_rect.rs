use super::canvas::Canvas;

const EDGE_SAMPLE_COUNT: usize = 4;
const EDGE_SAMPLE_TOTAL: usize = EDGE_SAMPLE_COUNT * EDGE_SAMPLE_COUNT;
const OPAQUE_ALPHA: u8 = 255;
const SAMPLE_CENTER_OFFSET: f32 = 0.5;

pub(super) fn fill_physical(
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
    for current_y in rect.y..rect.bottom().min(canvas.height()) {
        for current_x in rect.x..rect.right().min(canvas.width()) {
            let alpha = round_rect_alpha(current_x, current_y, rect);
            if alpha == 0 {
                continue;
            }
            canvas.blend_physical(current_x, current_y, color, alpha);
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

#[cfg(test)]
mod tests {
    use super::Canvas;
    use std::collections::HashSet;

    const BACKGROUND: u32 = 0x101010;
    const SURFACE: u32 = 0xffffff;
    const RECT_X: usize = 4;
    const RECT_Y: usize = 4;
    const RECT_WIDTH: usize = 24;
    const RECT_HEIGHT: usize = 14;
    const RECT_RADIUS: usize = 7;
    const MIN_SMOOTH_COLOR_COUNT: usize = 3;

    #[test]
    fn round_rect_edges_are_antialiased() {
        let mut canvas = Canvas::new(40, 28, BACKGROUND);

        canvas.fill_round_rect(
            RECT_X,
            RECT_Y,
            RECT_WIDTH,
            RECT_HEIGHT,
            RECT_RADIUS,
            SURFACE,
        );

        assert!(round_rect_color_count(&canvas) >= MIN_SMOOTH_COLOR_COUNT);
    }

    #[test]
    fn fill_round_rect_scales_logical_rect_to_physical_canvas() {
        let mut canvas = Canvas::new_scaled(12, 12, 2.0, BACKGROUND);

        canvas.fill_round_rect(2, 3, 3, 2, 1, SURFACE);

        assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 3, 6));
        assert_eq!(Some(SURFACE), pixel_at(&canvas, 7, 7));
        assert_eq!(Some(BACKGROUND), pixel_at(&canvas, 10, 7));
    }

    #[test]
    fn zero_sized_round_rect_is_a_noop() {
        let mut canvas = Canvas::new(8, 8, BACKGROUND);

        canvas.fill_round_rect(2, 2, 0, 3, 1, SURFACE);
        canvas.fill_round_rect(2, 2, 3, 0, 1, SURFACE);

        assert!(canvas.pixels().iter().all(|pixel| *pixel == BACKGROUND));
    }

    fn round_rect_color_count(canvas: &Canvas) -> usize {
        let mut colors = HashSet::new();
        for y in RECT_Y..RECT_Y + RECT_HEIGHT {
            for x in RECT_X..RECT_X + RECT_WIDTH {
                colors.insert(canvas.pixels()[y * canvas.width() + x]);
            }
        }
        colors.len()
    }

    fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
        canvas.pixels().get(y * canvas.width() + x).copied()
    }
}
