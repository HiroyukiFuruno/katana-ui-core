use super::*;

const TEXT_INSET: f32 = 10.0;
const ALPHA_INDEX: usize = 3;
const OPAQUE_ALPHA: u16 = 255;

fn assert_row_text_matches_raster(
    frame: &Pixels,
    row: egui::Rect,
    label: &str,
    scale: f32,
    style: &TextCommandSurfaceStyle,
) {
    let raster = crate::text_raster::PlatformTextRasterizer::new(
        crate::text_raster::PlatformTextRasterConfig::default(),
    )
    .rasterize(&crate::text_raster::PlatformTextRasterRequest {
        spans: crate::render_model::UiTextSpan::emoji_marked_spans(
            label,
            crate::render_model::UiTextSpanStyle {
                color_rgba: style.chrome_raster.text_color_rgba,
                ..Default::default()
            },
        ),
        font: style.chrome_raster.font.clone(),
        fallback_color_rgba: style.chrome_raster.text_color_rgba,
        line_height_px: style.chrome_raster.line_height_px,
        max_width_px: None,
        scale_factor: scale,
    })
    .expect("expected platform raster");
    let logical_width = (raster.width as f32 / scale).ceil().max(1.0) as u32;
    let logical_height = (raster.height as f32 / scale).ceil().max(1.0) as u32;
    assert_eq!(raster.rgba_pixels.len(), raster.width * raster.height);

    let text_x = (row.min.x + TEXT_INSET).round() as i32;
    let text_y = (row.center().y - logical_height as f32 / 2.0).round() as i32;
    let text_width = logical_width as i32;
    let text_height = logical_height as i32;
    let row_x = row.min.x as i32;
    let row_y = row.min.y as i32;
    let row_width = row.width() as i32;
    let row_height = row.height() as i32;
    assert!(text_x >= row_x && text_x + text_width <= row_x + row_width);
    assert!(text_y >= row_y && text_y + text_height <= row_y + row_height);

    let background = style.chrome_paint.action_rgba;
    assert_eq!(
        u16::from(background[ALPHA_INDEX]),
        OPAQUE_ALPHA,
        "row background must be opaque"
    );
    let mut ink_pixels = 0;
    for target_y in row_y..row_y + row_height {
        for target_x in row_x..row_x + row_width {
            let mut expected = background;
            if (text_x..text_x + text_width).contains(&target_x)
                && (text_y..text_y + text_height).contains(&target_y)
            {
                let source_x =
                    ((target_x - text_x) as usize * raster.width) / logical_width as usize;
                let source_y =
                    ((target_y - text_y) as usize * raster.height) / logical_height as usize;
                let source = raster.rgba_pixels[source_y * raster.width + source_x];
                let alpha = u16::from(source[ALPHA_INDEX]);
                expected = [
                    ((u16::from(source[0]) * alpha
                        + u16::from(background[0]) * (OPAQUE_ALPHA - alpha))
                        / OPAQUE_ALPHA) as u8,
                    ((u16::from(source[1]) * alpha
                        + u16::from(background[1]) * (OPAQUE_ALPHA - alpha))
                        / OPAQUE_ALPHA) as u8,
                    ((u16::from(source[2]) * alpha
                        + u16::from(background[2]) * (OPAQUE_ALPHA - alpha))
                        / OPAQUE_ALPHA) as u8,
                    OPAQUE_ALPHA as u8,
                ];
            }
            let actual = pixel(frame, target_x, target_y);
            assert_eq!(
                actual, expected,
                "RGBA mismatch at ({target_x}, {target_y}) for {label:?} at {scale}x"
            );
            ink_pixels += usize::from(actual != background);
        }
    }
    assert!(ink_pixels > 0, "expected visible raster ink for {label:?}");
}

#[test]
fn global_menu_keeps_logical_layout_and_ink_bounds_at_two_x() {
    let context = egui::Context::default();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1))
        .expect("root");
    let one_x = frame(&context, &mut root, Vec::new(), 1.0);
    let two_x = frame(&context, &mut root, Vec::new(), 2.0);
    assert_eq!(context.pixels_per_point(), 2.0);

    assert_eq!(one_x.trigger.width(), two_x.trigger.width());
    assert_eq!(one_x.trigger.height(), 28.0);
    assert_eq!(two_x.trigger.height(), 28.0);

    let center = two_x.trigger.center();
    let opened = frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(center),
            egui::Event::PointerButton {
                pos: center,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: center,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        2.0,
    );
    let action = opened.action.expect("2x trigger hit opens action row");
    assert_eq!(action.width(), two_x.trigger.width());
    assert_eq!(action.height(), 28.0);

    let style = TextCommandSurfaceStyle::standard().expect("style");
    assert_row_text_matches_raster(&one_x, one_x.trigger, "Global actions", 1.0, &style);
    assert_row_text_matches_raster(&two_x, two_x.trigger, "Global actions", 2.0, &style);
    let mut root_one_x = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1))
        .expect("1x root");
    let opened_one_x = frame(
        &context,
        &mut root_one_x,
        vec![
            egui::Event::PointerMoved(one_x.trigger.center()),
            egui::Event::PointerButton {
                pos: one_x.trigger.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: one_x.trigger.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        1.0,
    );
    let one_x_action = opened_one_x.action.expect("1x action row");
    assert_row_text_matches_raster(&opened_one_x, one_x_action, "Pixel action", 1.0, &style);
    assert_row_text_matches_raster(&opened, action, "Pixel action", 2.0, &style);
}
