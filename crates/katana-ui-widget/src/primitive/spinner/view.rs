use std::fmt::Write as _;

const ARC_SWEEP_DEG: f32 = 270.0;
const STROKE_RATIO: f32 = 0.1;

/// Build an SVG string that shows a spinner arc at the given rotation angle (degrees).
pub(super) fn build_svg(size_px: f32, r: u8, g: u8, b: u8, angle_deg: f32) -> String {
    let cx = size_px / 2.0;
    let radius = cx * (1.0 - STROKE_RATIO * 2.0);
    let stroke_width = size_px * STROKE_RATIO;

    let start_rad = (angle_deg - ARC_SWEEP_DEG / 2.0).to_radians();
    let end_rad = (angle_deg + ARC_SWEEP_DEG / 2.0).to_radians();

    let x1 = cx + radius * start_rad.cos();
    let y1 = cx + radius * start_rad.sin();
    let x2 = cx + radius * end_rad.cos();
    let y2 = cx + radius * end_rad.sin();

    let mut svg = String::new();
    let _ = write!(
        svg,
        "<svg xmlns='http://www.w3.org/2000/svg' width='{size}' height='{size}' viewBox='0 0 {size} {size}'>\
         <path d='M {x1:.2} {y1:.2} A {r:.2} {r:.2} 0 1 1 {x2:.2} {y2:.2}' \
         fill='none' stroke='#{red:02X}{green:02X}{blue:02X}' stroke-width='{sw:.2}' stroke-linecap='round'/>\
         </svg>",
        size = size_px,
        x1 = x1,
        y1 = y1,
        r = radius,
        x2 = x2,
        y2 = y2,
        red = r,
        green = g,
        blue = b,
        sw = stroke_width,
    );
    svg
}
