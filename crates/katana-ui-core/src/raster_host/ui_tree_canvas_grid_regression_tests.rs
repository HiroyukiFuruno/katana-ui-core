use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use crate::render_model::{
    UiGridBorderLineStyle, UiGridBorderSide, UiGridCell, UiGridCellAppearance, UiGridCellBorders,
    UiGridProps, UiGridViewport, UiNode, UiNodeKind, UiRect,
};
use crate::theme::ThemeSnapshot;

const BACKGROUND: u32 = 0xFEFEFE;
const BORDER_COLOR: u32 = 0xAA3311;
const TEST_CELL_X: i32 = 1;
const BORDER_CELL_WIDTH: u32 = 20;
const STATE_CELL_WIDTH: u32 = 5;
const TEST_CELL_HEIGHT: u32 = 4;
const BORDER_ROW_SPACING: usize = 5;

#[test]
fn grid_border_line_styles_keep_visual_weight_and_pattern_contracts() {
    let styles = [
        UiGridBorderLineStyle::Dotted,
        UiGridBorderLineStyle::Dashed,
        UiGridBorderLineStyle::DashDot,
        UiGridBorderLineStyle::DashDotDot,
        UiGridBorderLineStyle::MediumDashed,
        UiGridBorderLineStyle::MediumDashDot,
        UiGridBorderLineStyle::MediumDashDotDot,
        UiGridBorderLineStyle::SlantDashDot,
        UiGridBorderLineStyle::Hair,
        UiGridBorderLineStyle::Thin,
        UiGridBorderLineStyle::Medium,
        UiGridBorderLineStyle::Thick,
        UiGridBorderLineStyle::Double,
        UiGridBorderLineStyle::Solid,
    ];
    let cells = styles
        .iter()
        .enumerate()
        .map(|(index, style)| border_cell(index, *style))
        .collect();
    let canvas = render_grid(
        UiGridProps {
            viewport: UiGridViewport {
                width: 24,
                height: 72,
                ..UiGridViewport::default()
            },
            show_grid_lines: false,
            cells,
            ..UiGridProps::default()
        },
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 32,
            height: 80,
            scroll_y: 0.0,
        },
        32,
        80,
    );

    for index in 0..styles.len() {
        assert_eq!(BORDER_COLOR, pixel_at(&canvas, 1, border_row(index)));
    }
    assert_eq!(BACKGROUND, pixel_at(&canvas, 2, border_row(0)));
    assert_eq!(BACKGROUND, pixel_at(&canvas, 8, border_row(1)));
    assert_eq!(BACKGROUND, pixel_at(&canvas, 7, border_row(2)));
    assert_eq!(BACKGROUND, pixel_at(&canvas, 7, border_row(3)));
    assert_eq!(BORDER_COLOR, pixel_at(&canvas, 1, border_row(10) + 1));
    assert_eq!(BORDER_COLOR, pixel_at(&canvas, 1, border_row(11) + 2));
    assert_eq!(BORDER_COLOR, pixel_at(&canvas, 1, border_row(12) + 2));
    assert_eq!(BACKGROUND, pixel_at(&canvas, 1, border_row(12) + 1));
}

#[test]
fn grid_renderer_uses_viewport_and_color_fallbacks_for_selected_and_active_cells() {
    let selected = state_cell(1, true, false);
    let active = state_cell(6, false, true);
    let canvas = render_grid(
        UiGridProps {
            total_width: 8,
            total_height: 12,
            show_grid_lines: true,
            cells: vec![selected, active],
            ..UiGridProps::default()
        },
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 12,
            height: 16,
            scroll_y: 0.0,
        },
        12,
        16,
    );

    assert_ne!(BACKGROUND, pixel_at(&canvas, 2, 2));
    assert_ne!(BACKGROUND, pixel_at(&canvas, 2, 7));
    assert_ne!(BACKGROUND, pixel_at(&canvas, 5, 2));
}

#[test]
fn grid_renderer_clips_the_render_area_and_rejects_negative_clipped_coordinates() {
    let visible = UiGridCell {
        bounds: UiRect::new(0, 0, 6, 4),
        clipped_bounds: UiRect::new(0, 0, 6, 4),
        appearance: UiGridCellAppearance {
            borders: UiGridCellBorders {
                top: UiGridBorderSide::solid("#AA3311"),
                ..UiGridCellBorders::default()
            },
            ..UiGridCellAppearance::default()
        },
        ..UiGridCell::default()
    };
    let clipped = render_grid(
        UiGridProps {
            viewport: UiGridViewport {
                width: 10,
                height: 4,
                ..UiGridViewport::default()
            },
            show_grid_lines: false,
            cells: vec![visible],
            ..UiGridProps::default()
        },
        UiTreeRenderArea {
            x: 2,
            y: 2,
            width: 4,
            height: 4,
            scroll_y: 0.0,
        },
        10,
        10,
    );
    assert_eq!(BORDER_COLOR, pixel_at(&clipped, 5, 2));
    assert_eq!(BACKGROUND, pixel_at(&clipped, 6, 2));

    let negative = UiGridCell {
        bounds: UiRect::new(-1, 0, 4, 3),
        clipped_bounds: UiRect::new(-1, 0, 4, 3),
        appearance: UiGridCellAppearance {
            borders: UiGridCellBorders {
                top: UiGridBorderSide::solid("#AA3311"),
                ..UiGridCellBorders::default()
            },
            ..UiGridCellAppearance::default()
        },
        ..UiGridCell::default()
    };
    let omitted = render_grid(
        UiGridProps {
            viewport: UiGridViewport {
                width: 8,
                height: 6,
                ..UiGridViewport::default()
            },
            show_grid_lines: false,
            cells: vec![negative],
            ..UiGridProps::default()
        },
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 8,
            height: 6,
            scroll_y: 0.0,
        },
        8,
        6,
    );
    assert_eq!(0, omitted.non_background_pixels(BACKGROUND));

    let empty = render_grid(
        UiGridProps {
            show_grid_lines: false,
            cells: vec![UiGridCell::default()],
            ..UiGridProps::default()
        },
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 8,
            height: 6,
            scroll_y: 0.0,
        },
        8,
        6,
    );
    assert_eq!(0, empty.non_background_pixels(BACKGROUND));
}

fn border_cell(index: usize, line_style: UiGridBorderLineStyle) -> UiGridCell {
    let y = i32::try_from(border_row(index)).expect("test row must fit i32");
    UiGridCell {
        bounds: UiRect::new(TEST_CELL_X, y, BORDER_CELL_WIDTH, TEST_CELL_HEIGHT),
        clipped_bounds: UiRect::new(TEST_CELL_X, y, BORDER_CELL_WIDTH, TEST_CELL_HEIGHT),
        appearance: UiGridCellAppearance {
            borders: UiGridCellBorders {
                top: UiGridBorderSide {
                    line_style,
                    color: Some("#AA3311".to_owned()),
                },
                ..UiGridCellBorders::default()
            },
            ..UiGridCellAppearance::default()
        },
        ..UiGridCell::default()
    }
}

fn state_cell(y: i32, selected: bool, active: bool) -> UiGridCell {
    UiGridCell {
        bounds: UiRect::new(TEST_CELL_X, y, STATE_CELL_WIDTH, TEST_CELL_HEIGHT),
        clipped_bounds: UiRect::new(TEST_CELL_X, y, STATE_CELL_WIDTH, TEST_CELL_HEIGHT),
        text: "A".to_owned(),
        selected,
        active,
        appearance: UiGridCellAppearance {
            fill_color: Some("invalid".to_owned()),
            text_color: Some("invalid".to_owned()),
            borders: UiGridCellBorders {
                right: UiGridBorderSide {
                    line_style: UiGridBorderLineStyle::Solid,
                    color: None,
                },
                ..UiGridCellBorders::default()
            },
            ..UiGridCellAppearance::default()
        },
        ..UiGridCell::default()
    }
}

fn border_row(index: usize) -> usize {
    1 + index * BORDER_ROW_SPACING
}

fn render_grid(props: UiGridProps, area: UiTreeRenderArea, width: usize, height: usize) -> Canvas {
    let root = UiNode::new(UiNodeKind::Grid, "grid").grid(props);
    let mut canvas = Canvas::new(width, height, BACKGROUND);
    UiTreeCanvasRenderer::new(ThemeSnapshot::light()).render(&mut canvas, &root, area);
    canvas
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> u32 {
    canvas.pixels()[y * canvas.width() + x]
}
