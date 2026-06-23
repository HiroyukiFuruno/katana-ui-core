use super::action::UiAction;
use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

macro_rules! target_action {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub target: UiStateId,
        }

        impl $name {
            #[must_use]
            pub fn new(target: UiStateId) -> Self {
                Self { target }
            }
        }
    };
}

target_action!(ButtonAction);
target_action!(ClickAction);
target_action!(RadioAction);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputAction {
    pub target: UiStateId,
    pub value: String,
}

impl InputAction {
    #[must_use]
    pub fn new(target: UiStateId, value: impl Into<String>) -> Self {
        Self {
            target,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlideAction {
    pub target: UiStateId,
    pub value: String,
}

impl SlideAction {
    #[must_use]
    pub fn new(target: UiStateId, value: impl Into<String>) -> Self {
        Self {
            target,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitPaneAction {
    pub target: UiStateId,
    pub percent: u8,
}

impl SplitPaneAction {
    #[must_use]
    pub const fn new(target: UiStateId, percent: u8) -> Self {
        Self { target, percent }
    }
}

macro_rules! checked_action {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub target: UiStateId,
            pub checked: bool,
        }

        impl $name {
            #[must_use]
            pub fn new(target: UiStateId, checked: bool) -> Self {
                Self { target, checked }
            }
        }
    };
}

checked_action!(CheckboxAction);
checked_action!(ToggleAction);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiHoverTarget {
    pub target: UiStateId,
    pub hovered: bool,
}

impl UiHoverTarget {
    #[must_use]
    pub const fn new(target: UiStateId, hovered: bool) -> Self {
        Self { target, hovered }
    }

    #[must_use]
    pub fn action(&self) -> UiAction {
        UiAction::hover(self.target.clone(), self.hovered)
    }
}
