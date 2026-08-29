use super::button_options::{
    CONTROL_COUNT, FIRST_ROW_Y_OFFSET, ROW_GAP, ROW_HEIGHT, ROW_WIDTH, ROW_X, SECTION_WIDTH,
    SECTION_X, StorybookButtonHeightMode, StorybookButtonOptionControl, StorybookButtonOptions,
    StorybookButtonWidthMode, control_index,
};
use super::canvas::Canvas;
use super::dedicated_dod_atom_button_live::ButtonLiveKind;
use super::dedicated_dod_atom_button_live_surface::{button_layout, measure_button_label_width};
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::switch_control;
use super::text::{TextRenderer, TextVerticalBox};

const SECTION_HEADER_HEIGHT: usize = 38;
const SECTION_ACCENT_WIDTH: usize = 4;
const SECTION_TITLE_X_OFFSET: usize = 10;
const SECTION_TITLE_Y_OFFSET: usize = 12;
const LABEL_X_OFFSET: usize = 8;
const LABEL_SIZE: f32 = 10.5;
const CONTROL_WIDTH: usize = 106;
const CONTROL_HEIGHT: usize = 22;
const CONTROL_X_OFFSET: usize = ROW_WIDTH - CONTROL_WIDTH - 8;
const CONTROL_Y_OFFSET: usize = 4;
const TOGGLE_WIDTH: usize = 48;
const TOGGLE_HEIGHT: usize = 22;
const TOGGLE_X_OFFSET: usize = ROW_WIDTH - TOGGLE_WIDTH - 12;
const TOGGLE_Y_OFFSET: usize = 4;
const TEXT_X_OFFSET: usize = 10;
const TEXT_SIZE: f32 = 10.0;
const WIDTH_VALUE_SUFFIX: &str = "px";
const HEIGHT_VALUE_SUFFIX: &str = "px";

pub(super) fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    y: usize,
) -> usize {
    let height = SECTION_HEADER_HEIGHT + CONTROL_COUNT * (ROW_HEIGHT + ROW_GAP);
    canvas.fill_rect(SECTION_X, y, SECTION_WIDTH, height, palette.code_background);
    canvas.stroke_rect(SECTION_X, y, SECTION_WIDTH, height, palette.border);
    canvas.fill_rect(SECTION_X, y, SECTION_ACCENT_WIDTH, height, palette.accent);
    text.draw(
        canvas,
        "Button options",
        SECTION_X + SECTION_TITLE_X_OFFSET,
        y + SECTION_TITLE_Y_OFFSET,
        LABEL_SIZE,
        palette.text,
    );
    for control in StorybookButtonOptionControl::all() {
        draw_control(canvas, text, palette, scenario, y, control);
    }
    y + height
}

fn draw_control(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    section_y: usize,
    control: StorybookButtonOptionControl,
) {
    let index = control_index(control);
    let row_y = section_y + FIRST_ROW_Y_OFFSET + index * (ROW_HEIGHT + ROW_GAP);
    let row = LayoutRect::new(ROW_X, row_y, ROW_WIDTH, ROW_HEIGHT);
    canvas.fill_rect(row.x, row.y, row.width, row.height, palette.panel);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.border);
    text.draw_centered(
        canvas,
        control.setting_name(),
        row.x + LABEL_X_OFFSET,
        TextVerticalBox::new(row.y, row.height as f32),
        LABEL_SIZE,
        palette.text,
    );
    draw_value(canvas, text, palette, scenario, row, control);
}

fn draw_value(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    row: LayoutRect,
    control: StorybookButtonOptionControl,
) {
    let options = scenario.screen_state.button_options;
    match control {
        StorybookButtonOptionControl::Visible
        | StorybookButtonOptionControl::Disabled
        | StorybookButtonOptionControl::Focusable
        | StorybookButtonOptionControl::Border
        | StorybookButtonOptionControl::KeyboardActivation => {
            draw_toggle(canvas, palette, options, row, control)
        }
        StorybookButtonOptionControl::Label
        | StorybookButtonOptionControl::Width
        | StorybookButtonOptionControl::Height
        | StorybookButtonOptionControl::TabIndex
        | StorybookButtonOptionControl::ZIndex
        | StorybookButtonOptionControl::Command
        | StorybookButtonOptionControl::IconPosition
        | StorybookButtonOptionControl::LayoutPreset
        | StorybookButtonOptionControl::SvgSource
        | StorybookButtonOptionControl::AriaLabel => {
            draw_value_button(canvas, text, palette, scenario, row, control);
        }
    }
}

fn draw_toggle(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    options: StorybookButtonOptions,
    row: LayoutRect,
    control: StorybookButtonOptionControl,
) {
    let enabled = match control {
        StorybookButtonOptionControl::Visible => options.visible,
        StorybookButtonOptionControl::Disabled => options.disabled,
        StorybookButtonOptionControl::Focusable => options.focusable,
        StorybookButtonOptionControl::Border => options.border,
        StorybookButtonOptionControl::KeyboardActivation => options.keyboard_activation,
        StorybookButtonOptionControl::Label
        | StorybookButtonOptionControl::Width
        | StorybookButtonOptionControl::Height
        | StorybookButtonOptionControl::TabIndex
        | StorybookButtonOptionControl::ZIndex
        | StorybookButtonOptionControl::Command
        | StorybookButtonOptionControl::IconPosition
        | StorybookButtonOptionControl::LayoutPreset
        | StorybookButtonOptionControl::SvgSource
        | StorybookButtonOptionControl::AriaLabel => false,
    };
    let x = row.x + TOGGLE_X_OFFSET;
    let y = row.y + TOGGLE_Y_OFFSET;
    switch_control::draw_switch(canvas, palette, x, y, TOGGLE_WIDTH, TOGGLE_HEIGHT, enabled);
}

fn draw_value_button(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    row: LayoutRect,
    control: StorybookButtonOptionControl,
) {
    let x = row.x + CONTROL_X_OFFSET;
    let y = row.y + CONTROL_Y_OFFSET;
    let value = effective_setting_value(text, scenario, control);
    canvas.fill_rect(x, y, CONTROL_WIDTH, CONTROL_HEIGHT, palette.surface);
    canvas.stroke_rect(x, y, CONTROL_WIDTH, CONTROL_HEIGHT, palette.accent);
    text.draw_centered(
        canvas,
        &value,
        x + TEXT_X_OFFSET,
        TextVerticalBox::new(y, CONTROL_HEIGHT as f32),
        TEXT_SIZE,
        palette.text,
    );
}

fn effective_setting_value(
    text: &TextRenderer,
    scenario: ScenarioContext<'_>,
    control: StorybookButtonOptionControl,
) -> String {
    let options = scenario.screen_state.button_options;
    match control {
        StorybookButtonOptionControl::Width
            if options.width_mode == StorybookButtonWidthMode::Auto =>
        {
            format!(
                "auto {}{WIDTH_VALUE_SUFFIX}",
                effective_width(text, scenario)
            )
        }
        StorybookButtonOptionControl::Height
            if options.height_mode == StorybookButtonHeightMode::Auto =>
        {
            format!("auto {}{HEIGHT_VALUE_SUFFIX}", effective_height(scenario))
        }
        _ => control.setting_value(options).to_string(),
    }
}

fn effective_width(text: &TextRenderer, scenario: ScenarioContext<'_>) -> usize {
    effective_layout_dimension(text, scenario).0
}

fn effective_height(scenario: ScenarioContext<'_>) -> usize {
    let kind = button_kind_for_page(scenario.selected_page);
    let layout = button_layout(
        scenario
            .screen_state
            .button_options
            .effective_preset_index(scenario.preset_index),
        scenario.screen_state.button_options.width_mode,
        scenario.screen_state.button_options.height_mode,
        0,
        kind.has_icon(),
        kind.has_visible_label(),
    );
    layout.height
}

fn effective_layout_dimension(
    text: &TextRenderer,
    scenario: ScenarioContext<'_>,
) -> (usize, usize) {
    let kind = button_kind_for_page(scenario.selected_page);
    let label_width = measure_button_label_width(text, button_label_for_kind(kind));
    let layout = button_layout(
        scenario
            .screen_state
            .button_options
            .effective_preset_index(scenario.preset_index),
        scenario.screen_state.button_options.width_mode,
        scenario.screen_state.button_options.height_mode,
        label_width,
        kind.has_icon(),
        kind.has_visible_label(),
    );
    (layout.width, layout.height)
}

fn button_kind_for_page(page: &str) -> ButtonLiveKind {
    match page {
        "text-button" => ButtonLiveKind::TextButton,
        "svg-button" => ButtonLiveKind::SvgButton,
        "icon-text-button" => ButtonLiveKind::IconTextButton,
        _ => ButtonLiveKind::Button,
    }
}

const fn button_label_for_kind(kind: ButtonLiveKind) -> &'static str {
    match kind {
        ButtonLiveKind::Button => "Save changes",
        ButtonLiveKind::TextButton => "Text action",
        ButtonLiveKind::SvgButton => "Svg action",
        ButtonLiveKind::IconTextButton => "Open folder",
    }
}

#[cfg(test)]
pub(super) fn effective_setting_value_for_test(
    scenario: ScenarioContext<'_>,
    control: StorybookButtonOptionControl,
) -> String {
    let facade =
        katana_ui_core::facade::UiCoreFacade::new(katana_ui_core::theme::ThemeSnapshot::dark());
    let text = TextRenderer::load(&facade, facade.default_font_role());
    effective_setting_value(&text, scenario, control)
}

#[cfg(test)]
mod tests {
    use super::{
        Canvas, LayoutRect, ROW_WIDTH, StorybookButtonOptionControl, StorybookButtonOptions,
        VisualPalette, draw_toggle,
    };
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn non_toggle_control_uses_the_disabled_switch_fallback() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let palette = VisualPalette::from_theme(facade.theme());
        let mut canvas = Canvas::new(ROW_WIDTH, 40, palette.background);

        draw_toggle(
            &mut canvas,
            &palette,
            StorybookButtonOptions::default(),
            LayoutRect::new(0, 0, ROW_WIDTH, 40),
            StorybookButtonOptionControl::AriaLabel,
        );

        assert!(
            canvas
                .pixels()
                .iter()
                .any(|pixel| *pixel != palette.background)
        );
    }
}
