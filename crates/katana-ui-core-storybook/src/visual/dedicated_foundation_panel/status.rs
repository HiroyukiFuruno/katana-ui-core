use super::super::canvas::Canvas;
use super::super::dedicated_dod_metrics as m;
use super::super::palette::VisualPalette;
use super::super::render_context::ScenarioContext;
use super::super::text::TextRenderer;
use super::model::{
    HORIZONTAL_PRESET_INDEX, NESTED_PRESET_INDEX, PREVIEW_SLOT, SCROLLBAR_PRESET_INDEX, STATUS_GAP,
    STATUS_HEIGHT, STATUS_TEXT_X, STATUS_TEXT_Y, STATUS_WIDTH, STATUS_X, STATUS_Y,
    VERTICAL_PRESET_INDEX, child_panel, component_scrollbars_visible, panel_props_for_slot,
};
use katana_ui_core::render_model::{UiNode, UiPanelProps};

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    node: &UiNode,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let preview = child_panel(node, "Preview panel")
        .map(|it| panel_props_for_slot(PREVIEW_SLOT, &it.props().panel, scenario));
    let rows = [
        format!("preset {}", preset_label(scenario.preset_index)),
        format!(
            "active {}",
            scenario.screen_state.panel.active_panel.label()
        ),
        format!(
            "scrollbar {}",
            scrollbar_label(component_scrollbars_visible(
                scenario,
                scenario.screen_state.panel.active_panel
            ))
        ),
        active_scroll_summary(preview.as_ref(), scenario),
    ];
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
            &row,
            row_x + STATUS_TEXT_X,
            y + STATUS_Y + STATUS_TEXT_Y,
            m::FONT_7,
            palette.muted,
        );
    }
}

fn active_scroll_summary(props: Option<&UiPanelProps>, scenario: ScenarioContext<'_>) -> String {
    if scenario.screen_state.panel.active_panel != PREVIEW_SLOT.key {
        let active = scenario
            .screen_state
            .panel
            .child(scenario.screen_state.panel.active_panel);
        return format!("active x{} y{}", active.scroll_x, active.scroll_y);
    }
    props.map_or_else(
        || "preview missing".to_string(),
        |it| format!("preview x{} y{}", it.scroll_x, it.scroll_y),
    )
}

fn scrollbar_label(visible: bool) -> &'static str {
    if visible { "shown" } else { "hidden" }
}

fn preset_label(index: usize) -> &'static str {
    match index {
        VERTICAL_PRESET_INDEX => "vertical",
        HORIZONTAL_PRESET_INDEX => "horizontal",
        SCROLLBAR_PRESET_INDEX => "toggle",
        NESTED_PRESET_INDEX => "nested",
        _ => "active",
    }
}
