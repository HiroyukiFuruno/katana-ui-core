use super::preview_detail;
use super::render;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{self, StorybookWindowState, TextAreaKey, apply_click};

const PAGE: &str = "text-area";
const RESIZE_PRESET_INDEX: usize = 3;
const VERTICAL_SCROLL_PRESET_INDEX: usize = 5;
const BODY_DIFF_THRESHOLD: usize = 80;

#[test]
fn text_area_state_store_keeps_instance_value_focus_and_caret_isolated() {
    let mut screen_state = StorybookScreenState::default();

    {
        let primary = screen_state.text_areas.runtime_mut("text-area.primary");
        primary.value = "primary draft".to_string();
        primary.focused = true;
        primary.caret_visible = true;
    }
    {
        let secondary = screen_state.text_areas.runtime_mut("text-area.secondary");
        secondary.value = "secondary note".to_string();
        secondary.focused = false;
        secondary.caret_visible = false;
    }

    let primary = screen_state.text_areas.runtime("text-area.primary");
    let secondary = screen_state.text_areas.runtime("text-area.secondary");
    let preview = screen_state.text_area_runtime();

    assert_eq!("primary draft", primary.value());
    assert_eq!("secondary note", secondary.value());
    assert_ne!(primary.value(), secondary.value());
    assert!(primary.focused());
    assert!(!secondary.focused());
    assert!(primary.caret_visible());
    assert!(!secondary.caret_visible());
    assert_ne!(primary.value(), preview.value());
    assert!(!preview.focused());
    assert!(!preview.caret_visible());
}

#[test]
fn text_area_keyboard_routes_to_selected_instance_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let field = text_area_field_rect(&state);

    state.select_instance("text-area.primary");
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('1')
    ));
    let primary = state
        .screen_state
        .text_area_value_for("text-area.primary")
        .to_string();

    state.select_instance("text-area.secondary");
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('2')
    ));
    let secondary = state
        .screen_state
        .text_area_value_for("text-area.secondary")
        .to_string();

    state.select_instance("text-area.primary");
    assert_eq!(
        primary,
        state.screen_state.text_area_value_for("text-area.primary")
    );
    assert!(primary.ends_with('1'));

    state.select_instance("text-area.secondary");
    assert_eq!(
        secondary,
        state
            .screen_state
            .text_area_value_for("text-area.secondary")
    );
    assert!(
        state
            .screen_state
            .text_area_value_for("text-area.secondary")
            .ends_with('2')
    );
    assert_ne!(primary, secondary);
}

#[test]
fn text_area_scroll_render_uses_selected_instance_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: VERTICAL_SCROLL_PRESET_INDEX,
        selected_instance_id: "text-area.secondary",
        ..StorybookWindowState::default()
    };
    let field = text_area_field_rect_for_instance(&state, "text-area.secondary");
    let before = render_with_instance(&state, "text-area.secondary");

    assert!(window_interaction::apply_scroll_delta_at_for_test(
        &mut state,
        field.x + 1,
        field.y + 1,
        1.0
    ));
    assert_eq!(
        1,
        state
            .screen_state
            .text_area_scroll_offset_for("text-area.secondary")
    );
    assert_eq!(0, state.screen_state.text_area_scroll_offset());

    let selected_after = render_with_instance(&state, "text-area.secondary");
    let default_after = render_with_instance(&state, "text-area.preview");

    assert!(component_body_pixel_diff(PAGE, &before, &selected_after) > BODY_DIFF_THRESHOLD);
    assert_ne!(
        0,
        component_body_pixel_diff(PAGE, &selected_after, &default_after)
    );
}

#[test]
fn text_area_resize_render_and_hit_rect_use_selected_instance_state() -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: RESIZE_PRESET_INDEX,
        selected_instance_id: "text-area.secondary",
        ..StorybookWindowState::default()
    };
    let field = text_area_field_rect_for_instance(&state, "text-area.secondary");
    let before = render_with_instance(&state, "text-area.secondary");

    assert!(window_interaction::apply_text_area_resize_drag_at_for_test(
        &mut state,
        field.right() + 36,
        field.bottom() + 8
    ));
    assert!(
        state
            .screen_state
            .text_area_resize_width_delta_for("text-area.secondary")
            > 0
    );
    assert!(
        state
            .screen_state
            .text_area_resize_height_delta_for("text-area.secondary")
            > 0
    );
    assert_eq!(0, state.screen_state.text_area_resize_width_delta());
    assert_eq!(0, state.screen_state.text_area_resize_height_delta());

    let selected_after = render_with_instance(&state, "text-area.secondary");
    let default_after = render_with_instance(&state, "text-area.preview");
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let selected_grip =
        super::dedicated_dod_form_input_live::text_area_resize_grip_rect_for_instance(
            origin.x,
            origin.y,
            RESIZE_PRESET_INDEX,
            &state.screen_state,
            "text-area.secondary",
        )
        .ok_or_else(|| "selected text area resize grip is missing".to_string())?;
    let default_grip =
        super::dedicated_dod_form_input_live::text_area_resize_grip_rect_for_instance(
            origin.x,
            origin.y,
            RESIZE_PRESET_INDEX,
            &state.screen_state,
            "text-area.preview",
        )
        .ok_or_else(|| "default text area resize grip is missing".to_string())?;

    assert!(selected_grip.x > default_grip.x);
    assert!(selected_grip.y > default_grip.y);
    assert!(component_body_pixel_diff(PAGE, &before, &selected_after) > BODY_DIFF_THRESHOLD);
    assert_ne!(
        0,
        component_body_pixel_diff(PAGE, &selected_after, &default_after)
    );
    Ok(())
}

fn text_area_field_rect(state: &StorybookWindowState) -> super::layout_metrics::LayoutRect {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_rect_for_screen_state(
        origin.x,
        origin.y,
        &state.screen_state,
    )
}

fn text_area_field_rect_for_instance(
    state: &StorybookWindowState,
    instance: &'static str,
) -> super::layout_metrics::LayoutRect {
    let origin = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_rect_for_screen_state_instance(
        origin.x,
        origin.y,
        &state.screen_state,
        instance,
    )
}

fn render_with_instance(state: &StorybookWindowState, instance: &'static str) -> super::Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: state.theme_id,
        selected_page: state.selected_page,
        selected_instance_id: instance,
        preset_index: state.preset_index,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        tree_expansion: Default::default(),
        screen_state: state.screen_state.clone(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    })
}
