use super::dedicated_dod_atom_button_live_surface::{
    centered_label_x_for_test, measure_button_label_width,
};
use super::dedicated_dod_common::Rect;
use super::render_context::ScenarioContext;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::ThemeSnapshot;

const BUTTON_PAGE: &str = "button";
const DEFAULT_PRESET: usize = 0;

#[test]
fn button_label_center_uses_measured_text_width() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let text = super::text::TextRenderer::load(&facade, facade.default_font_role());
    let scenario = button_scenario();
    let rect = Rect::new(40, 20, 160, 36);
    let narrow_width = measure_button_label_width(&text, "iii");
    let wide_width = measure_button_label_width(&text, "WWW");

    assert!(wide_width > narrow_width);
    assert_eq!("iii".chars().count(), "WWW".chars().count());
    assert!(
        centered_label_x_for_test(rect, wide_width, scenario, false)
            < centered_label_x_for_test(rect, narrow_width, scenario, false)
    );
}

fn button_scenario() -> ScenarioContext<'static> {
    let screen_state = Box::leak(Box::new(
        super::screen_state::StorybookScreenState::default(),
    ));
    ScenarioContext {
        selected_page: BUTTON_PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: DEFAULT_PRESET,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state,
    }
}
