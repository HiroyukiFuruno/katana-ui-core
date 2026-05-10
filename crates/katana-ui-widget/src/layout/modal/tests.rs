use super::*;
use crate::theme::Theme;
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
