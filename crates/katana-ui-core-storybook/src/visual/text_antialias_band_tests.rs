use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextProps, UiTextSpan};
use katana_ui_core::theme::{FontFamily, FontToken, ThemeSnapshot};

const SCORE_CROP_WIDTH: usize = 920;
const SCORE_CROP_HEIGHT: usize = 120;
const SCORE_LAYOUT_SCALE: f32 = 2374.0 / 1280.0;
const SCORE_RASTER_SCALE: f32 = 2.0;
const MIN_COVERED_TITLE_HEIGHT: usize = 23;
const DOCUMENT_BODY_FONT_SIZE: f32 = 14.0;
const DOCUMENT_BODY_FONT_WEIGHT: u16 = 400;
const HEADING_NODE_HEIGHT: u16 = 43;
const BRIGHT_PIXEL_THRESHOLD: u8 = 54;
const ADJACENT_BAND_GAP: usize = 2;
const MIN_BAND_PIXELS: usize = 4;
const MAX_BAND_PIXELS: usize = 900;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const COLOR_BYTE_MASK: u32 = 0xff;
const RGB_CHANNEL_COUNT: u32 = 3;

#[test]
fn dark_document_heading_keeps_reference_ink_height_after_fractional_hidpi_presentation() {
    let mut theme = ThemeSnapshot::dark();
    theme.fonts.push(FontToken {
        name: "document-body".to_string(),
        family: FontFamily::Proportional,
        size: DOCUMENT_BODY_FONT_SIZE,
        weight: DOCUMENT_BODY_FONT_WEIGHT,
    });
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new_scaled_with_raster_scale(
        SCORE_CROP_WIDTH,
        SCORE_CROP_HEIGHT,
        SCORE_LAYOUT_SCALE,
        SCORE_RASTER_SCALE,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::Text, "")
        .text(UiTextProps {
            role: "heading".to_string(),
            spans: vec![
                UiTextSpan::emoji("🧪"),
                UiTextSpan::plain(" KatanA Rendering — Diagrams (External Dependencies)"),
            ],
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(HEADING_NODE_HEIGHT));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: SCORE_CROP_WIDTH,
            height: SCORE_CROP_HEIGHT,
            scroll_y: 0.0,
        },
    );

    let height = first_bright_content_band_height_after_score_average(
        &canvas,
        SCORE_CROP_WIDTH,
        SCORE_CROP_HEIGHT,
    )
    .kuc_expect("heading should render visible ink");

    assert!(
        height >= MIN_COVERED_TITLE_HEIGHT,
        "fractional HiDPI heading ink must keep enough vertical coverage for KatanA score crops: height={height} min={MIN_COVERED_TITLE_HEIGHT}"
    );
}

fn first_bright_content_band_height_after_score_average(
    canvas: &Canvas,
    logical_width: usize,
    logical_height: usize,
) -> Option<usize> {
    let mut band: Option<ContentBand> = None;
    for y in 0..logical_height {
        let mut row = ContentBand::new(y);
        for x in 0..logical_width {
            let pixel = score_average_pixel(canvas, x, y);
            if pixel_brightness(pixel) <= BRIGHT_PIXEL_THRESHOLD {
                continue;
            }
            row.observe();
        }
        let Some(row) = row.valid() else {
            if band.is_some() {
                break;
            }
            continue;
        };
        match &mut band {
            Some(current) if row.min_y <= current.max_y + ADJACENT_BAND_GAP => current.merge(row),
            Some(_) => break,
            None => band = Some(row),
        }
    }
    band.map(ContentBand::height)
}

#[derive(Clone, Copy)]
struct ContentBand {
    min_y: usize,
    max_y: usize,
    pixels: usize,
}

impl ContentBand {
    const fn new(y: usize) -> Self {
        Self {
            min_y: y,
            max_y: y,
            pixels: 0,
        }
    }

    fn observe(&mut self) {
        self.pixels += 1;
    }

    fn valid(self) -> Option<Self> {
        (self.pixels >= MIN_BAND_PIXELS && self.pixels <= MAX_BAND_PIXELS).then_some(self)
    }

    fn merge(&mut self, other: Self) {
        self.max_y = other.max_y;
        self.pixels += other.pixels;
    }

    fn height(self) -> usize {
        self.max_y.saturating_sub(self.min_y).saturating_add(1)
    }
}

const fn pixel_brightness(pixel: u32) -> u8 {
    let red = (pixel >> RED_SHIFT) & COLOR_BYTE_MASK;
    let green = (pixel >> GREEN_SHIFT) & COLOR_BYTE_MASK;
    let blue = pixel & COLOR_BYTE_MASK;
    ((red + green + blue) / RGB_CHANNEL_COUNT) as u8
}

fn score_average_pixel(canvas: &Canvas, logical_x: usize, logical_y: usize) -> u32 {
    let (left, right) = physical_range(logical_x, canvas.scale_factor(), canvas.width());
    let (top, bottom) = physical_range(logical_y, canvas.scale_factor(), canvas.height());
    let mut red_sum = 0usize;
    let mut green_sum = 0usize;
    let mut blue_sum = 0usize;
    let mut count = 0usize;
    for y in top.min(canvas.height())..bottom.min(canvas.height()) {
        for x in left.min(canvas.width())..right.min(canvas.width()) {
            let color = canvas.pixels()[y * canvas.width() + x];
            red_sum += ((color >> RED_SHIFT) & COLOR_BYTE_MASK) as usize;
            green_sum += ((color >> GREEN_SHIFT) & COLOR_BYTE_MASK) as usize;
            blue_sum += (color & COLOR_BYTE_MASK) as usize;
            count += 1;
        }
    }
    if count == 0 {
        return 0;
    }
    let red = red_sum / count;
    let green = green_sum / count;
    let blue = blue_sum / count;
    (red << RED_SHIFT | green << GREEN_SHIFT | blue) as u32
}

fn physical_range(logical: usize, scale: f32, max: usize) -> (usize, usize) {
    let start = ((logical as f32) * scale).floor().max(0.0) as usize;
    let end = (((logical + 1) as f32) * scale)
        .ceil()
        .max(start as f32 + 1.0) as usize;
    (start.min(max), end.min(max))
}
