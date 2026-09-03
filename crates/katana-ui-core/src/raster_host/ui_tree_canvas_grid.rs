use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_grid_border::{UiTreeGridBorderRenderer, parse_color};
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiGridCell, UiNode, UiRect};

const CELL_TEXT_INSET_PX: usize = 3;
const DEFAULT_TEXT_SIZE_PX: f32 = 14.0;

pub(super) struct UiTreeGridRenderer;

impl UiTreeGridRenderer {
    pub(super) fn draw(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let (width, height) = grid_viewport_extent(node, area, x, *y);
        if width == 0 || height == 0 {
            return;
        }
        let grid_y = *y;
        canvas.with_clip(x, grid_y, width, height, &mut |canvas| {
            for cell in &node.props().grid.cells {
                Self::draw_cell(
                    canvas,
                    text,
                    cell,
                    x,
                    grid_y,
                    node.props().grid.show_grid_lines,
                    palette,
                );
            }
        });
        *y = y.saturating_add(height);
    }

    fn draw_cell(
        canvas: &mut Canvas,
        text: &TextRenderer,
        cell: &UiGridCell,
        origin_x: usize,
        origin_y: usize,
        show_grid_lines: bool,
        palette: UiTreeCanvasPalette,
    ) {
        let Some(clipped) = positioned_rect(cell.clipped_bounds, origin_x, origin_y) else {
            return;
        };
        if clipped.width == 0 || clipped.height == 0 {
            return;
        }
        if let Some(fill) = parse_color(cell.appearance.fill_color.as_deref()) {
            canvas.fill_rect(clipped.x, clipped.y, clipped.width, clipped.height, fill);
        } else if cell.selected || cell.active {
            canvas.fill_rect(
                clipped.x,
                clipped.y,
                clipped.width,
                clipped.height,
                palette.selection,
            );
        }
        if show_grid_lines {
            canvas.stroke_rect(
                clipped.x,
                clipped.y,
                clipped.width,
                clipped.height,
                palette.muted_border,
            );
        }
        if !cell.text.trim().is_empty() {
            let text_size = if cell.appearance.font_size_px == 0 {
                DEFAULT_TEXT_SIZE_PX
            } else {
                f32::from(cell.appearance.font_size_px)
            };
            let text_color =
                parse_color(cell.appearance.text_color.as_deref()).unwrap_or(palette.text);
            text.draw(
                canvas,
                &cell.text,
                clipped.x.saturating_add(CELL_TEXT_INSET_PX),
                clipped.y.saturating_add(CELL_TEXT_INSET_PX),
                text_size,
                text_color,
            );
        }

        UiTreeGridBorderRenderer::draw_cell_borders(
            canvas,
            &cell.appearance.borders,
            cell.bounds,
            origin_x,
            origin_y,
            palette.muted_border,
        );
    }
}

#[derive(Clone, Copy)]
struct PositionedRect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn grid_viewport_extent(
    node: &UiNode,
    area: UiTreeRenderArea,
    x: usize,
    y: usize,
) -> (usize, usize) {
    let grid = &node.props().grid;
    let available_width = area.x.saturating_add(area.width).saturating_sub(x);
    let available_height = area.y.saturating_add(area.height).saturating_sub(y);
    let requested_width = usize::try_from(grid.viewport.width).unwrap_or(available_width);
    let requested_height = usize::try_from(grid.viewport.height).unwrap_or(available_height);
    let width = if requested_width == 0 {
        usize::try_from(grid.total_width).unwrap_or(available_width)
    } else {
        requested_width
    };
    let height = if requested_height == 0 {
        usize::try_from(grid.total_height).unwrap_or(available_height)
    } else {
        requested_height
    };
    (width.min(available_width), height.min(available_height))
}

fn positioned_rect(rect: UiRect, origin_x: usize, origin_y: usize) -> Option<PositionedRect> {
    let x = signed_position(origin_x, rect.x)?;
    let y = signed_position(origin_y, rect.y)?;
    Some(PositionedRect {
        x,
        y,
        width: usize::try_from(rect.width).ok()?,
        height: usize::try_from(rect.height).ok()?,
    })
}

fn signed_position(origin: usize, offset: i32) -> Option<usize> {
    i64::try_from(origin)
        .ok()?
        .saturating_add(i64::from(offset))
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
    use crate::render_model::{
        UiGridBorderSide, UiGridCell, UiGridCellAppearance, UiGridCellBorders, UiGridProps,
        UiGridViewport, UiNode, UiNodeKind, UiRect,
    };
    use crate::theme::ThemeSnapshot;

    const BACKGROUND: u32 = 0xFEFEFE;

    #[test]
    fn grid_renderer_draws_independent_border_colors_on_corresponding_edges() {
        let cell = UiGridCell {
            bounds: UiRect::new(1, 1, 6, 4),
            clipped_bounds: UiRect::new(1, 1, 6, 4),
            appearance: UiGridCellAppearance {
                borders: UiGridCellBorders {
                    left: UiGridBorderSide::solid("#AA0000"),
                    right: UiGridBorderSide::solid("#0000CC"),
                    top: UiGridBorderSide::solid("#00BB00"),
                    bottom: UiGridBorderSide::solid("#884400"),
                },
                ..UiGridCellAppearance::default()
            },
            ..UiGridCell::default()
        };
        let canvas = render_grid(cell, 10, 8, true);

        assert_eq!(0x00BB00, pixel_at(&canvas, 3, 1));
        assert_eq!(0xAA0000, pixel_at(&canvas, 1, 2));
        assert_eq!(0x0000CC, pixel_at(&canvas, 6, 2));
        assert_eq!(0x884400, pixel_at(&canvas, 3, 4));
    }

    #[test]
    fn grid_renderer_clips_offscreen_border_without_moving_its_edge_to_viewport() {
        let cell = UiGridCell {
            bounds: UiRect::new(-2, 1, 6, 2),
            clipped_bounds: UiRect::new(0, 1, 4, 2),
            appearance: UiGridCellAppearance {
                borders: UiGridCellBorders {
                    left: UiGridBorderSide::solid("#AA0000"),
                    top: UiGridBorderSide::solid("#00BB00"),
                    ..UiGridCellBorders::default()
                },
                ..UiGridCellAppearance::default()
            },
            ..UiGridCell::default()
        };
        let canvas = render_grid(cell, 4, 4, false);

        assert_eq!(0x00BB00, pixel_at(&canvas, 0, 1));
        assert_ne!(0xAA0000, pixel_at(&canvas, 0, 2));
        assert_eq!(BACKGROUND, pixel_at(&canvas, 4, 1));
    }

    #[test]
    fn grid_renderer_keeps_default_cell_free_of_custom_border_pixels() {
        let cell = UiGridCell {
            bounds: UiRect::new(1, 1, 4, 3),
            clipped_bounds: UiRect::new(1, 1, 4, 3),
            ..UiGridCell::default()
        };
        let canvas = render_grid(cell, 6, 5, false);

        assert_eq!(0, canvas.non_background_pixels(BACKGROUND));
    }

    #[test]
    fn grid_renderer_draws_cell_fill_and_text() {
        let cell = UiGridCell {
            bounds: UiRect::new(1, 1, 20, 12),
            clipped_bounds: UiRect::new(1, 1, 20, 12),
            text: "A".to_owned(),
            appearance: UiGridCellAppearance {
                fill_color: Some("#112233".to_owned()),
                text_color: Some("#FFFFFF".to_owned()),
                font_size_px: 8,
                ..UiGridCellAppearance::default()
            },
            ..UiGridCell::default()
        };
        let canvas = render_grid(cell, 24, 16, false);

        assert_eq!(0x112233, pixel_at(&canvas, 2, 2));
        assert!(canvas.non_background_pixels(BACKGROUND) > 20);
    }

    #[test]
    fn grid_renderer_uses_merged_anchor_bounds_for_custom_border() {
        let cell = UiGridCell {
            bounds: UiRect::new(1, 1, 8, 4),
            clipped_bounds: UiRect::new(1, 1, 8, 4),
            row_span: 2,
            column_span: 2,
            appearance: UiGridCellAppearance {
                borders: UiGridCellBorders {
                    bottom: UiGridBorderSide::solid("#884400"),
                    ..UiGridCellBorders::default()
                },
                ..UiGridCellAppearance::default()
            },
            ..UiGridCell::default()
        };
        let canvas = render_grid(cell, 10, 8, false);

        assert_eq!(0x884400, pixel_at(&canvas, 4, 4));
        assert_eq!(BACKGROUND, pixel_at(&canvas, 4, 2));
    }

    #[test]
    fn grid_renderer_skips_zero_area_clipped_cells() {
        let cell = UiGridCell {
            bounds: UiRect::new(1, 1, 8, 4),
            clipped_bounds: UiRect::new(1, 1, 0, 4),
            appearance: UiGridCellAppearance {
                fill_color: Some("#112233".to_owned()),
                ..UiGridCellAppearance::default()
            },
            ..UiGridCell::default()
        };

        let canvas = render_grid(cell, 10, 8, true);

        assert_eq!(0, canvas.non_background_pixels(BACKGROUND));
    }

    fn render_grid(cell: UiGridCell, width: u32, height: u32, show_grid_lines: bool) -> Canvas {
        let root = UiNode::new(UiNodeKind::Grid, "grid").grid(UiGridProps {
            viewport: UiGridViewport {
                width,
                height,
                ..UiGridViewport::default()
            },
            show_grid_lines,
            cells: vec![cell],
            ..UiGridProps::default()
        });
        let mut canvas = Canvas::new(32, 24, BACKGROUND);
        UiTreeCanvasRenderer::new(ThemeSnapshot::light()).render(
            &mut canvas,
            &root,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 32,
                height: 24,
                scroll_y: 0.0,
            },
        );
        canvas
    }

    fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> u32 {
        canvas.pixels()[y * canvas.width() + x]
    }
}
