use super::canvas::Canvas;
use super::layout_metrics::PREVIEW_X;
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
    let labels = StoryPresetLabels::for_page(scenario.selected_page);
    let preset = labels
        .get(scenario.preset_index)
        .copied()
        .unwrap_or(labels[0]);
    let setting = if scenario.screen_state.last_setting == "none" {
        "none"
    } else {
        scenario.screen_state.last_setting
    };
    let samples = [
        format!("preset {}", short_value(preset)),
        format!("state {}", short_value(scenario.screen_state.state_label)),
        format!("setting {}", short_value(setting)),
        format!("count {}", scenario.screen_state.action_count),
    ];
    let mut x = PREVIEW_X;
    for sample in samples.into_iter().take(SUMMARY_COUNT) {
        canvas.stroke_rect(
            x,
            SUMMARY_Y,
            SUMMARY_WIDTH,
            SUMMARY_HEIGHT,
            render.palette.border,
        );
        render.code_text.draw_centered(
            canvas,
            &sample,
            x + SUMMARY_PADDING_X,
            TextVerticalBox::new(SUMMARY_Y, SUMMARY_HEIGHT as f32),
            SUMMARY_SIZE,
            render.palette.muted,
        );
        x += SUMMARY_WIDTH + SUMMARY_GAP;
    }
}

fn short_value(value: &str) -> String {
    const MAX_CHARS: usize = 12;
    const SUFFIX: &str = "...";
    if value.chars().count() <= MAX_CHARS {
        return value.to_string();
    }
    let keep = MAX_CHARS - SUFFIX.len();
    let prefix: String = value.chars().take(keep).collect();
    format!("{prefix}{SUFFIX}")
}

#[cfg(test)]
pub(super) const fn summary_controls_right_edge() -> usize {
    PREVIEW_X + SUMMARY_COUNT * SUMMARY_WIDTH + (SUMMARY_COUNT - 1) * SUMMARY_GAP
}

#[cfg(test)]
pub(super) const fn summary_control_height() -> usize {
    SUMMARY_HEIGHT
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}
