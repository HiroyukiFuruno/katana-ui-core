use super::*;
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::Divider;

#[test]
fn scroll_area_padded_heading_background_stays_inside_document_content_width() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(1280, 128, palette.background);
    let heading = UiNode::new(UiNodeKind::Text, "Heading")
        .text(UiTextProps {
            role: "heading".to_string(),
            ..UiTextProps::default()
        })
        .common(UiCommonProps::default().border(UiBorder::solid(2, 0, "heading.underline")))
        .height(UiDimension::Px(48));
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 1280,
            viewport_height: 128,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .common(
                    katana_ui_core::render_model::UiCommonProps::default().padding(UiEdgeInsets {
                        top: UiDimension::Px(56),
                        right: UiDimension::Px(56),
                        bottom: UiDimension::Px(56),
                        left: UiDimension::Px(56),
                    }),
                )
                .child(heading),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 128,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.selection), pixel_at(&canvas, 1223, 102));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 1279, 102));
}

#[test]
fn scroll_area_padded_viewer_blocks_keep_table_y_equal_to_normal_layout() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut direct_canvas = Canvas::new(1280, 420, palette.background);
    let mut scroll_canvas = Canvas::new(1280, 420, palette.background);

    UiTreeCanvasRenderer::new(theme.clone()).render(
        &mut direct_canvas,
        &viewer_table_stack_column_for_test(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 420,
            scroll_y: 0.0,
        },
    );
    UiTreeCanvasRenderer::new(theme).render(
        &mut scroll_canvas,
        &UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: 1280,
                viewport_height: 420,
                ..UiScrollAreaProps::default()
            })
            .child(viewer_table_stack_column_for_test()),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 420,
            scroll_y: 0.0,
        },
    );

    let direct_table_y =
        first_row_for_color(&direct_canvas, palette.table_header_background).kuc_unwrap();
    let scroll_table_y =
        first_row_for_color(&scroll_canvas, palette.table_header_background).kuc_unwrap();
    assert_eq!(233, direct_table_y);
    assert_eq!(direct_table_y, scroll_table_y);
}

#[test]
fn heading_without_border_suppresses_storybook_heading_underline() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 96, palette.background);
    let heading = UiNode::new(UiNodeKind::Text, "Heading")
        .text(UiTextProps {
            role: "heading".to_string(),
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(92));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &heading,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 96,
            scroll_y: 0.0,
        },
    );

    assert_eq!(0, count_pixel(&canvas, palette.selection));
}

#[test]
fn divider_top_padding_offsets_rule_line_inside_block() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(160, 48, palette.background);
    let divider: UiNode = Divider::new("horizontal rule").into();
    let divider = divider
        .common(UiCommonProps::default().padding(UiEdgeInsets {
            top: UiDimension::Px(9),
            ..UiEdgeInsets::default()
        }))
        .width(UiDimension::Px(120))
        .height(UiDimension::Px(34))
        .border(UiBorder::solid(2, 0, "document.rule.border"));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &divider,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 48,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 12, 25));
    assert_eq!(
        Some(palette.document_rule_border),
        pixel_at(&canvas, 12, 26)
    );
    assert_eq!(
        Some(palette.document_rule_border),
        pixel_at(&canvas, 12, 27)
    );
    assert_eq!(Some(palette.background), pixel_at(&canvas, 12, 28));
}

#[test]
fn document_code_common_props_draw_export_surface_box_and_padding() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 96, palette.background);
    let code = UiNode::new(UiNodeKind::Text, "let x = 42;")
        .text(UiTextProps {
            role: "code".to_string(),
            ..UiTextProps::default()
        })
        .common(document_code_common(20))
        .height(UiDimension::Px(84));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &code,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 96,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 13));
    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 0, 14));
    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 0, 69));
    assert_eq!(Some(palette.code_background), pixel_at(&canvas, 2, 15));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 70));
    assert_eq!(0, color_pixels_between_x(&canvas, palette.text, 0, 20));
    assert!(color_pixels_between_x(&canvas, palette.text, 24, 120) > 0);
}

#[test]
fn scroll_area_padded_text_does_not_rasterize_outside_document_content_width() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(1280, 128, palette.background);
    let text = "long text ".repeat(80);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 1280,
            viewport_height: 128,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .common(
                    katana_ui_core::render_model::UiCommonProps::default().padding(UiEdgeInsets {
                        top: UiDimension::Px(56),
                        right: UiDimension::Px(56),
                        bottom: UiDimension::Px(56),
                        left: UiDimension::Px(56),
                    }),
                )
                .child(UiNode::new(UiNodeKind::Text, text).text(UiTextProps {
                    role: "body".to_string(),
                    ..UiTextProps::default()
                })),
        );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 1280,
            height: 128,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 1279, 60));
}
