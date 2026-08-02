use super::button_options::{StorybookButtonHeightMode, StorybookButtonWidthMode};
use super::canvas::Canvas;
use super::dedicated::{self, DedicatedPageRequest};
use super::dedicated_breadcrumb;
use super::dedicated_breadcrumb_style;
use super::dedicated_card_style;
use super::dedicated_diagnostics_list_style;
use super::dedicated_dod_atom_button_live_surface;
use super::dedicated_dod_form_input_live;
use super::dedicated_status_bar;
use super::dedicated_tooltip;
use super::inspector;
use super::layout_metrics::LayoutRect;
use super::legacy_01_24_expected_kind;
use super::palette::VisualPalette;
use super::preset_tab_label;
use super::preview;
use super::preview_detail;
use super::preview_effects;
use super::render_context::{RenderContext, ScenarioContext};
use super::screen_state::StorybookScreenState;
use super::text::TextRenderer;
use super::window_interaction::diagnostics_list_operation::DiagnosticsListStoryAction;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn dedicated_fallback_and_geometry_boundaries_are_total() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let text = TextRenderer::load(&facade, facade.default_font_role());
    let palette = VisualPalette::from_theme(facade.theme());
    let mut canvas = Canvas::new(320, 240, palette.background);
    let node = UiNode::new(UiNodeKind::Text, "fallback");
    let state = StorybookScreenState::default();
    let scenario = ScenarioContext::for_test("unknown", usize::MAX, &state);

    dedicated::draw_page(
        &mut canvas,
        DedicatedPageRequest {
            text: &text,
            page: "unknown",
            node: &node,
            palette: &palette,
            scenario,
            x: 0,
            y: 0,
        },
    );
    assert!(
        canvas
            .pixels()
            .iter()
            .any(|pixel| *pixel != palette.background)
    );
    assert_eq!(
        dedicated_breadcrumb::file_crumb_rect(0, 0),
        dedicated_breadcrumb::crumb_rect_from_origin(0, 0, usize::MAX)
    );
    assert_eq!("state=ready", dedicated_card_style::state_label(scenario));
    assert!(
        dedicated_dod_form_input_live::text_area_resize_grip_rect_for_instance(
            0, 0, 0, &state, "primary",
        )
        .is_none()
    );
}

#[test]
fn dedicated_state_labels_cover_idle_routes_and_selected_diagnostics() {
    let mut state = StorybookScreenState {
        breadcrumb_selected_index: 0,
        ..StorybookScreenState::default()
    };
    assert_eq!(
        "route=0",
        dedicated_breadcrumb_style::state_label(
            ScenarioContext::for_test("breadcrumb", 0, &state,)
        )
    );
    state.breadcrumb_selected_index = 1;
    assert_eq!(
        "route=1",
        dedicated_breadcrumb_style::state_label(
            ScenarioContext::for_test("breadcrumb", 0, &state,)
        )
    );

    state.register_diagnostics_list_action(DiagnosticsListStoryAction::SelectItem);
    let scenario = ScenarioContext::for_test("diagnostics-list", 0, &state);
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let palette = VisualPalette::from_theme(facade.theme());
    assert_ne!(
        palette.surface,
        dedicated_diagnostics_list_style::row_fill(&palette, scenario, 0)
    );
}

#[test]
fn button_layout_covers_explicit_percent_and_fill_widths() {
    let percent = dedicated_dod_atom_button_live_surface::button_layout(
        0,
        StorybookButtonWidthMode::Percent,
        StorybookButtonHeightMode::Auto,
        20,
        false,
        true,
    );
    let fill = dedicated_dod_atom_button_live_surface::button_layout(
        0,
        StorybookButtonWidthMode::Fill,
        StorybookButtonHeightMode::Auto,
        20,
        false,
        true,
    );

    assert_ne!(percent.width, fill.width);
}

#[test]
fn bounded_control_geometry_rejects_invalid_indexes_and_uses_edge_tooltips() {
    assert!(dedicated_status_bar::segment_rect_for_test(usize::MAX).is_none());
    assert_ne!(
        dedicated_tooltip::anchor_hit_rect(0).x,
        dedicated_tooltip::anchor_hit_rect(2).x
    );
}

#[test]
fn empty_render_context_and_summary_bounds_have_defined_fallbacks() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let text = TextRenderer::load(&facade, facade.default_font_role());
    let palette = VisualPalette::from_theme(facade.theme());
    let render = RenderContext {
        text: &text,
        code_text: &text,
        examples: &[],
        palette: &palette,
    };
    let mut canvas = Canvas::new(1_600, 1_000, palette.background);
    let node = UiNode::new(UiNodeKind::Text, "empty");
    let state = StorybookScreenState::default();
    let scenario = ScenarioContext::for_test("unknown", 0, &state);

    inspector::draw(&mut canvas, render, None, scenario);
    preview_detail::draw_selected_hero(&mut canvas, render, &node, scenario);
    preview_effects::draw(&mut canvas, render, scenario, LayoutRect::new(0, 0, 0, 1));
    assert_eq!(
        UiNodeKind::Text,
        legacy_01_24_expected_kind::expected_kind("unknown")
    );
    assert_eq!(
        (0, 0),
        preset_tab_label::measured_width_for_test(&text, LayoutRect::new(0, 0, 0, 1), "too wide")
    );

    let mut hovered = StorybookScreenState {
        hovered_summary_index: Some(usize::MAX),
        ..StorybookScreenState::default()
    };
    preview::draw_overlay(
        &mut canvas,
        render,
        ScenarioContext::for_test("button", 0, &hovered),
    );
    hovered.hovered_summary_index = Some(0);
    preview::draw_overlay(
        &mut canvas,
        render,
        ScenarioContext::for_test("button", 0, &hovered),
    );

    assert!(
        canvas
            .pixels()
            .iter()
            .any(|pixel| *pixel != palette.background)
    );
}
