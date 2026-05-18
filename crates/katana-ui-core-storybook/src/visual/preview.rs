use super::canvas::Canvas;
use super::card;
use super::layout_metrics::{
    PREVIEW_FIRST_CARD_Y, PREVIEW_VISIBLE_STORIES, PREVIEW_X, STORY_CARD_COLUMNS,
    STORY_CARD_STEP_X, STORY_CARD_STEP_Y,
};
use super::palette::VisualPalette;
use super::preset_tabs;
use super::preview_contract::PreviewContract;
use super::preview_detail;
use super::render_context::{PreviewContext, RenderContext, ScenarioContext};
use super::text::{TextRenderer, TextVerticalBox};
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const PREVIEW_TITLE_Y: usize = 24;
const PREVIEW_META_Y: usize = 54;
const FONT_SAMPLE_Y: usize = 72;
const GRID_TITLE_Y: usize = 424;
const PREVIEW_TITLE_SIZE: f32 = 22.0;
const PREVIEW_META_SIZE: f32 = 13.0;
const FONT_SAMPLE_HEIGHT: usize = 24;
const FONT_SAMPLE_WIDTH: usize = 132;
const FONT_SAMPLE_GAP: usize = 10;
const FONT_SAMPLE_PADDING_X: usize = 8;
const FONT_SAMPLE_SIZE: f32 = 12.0;

pub(super) fn draw(
    canvas: &mut Canvas,
    root: &UiNode,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    draw_header(canvas, render, scenario);
    draw_font_alignment_samples(canvas, render.text, render.code_text, render.palette);
    if let Some(preview) = panel_child(root, "Preview") {
        preset_tabs::draw(canvas, render, scenario);
        preview_detail::draw_selected_hero(canvas, render, preview, scenario);
        draw_preview_stories(
            canvas,
            PreviewContext {
                preview,
                render,
                selected_page: scenario.selected_page,
            },
        );
        PreviewContract::draw(canvas, preview, render, scenario);
    }
}

fn draw_header(canvas: &mut Canvas, render: RenderContext<'_>, scenario: ScenarioContext<'_>) {
    render.text.draw(
        canvas,
        &format!("Storybook Panel / {}", scenario.selected_page),
        PREVIEW_X,
        PREVIEW_TITLE_Y,
        PREVIEW_TITLE_SIZE,
        render.palette.text,
    );
    render.text.draw(
        canvas,
        preview_meta(scenario),
        PREVIEW_X,
        PREVIEW_META_Y,
        PREVIEW_META_SIZE,
        render.palette.muted,
    );
}

fn preview_meta(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index > 0 {
        return "operation after / callback log visible";
    }
    "core-only / pure Rust / late-bound style"
}

fn draw_font_alignment_samples(
    canvas: &mut Canvas,
    text: &TextRenderer,
    code_text: &TextRenderer,
    palette: &VisualPalette,
) {
    let samples = [
        ("English UI", text),
        ("日本語 UI", text),
        ("Text 日本語", text),
        ("⌘ K", code_text),
    ];
    let mut x = PREVIEW_X;
    for (sample, renderer) in samples {
        canvas.stroke_rect(
            x,
            FONT_SAMPLE_Y,
            FONT_SAMPLE_WIDTH,
            FONT_SAMPLE_HEIGHT,
            palette.border,
        );
        renderer.draw_centered(
            canvas,
            sample,
            x + FONT_SAMPLE_PADDING_X,
            TextVerticalBox::new(FONT_SAMPLE_Y, FONT_SAMPLE_HEIGHT as f32),
            FONT_SAMPLE_SIZE,
            palette.muted,
        );
        x += FONT_SAMPLE_WIDTH + FONT_SAMPLE_GAP;
    }
}

fn draw_preview_stories(canvas: &mut Canvas, context: PreviewContext<'_>) {
    context.render.text.draw(
        canvas,
        "All components",
        PREVIEW_X,
        GRID_TITLE_Y,
        PREVIEW_META_SIZE,
        context.render.palette.text,
    );
    let mut x = PREVIEW_X;
    let mut y = PREVIEW_FIRST_CARD_Y;
    let mut column = 0;
    for (child, example) in ordered_stories(&context).take(PREVIEW_VISIBLE_STORIES) {
        let context = card::StoryCardContext {
            text: context.render.text,
            code_text: context.render.code_text,
            style_sheet: context.render.style_sheet,
            palette: context.render.palette,
        };
        let frame = card::StoryCardFrame { x, y };
        card::draw_story_card(canvas, &context, child, &example.callback_logs, frame);
        column += 1;
        if column == STORY_CARD_COLUMNS {
            column = 0;
            x = PREVIEW_X;
            y += STORY_CARD_STEP_Y;
        } else {
            x += STORY_CARD_STEP_X;
        }
    }
}

fn ordered_stories<'a>(
    context: &PreviewContext<'a>,
) -> impl Iterator<Item = (&'a UiNode, &'a crate::catalog::StoryExample)> {
    let pairs = context
        .preview
        .children()
        .iter()
        .zip(context.render.examples.iter());
    let selected = pairs
        .clone()
        .find(|(_, example)| example.page == context.selected_page);
    selected
        .into_iter()
        .chain(pairs.filter(move |(_, example)| example.page != context.selected_page))
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}
