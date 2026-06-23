use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_tabs_metrics::{
    ADD_CLOSE_PRESET_INDEX, GROUP_PRESET_INDEX, MOVE_PRESET_INDEX, OVERFLOW_MENU_FIRST_TEXT_Y,
    OVERFLOW_MENU_HEIGHT, OVERFLOW_MENU_SECOND_TEXT_Y, OVERFLOW_MENU_TEXT_X, OVERFLOW_MENU_WIDTH,
    OVERFLOW_MENU_X, OVERFLOW_MENU_Y, OVERFLOW_PRESET_INDEX, PIN_PRESET_INDEX, STATUS_HEIGHT,
    STATUS_TEXT_RIGHT_PADDING, STATUS_TEXT_X_PADDING, STATUS_WIDTH, STATUS_X, STATUS_Y,
    control_rects, overflow_button_rect, rect_to_common,
};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::screen_state_tabs::{TabsScreenAction, TabsScreenState};
use super::text::{TextBox, TextRenderer};

pub(super) fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (action, rect) in control_rects(x, y) {
        let fill = control_fill(palette, scenario, action);
        common::fill(canvas, rect_to_common(rect), fill);
        common::outline(canvas, palette, rect_to_common(rect));
        text.draw_in_box(
            canvas,
            control_label(action),
            TextBox::centered(rect.x, rect.y, rect.width, rect.height),
            m::FONT_7,
            palette.text,
        );
    }
}

pub(super) fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    state: &TabsScreenState,
    x: usize,
    y: usize,
) {
    let status = if scenario.screen_state.state_label == "idle" {
        state.state_label()
    } else {
        scenario.screen_state.state_label
    };
    let rect = Rect::new(x + STATUS_X, y + STATUS_Y, STATUS_WIDTH, STATUS_HEIGHT);
    common::fill(canvas, rect, palette.panel);
    common::outline(canvas, palette, rect);
    text.draw_in_box(
        canvas,
        status,
        TextBox::new(
            rect.x + STATUS_TEXT_X_PADDING,
            rect.y,
            STATUS_WIDTH - STATUS_TEXT_RIGHT_PADDING,
            STATUS_HEIGHT,
        ),
        m::FONT_7,
        palette.muted,
    );
}

pub(super) fn draw_overflow_button(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    x: usize,
    y: usize,
) {
    let rect = overflow_button_rect(x, y);
    let fill = if state.overflow_open {
        common::TOKEN
    } else {
        palette.panel
    };
    common::fill(canvas, rect, fill);
    common::outline(canvas, palette, rect);
    text.draw_in_box(
        canvas,
        "...",
        TextBox::centered(rect.x, rect.y, rect.width, rect.height),
        m::FONT_7,
        palette.text,
    );
}

pub(super) fn draw_overflow_menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let rect = Rect::new(
        x + OVERFLOW_MENU_X,
        y + OVERFLOW_MENU_Y,
        OVERFLOW_MENU_WIDTH,
        OVERFLOW_MENU_HEIGHT,
    );
    common::fill(canvas, rect, palette.surface);
    common::outline(canvas, palette, rect);
    text.draw(
        canvas,
        "hidden: lint.md",
        rect.x + OVERFLOW_MENU_TEXT_X,
        rect.y + OVERFLOW_MENU_FIRST_TEXT_Y,
        m::FONT_7,
        palette.text,
    );
    text.draw(
        canvas,
        "theme.rs",
        rect.x + OVERFLOW_MENU_TEXT_X,
        rect.y + OVERFLOW_MENU_SECOND_TEXT_Y,
        m::FONT_7,
        palette.muted,
    );
}

fn control_fill(
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    action: TabsScreenAction,
) -> u32 {
    if is_selected_control(scenario.preset_index, action) {
        return palette.accent;
    }
    palette.panel
}

fn is_selected_control(preset_index: usize, action: TabsScreenAction) -> bool {
    matches!(
        (preset_index, action),
        (ADD_CLOSE_PRESET_INDEX, TabsScreenAction::AddTab)
            | (PIN_PRESET_INDEX, TabsScreenAction::TogglePinActive)
            | (MOVE_PRESET_INDEX, TabsScreenAction::MoveActiveRight)
            | (GROUP_PRESET_INDEX, TabsScreenAction::GroupActive)
            | (OVERFLOW_PRESET_INDEX, TabsScreenAction::ToggleOverflow)
    )
}

fn control_label(action: TabsScreenAction) -> &'static str {
    match action {
        TabsScreenAction::AddTab => "+",
        TabsScreenAction::CloseActive => "close",
        TabsScreenAction::TogglePinActive => "pin",
        TabsScreenAction::MoveActiveRight => "move",
        TabsScreenAction::GroupActive => "group",
        TabsScreenAction::ToggleOverflow => "...",
    }
}
