use crate::visual::canvas::Canvas;
use cosmic_text::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache, Weight, Wrap,
};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::{FontFamily, FontToken};
use std::cell::RefCell;

const TEXT_BUFFER_WIDTH: f32 = 4096.0;
const LINE_HEIGHT_RATIO: f32 = 1.45;
const REGULAR_WEIGHT: u16 = 400;
const FALLBACK_FONT_SIZE: f32 = 14.0;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const OPAQUE_ALPHA: u8 = 0xff;
const FALLBACK_FONT_NAME: &str = "body";
const CODE_FONT_ROLE: &str = "code";
const SHORTCUT_FONT_ROLE: &str = "shortcut";

pub(crate) struct TextRenderer {
    font_system: RefCell<FontSystem>,
    swash_cache: RefCell<SwashCache>,
    font: FontToken,
}

impl TextRenderer {
    pub(crate) fn load(facade: &UiCoreFacade, role: &str) -> Self {
        let font = resolve_font(facade, role);
        Self {
            font_system: RefCell::new(FontSystem::new()),
            swash_cache: RefCell::new(SwashCache::new()),
            font,
        }
    }

    pub(crate) fn draw(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: usize,
        y: usize,
        size: f32,
        color: u32,
    ) {
        self.draw_layout(
            canvas,
            text,
            x,
            y,
            TextStyle::new(size, size * LINE_HEIGHT_RATIO, color),
        );
    }

    pub(crate) fn draw_centered(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: usize,
        vertical_box: TextVerticalBox,
        size: f32,
        color: u32,
    ) {
        self.draw_layout(
            canvas,
            text,
            x,
            vertical_box.y,
            TextStyle::new(size, vertical_box.height, color),
        );
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn font_family(&self) -> FontFamily {
        self.font.family
    }

    fn draw_layout(&self, canvas: &mut Canvas, text: &str, x: usize, y: usize, style: TextStyle) {
        let metrics = Metrics::new(style.size, style.line_height);
        let mut font_system = self.font_system.borrow_mut();
        let mut swash_cache = self.swash_cache.borrow_mut();
        let mut buffer = Buffer::new(&mut font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut font_system);
        buffer.set_wrap(Wrap::None);
        buffer.set_size(Some(TEXT_BUFFER_WIDTH), Some(metrics.line_height));
        buffer.set_text(
            text,
            &attrs_for_text(&self.font, text),
            Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(false);
        buffer.draw(
            &mut swash_cache,
            text_color(style.color),
            |left, top, _, _, color| {
                draw_text_pixel(canvas, left, top, x, y, color);
            },
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TextVerticalBox {
    y: usize,
    height: f32,
}

impl TextVerticalBox {
    pub(crate) fn new(y: usize, height: f32) -> Self {
        Self { y, height }
    }
}

struct TextStyle {
    size: f32,
    line_height: f32,
    color: u32,
}

impl TextStyle {
    fn new(size: f32, line_height: f32, color: u32) -> Self {
        Self {
            size,
            line_height,
            color,
        }
    }
}

fn resolve_font(facade: &UiCoreFacade, role: &str) -> FontToken {
    if let Some(font) = facade.theme().font(role) {
        return font.clone();
    }
    if role == SHORTCUT_FONT_ROLE
        && let Some(font) = facade.theme().font(CODE_FONT_ROLE)
    {
        return font.clone();
    }
    if let Some(font) = facade.font(facade.default_font_role()) {
        return font.clone();
    }
    FontToken {
        name: FALLBACK_FONT_NAME.to_string(),
        family: FontFamily::Proportional,
        size: FALLBACK_FONT_SIZE,
        weight: REGULAR_WEIGHT,
    }
}

fn attrs_for_text<'a>(font: &'a FontToken, text: &str) -> Attrs<'a> {
    Attrs::new()
        .family(family_for_text(font.family, text))
        .weight(Weight(font.weight.max(REGULAR_WEIGHT)))
}

fn family_for_text(family: FontFamily, text: &str) -> Family<'static> {
    match family {
        FontFamily::Proportional => Family::SansSerif,
        FontFamily::Monospace if text.is_ascii() => Family::Monospace,
        FontFamily::Monospace => Family::SansSerif,
    }
}

fn text_color(color: u32) -> Color {
    Color::rgba(red(color), green(color), blue(color), OPAQUE_ALPHA)
}

fn draw_text_pixel(
    canvas: &mut Canvas,
    left: i32,
    top: i32,
    origin_x: usize,
    origin_y: usize,
    color: Color,
) {
    let x = left + origin_x as i32;
    let y = top + origin_y as i32;
    if x < 0 || y < 0 {
        return;
    }
    canvas.blend(x as usize, y as usize, rgb(color), color.a());
}

fn rgb(color: Color) -> u32 {
    ((color.r() as u32) << RED_SHIFT) | ((color.g() as u32) << GREEN_SHIFT) | color.b() as u32
}

fn red(color: u32) -> u8 {
    ((color >> RED_SHIFT) & CHANNEL_MASK) as u8
}

fn green(color: u32) -> u8 {
    ((color >> GREEN_SHIFT) & CHANNEL_MASK) as u8
}

fn blue(color: u32) -> u8 {
    (color & CHANNEL_MASK) as u8
}
