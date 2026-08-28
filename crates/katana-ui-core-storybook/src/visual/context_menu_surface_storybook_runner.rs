use super::super::command_chrome_fixture::{FRAME_HEIGHT, FRAME_WIDTH};
use super::context_menu_presentation;
use super::context_menu_surface_integration_support::{
    ContextMenuEvidence, accesskit_context_event, accesskit_labels, compose_evidence, escape_event,
    item_bounds, pointer_anchor, pointer_event, pointer_from_bounds, run_combined_frame,
    run_text_frame, shift_f10_event, text_root_id,
};
use katana_ui_core::molecule::selection::{ContextMenuCloseReason, ContextMenuEvent};
use katana_ui_core_egui_adapter::context_menu::EguiContextMenuAdapter;
use katana_ui_core_egui_adapter::text_surface::EguiTextSurfaceAdapter;
use std::io;

const OUTSIDE_MARGIN_PX: f32 = 8.0;
const OUTSIDE_X: f32 = FRAME_WIDTH - OUTSIDE_MARGIN_PX;
const OUTSIDE_Y: f32 = FRAME_HEIGHT - OUTSIDE_MARGIN_PX;

pub(super) fn run_storybook_context_menu() -> Result<ContextMenuEvidence, Box<dyn std::error::Error>>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut text_adapter = EguiTextSurfaceAdapter::default();
    let mut text_surface = super::super::text_surface_fixture::text_surface_fixture();
    let mut menu_adapter = EguiContextMenuAdapter::default();
    let pointer_anchor = pointer_anchor(&context, &mut text_adapter, &mut text_surface)?;

    menu_adapter.synchronize_presentation(context_menu_presentation());
    menu_adapter.request_open(pointer_anchor);
    let initial_menu = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    );
    let initial_menu = initial_menu?;
    let record = initial_menu
        .1
        .record
        .as_ref()
        .ok_or(io::Error::other("pointer invocation did not open the menu"))?;
    let mut evidence = compose_evidence(&initial_menu, record)?;
    let text_before_typeahead = text_surface.state().text_area.value.clone();
    let typeahead_first = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![egui::Event::Text("整".to_string())],
    );
    let typeahead_first = typeahead_first?;
    let typeahead_second = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![egui::Event::Text("形".to_string())],
    );
    let typeahead_second = typeahead_second?;
    assert!(typeahead_first.1.events.iter().any(|event| {
        matches!(event, ContextMenuEvent::TypeAheadMatched { prefix, path } if prefix == "整" && path == &vec![0])
    }));
    assert!(typeahead_second.1.events.iter().any(|event| {
        matches!(event, ContextMenuEvent::TypeAheadMatched { prefix, path } if prefix == "整形" && path == &vec![0])
    }));
    let typeahead_record = typeahead_second
        .1
        .record
        .as_ref()
        .ok_or(io::Error::other("type-ahead closed the menu"))?;
    assert_eq!(vec![0], typeahead_record.highlighted_path);
    assert!(typeahead_record.focused);
    assert_eq!(text_before_typeahead, text_surface.state().text_area.value);

    let disabled = item_bounds(typeahead_record, "disabled")?;
    assert!(item_bounds(typeahead_record, "does-not-exist").is_err());
    let disabled_pressed = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(disabled, true)],
    );
    let disabled_pressed = disabled_pressed?;
    assert!(disabled_pressed.1.record.is_some());
    let disabled_release = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(disabled, false)],
    );
    let disabled_release = disabled_release?;
    assert!(disabled_release.1.events.is_empty());

    let root_record = disabled_release
        .1
        .record
        .as_ref()
        .ok_or(io::Error::other("disabled click closed the menu"))?;
    let submenu = item_bounds(root_record, "code-kind")?;
    let submenu_press = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(submenu, true)],
    );
    let _ = submenu_press?;
    let submenu_release = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![pointer_from_bounds(submenu, false)],
    );
    let submenu_release = submenu_release?;
    assert!(
        submenu_release
            .1
            .events
            .iter()
            .any(|event| { matches!(event, ContextMenuEvent::SubmenuOpened { .. }) })
    );
    let nested = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    );
    let nested = nested?;
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
        vec![shift_f10_event()],
    );
    let keyboard = keyboard?;
    let keyboard_anchor = keyboard
        .0
        .context_target
        .ok_or(io::Error::other("keyboard did not produce an anchor"))?;
    let root_id = text_root_id(&keyboard.1)?;
    let accesskit = run_text_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        vec![accesskit_context_event(root_id)],
    );
    let accesskit = accesskit?;
    let accesskit_anchor = accesskit
        .0
        .context_target
        .ok_or(io::Error::other("AccessKit did not produce an anchor"))?;

    menu_adapter.request_open(keyboard_anchor);
    let keyboard_open = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    );
    let _ = keyboard_open?;
    let escaped = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        vec![escape_event()],
    );
    let escaped = escaped?;
    assert!(escaped.1.record.is_none());
    assert!(escaped.1.events.iter().any(|event| {
        event
            == &ContextMenuEvent::Closed {
                reason: ContextMenuCloseReason::Escape,
            }
    }));
    let focus_returned = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    );
    let focus_returned = focus_returned?;
    assert!(focus_returned.0.record.frame.accessibility.root.focused);

    menu_adapter.request_open(accesskit_anchor);
    let accesskit_open = run_combined_frame(
        &context,
        &mut text_adapter,
        &mut text_surface,
        &mut menu_adapter,
        Vec::new(),
    );
    let _ = accesskit_open?;
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
    );
    let outside_press = outside_press?;
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
    );
    let outside = outside?;
    assert!(outside.1.record.is_none());
    assert!(
        outside_press
            .1
            .events
            .iter()
            .chain(&outside.1.events)
            .any(|event| {
                event
                    == &ContextMenuEvent::Closed {
                        reason: ContextMenuCloseReason::OutsideClick,
                    }
            })
    );
    Ok(evidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_storybook_context_menu_contracts_yield_stable_evidence()
    -> Result<(), Box<dyn std::error::Error>> {
        let evidence = run_storybook_context_menu()?;
        assert!(evidence.pointer_clamped);
        assert!(!evidence.composite_hash.is_empty());
        assert!(!evidence.plan_hash.is_empty());
        assert!(!evidence.frame_hash.is_empty());
        assert!(!evidence.accesskit_labels.is_empty());
        assert!(
            evidence
                .accesskit_labels
                .iter()
                .all(|label| !label.is_empty())
        );
        Ok(())
    }

    #[test]
    fn run_storybook_context_menu_records_escape_and_outside_closure_events()
    -> Result<(), Box<dyn std::error::Error>> {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut text_adapter = EguiTextSurfaceAdapter::default();
        let mut text_surface = crate::visual::text_surface_fixture::text_surface_fixture();
        let mut menu_adapter = EguiContextMenuAdapter::default();
        let pointer_anchor = pointer_anchor(&context, &mut text_adapter, &mut text_surface)?;

        menu_adapter.synchronize_presentation(context_menu_presentation());
        menu_adapter.request_open(pointer_anchor.clone());
        let opened = run_combined_frame(
            &context,
            &mut text_adapter,
            &mut text_surface,
            &mut menu_adapter,
            Vec::new(),
        );
        let _ = opened?;

        let escaped = run_combined_frame(
            &context,
            &mut text_adapter,
            &mut text_surface,
            &mut menu_adapter,
            vec![escape_event()],
        );
        let escaped = escaped?;
        assert!(escaped.1.events.iter().any(|event| {
            event
                == &ContextMenuEvent::Closed {
                    reason: ContextMenuCloseReason::Escape,
                }
        }));

        menu_adapter.request_open(pointer_anchor);
        let reopened = run_combined_frame(
            &context,
            &mut text_adapter,
            &mut text_surface,
            &mut menu_adapter,
            Vec::new(),
        );
        let _ = reopened?;

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
        );
        let outside_press = outside_press?;
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
        );
        let outside = outside?;
        assert!(
            outside_press
                .1
                .events
                .iter()
                .chain(&outside.1.events)
                .any(|event| {
                    event
                        == &ContextMenuEvent::Closed {
                            reason: ContextMenuCloseReason::OutsideClick,
                        }
                })
        );
        Ok(())
    }
}
