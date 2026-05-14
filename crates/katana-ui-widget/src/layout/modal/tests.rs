use super::*;
use crate::theme::Theme;
use floem::peniko::kurbo::Point;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn resolve_defaults_have_expected_flags() {
    let theme = Theme::default_light();
    let r = Modal::new().open(true).resolve(&theme);
    assert!(r.dismiss_on_backdrop);
    assert!(r.dismiss_on_esc);
}

#[test]
fn parent_interaction_policy_is_explicit() {
    let modal = Modal::new();
    assert_eq!(
        modal.props.parent_interaction,
        ModalParentInteraction::Block
    );

    let modal = Modal::new().parent_interaction(ModalParentInteraction::Allow);
    assert_eq!(
        modal.props.parent_interaction,
        ModalParentInteraction::Allow
    );
}

#[test]
fn native_window_level_follows_parent_interaction() {
    assert!(matches!(
        native_window::window_level_for_parent_interaction(&ModalParentInteraction::Block),
        floem::window::WindowLevel::AlwaysOnTop
    ));
    assert!(matches!(
        native_window::window_level_for_parent_interaction(&ModalParentInteraction::Allow),
        floem::window::WindowLevel::Normal
    ));
}

#[test]
fn children_footer_and_close_are_resolved() {
    let called = Rc::new(RefCell::new(false));
    let flag = Rc::clone(&called);
    let theme = Theme::default_light();
    let r = Modal::new()
        .children("Body")
        .footer("Footer")
        .on_close(move || {
            *flag.borrow_mut() = true;
        })
        .resolve(&theme);

    assert_eq!(r.children.as_deref(), Some("Body"));
    assert_eq!(r.footer.as_deref(), Some("Footer"));
    (r.on_close)();
    assert!(*called.borrow());
}

#[test]
fn open_handler_is_stored_on_props() {
    let called = Rc::new(RefCell::new(false));
    let flag = Rc::clone(&called);
    let modal = Modal::new().on_open(move || {
        *flag.borrow_mut() = true;
    });

    (modal.props.on_open)();
    assert!(*called.borrow());
}

#[test]
fn native_window_position_is_stored_on_props() {
    let position = Point::new(120.0, 240.0);
    let modal = Modal::new().window_position(position);

    assert_eq!(
        modal.props.window_placement,
        ModalWindowPlacement::At(position)
    );
}

#[test]
fn native_window_open_returns_false_when_closed() {
    let result = Modal::new().open_window(Theme::default_light());

    assert_eq!(result, Ok(false));
}

#[test]
fn native_window_open_rejects_invalid_position_before_request() {
    let result = Modal::new()
        .open(true)
        .window_position(Point::new(f64::NAN, 240.0))
        .open_window(Theme::default_light());

    assert!(matches!(
        result,
        Err(ModalOpenError::InvalidWindowPosition { .. })
    ));
}

#[test]
fn dismiss_rules_depend_on_open_state() {
    let theme = Theme::default_light();
    let closed = Modal::new().dismiss_on_esc(true).resolve(&theme);
    let open = Modal::new().open(true).dismiss_on_esc(true).resolve(&theme);
    assert!(!closed.dismiss_on_backdrop);
    assert!(!closed.dismiss_on_esc);
    assert!(open.dismiss_on_backdrop);
    assert!(open.dismiss_on_esc);
    assert!(open.trap_focus);
    assert!(!closed.trap_focus);
}

#[test]
fn dismiss_flags_respect_open_state() {
    let theme = Theme::default_light();
    let closed = Modal::new().dismiss_on_esc(true).resolve(&theme);
    assert!(!closed.dismiss_on_esc);
    let open = Modal::new().open(true).dismiss_on_esc(true).resolve(&theme);
    assert!(open.dismiss_on_esc);
}

#[test]
fn close_handlers_run_when_conditions_match() {
    let called_backdrop = Rc::new(RefCell::new(false));
    let called_esc = Rc::new(RefCell::new(false));
    let backdrop_flag = Rc::clone(&called_backdrop);

    let modal = Modal::new()
        .open(true)
        .on_close({
            let backdrop_flag = Rc::clone(&backdrop_flag);
            move || {
                *backdrop_flag.borrow_mut() = true;
            }
        })
        .resolve(&Theme::default_light());
    assert!(modal.should_close_with_backdrop());
    assert!(modal.should_close_with_esc());
    assert!(!*called_backdrop.borrow());

    assert!(modal.close_with_backdrop());
    assert!(*called_backdrop.borrow());
    assert!(!*called_esc.borrow());
}

#[test]
fn close_with_backdrop_respects_open_and_enabled_flag() {
    let called = Rc::new(RefCell::new(0u8));
    let enabled_ref = Rc::clone(&called);
    let disabled_ref = Rc::clone(&called);
    let closed_ref = Rc::clone(&called);

    let enabled = Modal::new()
        .open(true)
        .dismiss_on_backdrop(true)
        .on_close(move || {
            let mut count = enabled_ref.borrow_mut();
            *count += 1;
        })
        .resolve(&Theme::default_light());
    assert!(enabled.close_with_backdrop());

    let disabled = Modal::new()
        .open(true)
        .dismiss_on_backdrop(false)
        .on_close(move || {
            let mut count = disabled_ref.borrow_mut();
            *count += 1;
        })
        .resolve(&Theme::default_light());
    assert!(!disabled.close_with_backdrop());

    let closed = Modal::new()
        .dismiss_on_backdrop(true)
        .on_close(move || {
            let mut count = closed_ref.borrow_mut();
            *count += 1;
        })
        .resolve(&Theme::default_light());
    assert!(!closed.close_with_backdrop());

    assert_eq!(*called.borrow(), 1);
}

#[test]
fn close_with_esc_respects_open_and_enabled_flag() {
    let called = Rc::new(RefCell::new(0u8));
    let enabled_ref = Rc::clone(&called);
    let disabled_ref = Rc::clone(&called);
    let closed_ref = Rc::clone(&called);

    let enabled = Modal::new()
        .open(true)
        .dismiss_on_esc(true)
        .on_close(move || {
            let mut count = enabled_ref.borrow_mut();
            *count += 1;
        })
        .resolve(&Theme::default_light());
    assert!(enabled.close_with_esc());

    let disabled = Modal::new()
        .open(true)
        .dismiss_on_esc(false)
        .on_close(move || {
            let mut count = disabled_ref.borrow_mut();
            *count += 1;
        })
        .resolve(&Theme::default_light());
    assert!(!disabled.close_with_esc());

    let closed = Modal::new()
        .dismiss_on_esc(true)
        .on_close(move || {
            let mut count = closed_ref.borrow_mut();
            *count += 1;
        })
        .resolve(&Theme::default_light());
    assert!(!closed.close_with_esc());

    assert_eq!(*called.borrow(), 1);
}

#[test]
fn focus_trap_is_tied_to_open_state() {
    let open = Modal::new().open(true).resolve(&Theme::default_light());
    let closed = Modal::new().resolve(&Theme::default_light());

    assert!(open.trap_focus);
    assert!(!closed.trap_focus);
    assert_eq!(open.focus_on_open, FocusTransition::EnterDialog);
    assert_eq!(open.focus_on_close, FocusTransition::ReturnToTrigger);
    assert_eq!(closed.focus_on_open, FocusTransition::None);
    assert!(!open.focus_returns_to_trigger());
    assert!(closed.focus_returns_to_trigger());
}

#[test]
fn close_returns_focus_after_dismiss() {
    let close_called = Rc::new(RefCell::new(false));
    let focus_called = Rc::new(RefCell::new(false));
    let close_ref = Rc::clone(&close_called);
    let focus_ref = Rc::clone(&focus_called);

    let modal = Modal::new()
        .open(true)
        .on_close(move || {
            *close_ref.borrow_mut() = true;
        })
        .on_focus_return(move || {
            *focus_ref.borrow_mut() = true;
        })
        .resolve(&Theme::default_light());

    assert!(modal.close_with_esc());
    assert!(*close_called.borrow());
    assert!(*focus_called.borrow());
}

#[test]
fn focus_trap_and_focus_return_rules_are_testable() {
    let closed = Modal::new();
    let open = Modal::new().open(true);

    assert!(!ops::should_trap_tab_navigation(&closed.props));
    assert!(ops::should_trap_tab_navigation(&open.props));
    assert!(!ops::should_return_focus_after_close(
        &closed.props,
        ops::DismissReason::Escape
    ));
    assert!(ops::should_return_focus_after_close(
        &open.props,
        ops::DismissReason::Escape
    ));
}

#[test]
fn esc_close_ignores_when_not_open() {
    let called = Rc::new(RefCell::new(0u8));
    let called_ref = Rc::clone(&called);
    let modal = Modal::new().on_close(move || {
        let mut count = called_ref.borrow_mut();
        *count += 1;
    });
    let resolved = modal.resolve(&Theme::default_light());
    assert!(!resolved.close_with_esc());
    assert_eq!(*called.borrow(), 0);
}
