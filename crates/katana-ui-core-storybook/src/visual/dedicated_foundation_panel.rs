use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const ROOT_X: usize = 18;
const ROOT_Y: usize = 30;
const ROOT_WIDTH: usize = 438;
const ROOT_HEIGHT: usize = 84;
const NAV_X: usize = 32;
const NAV_Y: usize = 48;
const NAV_WIDTH: usize = 102;
const NAV_HEIGHT: usize = 48;
const PREVIEW_X: usize = 150;
const PREVIEW_Y: usize = 48;
const PREVIEW_WIDTH: usize = 176;
const PREVIEW_HEIGHT: usize = 48;
const DETAIL_X: usize = 342;
const DETAIL_Y: usize = 48;
const DETAIL_WIDTH: usize = 96;
const DETAIL_HEIGHT: usize = 48;
const BAR_WIDTH: usize = 4;
const H_BAR_HEIGHT: usize = 4;
const THUMB_HEIGHT: usize = 18;
const THUMB_WIDTH: usize = 48;
const TEXT_X_OFFSET: usize = 8;
const TEXT_Y_OFFSET: usize = 7;
const LABEL_SIZE: f32 = 8.0;
const STATUS_X: usize = 18;
const STATUS_Y: usize = 100;
const STATUS_WIDTH: usize = 112;
const STATUS_HEIGHT: usize = 18;
const STATUS_GAP: usize = 8;
const STATUS_TEXT_X: usize = 7;
const STATUS_TEXT_Y: usize = 5;
const NESTED_PRESET_INDEX: usize = 0;
const CLIP_PRESET_INDEX: usize = 1;
const HORIZONTAL_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Panel foundation");
    draw_root_panel(canvas, text, palette, scenario, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_root_panel(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::draw_blocks(
        canvas,
        palette,
        x,
        y,
        &[
            Block::outlined(ROOT_X, ROOT_Y, ROOT_WIDTH, ROOT_HEIGHT, palette.surface),
            Block::outlined(NAV_X, NAV_Y, NAV_WIDTH, NAV_HEIGHT, palette.panel),
            Block::outlined(
                PREVIEW_X,
                PREVIEW_Y,
                PREVIEW_WIDTH,
                PREVIEW_HEIGHT,
                palette.panel,
            ),
            Block::outlined(
                DETAIL_X,
                DETAIL_Y,
                DETAIL_WIDTH,
                DETAIL_HEIGHT,
                palette.panel,
            ),
        ],
    );
    draw_panel_label(canvas, text, palette, x + NAV_X, y + NAV_Y, "nav");
    draw_panel_label(
        canvas,
        text,
        palette,
        x + PREVIEW_X,
        y + PREVIEW_Y,
        "preview",
    );
    draw_panel_label(canvas, text, palette, x + DETAIL_X, y + DETAIL_Y, "details");
    match scenario.preset_index {
        NESTED_PRESET_INDEX => draw_vertical_scroll(canvas, palette, x + NAV_X, y + NAV_Y),
        CLIP_PRESET_INDEX => draw_clipped_child(canvas, text, palette, x, y),
        HORIZONTAL_PRESET_INDEX => {
            draw_horizontal_scroll(canvas, palette, x + PREVIEW_X, y + PREVIEW_Y);
        }
        THEME_PRESET_INDEX => draw_theme_tokens(canvas, palette, x, y),
        _ => draw_vertical_scroll(canvas, palette, x + NAV_X, y + NAV_Y),
    }
}

fn draw_panel_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    label: &'static str,
) {
    text.draw(
        canvas,
        label,
        x + TEXT_X_OFFSET,
        y + TEXT_Y_OFFSET,
        LABEL_SIZE,
        palette.text,
    );
}

fn draw_vertical_scroll(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    canvas.fill_round_rect(
        x + NAV_WIDTH - m::PX_10,
        y + m::PX_6,
        BAR_WIDTH,
        NAV_HEIGHT - m::PX_12,
        m::PX_2,
        palette.border,
    );
    canvas.fill_round_rect(
        x + NAV_WIDTH - m::PX_10,
        y + m::PX_16,
        BAR_WIDTH,
        THUMB_HEIGHT,
        m::PX_2,
        palette.accent,
    );
}

fn draw_horizontal_scroll(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    canvas.fill_round_rect(
        x + m::PX_10,
        y + PREVIEW_HEIGHT - m::PX_10,
        PREVIEW_WIDTH - m::PX_20,
        H_BAR_HEIGHT,
        m::PX_2,
        palette.border,
    );
    canvas.fill_round_rect(
        x + m::PX_54,
        y + PREVIEW_HEIGHT - m::PX_10,
        THUMB_WIDTH,
        H_BAR_HEIGHT,
        m::PX_2,
        palette.accent,
    );
}

fn draw_clipped_child(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let clip = Rect::new(
        x + PREVIEW_X + m::PX_8,
        y + PREVIEW_Y + m::PX_18,
        PREVIEW_WIDTH - m::PX_16,
        m::PX_16,
    );
    canvas.with_clip(clip.x, clip.y, clip.width, clip.height, |canvas| {
        canvas.fill_rect(
            clip.x,
            clip.y,
            clip.width + m::PX_80,
            clip.height,
            palette.accent,
        );
        text.draw(
            canvas,
            "clipped long child surface",
            clip.x + m::PX_6,
            clip.y + m::PX_4,
            LABEL_SIZE,
            palette.background,
        );
    });
}

fn draw_theme_tokens(canvas: &mut Canvas, palette: &VisualPalette, x: usize, y: usize) {
    common::draw_blocks(
        canvas,
        palette,
        x,
        y,
        &[
            Block::new(
                PREVIEW_X + m::PX_14,
                PREVIEW_Y + m::PX_24,
                m::PX_20,
                m::PX_10,
                palette.accent,
            ),
            Block::new(
                PREVIEW_X + m::PX_42,
                PREVIEW_Y + m::PX_24,
                m::PX_20,
                m::PX_10,
                common::SUCCESS,
            ),
            Block::new(
                PREVIEW_X + m::PX_70,
                PREVIEW_Y + m::PX_24,
                m::PX_20,
                m::PX_10,
                common::WARN,
            ),
        ],
    );
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rows = ["clip", preset_label(scenario.preset_index), "nested state"];
    for (index, row) in rows.into_iter().enumerate() {
        let row_x = x + STATUS_X + index * (STATUS_WIDTH + STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn preset_label(index: usize) -> &'static str {
    match index {
        CLIP_PRESET_INDEX => "overflow clip",
        HORIZONTAL_PRESET_INDEX => "horizontal",
        THEME_PRESET_INDEX => "theme",
        _ => "nested",
    }
}
