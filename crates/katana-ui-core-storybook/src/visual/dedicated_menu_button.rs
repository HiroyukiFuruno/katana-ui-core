use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Block, Rect, TextSpec};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const TRIGGER_X: usize = 34;
const TRIGGER_Y: usize = 34;
const TRIGGER_WIDTH: usize = 154;
const TRIGGER_HEIGHT: usize = 26;
const STRIPE_WIDTH: usize = 5;
const MENU_X: usize = 34;
const MENU_Y: usize = 70;
const MENU_WIDTH: usize = 198;
const MENU_HEIGHT: usize = 50;
const ITEM_X: usize = MENU_X + 8;
const FIRST_ITEM_Y: usize = MENU_Y + 7;
const SECOND_ITEM_Y: usize = MENU_Y + 28;
const ITEM_WIDTH: usize = MENU_WIDTH - 16;
const ITEM_HEIGHT: usize = 16;
const LABEL_X_OFFSET: usize = 10;
const LABEL_Y_OFFSET: usize = 8;
const ITEM_LABEL_X_OFFSET: usize = 8;
const ITEM_LABEL_Y_OFFSET: usize = 5;
const BLOCK_COUNT: usize = 5;
const LABEL_COUNT: usize = 3;
const OPEN_PRESET_INDEX: usize = 1;
const DISABLED_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;

pub(super) fn menu_button(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::preview(
        canvas,
        text,
        palette,
        Rect::new(x, y, m::PX_0, m::PX_0),
        "Menu button panel",
        &blocks(palette, scenario),
        &labels(palette, scenario),
    );
}

fn blocks(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [Block; BLOCK_COUNT] {
    [
        Block::outlined(
            TRIGGER_X,
            TRIGGER_Y,
            TRIGGER_WIDTH,
            TRIGGER_HEIGHT,
            trigger_fill(palette, scenario),
        ),
        Block::new(
            TRIGGER_X,
            TRIGGER_Y,
            STRIPE_WIDTH,
            TRIGGER_HEIGHT,
            trigger_marker(palette, scenario),
        ),
        Block::outlined(
            MENU_X,
            MENU_Y,
            MENU_WIDTH,
            MENU_HEIGHT,
            menu_fill(palette, scenario),
        ),
        Block::outlined(
            ITEM_X,
            FIRST_ITEM_Y,
            ITEM_WIDTH,
            ITEM_HEIGHT,
            first_item_fill(palette, scenario),
        ),
        Block::outlined(
            ITEM_X,
            SECOND_ITEM_Y,
            ITEM_WIDTH,
            ITEM_HEIGHT,
            second_item_fill(palette, scenario),
        ),
    ]
}

fn labels(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> [TextSpec; LABEL_COUNT] {
    [
        TextSpec::new(
            TRIGGER_X + LABEL_X_OFFSET,
            TRIGGER_Y + LABEL_Y_OFFSET,
            m::FONT_8,
            trigger_text(palette, scenario),
            trigger_label(scenario),
        ),
        TextSpec::new(
            ITEM_X + ITEM_LABEL_X_OFFSET,
            FIRST_ITEM_Y + ITEM_LABEL_Y_OFFSET,
            m::FONT_8,
            first_item_text(palette, scenario),
            first_item_label(scenario),
        ),
        TextSpec::new(
            ITEM_X + ITEM_LABEL_X_OFFSET,
            SECOND_ITEM_Y + ITEM_LABEL_Y_OFFSET,
            m::FONT_8,
            second_item_text(palette, scenario),
            second_item_label(scenario),
        ),
    ]
}

fn trigger_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    if scenario.screen_state.has_settings_override() {
        return palette.accent;
    }
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

fn trigger_marker(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if is_open(scenario) {
        return common::SUCCESS;
    }
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.muted;
    }
    palette.accent
}

fn menu_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if is_open(scenario) || scenario.preset_index == THEME_PRESET_INDEX {
        return palette.panel;
    }
    palette.background
}

fn first_item_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return palette.background;
    }
    if is_open(scenario) {
        return palette.accent;
    }
    menu_fill(palette, scenario)
}

fn second_item_fill(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.panel;
    }
    if is_open(scenario) || scenario.preset_index == THEME_PRESET_INDEX {
        return palette.surface;
    }
    menu_fill(palette, scenario)
}

fn trigger_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.muted;
    }
    palette.text
}

fn first_item_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if is_open(scenario) {
        return palette.background;
    }
    palette.text
}

fn second_item_text(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return palette.muted;
    }
    palette.text
}

fn trigger_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return "Actions disabled";
    }
    if is_open(scenario) {
        return "Actions open";
    }
    "Actions"
}

fn first_item_label(scenario: ScenarioContext<'_>) -> &'static str {
    if is_open(scenario) {
        return "New file";
    }
    "Closed"
}

fn second_item_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.preset_index == DISABLED_PRESET_INDEX {
        return "Unavailable";
    }
    "Rename"
}

fn is_open(scenario: ScenarioContext<'_>) -> bool {
    scenario.preset_index == OPEN_PRESET_INDEX || scenario.screen_state.has_widget_action()
}
