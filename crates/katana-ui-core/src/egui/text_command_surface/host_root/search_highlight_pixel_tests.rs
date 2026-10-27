use super::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceRootFactory,
};
use crate::atom::TextArea;
use crate::egui::text_command_surface::{
    EguiTextCommandSurfacePresentation, KucDialogControls, KucDialogTarget,
    SearchHighlightProjection, SearchResultMode, SearchResultProjection, SearchResultRow,
    SelectionDialogProjection, SelectionDialogProjectionLease, TextCommandSurfaceStyle,
};
use crate::render_model::{UiStateId, UiTextSpan, UiTextSpanStyle};
use crate::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterRequest, PlatformTextRasterizer,
};
use crate::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const WIDTH: u32 = 900;
const HEIGHT: u32 = 640;

fn target(value: u8) -> KucDialogTarget {
    KucDialogTarget::from_opaque_bytes([value])
}
fn controls() -> KucDialogControls {
    KucDialogControls {
        cancel: "取消".into(),
        select: "選択".into(),
        search: "検索".into(),
        include: "含む".into(),
        exclude: "除く".into(),
        clear_history: "消去".into(),
        delete_history: "削除".into(),
        empty: "空".into(),
    }
}
fn root(style: TextCommandSurfaceStyle) -> EguiTextCommandSurfaceHostRoot {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("body"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, WIDTH, HEIGHT),
    ));
    let presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("body")),
        text: TextSurfacePresentation::from_props(surface.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    };
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"highlight-pixels",
        presentation,
        style,
    )
    .unwrap();
    EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .unwrap()
}
fn lease(revision: u64, query: &str, rows: Vec<SearchResultRow>) -> SelectionDialogProjectionLease {
    let mode = SearchResultMode {
        target: target(1),
        label: "query".into(),
        query: query.into(),
        include: None,
        exclude: None,
        options: vec![],
        history: vec![],
        results: rows,
        searched: true,
    };
    SelectionDialogProjectionLease::new(
        revision,
        "検索",
        controls(),
        SelectionDialogProjection::Search(SearchResultProjection {
            modes: vec![mode],
            active_mode: target(1),
        }),
    )
    .unwrap()
}

#[test]
fn public_root_highlighted_color_glyph_matches_metrics_and_hit_test() {
    let style = TextCommandSurfaceStyle::standard().unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = root(style.clone());
    root.synchronize_selection_dialog(lease(1, "", vec![]))
        .unwrap();
    {
        let mut frame = |events| {
            let mut output = None;
            let mut full = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(WIDTH as f32, HEIGHT as f32),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| output = Some(root.show(ui).unwrap()),
            );
            full.textures_delta.clear();
            (output.unwrap(), full)
        };
        for _ in 0..3 {
            frame(vec![]);
        }
        frame(vec![egui::Event::Text("検索".into())]);
    }
    let logical = [(3, 6)];
    let projected = SearchHighlightProjection::new("⭐️", &logical).unwrap();
    let row = SearchResultRow {
        target: target(2),
        label: "結果".into(),
        auxiliary_label: None,
        snippet: Some("⭐️".into()),
        highlights: projected.display_ranges().to_vec(),
        disabled: false,
    };
    root.synchronize_selection_dialog(lease(2, "検索", vec![row]))
        .unwrap();
    for _ in 0..3 {
        let mut full = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(WIDTH as f32, HEIGHT as f32),
                )),
                ..Default::default()
            },
            |ui| {
                root.show(ui).unwrap();
            },
        );
        full.textures_delta.clear();
    }
    let mut observed = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(WIDTH as f32, HEIGHT as f32),
            )),
            ..Default::default()
        },
        |ui| observed = Some(root.show(ui).unwrap()),
    );
    full.textures_delta.clear();
    let frame = observed.unwrap();
    let nodes = &full.platform_output.accesskit_update.unwrap().nodes;
    let bounds = nodes
        .iter()
        .find_map(|(_, node)| {
            (node.label() == Some("⭐️"))
                .then(|| node.bounds())
                .flatten()
        })
        .unwrap();
    let spans = UiTextSpan::emoji_marked_spans(
        "⭐️",
        UiTextSpanStyle {
            bold: true,
            highlight: true,
            color_rgba: style.text_paint.caret_rgba,
            ..Default::default()
        },
    );
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = rasterizer
        .rasterize(&PlatformTextRasterRequest {
            spans,
            font: style.text_raster.font.clone(),
            fallback_color_rgba: style.text_raster.fallback_color_rgba,
            line_height_px: style.text_raster.line_height_px,
            max_width_px: Some((bounds.x1 - bounds.x0) as f32 - 16.0),
            scale_factor: 1.0,
        })
        .unwrap();
    assert_eq!(raster.text, "⭐️");
    assert_eq!(projected.logical_ranges(), &logical);
    let glyph = raster
        .grapheme_bounds
        .iter()
        .find(|g| g.byte_start == 0 && g.byte_end == 6)
        .unwrap();
    assert!(glyph.width > 0.0 && glyph.height > 0.0);
    let hit = raster
        .hit_test(glyph.x + glyph.width / 2.0, glyph.y + glyph.height / 2.0)
        .unwrap();
    assert_eq!((hit.byte_start, hit.byte_end), (0, 6));
    let canvas = frame.output.evidence_composite.canvas.ui_rect();
    let pixels = frame.output.rgba_pixels();
    let background = style.text_paint.background_rgba;
    assert_eq!(background[3], 255);
    let mut matched = 0;
    let mut mismatched = 0;
    let mut matched_chromatic = 0;
    for (i, pixel) in raster.rgba_pixels.iter().enumerate() {
        if pixel[3] == 0 {
            continue;
        }
        let x = (bounds.x0 as f32 + 8.0).round() as i32 + (i % raster.width) as i32;
        let y = (bounds.y0 as f32 + 6.0).round() as i32 + (i / raster.width) as i32;
        assert!(
            x >= canvas.x
                && y >= canvas.y
                && x < canvas.x + canvas.width as i32
                && y < canvas.y + canvas.height as i32,
            "the complete glyph must remain inside the canvas"
        );
        let offset =
            ((y - canvas.y) as usize * canvas.width as usize + (x - canvas.x) as usize) * 4;
        let alpha = u32::from(pixel[3]);
        let mut expected = [0, 0, 0, 255];
        for channel in 0..3 {
            expected[channel] = ((u32::from(pixel[channel]) * alpha
                + u32::from(background[channel]) * (255 - alpha))
                / 255) as u8;
        }
        if pixels[offset..offset + 4] == expected {
            if expected != background {
                matched += 1;
                if (pixel[0] != pixel[1] || pixel[1] != pixel[2])
                    && (expected[0] != expected[1] || expected[1] != expected[2])
                {
                    matched_chromatic += 1;
                }
            }
        } else {
            mismatched += 1;
        }
    }
    assert_eq!(
        mismatched, 0,
        "every visible glyph pixel must match source-over at {bounds:?}"
    );
    assert!(
        matched > 0,
        "root pixels must contain the independently measured glyph: bounds={bounds:?}, raster={}x{}, opaque={}, visible={}, max_alpha={:?}",
        raster.width,
        raster.height,
        raster
            .rgba_pixels
            .iter()
            .filter(|pixel| pixel[3] == 255)
            .count(),
        raster
            .rgba_pixels
            .iter()
            .filter(|pixel| pixel[3] > 0)
            .count(),
        raster.rgba_pixels.iter().map(|pixel| pixel[3]).max(),
    );
    #[cfg(target_os = "macos")]
    assert!(
        raster
            .grapheme_crop(glyph, 1.0)
            .unwrap()
            .chromatic_pixel_count()
            > 0,
        "VS16 must select the color glyph"
    );
    if raster.report.color_emoji_font_available {
        assert!(
            matched_chromatic > 0,
            "root must paint the measured color glyph"
        );
    }
}
