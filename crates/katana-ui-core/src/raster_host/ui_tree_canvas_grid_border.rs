use super::canvas::Canvas;
use katana_ui_core::render_model::{
    UiGridBorderLineStyle, UiGridBorderSide, UiGridCellBorders, UiRect,
};

const DOUBLE_BORDER_INSET_PX: usize = 2;
const DOTTED_PERIOD_PX: usize = 2;
const DASH_PERIOD_PX: usize = 8;
const DASH_VISIBLE_LENGTH_PX: usize = 5;
const DASH_DOT_PERIOD_PX: usize = 12;
const DASH_DOT_POINT_OFFSET_PX: usize = 7;
const DASH_DOT_DOT_PERIOD_PX: usize = 16;
const DASH_DOT_DOT_FIRST_POINT_OFFSET_PX: usize = 7;
const DASH_DOT_DOT_SECOND_POINT_OFFSET_PX: usize = 10;
const SHORT_HEX_COLOR_LENGTH: usize = 3;
const FULL_HEX_COLOR_LENGTH: usize = 6;
const HEXADECIMAL_RADIX: u32 = 16;

pub(super) struct UiTreeGridBorderRenderer;

impl UiTreeGridBorderRenderer {
    pub(super) fn draw_cell_borders(
        canvas: &mut Canvas,
        borders: &UiGridCellBorders,
        rect: UiRect,
        origin_x: usize,
        origin_y: usize,
        fallback_color: u32,
    ) {
        let bounds = positioned_signed_rect(rect, origin_x, origin_y);
        Self::draw_border_side(
            canvas,
            &borders.left,
            GridEdge::Left,
            bounds,
            fallback_color,
        );
        Self::draw_border_side(
            canvas,
            &borders.right,
            GridEdge::Right,
            bounds,
            fallback_color,
        );
        Self::draw_border_side(canvas, &borders.top, GridEdge::Top, bounds, fallback_color);
        Self::draw_border_side(
            canvas,
            &borders.bottom,
            GridEdge::Bottom,
            bounds,
            fallback_color,
        );
    }

    fn draw_border_side(
        canvas: &mut Canvas,
        side: &UiGridBorderSide,
        edge: GridEdge,
        bounds: SignedRect,
        fallback_color: u32,
    ) {
        if !side.is_visible() || bounds.width == 0 || bounds.height == 0 {
            return;
        }
        let color = parse_color(side.color.as_deref()).unwrap_or(fallback_color);
        if side.line_style == UiGridBorderLineStyle::Double {
            Self::draw_edge_line(canvas, edge, bounds, 0, side.line_style, color);
            Self::draw_edge_line(
                canvas,
                edge,
                bounds,
                DOUBLE_BORDER_INSET_PX,
                side.line_style,
                color,
            );
            return;
        }
        for inset in 0..side.line_style.stroke_width_px() {
            Self::draw_edge_line(canvas, edge, bounds, inset, side.line_style, color);
        }
    }

    fn draw_edge_line(
        canvas: &mut Canvas,
        edge: GridEdge,
        bounds: SignedRect,
        inset: usize,
        style: UiGridBorderLineStyle,
        color: u32,
    ) {
        let inset = i64::try_from(inset).unwrap_or(i64::MAX);
        match edge {
            GridEdge::Top => draw_horizontal(
                canvas,
                bounds.x,
                bounds.y.saturating_add(inset),
                bounds.width,
                style,
                color,
            ),
            GridEdge::Bottom => draw_horizontal(
                canvas,
                bounds.x,
                bounds
                    .y
                    .saturating_add(i64::try_from(bounds.height).unwrap_or(i64::MAX))
                    .saturating_sub(1)
                    .saturating_sub(inset),
                bounds.width,
                style,
                color,
            ),
            GridEdge::Left => draw_vertical(
                canvas,
                bounds.x.saturating_add(inset),
                bounds.y,
                bounds.height,
                style,
                color,
            ),
            GridEdge::Right => draw_vertical(
                canvas,
                bounds
                    .x
                    .saturating_add(i64::try_from(bounds.width).unwrap_or(i64::MAX))
                    .saturating_sub(1)
                    .saturating_sub(inset),
                bounds.y,
                bounds.height,
                style,
                color,
            ),
        }
    }
}

#[derive(Clone, Copy)]
enum GridEdge {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy)]
struct SignedRect {
    x: i64,
    y: i64,
    width: usize,
    height: usize,
}

fn positioned_signed_rect(rect: UiRect, origin_x: usize, origin_y: usize) -> SignedRect {
    SignedRect {
        x: i64::try_from(origin_x)
            .unwrap_or(i64::MAX)
            .saturating_add(i64::from(rect.x)),
        y: i64::try_from(origin_y)
            .unwrap_or(i64::MAX)
            .saturating_add(i64::from(rect.y)),
        width: usize::try_from(rect.width).unwrap_or(usize::MAX),
        height: usize::try_from(rect.height).unwrap_or(usize::MAX),
    }
}

fn draw_horizontal(
    canvas: &mut Canvas,
    x: i64,
    y: i64,
    width: usize,
    style: UiGridBorderLineStyle,
    color: u32,
) {
    for offset in 0..width {
        if line_pixel_visible(style, offset) {
            set_signed(
                canvas,
                x.saturating_add(i64::try_from(offset).unwrap_or(i64::MAX)),
                y,
                color,
            );
        }
    }
}

fn draw_vertical(
    canvas: &mut Canvas,
    x: i64,
    y: i64,
    height: usize,
    style: UiGridBorderLineStyle,
    color: u32,
) {
    for offset in 0..height {
        if line_pixel_visible(style, offset) {
            set_signed(
                canvas,
                x,
                y.saturating_add(i64::try_from(offset).unwrap_or(i64::MAX)),
                color,
            );
        }
    }
}

fn set_signed(canvas: &mut Canvas, x: i64, y: i64, color: u32) {
    let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) else {
        return;
    };
    canvas.set(x, y, color);
}

fn line_pixel_visible(style: UiGridBorderLineStyle, offset: usize) -> bool {
    match style {
        UiGridBorderLineStyle::None => false,
        UiGridBorderLineStyle::Dotted => offset.is_multiple_of(DOTTED_PERIOD_PX),
        UiGridBorderLineStyle::Dashed | UiGridBorderLineStyle::MediumDashed => {
            offset % DASH_PERIOD_PX < DASH_VISIBLE_LENGTH_PX
        }
        UiGridBorderLineStyle::DashDot
        | UiGridBorderLineStyle::MediumDashDot
        | UiGridBorderLineStyle::SlantDashDot => {
            offset % DASH_DOT_PERIOD_PX < DASH_VISIBLE_LENGTH_PX
                || offset % DASH_DOT_PERIOD_PX == DASH_DOT_POINT_OFFSET_PX
        }
        UiGridBorderLineStyle::DashDotDot | UiGridBorderLineStyle::MediumDashDotDot => {
            offset % DASH_DOT_DOT_PERIOD_PX < DASH_VISIBLE_LENGTH_PX
                || offset % DASH_DOT_DOT_PERIOD_PX == DASH_DOT_DOT_FIRST_POINT_OFFSET_PX
                || offset % DASH_DOT_DOT_PERIOD_PX == DASH_DOT_DOT_SECOND_POINT_OFFSET_PX
        }
        UiGridBorderLineStyle::Hair
        | UiGridBorderLineStyle::Thin
        | UiGridBorderLineStyle::Medium
        | UiGridBorderLineStyle::Thick
        | UiGridBorderLineStyle::Double
        | UiGridBorderLineStyle::Solid => true,
    }
}

pub(super) fn parse_color(value: Option<&str>) -> Option<u32> {
    let value = value?.trim().trim_start_matches('#');
    match value.len() {
        SHORT_HEX_COLOR_LENGTH => {
            let mut hex = String::with_capacity(FULL_HEX_COLOR_LENGTH);
            for channel in value.chars() {
                hex.push(channel);
                hex.push(channel);
            }
            u32::from_str_radix(&hex, HEXADECIMAL_RADIX).ok()
        }
        FULL_HEX_COLOR_LENGTH => u32::from_str_radix(value, HEXADECIMAL_RADIX).ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{line_pixel_visible, parse_color};
    use crate::render_model::UiGridBorderLineStyle;

    const EXPECTED_COLOR: u32 = 0xAABBCC;
    const DOTTED_HIDDEN_OFFSET: usize = 1;
    const DASHED_HIDDEN_OFFSET: usize = 7;

    #[test]
    fn grid_border_patterns_and_colors_cover_supported_input_forms() {
        let styles = [
            UiGridBorderLineStyle::Hair,
            UiGridBorderLineStyle::Thin,
            UiGridBorderLineStyle::Medium,
            UiGridBorderLineStyle::Thick,
            UiGridBorderLineStyle::Double,
            UiGridBorderLineStyle::Dotted,
            UiGridBorderLineStyle::Dashed,
            UiGridBorderLineStyle::DashDot,
            UiGridBorderLineStyle::DashDotDot,
            UiGridBorderLineStyle::MediumDashed,
            UiGridBorderLineStyle::MediumDashDot,
            UiGridBorderLineStyle::MediumDashDotDot,
            UiGridBorderLineStyle::SlantDashDot,
            UiGridBorderLineStyle::Solid,
        ];

        assert!(!line_pixel_visible(UiGridBorderLineStyle::None, 0));
        assert_eq!(Some(EXPECTED_COLOR), parse_color(Some("#ABC")));
        assert_eq!(Some(EXPECTED_COLOR), parse_color(Some("#AABBCC")));
        assert_eq!(None, parse_color(Some("invalid")));
        assert!(styles.iter().all(|style| line_pixel_visible(*style, 0)));
        assert!(!line_pixel_visible(
            UiGridBorderLineStyle::Dotted,
            DOTTED_HIDDEN_OFFSET
        ));
        assert!(!line_pixel_visible(
            UiGridBorderLineStyle::Dashed,
            DASHED_HIDDEN_OFFSET
        ));
    }
}
