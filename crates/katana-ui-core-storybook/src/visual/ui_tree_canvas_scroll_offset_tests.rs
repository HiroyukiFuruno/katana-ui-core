use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::layout::{Column, Length};
use katana_ui_core::render_model::{
    UiDimension, UiNode, UiNodeKind, UiScrollAreaProps, UiTextProps, UiTextWrapMode,
};
use katana_ui_core::theme::ThemeSnapshot;

const SCROLL_TEST_VIEWPORT_WIDTH: usize = 240;
const SCROLL_TEST_ROW_HEIGHT: u16 = 24;
const SCROLL_TEST_ROW_OFFSET_Y: u32 = 24;
const SCROLL_TEST_DOUBLE_ROW_HEIGHT: usize = 48;
const SCROLL_TEST_THREE_ROW_HEIGHT: u32 = 72;
const SCROLL_TEST_GAP_HEIGHT: f32 = 12.0;
const SCROLL_TEST_GAP_VIEWPORT_HEIGHT: usize = 12;
const SCROLL_TEST_GAP_CONTENT_HEIGHT: u32 = 60;
const SCROLL_TEST_PRIMARY_VIEWPORT_HEIGHT: usize = 36;

#[test]
fn scroll_area_offset_moves_rendered_content() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        SCROLL_TEST_VIEWPORT_WIDTH,
        SCROLL_TEST_PRIMARY_VIEWPORT_HEIGHT,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: SCROLL_TEST_VIEWPORT_WIDTH as u32,
            viewport_height: SCROLL_TEST_PRIMARY_VIEWPORT_HEIGHT as u32,
            offset_y: SCROLL_TEST_ROW_OFFSET_Y,
            content_height: SCROLL_TEST_THREE_ROW_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(role_text("alert", "first"))
                .child(role_text("code", "second")),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: SCROLL_TEST_VIEWPORT_WIDTH,
            height: SCROLL_TEST_PRIMARY_VIEWPORT_HEIGHT,
            scroll_y: 0.0,
        },
    );

    let code_pixels = count_pixel(&canvas, palette.code_background);
    let alert_pixels = count_pixel(&canvas, palette.alert_note_accent);

    assert!(code_pixels > 100);
    assert!(
        code_pixels > alert_pixels,
        "scroll offset should expose the second row more than the first row: code={code_pixels} alert={alert_pixels}"
    );
}

#[test]
fn scroll_area_preserves_column_gap_in_virtualized_offsets() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        SCROLL_TEST_VIEWPORT_WIDTH,
        SCROLL_TEST_GAP_VIEWPORT_HEIGHT,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: SCROLL_TEST_VIEWPORT_WIDTH as u32,
            viewport_height: SCROLL_TEST_GAP_VIEWPORT_HEIGHT as u32,
            offset_y: SCROLL_TEST_ROW_OFFSET_Y,
            content_height: SCROLL_TEST_GAP_CONTENT_HEIGHT,
            ..UiScrollAreaProps::default()
        })
        .child(UiNode::from(
            Column::new()
                .gap(Length::px(SCROLL_TEST_GAP_HEIGHT))
                .child(role_text("code", "first"))
                .child(role_text("alert", "second")),
        ));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: SCROLL_TEST_VIEWPORT_WIDTH,
            height: SCROLL_TEST_GAP_VIEWPORT_HEIGHT,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        0,
        canvas.non_background_pixels(palette.background),
        "row-height offset must show the column gap, not the next row"
    );
}

#[test]
fn scroll_area_measures_wrapped_text_with_rendered_line_count() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(160, 36, palette.background);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 160,
            viewport_height: 36,
            offset_y: 30,
            content_height: 160,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(intrinsic_role_text(
                    "alert",
                    "first first first first first first first first first",
                ))
                .child(intrinsic_role_text("code", "second")),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 36,
            scroll_y: 0.0,
        },
    );

    let code_pixels = count_pixel(&canvas, palette.code_background);
    let visible_pixels = canvas.non_background_pixels(palette.background);

    assert!(
        code_pixels <= 1,
        "scrolled code row must stay outside the viewport with at most antialias residue: code_pixels={code_pixels}"
    );
    assert!(
        visible_pixels > 100,
        "wrapped first text must still occupy the scrolled viewport: visible={visible_pixels}"
    );
}

#[test]
fn deep_partial_text_scroll_renders_visible_lines_from_viewport_sized_canvas() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(220, 48, palette.background);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 220,
            viewport_height: 48,
            offset_y: 4_800,
            content_height: 8_000,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Text, repeated_code_lines(420))
                .text(UiTextProps {
                    role: "code".to_string(),
                    line_height_px: SCROLL_TEST_ROW_HEIGHT,
                    wrap: UiTextWrapMode::NoWrap,
                    ..UiTextProps::default()
                })
                .height(UiDimension::Px(8_000)),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 220,
            height: 48,
            scroll_y: 0.0,
        },
    );

    assert!(
        canvas.non_background_pixels(palette.background) > 100,
        "deep partial text scroll must still render visible code lines"
    );
}

#[test]
fn adjacent_scroll_areas_do_not_insert_implicit_gap() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        SCROLL_TEST_VIEWPORT_WIDTH,
        SCROLL_TEST_DOUBLE_ROW_HEIGHT,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(scroll_area_with_child("code", "first"))
        .child(scroll_area_with_child("alert", "second"));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: SCROLL_TEST_VIEWPORT_WIDTH,
            height: SCROLL_TEST_DOUBLE_ROW_HEIGHT,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.code_background), pixel_at(&canvas, 4, 4));
    assert_eq!(
        Some(palette.alert_note_accent),
        pixel_at(&canvas, 2, 28),
        "the second scroll area must start immediately after the first viewport"
    );
}

fn scroll_area_with_child(role: &str, label: &str) -> UiNode {
    UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: SCROLL_TEST_VIEWPORT_WIDTH as u32,
            viewport_height: u32::from(SCROLL_TEST_ROW_HEIGHT),
            content_height: u32::from(SCROLL_TEST_ROW_HEIGHT),
            ..UiScrollAreaProps::default()
        })
        .child(role_text(role, label))
}

fn role_text(role: &str, label: &str) -> UiNode {
    UiNode::new(UiNodeKind::Text, label)
        .text(UiTextProps {
            role: role.to_string(),
            line_height_px: SCROLL_TEST_ROW_HEIGHT,
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(SCROLL_TEST_ROW_HEIGHT))
}

fn intrinsic_role_text(role: &str, label: &str) -> UiNode {
    UiNode::new(UiNodeKind::Text, label).text(UiTextProps {
        role: role.to_string(),
        wrap: UiTextWrapMode::Wrap,
        ..UiTextProps::default()
    })
}

fn repeated_code_lines(count: usize) -> String {
    (0..count)
        .map(|index| format!("let value_{index} = {index};"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn count_pixel(canvas: &Canvas, color: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
