use floem::keyboard::{Key, NamedKey};
use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
};

use super::contract::{MenuButtonInteractionState, MenuButtonTransition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CloseIntent {
    Close,
    KeepOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MenuButtonId(u64);

struct MenuCloseHandler {
    id: MenuButtonId,
    close: Weak<dyn Fn()>,
}

thread_local! {
    static NEXT_MENU_ID: Cell<u64> = const { Cell::new(1) };
    static ACTIVE_MENU_ID: Cell<Option<MenuButtonId>> = const { Cell::new(None) };
    static CLOSE_HANDLERS: RefCell<Vec<MenuCloseHandler>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn close_intent_for_escape(is_open: bool) -> CloseIntent {
    let mut state = state_for(is_open);
    close_intent_from_transition(state.escape_key())
}

pub(super) fn close_intent_for_key(key: &Key, is_open: bool) -> CloseIntent {
    if should_close_on_key(key) {
        return close_intent_for_escape(is_open);
    }
    CloseIntent::KeepOpen
}

pub(super) fn close_intent_for_trigger_press(is_open: bool) -> CloseIntent {
    let mut state = state_for(is_open);
    close_intent_from_transition(state.trigger_press())
}

pub(super) fn close_intent_for_outside_pointer(is_open: bool) -> CloseIntent {
    let mut state = state_for(is_open);
    close_intent_from_transition(state.outside_pointer())
}

pub(super) fn next_menu_id() -> MenuButtonId {
    NEXT_MENU_ID.with(|next_id| {
        let id = next_id.get();
        next_id.set(id.wrapping_add(1));
        MenuButtonId(id)
    })
}

pub(super) fn register_menu(id: MenuButtonId, close: &Rc<dyn Fn()>) {
    CLOSE_HANDLERS.with(|handlers| {
        let mut handlers = handlers.borrow_mut();
        handlers.retain(|handler| handler.id != id && handler.close.strong_count() > 0);
        handlers.push(MenuCloseHandler {
            id,
            close: Rc::downgrade(close),
        });
    });
}

pub(super) fn unregister_menu(id: MenuButtonId) {
    CLOSE_HANDLERS.with(|handlers| {
        handlers
            .borrow_mut()
            .retain(|handler| handler.id != id && handler.close.strong_count() > 0);
    });
    deactivate_menu(id);
}

pub(super) fn activate_menu(id: MenuButtonId) {
    let previously_active = ACTIVE_MENU_ID.with(|active| active.get());
    if previously_active == Some(id) {
        return;
    }

    if let Some(previous_id) = previously_active
        && let Some(close) = close_handler(previous_id)
    {
        close();
    }

    ACTIVE_MENU_ID.with(|active| active.set(Some(id)));
}

pub(super) fn deactivate_menu(id: MenuButtonId) {
    ACTIVE_MENU_ID.with(|active| {
        if active.get() == Some(id) {
            active.set(None);
        }
    });
}

pub(super) fn should_close_on_outside_pointer() -> bool {
    true
}

pub(super) fn should_close_on_key(key: &Key) -> bool {
    matches!(key, Key::Named(NamedKey::Escape))
}

fn state_for(is_open: bool) -> MenuButtonInteractionState {
    if is_open {
        MenuButtonInteractionState::opened()
    } else {
        MenuButtonInteractionState::closed()
    }
}

fn close_intent_from_transition(transition: MenuButtonTransition) -> CloseIntent {
    match transition {
        MenuButtonTransition::Closed => CloseIntent::Close,
        MenuButtonTransition::Opened | MenuButtonTransition::Unchanged => CloseIntent::KeepOpen,
    }
}

fn close_handler(id: MenuButtonId) -> Option<Rc<dyn Fn()>> {
    CLOSE_HANDLERS.with(|handlers| {
        let mut handlers = handlers.borrow_mut();
        handlers.retain(|handler| handler.close.strong_count() > 0);
        handlers
            .iter()
            .find(|handler| handler.id == id)
            .and_then(|handler| handler.close.upgrade())
    })
}

#[cfg(test)]
pub(super) fn menu_id_for_test(id: u64) -> MenuButtonId {
    MenuButtonId(id)
}

#[cfg(test)]
pub(super) fn active_menu_for_test() -> Option<MenuButtonId> {
    ACTIVE_MENU_ID.with(|active| active.get())
}

#[cfg(test)]
pub(super) fn reset_menu_registry_for_test() {
    NEXT_MENU_ID.with(|next_id| next_id.set(1));
    ACTIVE_MENU_ID.with(|active| active.set(None));
    CLOSE_HANDLERS.with(|handlers| handlers.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_intent_only_closes_when_open() {
        assert_eq!(close_intent_for_escape(true), CloseIntent::Close);
        assert_eq!(close_intent_for_escape(false), CloseIntent::KeepOpen);
        assert_eq!(close_intent_for_trigger_press(true), CloseIntent::Close);
        assert_eq!(close_intent_for_trigger_press(false), CloseIntent::KeepOpen);
        assert_eq!(close_intent_for_outside_pointer(true), CloseIntent::Close);
        assert_eq!(
            close_intent_for_outside_pointer(false),
            CloseIntent::KeepOpen
        );
    }

    #[test]
    fn outside_pointer_is_a_close_gesture() {
        assert!(should_close_on_outside_pointer());
    }

    #[test]
    fn escape_key_is_a_close_gesture() {
        assert!(should_close_on_key(&Key::Named(NamedKey::Escape)));
        assert!(!should_close_on_key(&Key::Named(NamedKey::Enter)));
        assert_eq!(
            close_intent_for_key(&Key::Named(NamedKey::Escape), true),
            CloseIntent::Close
        );
        assert_eq!(
            close_intent_for_key(&Key::Named(NamedKey::Enter), true),
            CloseIntent::KeepOpen
        );
    }

    #[test]
    fn activating_new_menu_closes_previous_menu() {
        reset_menu_registry_for_test();
        let first_closed = Rc::new(Cell::new(false));
        let second_closed = Rc::new(Cell::new(false));
        let first_id = menu_id_for_test(1);
        let second_id = menu_id_for_test(2);
        let first_close: Rc<dyn Fn()> = {
            let first_closed = Rc::clone(&first_closed);
            Rc::new(move || first_closed.set(true))
        };
        let second_close: Rc<dyn Fn()> = {
            let second_closed = Rc::clone(&second_closed);
            Rc::new(move || second_closed.set(true))
        };

        register_menu(first_id, &first_close);
        register_menu(second_id, &second_close);
        activate_menu(first_id);
        activate_menu(second_id);

        assert!(first_closed.get());
        assert!(!second_closed.get());
        assert_eq!(active_menu_for_test(), Some(second_id));
    }
}
