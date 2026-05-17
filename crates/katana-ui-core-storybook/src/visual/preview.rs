use super::canvas::Canvas;
use super::card;
use super::palette::VisualPalette;
use super::render_context::{PreviewContext, RenderContext, ScenarioContext};
use super::text::{TextRenderer, TextVerticalBox};
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const PREVIEW_X: usize = 310;
const PREVIEW_TITLE_Y: usize = 24;
const PREVIEW_META_Y: usize = 54;
const FONT_SAMPLE_Y: usize = 72;
const PREVIEW_FIRST_CARD_Y: usize = 112;
const PREVIEW_TITLE_SIZE: f32 = 22.0;
const PREVIEW_META_SIZE: f32 = 13.0;
const FONT_SAMPLE_HEIGHT: usize = 24;
const FONT_SAMPLE_WIDTH: usize = 132;
const FONT_SAMPLE_GAP: usize = 10;
const FONT_SAMPLE_PADDING_X: usize = 8;
const FONT_SAMPLE_SIZE: f32 = 12.0;
const PREVIEW_VISIBLE_STORIES: usize = 24;
const STORY_CARD_STEP_X: usize = 228;
const STORY_CARD_WRAP_X: usize = 1040;
const STORY_CARD_STEP_Y: usize = 144;

pub(super) fn draw(
    canvas: &mut Canvas,
    root: &UiNode,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    draw_header(canvas, render, scenario);
    draw_font_alignment_samples(canvas, render.text, render.code_text, render.palette);
    if let Some(preview) = panel_child(root, "Preview") {
        draw_preview_stories(
            canvas,
            PreviewContext {
                preview,
                render,
                selected_page: scenario.selected_page,
            },
        );
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
    if scenario.operation {
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
    let mut x = PREVIEW_X;
    let mut y = PREVIEW_FIRST_CARD_Y;
    for (child, example) in ordered_stories(&context).take(PREVIEW_VISIBLE_STORIES) {
        let context = card::StoryCardContext {
            text: context.render.text,
            code_text: context.render.code_text,
            style_sheet: context.render.style_sheet,
            palette: context.render.palette,
        };
        let frame = card::StoryCardFrame { x, y };
        card::draw_story_card(canvas, &context, child, &example.callback_logs, frame);
        x += STORY_CARD_STEP_X;
        if x > STORY_CARD_WRAP_X {
            x = PREVIEW_X;
            y += STORY_CARD_STEP_Y;
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
