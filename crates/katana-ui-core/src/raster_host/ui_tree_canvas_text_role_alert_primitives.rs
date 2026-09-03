use super::super::{
    CIRCLE_ARC_CLEAR_EXTRA_HEIGHT, CIRCLE_ARC_CLEAR_EXTRA_WIDTH, CIRCLE_ARC_CLEAR_Y_OFFSET, Canvas,
};

pub(super) fn draw_stroked_circle(
    canvas: &mut Canvas,
    center_x: usize,
    center_y: usize,
    radius: usize,
    color: u32,
) {
    let center_x = center_x as isize;
    let center_y = center_y as isize;
    let radius = radius as isize;
    let inner = radius.saturating_sub(2);
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let distance_squared = dx * dx + dy * dy;
            if distance_squared < inner * inner || distance_squared > radius * radius {
                continue;
            }
            let point_x = center_x + dx;
            let point_y = center_y + dy;
            if point_x < 0 || point_y < 0 {
                continue;
            }
            canvas.set(point_x as usize, point_y as usize, color);
        }
    }
}

pub(super) fn draw_stroked_circle_arc(
    canvas: &mut Canvas,
    center_x: usize,
    center_y: usize,
    radius: usize,
    color: u32,
    background: u32,
) {
    draw_stroked_circle(canvas, center_x, center_y, radius, color);
    let clear_x = center_x.saturating_sub(radius.saturating_add(1));
    let clear_y = center_y.saturating_add(CIRCLE_ARC_CLEAR_Y_OFFSET);
    canvas.fill_rect(
        clear_x,
        clear_y,
        radius
            .saturating_mul(CIRCLE_ARC_CLEAR_EXTRA_WIDTH)
            .saturating_add(CIRCLE_ARC_CLEAR_EXTRA_WIDTH),
        radius.saturating_add(CIRCLE_ARC_CLEAR_EXTRA_HEIGHT),
        background,
    );
}

pub(super) fn draw_outline(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    color: u32,
    points: &[(usize, usize)],
) {
    for pair in points.windows(2) {
        draw_stroked_line(
            canvas,
            x.saturating_add(pair[0].0),
            y.saturating_add(pair[0].1),
            x.saturating_add(pair[1].0),
            y.saturating_add(pair[1].1),
            color,
        );
    }
}

pub(super) fn draw_filled_circle(
    canvas: &mut Canvas,
    center_x: usize,
    center_y: usize,
    radius: usize,
    color: u32,
) {
    let center_x = center_x as isize;
    let center_y = center_y as isize;
    let radius = radius as isize;
    let radius_squared = radius * radius;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy > radius_squared {
                continue;
            }
            let point_x = center_x + dx;
            let point_y = center_y + dy;
            if point_x < 0 || point_y < 0 {
                continue;
            }
            canvas.set(point_x as usize, point_y as usize, color);
        }
    }
}

pub(super) fn draw_stroked_line(
    canvas: &mut Canvas,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: u32,
) {
    draw_line_with_point(canvas, x0, y0, x1, y1, color, draw_stroked_point);
}

pub(super) fn draw_line_with_point(
    canvas: &mut Canvas,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: u32,
    draw_point: fn(&mut Canvas, isize, isize, u32),
) {
    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        if x0 >= 0 && y0 >= 0 {
            draw_point(canvas, x0, y0, color);
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let doubled_error = error.saturating_mul(2);
        if doubled_error >= dy {
            error += dy;
            x0 += sx;
        }
        if doubled_error <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

pub(super) fn draw_stroked_point(canvas: &mut Canvas, x: isize, y: isize, color: u32) {
    for offset_y in -1..=1 {
        for offset_x in -1..=1 {
            let point_x = x + offset_x;
            let point_y = y + offset_y;
            if point_x < 0 || point_y < 0 {
                continue;
            }
            canvas.set(point_x as usize, point_y as usize, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_and_stroke_primitives_clip_negative_coordinates() {
        let mut canvas = Canvas::new(8, 8, 0);
        draw_stroked_circle(&mut canvas, 0, 0, 3, 1);
        draw_filled_circle(&mut canvas, 0, 0, 3, 2);
        draw_stroked_point(&mut canvas, 0, 0, 3);
        assert!(canvas.non_background_pixels(0) > 0);
    }

    #[test]
    fn outline_arc_and_diagonal_strokes_cover_positive_canvas_geometry() {
        let mut canvas = Canvas::new(32, 32, 0);
        draw_stroked_circle_arc(&mut canvas, 10, 10, 6, 1, 0);
        draw_outline(&mut canvas, 4, 4, 2, &[(0, 0), (12, 0), (6, 12), (0, 0)]);
        draw_stroked_line(&mut canvas, 24, 4, 12, 28, 3);
        draw_line_with_point(&mut canvas, 4, 28, 28, 16, 4, draw_stroked_point);

        for color in 1..=4 {
            assert!(canvas.pixels().contains(&color));
        }
    }
}
