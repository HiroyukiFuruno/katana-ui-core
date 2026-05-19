use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiDimension {
    #[default]
    Auto,
    Px(u16),
    Percent(u16),
    Fill,
    FitContent,
    Token(String),
}

impl UiDimension {
    #[must_use]
    pub const fn px(value: u16) -> Self {
        Self::Px(value)
    }

    #[must_use]
    pub const fn percent(value: u16) -> Self {
        Self::Percent(value)
    }

    #[must_use]
    pub fn token(value: impl Into<String>) -> Self {
        Self::Token(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiEdgeInsets {
    pub top: UiDimension,
    pub right: UiDimension,
    pub bottom: UiDimension,
    pub left: UiDimension,
}

impl UiEdgeInsets {
    #[must_use]
    pub fn all(value: UiDimension) -> Self {
        Self {
            top: value.clone(),
            right: value.clone(),
            bottom: value.clone(),
            left: value,
        }
    }

    #[must_use]
    pub fn axis(horizontal: UiDimension, vertical: UiDimension) -> Self {
        Self {
            top: vertical.clone(),
            right: horizontal.clone(),
            bottom: vertical,
            left: horizontal,
        }
    }
}

impl Default for UiEdgeInsets {
    fn default() -> Self {
        Self::all(UiDimension::Px(0))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiBorder {
    pub visible: bool,
    pub width_px: u16,
    pub radius_px: u16,
    pub color_token: String,
}

impl UiBorder {
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn solid(width_px: u16, radius_px: u16, color_token: impl Into<String>) -> Self {
        Self {
            visible: true,
            width_px,
            radius_px,
            color_token: color_token.into(),
        }
    }

    #[must_use]
    pub fn visible(mut self, value: bool) -> Self {
        self.visible = value;
        self
    }

    #[must_use]
    pub fn radius_px(mut self, value: u16) -> Self {
        self.radius_px = value;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiDisplay {
    #[default]
    Block,
    Inline,
    Flex,
    Grid,
    Contents,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiPosition {
    #[default]
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiAlignItems {
    Start,
    #[default]
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiJustifyContent {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiCursor {
    #[default]
    Default,
    Pointer,
    Text,
    Move,
    Grab,
    Resize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiPointerEvents {
    #[default]
    Auto,
    None,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiZIndex {
    #[default]
    Auto,
    Value(i32),
    Token(String),
}

impl UiZIndex {
    #[must_use]
    pub const fn value(value: i32) -> Self {
        Self::Value(value)
    }

    #[must_use]
    pub fn token(value: impl Into<String>) -> Self {
        Self::Token(value.into())
    }
}
