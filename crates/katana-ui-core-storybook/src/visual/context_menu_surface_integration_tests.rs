use super::super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};
use super::context_menu_presentation;
use super::context_menu_surface_integration_support::{
    ContextMenuEvidence, accesskit_labels, compose_evidence, escape_event, item_bounds,
    pointer_anchor, pointer_event, pointer_from_bounds, run_combined_frame, run_text_frame,
    text_root_id,
};
use katana_ui_core::molecule::selection::{ContextMenuCloseReason, ContextMenuEvent};
use katana_ui_core_egui_adapter::context_menu::EguiContextMenuAdapter;
use katana_ui_core_egui_adapter::text_surface::EguiTextSurfaceAdapter;
use katana_ui_core_text_raster::PlatformTextRasterConfig;
use std::io;

const OUTSIDE_MARGIN_PX: f32 = 8.0;
const OUTSIDE_X: f32 = FRAME_WIDTH - OUTSIDE_MARGIN_PX;
const OUTSIDE_Y: f32 = FRAME_HEIGHT - OUTSIDE_MARGIN_PX;

#[test]
fn actual_egui_context_menu_storybook_integration_is_repeatable()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_storybook_context_menu()?;
    let second = run_storybook_context_menu()?;
    assert_eq!(first, second);
    assert!(first.pointer_clamped);
    assert!(first.colored_star_texture);
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("整形 ⭐️"))
    );
    assert!(
        first
            .accesskit_labels
            .iter()
            .any(|label| label.contains("opaque code kind"))
    );
    Ok(())
}

fn run_storybook_context_menu() -> Result<ContextMenuEvidence, Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut text_adapter = EguiTextSurfaceAdapter::default();
    let mut text_surface = super::super::text_surface_fixture::text_surface_fixture();
    let mut menu_adapter = EguiContextMenuAdapter::new(PlatformTextRasterConfig::default())?;
    let pointer_anchor = pointer_anchor(&context, &mut text_adapter, &mut text_surface)?;

    menu_adapter.synchronize_presentation(context_menu_presentation());
    menu_adapter.request_open(pointer_anchor);
    let initial_menu = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    )?;
    let record = initial_menu
        .1
        .record
        .as_ref()
        .ok_or_else(|| io::Error::other("pointer invocation did not open actual menu"))?;
    let mut evidence = compose_evidence(&initial_menu, record)?;
    let text_before_typeahead = text_surface.state().text_area.value.clone();
    let typeahead_first = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![egui::Event::Text("整".to_string())],
    )?;
    let typeahead_second = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![egui::Event::Text("形".to_string())],
    )?;
    assert!(typeahead_first.1.events.iter().any(|event| {
        matches!(
            event,
            ContextMenuEvent::TypeAheadMatched { prefix, path }
                if prefix == "整" && path == &vec![0]
        )
    }));
    assert!(typeahead_second.1.events.iter().any(|event| {
        matches!(
            event,
            ContextMenuEvent::TypeAheadMatched { prefix, path }
                if prefix == "整形" && path == &vec![0]
        )
    }));
    let typeahead_record = typeahead_second
        .1
        .record
        .as_ref()
        .ok_or_else(|| io::Error::other("type-ahead closed the menu"))?;
    assert_eq!(vec![0], typeahead_record.highlighted_path);
    assert!(typeahead_record.focused);
    assert_eq!(text_before_typeahead, text_surface.state().text_area.value);

    let disabled = item_bounds(typeahead_record, "disabled")?;
    let disabled_pressed = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(disabled, true)],
    )?;
    assert!(disabled_pressed.1.record.is_some());
    let disabled_release = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(disabled, false)],
    )?;
    assert!(!disabled_release.1.events.iter().any(|event| {
        matches!(event, ContextMenuEvent::ItemSelected { command, .. } if command == "disabled")
    }));

    let root_record = disabled_release
        .1
        .record
        .as_ref()
        .ok_or_else(|| io::Error::other("disabled click closed the menu"))?;
    let submenu = item_bounds(root_record, "code-kind")?;
    let _ = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(submenu, true)],
    )?;
    let submenu_release = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(submenu, false)],
    )?;
    assert!(
        submenu_release
            .1
            .events
            .iter()
            .any(|event| matches!(event, ContextMenuEvent::SubmenuOpened { .. }))
    );
    let nested = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    )?;
    assert!(nested.1.record.as_ref().is_some_and(|record| {
        record
            .items
            .iter()
            .any(|item| item.id == "opaque-code-kind")
    }));
    evidence
        .accesskit_labels
        .extend(accesskit_labels(&nested.2));
    evidence.accesskit_labels.sort();
    evidence.accesskit_labels.dedup();

    let keyboard = run_text_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        vec![super::context_menu_surface_integration_support::shift_f10_event()],
    )?;
    let keyboard_anchor = keyboard
        .0
        .context_target
        .ok_or_else(|| io::Error::other("keyboard did not produce an anchor"))?;
    let root_id = text_root_id(&keyboard.1)?;
    let accesskit = run_text_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        vec![super::context_menu_surface_integration_support::accesskit_context_event(root_id)],
    )?;
    let accesskit_anchor = accesskit
        .0
        .context_target
        .ok_or_else(|| io::Error::other("AccessKit did not produce an anchor"))?;

    menu_adapter.request_open(keyboard_anchor);
    let _ = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    )?;
    let escaped = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![escape_event()],
    )?;
    assert!(escaped.1.record.is_none());
    assert!(escaped.1.events.iter().any(|event| {
        matches!(
            event,
            ContextMenuEvent::Closed {
                reason: ContextMenuCloseReason::Escape
            }
        )
    }));
    let focus_returned = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    )?;
    assert!(focus_returned.0.record.frame.accessibility.root.focused);

    menu_adapter.request_open(accesskit_anchor);
    let _ = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    )?;
    let outside_press = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_event(
            OUTSIDE_X,
            OUTSIDE_Y,
            egui::PointerButton::Primary,
            true,
        )],
    )?;
    let outside = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_event(
            OUTSIDE_X,
            OUTSIDE_Y,
            egui::PointerButton::Primary,
            false,
        )],
    )?;
    assert!(outside.1.record.is_none());
    assert!(
        outside_press
            .1
            .events
            .iter()
            .chain(&outside.1.events)
            .any(|event| {
                matches!(
                    event,
                    ContextMenuEvent::Closed {
                        reason: ContextMenuCloseReason::OutsideClick
                    }
                )
            },)
    );
    Ok(evidence)
}
