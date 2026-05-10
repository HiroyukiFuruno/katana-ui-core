use super::*;
use crate::theme::Theme;
use std::cell::RefCell;

const ANCHOR_X: f32 = 200.0;
const ANCHOR_Y: f32 = 200.0;
const ANCHOR_WIDTH: f32 = 100.0;
const ANCHOR_HEIGHT: f32 = 40.0;

fn anchor() -> AnchorRect {
    AnchorRect {
        x: ANCHOR_X,
        y: ANCHOR_Y,
        width: ANCHOR_WIDTH,
        height: ANCHOR_HEIGHT,
    }
}

#[test]
fn dismiss_defaults_true() {
    let theme = Theme::default_light();
    let r = Popover::new().open(true).resolve(&theme);
    assert!(r.dismiss_on_outside_click);
    assert!(r.dismiss_on_esc);
}

#[test]
fn dismiss_can_be_disabled() {
    let theme = Theme::default_light();
    let r = Popover::new()
        .dismiss_on_outside_click(false)
        .dismiss_on_esc(false)
        .resolve(&theme);
    assert!(!r.dismiss_on_outside_click);
    assert!(!r.dismiss_on_esc);
}

#[test]
fn default_placement_is_bottom() {
    let theme = Theme::default_light();
    let r = Popover::new().resolve(&theme);
    assert_eq!(r.placement, Placement::Bottom);
}

#[test]
fn placement_set_correctly() {
    let theme = Theme::default_light();
    let r = Popover::new().placement(Placement::Top).resolve(&theme);
    assert_eq!(r.placement, Placement::Top);
}

#[test]
fn compute_origin_bottom() {
    let p = Popover::new().offset(4.0);
    let o = p.compute_origin(anchor(), 120.0, 60.0, 800.0, 600.0);
    assert!((o.y - (200.0 + 40.0 + 4.0)).abs() < f32::EPSILON);
}

#[test]
fn light_and_dark_both_resolve() {
    let light = Popover::new().resolve(&Theme::default_light());
    let dark = Popover::new().resolve(&Theme::default_dark());
    assert_ne!(light.popover_bg.r, dark.popover_bg.r);
}

#[test]
fn close_if_needed_considers_open_state() {
    let called = Rc::new(RefCell::new(0u8));
    let called_1 = Rc::clone(&called);
    let called_2 = Rc::clone(&called);

    let open_pop = Popover::new().open(true).on_close(move || {
        let mut value = called_1.borrow_mut();
        *value += 1;
    });
    let open_theme = Theme::default_light();
    let open_pop_resolved = open_pop.resolve(&open_theme);
    assert!(open_pop_resolved.dismiss_on_outside_click);
    assert!(open_pop_resolved.dismiss_on_esc);
    assert!(open_pop_resolved.close_with_outside_click());
    assert_eq!(*called.borrow(), 1);
    assert!(open_pop_resolved.close_with_esc());
    assert_eq!(*called.borrow(), 2);

    let closed_pop = Popover::new().open(false).on_close(move || {
        let mut value = called_2.borrow_mut();
        *value += 1;
    });
    let closed_pop_resolved = closed_pop.resolve(&Theme::default_light());
    assert!(!closed_pop_resolved.dismiss_on_outside_click);
    assert!(!closed_pop_resolved.dismiss_on_esc);
    assert!(!closed_pop_resolved.close_with_outside_click());
    assert!(!closed_pop_resolved.close_with_esc());
    assert_eq!(*called.borrow(), 2);
}

#[test]
fn close_if_needed_respects_flag_disable() {
    let called = Rc::new(RefCell::new(0u8));
    let called_ref = Rc::clone(&called);
    let called_theme = Theme::default_light();
    let pop = Popover::new()
        .open(true)
        .dismiss_on_outside_click(false)
        .dismiss_on_esc(false)
        .on_close(move || {
            let mut value = called_ref.borrow_mut();
            *value += 1;
        });
    let pop_resolved = pop.resolve(&called_theme);
    assert!(!pop_resolved.dismiss_on_outside_click);
    assert!(!pop_resolved.dismiss_on_esc);
    assert!(!pop_resolved.close_with_outside_click());
    assert!(!pop_resolved.close_with_esc());
    assert_eq!(*called.borrow(), 0);
}

#[test]
fn overlay_layout_is_none_without_anchor() {
    let theme = Theme::default_light();
    let resolved = Popover::new().open(true).resolve(&theme);
    assert!(resolved.anchor.is_none());
    assert!(resolved.overlay_layout(120.0, 80.0, 800.0, 600.0).is_none());
}

#[test]
fn overlay_layout_is_none_when_closed() {
    let theme = Theme::default_light();
    let resolved = Popover::new()
        .anchor(AnchorRef::new(anchor()))
        .open(false)
        .resolve(&theme);
    assert!(resolved.anchor.is_some());
    assert!(resolved.overlay_layout(120.0, 80.0, 800.0, 600.0).is_none());
}
