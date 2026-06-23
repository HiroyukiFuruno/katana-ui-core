use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{Accordion, DisclosureTriggerArea, Modal, Popover, Tooltip};

const TOOLTIP_DELAY_MS: u16 = 320;
const TOOLTIP_MAX_WIDTH: u16 = 240;

#[test]
fn tooltip_tracks_delay_width_and_hover_focus_triggers() {
    let mut tooltip = Tooltip::new("Hint")
        .placement("top")
        .delay_ms(TOOLTIP_DELAY_MS)
        .max_width(TOOLTIP_MAX_WIDTH)
        .hover_trigger(true)
        .focus_trigger(true)
        .timer_summary("delay pending");

    let hover = tooltip.apply_action(&UiAction::hover(tooltip.state_id().clone(), true));
    let blur = tooltip.apply_action(&UiAction::blur(tooltip.state_id().clone()));

    assert!(hover.handled);
    assert!(hover.after.open);
    assert!(blur.handled);
    assert!(!blur.after.open);
    assert_eq!(TOOLTIP_DELAY_MS, tooltip.delay_ms_model());
    assert_eq!(TOOLTIP_MAX_WIDTH, tooltip.max_width_model());
    assert!(tooltip.opens_on_hover());
    assert!(tooltip.opens_on_focus());
    assert_eq!("delay pending", tooltip.timer_summary_model());
}

#[test]
fn accordion_toggles_and_keeps_tree_disclosure_options() {
    let mut accordion = Accordion::new("Section")
        .trigger_area(DisclosureTriggerArea::WholeElement)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .tree_mode(true)
        .reduced_motion(true)
        .body_border(true)
        .selected(true)
        .depth(2)
        .show_lines(true);

    let result = accordion.apply_action(&UiAction::accordion_toggle(accordion.state_id().clone()));

    assert!(result.handled);
    assert!(result.after.open);
    assert_eq!(
        DisclosureTriggerArea::WholeElement,
        accordion.trigger_area_model()
    );
    assert!(accordion.uses_reduced_motion());
    assert!(accordion.has_body_border());
    assert!(accordion.is_selected());
    assert_eq!(2, accordion.depth_model());
    assert!(accordion.shows_lines());
}

#[test]
fn modal_and_popover_keep_dismiss_focus_and_native_options() {
    let modal = Modal::new("Dialog")
        .title("Settings")
        .panel_size("medium")
        .footer("Cancel / Save")
        .native_window_mode(true)
        .focus_return("settings-button");
    let mut popover = Popover::new("Actions")
        .open(true)
        .placement("bottom-start")
        .width("320px")
        .focus_handling("return-to-anchor")
        .escape_dismiss(true);

    let result = popover.apply_action(&UiAction::modal_escape(popover.state_id().clone()));

    assert_eq!("Settings", modal.title_model());
    assert_eq!("medium", modal.panel_size_model());
    assert_eq!("Cancel / Save", modal.footer_model());
    assert!(modal.uses_native_window_mode());
    assert_eq!("settings-button", modal.focus_return_model());
    assert!(result.handled);
    assert!(!result.after.open);
    assert_eq!("bottom-start", popover.placement_model());
    assert_eq!("320px", popover.width_model());
    assert_eq!("return-to-anchor", popover.focus_handling_model());
}
