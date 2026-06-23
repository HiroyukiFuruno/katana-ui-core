use super::*;

const DIAGRAM_PIXEL_RED_SHIFT: u32 = 16;
const DIAGRAM_PIXEL_GREEN_SHIFT: u32 = 8;

#[test]
fn kdv_storybook_left_pane_regression_settings_list_renders_controls() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let visual = VisualPalette::from_theme(&theme);
    let mut canvas = Canvas::new(360, 160, palette.background);
    let root = settings_list_root();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 360,
            height: 160,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.muted_border) > 120);
    assert!(count_pixel(&canvas, palette.code_background) > 120);
    assert!(count_pixel(&canvas, visual.accent) > 60);
}

#[test]
fn generic_toggle_uses_kuc_switch_geometry_and_theme() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let visual = VisualPalette::from_theme(&theme);
    let mut canvas = Canvas::new(80, 36, palette.background);
    let root: UiNode = Toggle::new("Dark").checked(true).into();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 36,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(visual.accent), pixel_at(&canvas, 8, 11));
    assert_ne!(Some(palette.background), pixel_at(&canvas, 46, 11));
}

#[test]
fn generic_button_hover_draws_kuc_interactive_preset_border() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(120, 40, palette.background);
    let root = UiNode::from(Button::new("copy")).interaction(hovered_interaction());

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.visual.hover_border), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(palette.visual.hover_border), pixel_at(&canvas, 95, 19));
}

#[test]
fn generic_toggle_hover_draws_kuc_interactive_preset_border() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(80, 36, palette.background);
    let root = UiNode::from(Toggle::new("Dark"))
        .checked(true)
        .interaction(hovered_interaction());

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 36,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.visual.hover_border), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(palette.visual.hover_border), pixel_at(&canvas, 47, 21));
}

#[test]
fn generic_text_hover_draws_kuc_hover_background_before_text() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(180, 48, palette.background);
    let root = UiNode::from(Text::new("Hover text"))
        .width(UiDimension::px(96))
        .height(UiDimension::px(28))
        .interaction(hovered_interaction());

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 48,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.hover_background) > 200);
    assert_eq!(Some(palette.hover_background), pixel_at(&canvas, 4, 27));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 120, 27));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 4, 28));
    assert!(
        rect_pixels_excluding(
            &canvas,
            0,
            0,
            96,
            28,
            &[palette.background, palette.hover_background]
        ) > 10
    );
}

#[test]
fn hover_surface_blends_over_existing_viewer_content() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(80, 32, palette.background);
    canvas.fill_rect(12, 12, 1, 1, palette.text);
    let root = UiNode::new(UiNodeKind::Stack, "")
        .width(UiDimension::px(80))
        .height(UiDimension::px(32))
        .visual_role(UiVisualRole::HoverSurface);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 32,
            scroll_y: 0.0,
        },
    );

    let blended = pixel_at(&canvas, 12, 12);
    assert_ne!(Some(palette.hover_background), blended);
    assert_ne!(Some(palette.text), blended);
}

#[test]
fn stack_absolute_children_overlay_top_and_bottom_controls() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(420, 220, palette.background);
    let root = diagram_media_frame_root();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 420,
            height: 220,
            scroll_y: 0.0,
        },
    );

    assert!(overlay_glyph_pixels(&canvas, 356, 8, 28, 28, palette) > 0);
    assert!(overlay_glyph_pixels(&canvas, 384, 8, 28, 28, palette) > 0);
    assert!(overlay_glyph_pixels(&canvas, 356, 108, 28, 28, palette) > 0);
    assert!(overlay_glyph_pixels(&canvas, 328, 136, 28, 28, palette) > 0);
    assert_eq!(Some(palette.background), pixel_at(&canvas, 324, 210));
}

#[test]
fn icon_variant_button_keeps_transparent_base_on_tree_canvas() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(64, 40, palette.background);
    let root = UiNode::from(Button::new("F").variant(UiVariant::Icon))
        .width(UiDimension::Px(28))
        .height(UiDimension::Px(28));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 64,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 3, 3));
}

#[test]
fn row_respects_explicit_stack_slot_width_for_diagram_toolbar() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(120, 48, palette.background);
    let root = UiNode::from(
        Row::new()
            .child(overlay_control_spacer())
            .child(overlay_control_button("F")),
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 48,
            scroll_y: 0.0,
        },
    );

    assert!(rect_non_background_pixels(&canvas, 28, 0, 28, 28, palette.background) > 0);
    assert_eq!(
        0,
        rect_non_background_pixels(&canvas, 104, 0, 16, 28, palette.background)
    );
}

fn overlay_glyph_pixels(
    canvas: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    palette: UiTreeCanvasPalette,
) -> usize {
    let diagram = (u32::from(DIAGRAM_PIXEL_RGBA[0]) << DIAGRAM_PIXEL_RED_SHIFT)
        | (u32::from(DIAGRAM_PIXEL_RGBA[1]) << DIAGRAM_PIXEL_GREEN_SHIFT)
        | u32::from(DIAGRAM_PIXEL_RGBA[2]);
    rect_pixels_excluding(canvas, x, y, width, height, &[palette.background, diagram])
}

fn rect_non_background_pixels(
    canvas: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    background: u32,
) -> usize {
    rect_pixels_excluding(canvas, x, y, width, height, &[background])
}

fn rect_pixels_excluding(
    canvas: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    excluded: &[u32],
) -> usize {
    let mut count = 0;
    for row in y..y.saturating_add(height).min(canvas.height()) {
        for column in x..x.saturating_add(width).min(canvas.width()) {
            let Some(pixel) = pixel_at(canvas, column, row) else {
                continue;
            };
            if excluded.contains(&pixel) {
                continue;
            }
            count += 1;
        }
    }
    count
}
