use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSvgPaintPolicy {
    #[default]
    CurrentColor,
    StrokeOnly,
    FillOnly,
    StrokeAndFill,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiIconProps {
    pub svg_source: String,
    pub view_box: String,
    pub path_summary: String,
    pub paint_policy: UiSvgPaintPolicy,
    pub role: String,
    pub color_token: String,
    pub theme_token: String,
}

impl UiIconProps {
    #[must_use]
    pub fn new(svg_source: impl Into<String>) -> Self {
        Self {
            svg_source: svg_source.into(),
            view_box: String::new(),
            path_summary: String::new(),
            paint_policy: UiSvgPaintPolicy::CurrentColor,
            role: String::new(),
            color_token: String::new(),
            theme_token: String::new(),
        }
    }

    #[must_use]
    pub fn view_box(mut self, value: impl Into<String>) -> Self {
        self.view_box = value.into();
        self
    }

    #[must_use]
    pub fn path_summary(mut self, value: impl Into<String>) -> Self {
        self.path_summary = value.into();
        self
    }

    #[must_use]
    pub fn paint_policy(mut self, value: UiSvgPaintPolicy) -> Self {
        self.paint_policy = value;
        self
    }

    #[must_use]
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = value.into();
        self
    }

    #[must_use]
    pub fn color_token(mut self, value: impl Into<String>) -> Self {
        self.color_token = value.into();
        self
    }

    #[must_use]
    pub fn theme_token(mut self, value: impl Into<String>) -> Self {
        self.theme_token = value.into();
        self
    }
}
