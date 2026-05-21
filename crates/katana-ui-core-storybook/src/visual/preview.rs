use super::canvas::Canvas;
use super::layout_metrics::{LayoutRect, PREVIEW_X};
use super::preset_tabs;
use super::preview_detail;
use super::render_context::{RenderContext, ScenarioContext};
use super::text::TextVerticalBox;
use crate::catalog::StoryPresetLabels;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const PREVIEW_TITLE_Y: usize = 24;
const PREVIEW_META_Y: usize = 54;
const SUMMARY_Y: usize = 72;
const PREVIEW_TITLE_SIZE: f32 = 22.0;
const PREVIEW_META_SIZE: f32 = 13.0;
const SUMMARY_HEIGHT: usize = 24;
const SUMMARY_WIDTH: usize = 168;
const SUMMARY_GAP: usize = 10;
const SUMMARY_PADDING_X: usize = 8;
const SUMMARY_SIZE: f32 = 10.0;
const SUMMARY_COUNT: usize = 4;
const SUMMARY_TOOLTIP_Y_GAP: usize = 6;
const SUMMARY_TOOLTIP_HEIGHT: usize = 28;
const SUMMARY_TOOLTIP_PADDING_X: usize = 10;
const SUMMARY_TOOLTIP_MIN_WIDTH: usize = 160;
const SUMMARY_TOOLTIP_MAX_WIDTH: usize = 360;
const SUMMARY_TOOLTIP_CHAR_WIDTH: usize = 8;

struct SummarySample {
    full: String,
    visible: String,
}

pub(super) fn draw(
    canvas: &mut Canvas,
    root: &UiNode,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    draw_header(canvas, render, scenario);
    draw_summary_controls(canvas, render, scenario);
    if let Some(preview) = panel_child(root, "Preview") {
        preset_tabs::draw(canvas, render, scenario);
        preview_detail::draw_selected_hero(canvas, render, preview, scenario);
    }
}

pub(super) fn draw_overlay(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    let samples = summary_samples(scenario);
    draw_summary_tooltip(canvas, render, scenario, &samples);
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

fn draw_summary_controls(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
) {
    let samples = summary_samples(scenario);
    for (index, sample) in samples.iter().enumerate().take(SUMMARY_COUNT) {
        let rect = summary_control_rect(index);
        canvas.stroke_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            render.palette.border,
        );
        render.code_text.draw_centered(
            canvas,
            &sample.visible,
            rect.x + SUMMARY_PADDING_X,
            TextVerticalBox::new(SUMMARY_Y, SUMMARY_HEIGHT as f32),
            SUMMARY_SIZE,
            render.palette.muted,
        );
    }
}

fn short_value(value: &str) -> String {
    const MAX_CHARS: usize = 18;
    const SUFFIX: &str = "...";
    if value.chars().count() <= MAX_CHARS {
        return value.to_string();
    }
    let keep = MAX_CHARS - SUFFIX.len();
    let prefix: String = value.chars().take(keep).collect();
    format!("{prefix}{SUFFIX}")
}

fn summary_samples(scenario: ScenarioContext<'_>) -> [SummarySample; SUMMARY_COUNT] {
    let preset = preset_label(scenario);
    let values = [
        format!("preset {preset}"),
        format!("state {}", scenario.screen_state.state_label),
        format!("setting {}", setting_summary(scenario, preset)),
        format!("count {}", scenario.screen_state.action_count),
    ];
    values.map(|full| SummarySample {
        visible: short_value(&full),
        full,
    })
}

fn preset_label(scenario: ScenarioContext<'_>) -> &'static str {
    let labels = StoryPresetLabels::for_page(scenario.selected_page);
    labels
        .get(scenario.preset_index)
        .copied()
        .unwrap_or(labels[0])
}

fn setting_summary(scenario: ScenarioContext<'_>, preset: &str) -> String {
    if scenario.screen_state.last_setting != "none" {
        return format!(
            "{}={}",
            scenario.screen_state.last_setting, scenario.screen_state.last_setting_value
        );
    }
    if is_button_page(scenario.selected_page) {
        return format!("layout={preset}");
    }
    "none".to_string()
}

fn is_button_page(page: &str) -> bool {
    matches!(
        page,
        "button" | "text-button" | "svg-button" | "icon-text-button"
    )
}

fn draw_summary_tooltip(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
    samples: &[SummarySample; SUMMARY_COUNT],
) {
    let Some(index) = scenario.screen_state.hovered_summary_index else {
        return;
    };
    let Some(sample) = samples.get(index) else {
        return;
    };
    if sample.visible == sample.full {
        return;
    }
    let source = summary_control_rect(index);
    let width = tooltip_width(&sample.full);
    let y = source.bottom() + SUMMARY_TOOLTIP_Y_GAP;
    canvas.fill_rect(
        source.x,
        y,
        width,
        SUMMARY_TOOLTIP_HEIGHT,
        render.palette.surface,
    );
    canvas.stroke_rect(
        source.x,
        y,
        width,
        SUMMARY_TOOLTIP_HEIGHT,
        render.palette.accent,
    );
    render.code_text.draw_centered(
        canvas,
        &sample.full,
        source.x + SUMMARY_TOOLTIP_PADDING_X,
        TextVerticalBox::new(y, SUMMARY_TOOLTIP_HEIGHT as f32),
        SUMMARY_SIZE,
        render.palette.text,
    );
}

fn tooltip_width(value: &str) -> usize {
    let content_width = value.chars().count() * SUMMARY_TOOLTIP_CHAR_WIDTH;
    (content_width + SUMMARY_TOOLTIP_PADDING_X * 2)
        .clamp(SUMMARY_TOOLTIP_MIN_WIDTH, SUMMARY_TOOLTIP_MAX_WIDTH)
}

pub(super) fn summary_control_index_at(x: usize, y: usize) -> Option<usize> {
    (0..SUMMARY_COUNT).find(|index| summary_control_rect(*index).contains(x, y))
}

fn summary_control_rect(index: usize) -> LayoutRect {
    LayoutRect::new(
        PREVIEW_X + index * (SUMMARY_WIDTH + SUMMARY_GAP),
        SUMMARY_Y,
        SUMMARY_WIDTH,
        SUMMARY_HEIGHT,
    )
}

#[cfg(test)]
pub(super) const fn summary_controls_right_edge() -> usize {
    PREVIEW_X + SUMMARY_COUNT * SUMMARY_WIDTH + (SUMMARY_COUNT - 1) * SUMMARY_GAP
}

#[cfg(test)]
pub(super) const fn summary_control_height() -> usize {
    SUMMARY_HEIGHT
}

#[cfg(test)]
pub(super) fn summary_visible_samples_for_test(
    scenario: ScenarioContext<'_>,
) -> [String; SUMMARY_COUNT] {
    summary_samples(scenario).map(|sample| sample.visible)
}

#[cfg(test)]
pub(super) fn summary_full_samples_for_test(
    scenario: ScenarioContext<'_>,
) -> [String; SUMMARY_COUNT] {
    summary_samples(scenario).map(|sample| sample.full)
}

#[cfg(test)]
pub(super) fn summary_control_rect_for_test(index: usize) -> LayoutRect {
    summary_control_rect(index)
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}
