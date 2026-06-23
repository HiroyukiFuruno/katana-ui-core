use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AdapterActionBridge;

impl AdapterActionBridge {
    #[must_use]
    pub fn dispatch<C>(component: &mut C, action: &UiAction) -> UiActionResult
    where
        C: ComponentAction,
    {
        component.apply_action(action)
    }
}
