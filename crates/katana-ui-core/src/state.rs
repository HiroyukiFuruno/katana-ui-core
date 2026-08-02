use crate::render_model::{UiCommonProps, UiInteractionState, UiStateId};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct UiStateHandle<T> {
    state: Arc<RwLock<T>>,
}

impl<T> UiStateHandle<T> {
    #[must_use]
    pub fn new(initial_state: T) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial_state)),
        }
    }

    #[must_use]
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.with(Clone::clone)
    }

    pub fn set(&self, next_state: T) {
        let mut state = self.write_state();
        *state = next_state;
    }

    pub fn update(&self, update_state: impl FnOnce(&mut T)) {
        let mut state = self.write_state();
        update_state(&mut state);
    }

    pub fn with<R>(&self, read_state: impl FnOnce(&T) -> R) -> R {
        let state = self.read_state();
        read_state(&state)
    }

    fn read_state(&self) -> RwLockReadGuard<'_, T> {
        match self.state.read() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        }
    }

    fn write_state(&self) -> RwLockWriteGuard<'_, T> {
        match self.state.write() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        }
    }
}

impl<T> Clone for UiStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiComponentState {
    pub state_id: UiStateId,
    pub common: UiCommonProps,
    pub disabled: bool,
    pub focusable: bool,
    pub loading: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub checked: bool,
    pub interaction: UiInteractionState,
}

impl UiComponentState {
    #[must_use]
    pub fn new(state_id: UiStateId) -> Self {
        Self {
            state_id,
            common: UiCommonProps::default(),
            disabled: false,
            focusable: false,
            loading: false,
            readonly: false,
            invalid: false,
            checked: false,
            interaction: UiInteractionState::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UiComponentState, UiStateHandle};
    use crate::render_model::UiStateId;
    use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

    #[test]
    fn state_handle_shares_updates_and_recovers_poisoned_lock() {
        let handle = UiStateHandle::new(1usize);
        let clone = handle.clone();
        clone.set(2);
        handle.update(|value| *value += 3);
        assert_eq!(5, clone.get());
        assert_eq!(10, handle.with(|value| value * 2));

        let poisoned = handle.clone();
        let panic_result = catch_unwind(AssertUnwindSafe(|| {
            poisoned.update(|_| resume_unwind(Box::new("poison state lock")));
        }));
        assert!(panic_result.is_err());
        assert_eq!(5, handle.get());
        handle.set(8);
        assert_eq!(8, clone.get());
    }

    #[test]
    fn component_state_starts_with_neutral_flags() {
        let state = UiComponentState::new(UiStateId::new("component"));

        assert_eq!("component", state.state_id.as_str());
        assert!(!state.disabled);
        assert!(!state.focusable);
        assert!(!state.loading);
        assert!(!state.readonly);
        assert!(!state.invalid);
        assert!(!state.checked);
    }
}
