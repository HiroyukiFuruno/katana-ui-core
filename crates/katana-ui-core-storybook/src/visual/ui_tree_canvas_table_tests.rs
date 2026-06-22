use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use crate::test_assert::KucTestExpect;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextProps};
use katana_ui_core::theme::{ColorToken, ThemeSnapshot};

const TABLE_ROW_BACKGROUND: u32 = 0xffffff;
const TABLE_HEADER_BACKGROUND: u32 = 0xeaf5ff;
const TABLE_EVEN_ROW_BACKGROUND: u32 = 0xf7fbff;
const TABLE_BORDER: u32 = 0xd0d7de;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const OPAQUE_ALPHA: u8 = 0xff;

#[test]
fn table_text_role_draws_cells_across_columns() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(1280, 160, palette.background);
    let root = text_table_node(
        "Feature | Status\nHTML alignment | covered",
        UiDimension::Px(0),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 160,
            scroll_y: 0.0,
        },
    );

    assert!(text_pixels_after_x(&canvas, palette.text, 650) > 10);
}

#[test]
fn table_text_role_draws_row_grid_and_backgrounds() {
    let theme = theme_with_table_tokens();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(420, 180, palette.background);
    let root = text_table_node(
        "Feature | Status\nHeader | Active\nAnother row | Completed",
        UiDimension::Px(0),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 420,
            height: 180,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.muted_border) > 80);

    let row0 = pixel_at(&canvas, 390, 20).kuc_expect("row background should exist");
    let row1 = pixel_at(&canvas, 390, 80).kuc_expect("row background should exist");
    let row2 = pixel_at(&canvas, 390, 140).kuc_expect("row background should exist");

    assert_eq!(TABLE_HEADER_BACKGROUND, row0);
    assert_eq!(TABLE_ROW_BACKGROUND, row1);
    assert_eq!(TABLE_EVEN_ROW_BACKGROUND, row2);
}

#[test]
fn table_text_role_uses_separator_alignment() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(900, 140, palette.background);
    let root = text_table_node(
        "| Left | Center | Right |\n| :--- | :---: | ---: |\n| A | B | C |",
        UiDimension::Px(0),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 900,
            height: 140,
            scroll_y: 0.0,
        },
    );

    assert!(text_pixels_between_x(&canvas, palette.text, 16, 80) > 0);
    assert!(text_pixels_between_x(&canvas, palette.text, 430, 480) > 0);
    assert!(text_pixels_between_x(&canvas, palette.text, 850, 890) > 0);
}

#[test]
fn table_text_role_wraps_long_cell_content_and_increases_height() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 220, palette.background);
    let root = text_table_node(
        "Feature | Status\nA very long status string that should wrap across multiple wrapped lines | Completed",
        UiDimension::Px(0),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 220,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.muted_border) > 80);
    let rows = rows_with_color(&canvas, palette.text);
    assert!(
        !rows.is_empty(),
        "wrapped table text should draw rows with text pixels"
    );
    let Some(first_row) = rows.first() else {
        return;
    };
    let Some(last_row) = rows.last() else {
        return;
    };
    assert!(last_row.saturating_sub(*first_row) >= 60);
}

fn text_table_node(label: &str, height: UiDimension) -> UiNode {
    UiNode::new(UiNodeKind::Text, label)
        .text(UiTextProps {
            role: "table".to_string(),
            ..UiTextProps::default()
        })
        .height(height)
}

fn theme_with_table_tokens() -> ThemeSnapshot {
    let mut theme = ThemeSnapshot::light();
    theme.colors.extend([
        color_token("table-row-background", TABLE_ROW_BACKGROUND),
        color_token("table-header-background", TABLE_HEADER_BACKGROUND),
        color_token("table-even-row-background", TABLE_EVEN_ROW_BACKGROUND),
        color_token("border", TABLE_BORDER),
    ]);
    theme
}

fn color_token(name: &str, rgb: u32) -> ColorToken {
    ColorToken {
        name: name.to_string(),
        rgba: [
            ((rgb >> RED_SHIFT) & CHANNEL_MASK) as u8,
            ((rgb >> GREEN_SHIFT) & CHANNEL_MASK) as u8,
            (rgb & CHANNEL_MASK) as u8,
            OPAQUE_ALPHA,
        ],
    }
}

fn text_pixels_after_x(canvas: &Canvas, color: u32, minimum_x: usize) -> usize {
    canvas
        .pixels()
        .iter()
        .enumerate()
        .filter(|(index, pixel)| **pixel == color && index % canvas.width() >= minimum_x)
        .count()
}

fn text_pixels_between_x(canvas: &Canvas, color: u32, minimum_x: usize, maximum_x: usize) -> usize {
    canvas
        .pixels()
        .iter()
        .enumerate()
        .filter(|(index, pixel)| {
            let x = index % canvas.width();
            **pixel == color && x >= minimum_x && x <= maximum_x
        })
        .count()
}

fn rows_with_color(canvas: &Canvas, expected: u32) -> Vec<usize> {
    let width = canvas.width();
    (0..canvas.height())
        .filter(|row| {
            canvas
                .pixels()
                .iter()
                .skip(row.saturating_mul(width))
                .take(width)
                .any(|pixel| *pixel == expected)
        })
        .collect()
}

fn count_pixel(canvas: &Canvas, expected: u32) -> usize {
    canvas.pixels().iter().filter(|it| **it == expected).count()
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas
        .pixels()
        .get(y.saturating_mul(canvas.width()) + x)
        .copied()
}
