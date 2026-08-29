use super::StorybookWindowState;
use super::button_operation::{StorybookButtonOperation, button_operation_at};
use super::clickable_operation;
use super::collapsible_panel_state::{CollapsiblePanelScreenState, CollapsiblePanelStoryAction};
use super::color_picker_state::ColorPickerScreenState;
use super::command_palette_state::CommandPaletteScreenState;
use super::context_click::apply_context_click;
use super::diagnostics_list_option_state::DiagnosticsListOptionState;
use super::diagnostics_list_state::DiagnosticsListScreenState;
use super::runtime_structured_state::RuntimeStructuredScreenState;
use super::tabs_keyboard::apply_tabs_keyboard_shortcut;
use super::text_area_resize;
use crate::visual::{layout_metrics, preview_detail};
use katana_ui_core::widget::molecules::{CloseableTabKey, CloseableTabKeyboardShortcut};

#[test]
fn unknown_runtime_options_are_noops() {
    let mut color = ColorPickerScreenState::default();
    let color_before = color.clone();
    color.apply_option("unknown");
    assert_eq!(color_before, color);

    let mut command = CommandPaletteScreenState::default();
    let command_before = command.clone();
    command.apply_option("unknown");
    assert_eq!(command_before, command);

    let mut diagnostic_options = DiagnosticsListOptionState::default();
    let diagnostic_options_before = diagnostic_options;
    diagnostic_options.apply("unknown");
    assert_eq!(diagnostic_options_before, diagnostic_options);

    let mut diagnostics = DiagnosticsListScreenState::default();
    let diagnostics_before = diagnostics.clone();
    diagnostics.apply_option("unknown");
    assert_eq!(diagnostics_before, diagnostics);

    let mut structured = RuntimeStructuredScreenState::default();
    let structured_before = structured.clone();
    structured.apply_option("unknown", "unknown");
    assert_eq!(structured_before, structured);
}

#[test]
fn interaction_boundaries_cover_noop_and_dismiss_paths() {
    let mut collapsible = CollapsiblePanelScreenState::default();
    let update = collapsible.apply(CollapsiblePanelStoryAction::ContextPinToggle);
    assert_eq!("collapsible_panel_pin_changed", update.event);
    let repeated = collapsible.apply(CollapsiblePanelStoryAction::ContextPinToggle);
    assert_eq!("collapsible_panel_context_opened", repeated.event);

    let mut unknown = StorybookWindowState {
        selected_page: "unknown",
        ..StorybookWindowState::default()
    };
    assert!(!clickable_operation::keyboard_activate(&mut unknown));
    assert!(!text_area_resize::apply_drag_at(&mut unknown, 0, 0));

    let mut tabs = StorybookWindowState {
        selected_page: "tabs",
        ..StorybookWindowState::default()
    };
    assert!(!apply_tabs_keyboard_shortcut(
        &mut tabs,
        CloseableTabKeyboardShortcut::new(CloseableTabKey::Escape, false, false),
    ));

    let mut context_menu = StorybookWindowState {
        selected_page: "context-menu",
        ..StorybookWindowState::default()
    };
    let component = preview_detail::component_action_hit_rect("context-menu");
    assert!(apply_context_click(
        &mut context_menu,
        component.x + 1,
        component.y + 1
    ));
    assert!(apply_context_click(&mut context_menu, 0, 0));
}

#[test]
fn dark_theme_button_operation_uses_the_declared_hit_rect() {
    let state = StorybookWindowState::default();
    let rect = layout_metrics::dark_theme_rect();

    assert!(matches!(
        button_operation_at(&state, rect.x + 1, rect.y + 1),
        Some(StorybookButtonOperation::DarkTheme)
    ));
}
