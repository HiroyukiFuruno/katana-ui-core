use super::canvas::Canvas;
use super::palette::VisualPalette;
use super::text::TextRenderer;

pub(super) const AREA_WIDTH: usize = 520;
pub(super) const AREA_HEIGHT: usize = 132;
pub(super) const TITLE_SIZE: f32 = 11.0;
pub(super) const BODY_SIZE: f32 = 10.0;
const ACCENT_WIDTH: usize = 4;
const TITLE_X_OFFSET: usize = 12;
const TITLE_Y_OFFSET: usize = 10;
const CHIP_TEXT_X_OFFSET: usize = 8;
const CHIP_TEXT_Y_OFFSET: usize = 6;
const MIN_CROSS_ICON_SIZE: usize = 6;
const CROSS_ICON_ARM_DIVISOR: usize = 3;
pub(super) const TOKEN: u32 = 0x4ec9b0;
pub(super) const WARN: u32 = 0xd7ba7d;
pub(super) const DANGER: u32 = 0xf44747;
pub(super) const SUCCESS: u32 = 0x6a9955;
pub(super) const PURPLE: u32 = 0xc586c0;

pub(super) fn frame(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    title: &str,
) {
    canvas.fill_rect(x, y, AREA_WIDTH, AREA_HEIGHT, palette.panel);
    canvas.stroke_rect(x, y, AREA_WIDTH, AREA_HEIGHT, palette.border);
    canvas.fill_rect(x, y, ACCENT_WIDTH, AREA_HEIGHT, palette.accent);
    text.draw(
        canvas,
        title,
        x + TITLE_X_OFFSET,
        y + TITLE_Y_OFFSET,
        TITLE_SIZE,
        palette.text,
    );
}

pub(super) fn chip(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    rect: Rect,
    value: &str,
    fill: u32,
) {
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, fill);
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
    text.draw(
        canvas,
        value,
        rect.x + CHIP_TEXT_X_OFFSET,
        rect.y + CHIP_TEXT_Y_OFFSET,
        BODY_SIZE,
        palette.background,
    );
}

pub(super) fn outline(canvas: &mut Canvas, palette: &VisualPalette, rect: Rect) {
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
}

pub(super) fn fill(canvas: &mut Canvas, rect: Rect, color: u32) {
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, color);
}

pub(super) fn preview(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    origin: Rect,
    title: &str,
    blocks: &[Block],
    labels: &[TextSpec],
) {
    frame(canvas, text, palette, origin.x, origin.y, title);
    draw_blocks(canvas, palette, origin.x, origin.y, blocks);
    draw_labels(canvas, text, origin.x, origin.y, labels);
}

pub(super) fn draw_blocks(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    blocks: &[Block],
) {
    for block in blocks {
        let rect = block.rect.at(x, y);
        fill(canvas, rect, block.color);
        if block.outline {
            outline(canvas, palette, rect);
        }
    }
}

pub(super) fn draw_labels(
    canvas: &mut Canvas,
    text: &TextRenderer,
    x: usize,
    y: usize,
    labels: &[TextSpec],
) {
    for label in labels {
        text.draw(
            canvas,
            label.value,
            x + label.x,
            y + label.y,
            label.size,
            label.color,
        );
    }
}

pub(super) fn draw_chips(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    chips: &[ChipSpec],
) {
    for chip_spec in chips {
        chip(
            canvas,
            text,
            palette,
            chip_spec.rect.at(x, y),
            chip_spec.value,
            chip_spec.fill,
        );
    }
}

pub(super) fn cross_icon(canvas: &mut Canvas, x: usize, y: usize, size: usize, color: u32) {
    let arm = size.max(MIN_CROSS_ICON_SIZE) / CROSS_ICON_ARM_DIVISOR;
    fill(canvas, Rect::new(x + arm, y, arm, size), color);
    fill(canvas, Rect::new(x, y + arm, size, arm), color);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Block {
    rect: Rect,
    color: u32,
    outline: bool,
}

impl Block {
    pub(super) const fn new(x: usize, y: usize, width: usize, height: usize, color: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color,
            outline: false,
        }
    }

    pub(super) const fn outlined(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color,
            outline: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TextSpec {
    x: usize,
    y: usize,
    size: f32,
    color: u32,
    value: &'static str,
}

impl TextSpec {
    pub(super) const fn new(
        x: usize,
        y: usize,
        size: f32,
        color: u32,
        value: &'static str,
    ) -> Self {
        Self {
            x,
            y,
            size,
            color,
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ChipSpec {
    rect: Rect,
    value: &'static str,
    fill: u32,
}

impl ChipSpec {
    pub(super) const fn new(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        value: &'static str,
        fill: u32,
    ) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            value,
            fill,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Rect {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) width: usize,
    pub(super) height: usize,
}

impl Rect {
    pub(super) const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub(super) const fn at(self, x: usize, y: usize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            width: self.width,
            height: self.height,
        }
    }
}
