use super::canvas::Canvas;
use super::card;
use super::text::TextRenderer;
use crate::catalog::StoryCatalog;
use crate::panel::{StorybookPanel, StorybookStyleSheet};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

pub(super) const WIDTH: usize = 1280;
pub(super) const HEIGHT: usize = 820;
pub(super) const BACKGROUND: u32 = 0x101418;
pub(super) const SURFACE: u32 = 0x171d23;
pub(super) const PANEL: u32 = 0x202932;
pub(super) const BORDER: u32 = 0x354453;
pub(super) const TEXT: u32 = 0xedf2f7;
pub(super) const MUTED: u32 = 0x9aa7b5;
pub(super) const ACCENT: u32 = 0x35c2a6;
pub(super) const FRAME_DELAY_MS: u64 = 16;

const NAV_WIDTH: usize = 280;
const BRAND_X: usize = 22;
const BRAND_TITLE_Y: usize = 20;
const BRAND_THEME_Y: usize = 46;
const BRAND_TITLE_SIZE: f32 = 18.0;
const BRAND_META_SIZE: f32 = 13.0;
const NAV_FIRST_ROW_Y: usize = 82;
const NAV_VISIBLE_ROWS: usize = 36;
const NAV_ROW_X: usize = 14;
const NAV_ROW_Y_OFFSET: usize = 5;
const NAV_ROW_WIDTH: usize = 248;
const NAV_ROW_HEIGHT: usize = 24;
const NAV_TEXT_X: usize = 24;
const NAV_TEXT_SIZE: f32 = 12.0;
const NAV_ROW_STEP: usize = 28;
const PREVIEW_X: usize = 310;
const PREVIEW_TITLE_Y: usize = 24;
const PREVIEW_META_Y: usize = 54;
const PREVIEW_FIRST_CARD_Y: usize = 92;
const PREVIEW_TITLE_SIZE: f32 = 22.0;
const PREVIEW_META_SIZE: f32 = 13.0;
const PREVIEW_VISIBLE_STORIES: usize = 24;
const STORY_CARD_STEP_X: usize = 228;
const STORY_CARD_WRAP_X: usize = 1040;
const STORY_CARD_STEP_Y: usize = 144;

pub(super) fn render_storybook_canvas() -> Canvas {
    let catalog = StoryCatalog;
    let examples = catalog.examples();
    let tree = StorybookPanel::new(ThemeSnapshot::dark()).build(&examples);
    let style_sheet = StorybookStyleSheet::default_sheet();
    let text = TextRenderer::load();
    let mut canvas = Canvas::new(WIDTH, HEIGHT, BACKGROUND);
    draw_shell(&mut canvas, &text, tree.root(), &style_sheet);
    canvas
}

fn draw_shell(
    canvas: &mut Canvas,
    text: &TextRenderer,
    root: &UiNode,
    style_sheet: &katana_ui_core::style::StyleSheet,
) {
    canvas.fill_rect(0, 0, NAV_WIDTH, HEIGHT, SURFACE);
    canvas.stroke_rect(0, 0, NAV_WIDTH, HEIGHT, BORDER);
    text.draw(
        canvas,
        "katana-ui-core",
        BRAND_X,
        BRAND_TITLE_Y,
        BRAND_TITLE_SIZE,
        TEXT,
    );
    text.draw(
        canvas,
        "panel theme: dark",
        BRAND_X,
        BRAND_THEME_Y,
        BRAND_META_SIZE,
        MUTED,
    );
    draw_navigation(canvas, text, root);
    draw_preview(canvas, text, root, style_sheet);
}

fn draw_navigation(canvas: &mut Canvas, text: &TextRenderer, root: &UiNode) {
    if let Some(nav) = panel_child(root, "Navigation") {
        let mut y = NAV_FIRST_ROW_Y;
        for child in nav.children().iter().take(NAV_VISIBLE_ROWS) {
            canvas.fill_rect(
                NAV_ROW_X,
                y - NAV_ROW_Y_OFFSET,
                NAV_ROW_WIDTH,
                NAV_ROW_HEIGHT,
                PANEL,
            );
            text.draw(
                canvas,
                &child.props().label,
                NAV_TEXT_X,
                y,
                NAV_TEXT_SIZE,
                TEXT,
            );
            y += NAV_ROW_STEP;
        }
    }
}

fn draw_preview(
    canvas: &mut Canvas,
    text: &TextRenderer,
    root: &UiNode,
    style_sheet: &katana_ui_core::style::StyleSheet,
) {
    text.draw(
        canvas,
        "Storybook Panel",
        PREVIEW_X,
        PREVIEW_TITLE_Y,
        PREVIEW_TITLE_SIZE,
        TEXT,
    );
    text.draw(
        canvas,
        "core-only / pure Rust / late-bound style",
        PREVIEW_X,
        PREVIEW_META_Y,
        PREVIEW_META_SIZE,
        MUTED,
    );
    if let Some(preview) = panel_child(root, "Preview") {
        draw_preview_stories(canvas, text, preview, style_sheet);
    }
}

fn draw_preview_stories(
    canvas: &mut Canvas,
    text: &TextRenderer,
    preview: &UiNode,
    style_sheet: &katana_ui_core::style::StyleSheet,
) {
    let mut x = PREVIEW_X;
    let mut y = PREVIEW_FIRST_CARD_Y;
    for child in preview.children().iter().take(PREVIEW_VISIBLE_STORIES) {
        card::draw_story_card(canvas, text, child, style_sheet, x, y);
        x += STORY_CARD_STEP_X;
        if x > STORY_CARD_WRAP_X {
            x = PREVIEW_X;
            y += STORY_CARD_STEP_Y;
        }
    }
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}
