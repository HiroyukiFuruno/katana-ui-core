use super::canvas::Canvas;
use super::dedicated_dod_common::{Rect, fill, outline};
use super::palette::VisualPalette;

pub(super) fn draw_blocks(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    blocks: &[Block],
) {
    for block in blocks {
        let rect = block.rect.at(x, y);
        if block.radius > 0 {
            draw_rounded_block(canvas, palette, rect, *block);
        } else {
            fill(canvas, rect, block.color);
        }
        if block.outline && block.radius == 0 {
            outline(canvas, palette, rect);
        }
    }
}

fn draw_rounded_block(canvas: &mut Canvas, palette: &VisualPalette, rect: Rect, block: Block) {
    if block.outline {
        canvas.fill_round_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            block.radius,
            palette.border,
        );
        canvas.fill_round_rect(
            rect.x + 1,
            rect.y + 1,
            rect.width.saturating_sub(2),
            rect.height.saturating_sub(2),
            block.radius.saturating_sub(1),
            block.color,
        );
        return;
    }
    canvas.fill_round_rect(
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        block.radius,
        block.color,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Block {
    rect: Rect,
    color: u32,
    outline: bool,
    radius: usize,
}

impl Block {
    pub(super) const fn new(x: usize, y: usize, width: usize, height: usize, color: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color,
            outline: false,
            radius: 0,
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
            radius: 0,
        }
    }

    pub(super) const fn rounded(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color,
            outline: false,
            radius,
        }
    }

    pub(super) const fn rounded_outlined(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color,
            outline: true,
            radius,
        }
    }
}
